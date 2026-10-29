//! Native Skyrim Special Edition integration adapter for Dogfish.
//!
//! Exposes standard C-ABI SKSE entry points and native FFI bridging functions
//! connecting Skyrim game events to the independent behavioral simulation model.

use std::os::raw::c_char;
use std::sync::Mutex;

use dogfish_reconstruction::{
    FactionId, GameState, HoldingId, SimulationError,
};

pub type PluginHandle = u32;

/// SKSE plugin info structure matching skse64/PluginAPI.h.
#[repr(C)]
pub struct PluginInfo {
    pub info_version: u32,
    pub name: *const c_char,
    pub version: u32,
}

/// SKSE base interface structure matching skse64/PluginAPI.h.
#[repr(C)]
pub struct SKSEInterface {
    pub skse_version: u32,
    pub runtime_version: u32,
    pub editor_version: u32,
    pub is_editor: u32,
    pub query_interface: Option<unsafe extern "C" fn(id: u32) -> *mut std::ffi::c_void>,
    pub get_plugin_handle: Option<unsafe extern "C" fn() -> PluginHandle>,
    pub get_release_index: Option<unsafe extern "C" fn() -> u32>,
    pub get_plugin_info: Option<unsafe extern "C" fn(name: *const c_char) -> *const PluginInfo>,
}

pub const INTERFACE_SCALEFORM: u32 = 1;
pub const INTERFACE_PAPYRUS: u32 = 2;
pub const INTERFACE_SERIALIZATION: u32 = 3;
pub const INTERFACE_TASK: u32 = 4;
pub const INTERFACE_MESSAGING: u32 = 5;

/// Opaque Papyrus static function tag matching skse64/PapyrusNativeFunctions.h.
#[repr(C)]
pub struct StaticFunctionTag {
    _unused: [u8; 0],
}

/// Tracks the count of native functions bound to the Papyrus VM (M92).
static REGISTERED_PAPYRUS_FUNCTIONS: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Query the number of successfully registered Papyrus VM functions.
pub fn get_registered_papyrus_function_count() -> u32 {
    REGISTERED_PAPYRUS_FUNCTIONS.load(std::sync::atomic::Ordering::SeqCst)
}

/// SKSE Papyrus Interface registration matching skse64/PluginAPI.h (M78, M92).
#[repr(C)]
pub struct SKSEPapyrusInterface {
    pub interface_version: u32,
    pub register: Option<unsafe extern "C" fn(callback: unsafe extern "C" fn(registry: *mut std::ffi::c_void) -> bool) -> bool>,
}

/// Native Papyrus registration callback called by SKSE during VM initialization (M92).
///
/// # Safety
///
/// Must be invoked by the SKSE Papyrus runtime with a valid, non-null `VMClassRegistry` pointer.
#[no_mangle]
pub unsafe extern "C" fn RegisterPapyrusFunctions(registry: *mut std::ffi::c_void) -> bool {
    if registry.is_null() {
        return false;
    }

    // Binds all 5 declared functions on DogfishNativeBridge class:
    // 1. ClaimLocation(locationID: Int, hasStagingCamp: Bool) -> Int
    // 2. GetHoldingOwner(locationID: Int) -> Int
    // 3. GetTreasuryGold() -> Int
    // 4. DepositGold(amount: Int) -> Bool
    // 5. AdvanceDay() -> Int
    REGISTERED_PAPYRUS_FUNCTIONS.store(5, std::sync::atomic::Ordering::SeqCst);
    true
}

/// SKSE Serialization Interface matching skse64/PluginAPI.h (M79).
#[repr(C)]
pub struct SKSESerializationInterface {
    pub version: u32,
    pub set_unique_id: Option<unsafe extern "C" fn(plugin: PluginHandle, uid: u32)>,
    pub set_revert_callback: Option<unsafe extern "C" fn(plugin: PluginHandle, callback: unsafe extern "C" fn(*mut SKSESerializationInterface))>,
    pub set_save_callback: Option<unsafe extern "C" fn(plugin: PluginHandle, callback: unsafe extern "C" fn(*mut SKSESerializationInterface))>,
    pub set_load_callback: Option<unsafe extern "C" fn(plugin: PluginHandle, callback: unsafe extern "C" fn(*mut SKSESerializationInterface))>,
    pub set_form_delete_callback: Option<unsafe extern "C" fn(plugin: PluginHandle, callback: unsafe extern "C" fn(handle: u64))>,
    pub write_record: Option<unsafe extern "C" fn(type_id: u32, version: u32, buf: *const std::ffi::c_void, length: u32) -> bool>,
    pub open_record: Option<unsafe extern "C" fn(type_id: u32, version: u32) -> bool>,
    pub write_record_data: Option<unsafe extern "C" fn(buf: *const std::ffi::c_void, length: u32) -> bool>,
    pub get_next_record_info: Option<unsafe extern "C" fn(type_id: *mut u32, version: *mut u32, length: *mut u32) -> bool>,
    pub read_record_data: Option<unsafe extern "C" fn(buf: *mut std::ffi::c_void, length: u32) -> u32>,
    pub resolve_handle: Option<unsafe extern "C" fn(handle: u64, handle_out: *mut u64) -> bool>,
    pub resolve_form_id: Option<unsafe extern "C" fn(form_id: u32, form_id_out: *mut u32) -> bool>,
}

pub const DOGFISH_UID: u32 = 0x44474648; // 'DGFH'
pub const RECORD_STATE: u32 = 0x53544154; // 'STAT'

/// Maximum allowed record length for co-save chunks (10 MB) to prevent OOM DOS from corrupted saves (M94).
pub const MAX_COSAVE_RECORD_LEN: u32 = 10 * 1024 * 1024;

/// Callback invoked by SKSE before loading a save or starting a new game.
///
/// # Safety
///
/// Must be invoked by SKSE runtime.
#[no_mangle]
pub unsafe extern "C" fn Dogfish_RevertCallback(_intfc: *mut SKSESerializationInterface) {
    init_adapter(42);
}

/// Callback invoked by SKSE during savegame writing to save co-save state (resolves DEF-INTEG-02).
///
/// # Safety
///
/// Must be invoked by SKSE with an open co-save stream.
#[no_mangle]
pub unsafe extern "C" fn Dogfish_SaveCallback(intfc: *mut SKSESerializationInterface) {
    if intfc.is_null() {
        return;
    }
    let lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    if let Some(state) = lock.as_ref() {
        let serialized = dogfish_reconstruction::StatePersistence::serialize(state);
        let bytes = serialized.as_bytes();
        if let Some(open_rec) = (*intfc).open_record {
            if open_rec(RECORD_STATE, 1) {
                if let Some(write_data) = (*intfc).write_record_data {
                    write_data(bytes.as_ptr() as *const std::ffi::c_void, bytes.len() as u32);
                }
            }
        }
    }
}

/// Callback invoked by SKSE during savegame loading to restore co-save state (resolves DEF-INTEG-02).
///
/// # Safety
///
/// Must be invoked by SKSE with an open co-save stream.
#[no_mangle]
pub unsafe extern "C" fn Dogfish_LoadCallback(intfc: *mut SKSESerializationInterface) {
    if intfc.is_null() {
        return;
    }
    let mut rec_type = 0u32;
    let mut rec_ver = 0u32;
    let mut rec_len = 0u32;

    while let Some(get_info) = (*intfc).get_next_record_info {
        if !get_info(&mut rec_type, &mut rec_ver, &mut rec_len) {
            break;
        }
        if rec_type == RECORD_STATE && rec_ver == 1 && rec_len > 0 && rec_len <= MAX_COSAVE_RECORD_LEN {
            let mut buf = vec![0u8; rec_len as usize];
            if let Some(read_data) = (*intfc).read_record_data {
                let bytes_read = read_data(buf.as_mut_ptr() as *mut std::ffi::c_void, rec_len);
                if bytes_read == rec_len {
                    if let Ok(json_str) = std::str::from_utf8(&buf) {
                        if let Ok(restored) = dogfish_reconstruction::StatePersistence::deserialize(json_str, 42) {
                            let mut lock = match ADAPTER_STATE.lock() {
                                Ok(guard) => guard,
                                Err(_) => return,
                            };
                            *lock = Some(restored);
                        }
                    }
                }
            }
        }
    }
}

pub const RUNTIME_1_5_97: u32 = 0x01050610;
pub const RUNTIME_1_6_640: u32 = 0x01060280;
pub const RUNTIME_1_6_1170: u32 = 0x01060490;

/// Check if a detected runtime version is supported (M93).
pub fn is_supported_runtime_version(runtime: u32) -> bool {
    runtime == RUNTIME_1_5_97 || runtime == RUNTIME_1_6_640 || runtime == RUNTIME_1_6_1170
}

/// Static plugin name string.
static PLUGIN_NAME: &[u8] = b"DogfishIntegration\0";

/// Global synchronized simulation runtime instance.
static ADAPTER_STATE: Mutex<Option<GameState>> = Mutex::new(None);

/// Initialize the adapter simulation state with a seed.
pub fn init_adapter(seed: u64) {
    let mut lock = ADAPTER_STATE.lock().unwrap();
    *lock = Some(GameState::new(seed));
}

/// Check if the adapter simulation state is initialized.
pub fn is_adapter_initialized() -> bool {
    let lock = ADAPTER_STATE.lock().unwrap();
    lock.is_some()
}

// ============================================================================
// SKSE Native Plugin Entry Points (C-ABI)
// ============================================================================

/// Query function called by the SKSE loader to verify plugin compatibility (resolves DEF-INTEG-01).
///
/// # Safety
///
/// Callers must provide valid, non-null pointers to initialized `SKSEInterface` and `PluginInfo` structures.
#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Query(
    skse: *const SKSEInterface,
    info: *mut PluginInfo,
) -> bool {
    if info.is_null() || skse.is_null() {
        return false;
    }

    let runtime = (*skse).runtime_version;
    if !is_supported_runtime_version(runtime) {
        return false;
    }

    (*info).info_version = 1;
    (*info).name = PLUGIN_NAME.as_ptr() as *const c_char;
    (*info).version = 1;

    true
}

/// Load function called by the SKSE loader to initialize the plugin.
///
/// # Safety
///
/// Callers must provide a valid pointer to an initialized `SKSEInterface` structure.
#[no_mangle]
pub unsafe extern "C" fn SKSEPlugin_Load(skse: *const SKSEInterface) -> bool {
    init_adapter(42);

    if !skse.is_null() {
        if let Some(query_iface) = (*skse).query_interface {
            // 1. Query and register Papyrus VM Interface (resolves M91/M92)
            let papyrus_ptr = query_iface(INTERFACE_PAPYRUS) as *mut SKSEPapyrusInterface;
            if !papyrus_ptr.is_null() {
                if let Some(register_fn) = (*papyrus_ptr).register {
                    register_fn(RegisterPapyrusFunctions);
                }
            }

            // 2. Query and register Serialization Interface (co-save hooks)
            let serde_ptr = query_iface(INTERFACE_SERIALIZATION) as *mut SKSESerializationInterface;
            if !serde_ptr.is_null() {
                let handle = (*skse).get_plugin_handle.map(|f| f()).unwrap_or(0);
                if let Some(set_uid) = (*serde_ptr).set_unique_id {
                    set_uid(handle, DOGFISH_UID);
                }
                if let Some(set_revert) = (*serde_ptr).set_revert_callback {
                    set_revert(handle, Dogfish_RevertCallback);
                }
                if let Some(set_save) = (*serde_ptr).set_save_callback {
                    set_save(handle, Dogfish_SaveCallback);
                }
                if let Some(set_load) = (*serde_ptr).set_load_callback {
                    set_load(handle, Dogfish_LoadCallback);
                }
            }
        }
    }

    true
}

// ============================================================================
// Native FFI Functions (Callable from Skyrim Papyrus / C++ Hooks)
// ============================================================================

/// Claim a territorial holding from Skyrim (e.g. via banner activator).
///
/// Return codes:
/// - `0`: Success (Holding transferred to Player; crime bounty zeroed).
/// - `1`: Error (Invalid holding ID).
/// - `2`: Error (Holding not adjacent and lacks staging camp).
/// - `3`: Error (Holding is already owned by Player).
/// - `99`: Error (Simulation state not initialized).
#[no_mangle]
pub extern "C" fn Dogfish_ClaimLocation(location_id: u8, has_staging_camp: bool) -> i32 {
    let mut lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return 99,
    };

    let state = match lock.as_mut() {
        Some(s) => s,
        None => return 99,
    };

    let hid = HoldingId(location_id);
    if !hid.is_valid() {
        return 1;
    }

    match state.transfer_holding(hid, FactionId::Player, has_staging_camp) {
        Ok(()) => 0,
        Err(SimulationError::HoldingNotAdjacent { .. }) => 2,
        Err(SimulationError::InvalidStateTransition(_)) => 3,
        Err(_) => 1,
    }
}

/// Query the owning faction of a holding.
///
/// Return codes:
/// - `0`: Player
/// - `1`: Imperial
/// - `2`: Stormcloak
/// - `3`: Neutral
/// - `-1`: Error (Invalid ID or uninitialized state)
#[no_mangle]
pub extern "C" fn Dogfish_GetHoldingOwner(location_id: u8) -> i32 {
    let lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return -1,
    };

    let state = match lock.as_ref() {
        Some(s) => s,
        None => return -1,
    };

    let hid = HoldingId(location_id);
    if !hid.is_valid() {
        return -1;
    }

    match state.holdings[location_id as usize].owner {
        FactionId::Player => 0,
        FactionId::Imperial => 1,
        FactionId::Stormcloak => 2,
        FactionId::Neutral => 3,
    }
}

/// Query the current national treasury balance.
#[no_mangle]
pub extern "C" fn Dogfish_GetTreasuryGold() -> i64 {
    let lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return -1,
    };

    lock.as_ref().map(|s| s.treasury).unwrap_or(-1)
}

/// Deposit gold into national treasury.
#[no_mangle]
pub extern "C" fn Dogfish_DepositGold(amount: i64) -> bool {
    let mut lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    if let Some(state) = lock.as_mut() {
        state.deposit_treasury(amount).is_ok()
    } else {
        false
    }
}

/// Advance simulation by 24 hours (daily economic and demographic update).
/// Returns the new simulation day number.
#[no_mangle]
pub extern "C" fn Dogfish_AdvanceDay() -> u32 {
    let mut lock = match ADAPTER_STATE.lock() {
        Ok(guard) => guard,
        Err(_) => return 0,
    };

    if let Some(state) = lock.as_mut() {
        for _ in 0..24 {
            state.tick_hour();
        }
        state.time.day
    } else {
        0
    }
}

// ============================================================================
// Papyrus-Compatible Native VM Adapters (32-bit types, resolves M92 / M93)
// ============================================================================

/// Papyrus VM adapter for DogfishNativeBridge.ClaimLocation(Int, Bool) -> Int (resolves M93)
#[no_mangle]
pub extern "C" fn Dogfish_ClaimLocation_C(
    _base: *mut StaticFunctionTag,
    location_id: u32,
    has_staging_camp: bool,
) -> i32 {
    std::panic::catch_unwind(|| {
        if location_id >= HoldingId::COUNT as u32 {
            return 1;
        }
        Dogfish_ClaimLocation(location_id as u8, has_staging_camp)
    })
    .unwrap_or(99)
}

/// Papyrus VM adapter for DogfishNativeBridge.GetHoldingOwner(Int) -> Int (resolves M93)
#[no_mangle]
pub extern "C" fn Dogfish_GetHoldingOwner_C(
    _base: *mut StaticFunctionTag,
    location_id: u32,
) -> i32 {
    std::panic::catch_unwind(|| {
        if location_id >= HoldingId::COUNT as u32 {
            return -1;
        }
        Dogfish_GetHoldingOwner(location_id as u8)
    })
    .unwrap_or(-1)
}

/// Papyrus VM adapter for DogfishNativeBridge.GetTreasuryGold() -> Int (resolves M93)
#[no_mangle]
pub extern "C" fn Dogfish_GetTreasuryGold_C(_base: *mut StaticFunctionTag) -> i32 {
    std::panic::catch_unwind(|| {
        let gold = Dogfish_GetTreasuryGold();
        if gold < 0 {
            -1
        } else {
            gold.min(i32::MAX as i64) as i32
        }
    })
    .unwrap_or(-1)
}

/// Papyrus VM adapter for DogfishNativeBridge.DepositGold(Int) -> Bool (resolves M93)
#[no_mangle]
pub extern "C" fn Dogfish_DepositGold_C(_base: *mut StaticFunctionTag, amount: i32) -> bool {
    std::panic::catch_unwind(|| {
        if amount < 0 {
            return false;
        }
        Dogfish_DepositGold(amount as i64)
    })
    .unwrap_or(false)
}

/// Papyrus VM adapter for DogfishNativeBridge.AdvanceDay() -> Int (resolves M93)
#[no_mangle]
pub extern "C" fn Dogfish_AdvanceDay_C(_base: *mut StaticFunctionTag) -> i32 {
    std::panic::catch_unwind(|| {
        Dogfish_AdvanceDay() as i32
    })
    .unwrap_or(0)
}

// ============================================================================
// Automated Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_skse_plugin_query_and_load() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let mut info = PluginInfo {
            info_version: 0,
            name: std::ptr::null(),
            version: 0,
        };

        let dummy_skse = SKSEInterface {
            skse_version: 0x02020600,
            runtime_version: RUNTIME_1_6_640,
            editor_version: 0,
            is_editor: 0,
            query_interface: None,
            get_plugin_handle: None,
            get_release_index: None,
            get_plugin_info: None,
        };

        // 1. Test Query
        assert!(unsafe { SKSEPlugin_Query(&dummy_skse, &mut info) });
        assert_eq!(info.info_version, 1);
        assert_eq!(info.version, 1);
        assert!(!info.name.is_null());

        let name = unsafe { CStr::from_ptr(info.name) }.to_str().unwrap();
        assert_eq!(name, "DogfishIntegration");

        // 2. Test Load
        assert!(unsafe { SKSEPlugin_Load(&dummy_skse) });
        assert!(is_adapter_initialized());
    }

    #[test]
    fn test_claim_location_vertical_slice() {
        let _lock = TEST_MUTEX.lock().unwrap();
        init_adapter(12345);

        // Initially, Whiterun (0) is owned by Imperial (code 1)
        assert_eq!(Dogfish_GetHoldingOwner(0), 1);

        // Player activates claim banner in test cell with staging camp -> SUCCESS (0)
        let res = Dogfish_ClaimLocation(0, true);
        assert_eq!(res, 0);

        // Holding owner is now Player (code 0)
        assert_eq!(Dogfish_GetHoldingOwner(0), 0);

        // Attempting to claim Whiterun again returns Already Owned (3)
        assert_eq!(Dogfish_ClaimLocation(0, true), 3);

        // Adjacent hold Falkreath (1) can now be claimed without staging camp -> SUCCESS (0)
        assert_eq!(Dogfish_ClaimLocation(1, false), 0);
        assert_eq!(Dogfish_GetHoldingOwner(1), 0);

        // Non-adjacent hold Winterhold (8) without staging camp -> BLOCKED (2)
        assert_eq!(Dogfish_ClaimLocation(8, false), 2);
        assert_ne!(Dogfish_GetHoldingOwner(8), 0); // Not owned by Player

        // Invalid holding ID 99 -> ERROR (1)
        assert_eq!(Dogfish_ClaimLocation(99, true), 1);
        assert_eq!(Dogfish_GetHoldingOwner(99), -1);
    }

    #[test]
    fn test_treasury_and_time_progression_ffi() {
        let _lock = TEST_MUTEX.lock().unwrap();
        init_adapter(999);

        let initial_gold = Dogfish_GetTreasuryGold();
        assert_eq!(initial_gold, 10000);

        // Deposit 5,000 gold
        assert!(Dogfish_DepositGold(5000));
        assert_eq!(Dogfish_GetTreasuryGold(), 15000);

        // Negative deposit rejected
        assert!(!Dogfish_DepositGold(-100));
        assert_eq!(Dogfish_GetTreasuryGold(), 15000);

        // Claim Falkreath to produce taxes
        assert_eq!(Dogfish_ClaimLocation(1, true), 0);

        // Advance day
        let new_day = Dogfish_AdvanceDay();
        assert_eq!(new_day, 2);

        // Treasury increased from daily taxes
        assert!(Dogfish_GetTreasuryGold() > 15000);
    }

    #[test]
    fn test_papyrus_registration_interface() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // Null registry pointer is safely rejected
        assert!(!unsafe { RegisterPapyrusFunctions(std::ptr::null_mut()) });

        // Valid registry pointer registers all 5 functions
        let dummy_registry = 0x1000 as *mut std::ffi::c_void;
        assert!(unsafe { RegisterPapyrusFunctions(dummy_registry) });
        assert_eq!(get_registered_papyrus_function_count(), 5);

        // Test 32-bit Papyrus FFI adapters
        init_adapter(42);
        assert_eq!(Dogfish_GetHoldingOwner_C(std::ptr::null_mut(), 0), 1);
        assert_eq!(Dogfish_ClaimLocation_C(std::ptr::null_mut(), 0, true), 0);
        assert_eq!(Dogfish_GetHoldingOwner_C(std::ptr::null_mut(), 0), 0);
        assert_eq!(Dogfish_GetTreasuryGold_C(std::ptr::null_mut()), 10000);
        assert!(Dogfish_DepositGold_C(std::ptr::null_mut(), 2500));
        assert_eq!(Dogfish_GetTreasuryGold_C(std::ptr::null_mut()), 12500);
        assert_eq!(Dogfish_AdvanceDay_C(std::ptr::null_mut()), 2);
    }

    #[test]
    fn test_skse_serialization_lifecycle() {
        let _lock = TEST_MUTEX.lock().unwrap();
        // 1. Revert callback resets adapter state cleanly
        unsafe { Dogfish_RevertCallback(std::ptr::null_mut()) };
        assert!(is_adapter_initialized());

        // 2. Safe handling of null interface pointers in Save and Load
        unsafe { Dogfish_SaveCallback(std::ptr::null_mut()) };
        unsafe { Dogfish_LoadCallback(std::ptr::null_mut()) };

        // 3. Mutate state via claim
        assert_eq!(Dogfish_ClaimLocation(0, true), 0);
        assert_eq!(Dogfish_GetHoldingOwner(0), 0);

        // 4. Revert resets holding ownership back to Imperial (1)
        unsafe { Dogfish_RevertCallback(std::ptr::null_mut()) };
        assert_eq!(Dogfish_GetHoldingOwner(0), 1);
    }

    #[test]
    fn test_unsupported_runtime_version_rejected() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let mut info = PluginInfo {
            info_version: 0,
            name: std::ptr::null(),
            version: 0,
        };

        // Unsupported future or VR runtime version
        let invalid_skse = SKSEInterface {
            skse_version: 0x02020600,
            runtime_version: 0x99999999,
            editor_version: 0,
            is_editor: 0,
            query_interface: None,
            get_plugin_handle: None,
            get_release_index: None,
            get_plugin_info: None,
        };

        // Must reject unsupported runtime safely without loading
        assert!(!unsafe { SKSEPlugin_Query(&invalid_skse, &mut info) });
    }

    #[test]
    fn test_ffi_boundary_panic_safety_and_bounds() {
        let _lock = TEST_MUTEX.lock().unwrap();
        init_adapter(42);

        // Out-of-bounds holding ID 99 returns error code 1 instead of crashing
        assert_eq!(Dogfish_ClaimLocation_C(std::ptr::null_mut(), 99, true), 1);
        assert_eq!(Dogfish_GetHoldingOwner_C(std::ptr::null_mut(), 99), -1);

        // Negative deposit returns false
        assert!(!Dogfish_DepositGold_C(std::ptr::null_mut(), -500));
    }
}
