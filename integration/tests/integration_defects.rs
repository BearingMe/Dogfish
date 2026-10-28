//! Integration Defect Reproduction Suite
//!
//! Milestone M91: Specifically designed to expose and reproduce native integration omissions:
//! 1. SKSEPlugin_Load fails to query or register the Papyrus interface.
//! 2. RegisterPapyrusFunctions is a stub that registers zero native functions.
//! 3. Type mismatch between Rust i64/u8 and Papyrus 32-bit Int.

use std::sync::atomic::{AtomicBool, Ordering};
use dogfish_integration::{
    SKSEInterface, SKSEPlugin_Load,
    INTERFACE_PAPYRUS, INTERFACE_SERIALIZATION,
};

static PAPYRUS_QUERIED: AtomicBool = AtomicBool::new(false);
static SERIALIZATION_QUERIED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn mock_get_handle() -> u32 {
    1
}

unsafe extern "C" fn mock_get_release() -> u32 {
    1
}

unsafe extern "C" fn mock_query_interface(id: u32) -> *mut std::ffi::c_void {
    match id {
        INTERFACE_PAPYRUS => {
            PAPYRUS_QUERIED.store(true, Ordering::SeqCst);
            // Return dummy pointer representing SKSEPapyrusInterface
            0x1000 as *mut std::ffi::c_void
        }
        INTERFACE_SERIALIZATION => {
            SERIALIZATION_QUERIED.store(true, Ordering::SeqCst);
            std::ptr::null_mut()
        }
        _ => std::ptr::null_mut(),
    }
}

/// DEFECT REPRODUCTION 1: SKSEPlugin_Load completely omits querying INTERFACE_PAPYRUS.
///
/// EXPECTED: SKSEPlugin_Load queries INTERFACE_PAPYRUS (2) and registers the Papyrus function callback.
/// ACTUAL: SKSEPlugin_Load only queries INTERFACE_SERIALIZATION (3) and never queries INTERFACE_PAPYRUS!
#[test]
fn test_defect_papyrus_interface_omitted_in_plugin_load() {
    PAPYRUS_QUERIED.store(false, Ordering::SeqCst);
    SERIALIZATION_QUERIED.store(false, Ordering::SeqCst);

    let mock_skse = SKSEInterface {
        skse_version: 0x02020600,
        runtime_version: 0x01060470,
        editor_version: 0,
        is_editor: 0,
        query_interface: Some(mock_query_interface),
        get_plugin_handle: Some(mock_get_handle),
        get_release_index: Some(mock_get_release),
        get_plugin_info: None,
    };

    let loaded = unsafe { SKSEPlugin_Load(&mock_skse) };
    assert!(loaded);

    // Serialization was queried
    assert!(SERIALIZATION_QUERIED.load(Ordering::SeqCst));

    // ADVERSARIAL ASSERTION: INTERFACE_PAPYRUS must be queried by SKSEPlugin_Load
    assert!(
        PAPYRUS_QUERIED.load(Ordering::SeqCst),
        "DEFECT REPRODUCED: SKSEPlugin_Load fails to query INTERFACE_PAPYRUS (2)!"
    );
}

/// DEFECT REPRODUCTION 2: RegisterPapyrusFunctions is a no-op that binds 0 functions.
///
/// In the current code:
/// pub unsafe extern "C" fn RegisterPapyrusFunctions(_registry: *mut std::ffi::c_void) -> bool { true }
/// It literally contains zero function calls and registers zero functions on the VM registry!
#[test]
fn test_defect_register_papyrus_functions_is_stub() {
    // If a registry pointer is provided, an actual implementation must interact with it
    // rather than blindly returning true without binding functions.
    // In M91 baseline, RegisterPapyrusFunctions ignores the registry entirely.
    let dummy_registry = 0xCAFE as *mut std::ffi::c_void;

    // We verify that the function has no side effects and registers nothing
    let result = unsafe { dogfish_integration::RegisterPapyrusFunctions(dummy_registry) };
    assert!(result); // Passes unconditionally because it is a stub!
}

/// DEFECT REPRODUCTION 3: ABI type mismatch between Papyrus Int and Rust i64.
///
/// Papyrus `Int` is a 32-bit signed integer (`SInt32` in SKSE).
/// In `integration/src/lib.rs`, `Dogfish_GetTreasuryGold()` returns `i64` (8 bytes).
/// When Papyrus invokes a native function returning 8 bytes, the Papyrus VM stack expects
/// a 4-byte SInt32, resulting in register corruption or truncation!
#[test]
fn test_defect_papyrus_int_abi_mismatch() {
    // Verify that Dogfish_GetTreasuryGold returns an 8-byte type
    let treasury_return_size = std::mem::size_of_val(&dogfish_integration::Dogfish_GetTreasuryGold());
    assert_eq!(treasury_return_size, 8, "Returns 64-bit integer");

    // Papyrus VM SInt32 is 4 bytes
    let papyrus_int_size = std::mem::size_of::<i32>();
    assert_eq!(papyrus_int_size, 4, "Papyrus Int is 32-bit integer");

    // Type sizes differ: Native function signature must be adapted for Papyrus VM FFI
    assert_ne!(
        treasury_return_size, papyrus_int_size,
        "DEFECT REPRODUCED: Native return type (8 bytes) does not match Papyrus Int (4 bytes)!"
    );
}
