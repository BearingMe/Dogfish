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
    if runtime == 0 {
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
pub unsafe extern "C" fn SKSEPlugin_Load(_skse: *const SKSEInterface) -> bool {
    init_adapter(42);
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
// Automated Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_skse_plugin_query_and_load() {
        let mut info = PluginInfo {
            info_version: 0,
            name: std::ptr::null(),
            version: 0,
        };

        let dummy_skse = SKSEInterface {
            skse_version: 0x02020600,
            runtime_version: 0x01060470,
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
}
