//! Integration Defect Verification & Regression Suite
//!
//! Milestone M91 & M92: Verifies that native integration defects are properly resolved:
//! 1. SKSEPlugin_Load queries INTERFACE_PAPYRUS and registers the Papyrus callback.
//! 2. RegisterPapyrusFunctions binds all 5 declared functions on DogfishNativeBridge.
//! 3. Papyrus 32-bit integer FFI adapters (Dogfish_*_C) prevent 64-bit ABI register corruption.

use std::sync::atomic::{AtomicBool, Ordering};
use dogfish_integration::{
    get_registered_papyrus_function_count, SKSEInterface, SKSEPapyrusInterface,
    SKSEPlugin_Load, INTERFACE_PAPYRUS, INTERFACE_SERIALIZATION,
};

static PAPYRUS_QUERIED: AtomicBool = AtomicBool::new(false);
static SERIALIZATION_QUERIED: AtomicBool = AtomicBool::new(false);
static PAPYRUS_CALLBACK_REGISTERED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn mock_get_handle() -> u32 {
    1
}

unsafe extern "C" fn mock_get_release() -> u32 {
    1
}

unsafe extern "C" fn mock_papyrus_register(
    cb: unsafe extern "C" fn(*mut std::ffi::c_void) -> bool,
) -> bool {
    PAPYRUS_CALLBACK_REGISTERED.store(true, Ordering::SeqCst);
    // Invoke the callback with a dummy valid pointer representing VMClassRegistry
    cb(0x2000 as *mut std::ffi::c_void)
}

static MOCK_PAPYRUS_INTERFACE: SKSEPapyrusInterface = SKSEPapyrusInterface {
    interface_version: 1,
    register: Some(mock_papyrus_register),
};

unsafe extern "C" fn mock_query_interface(id: u32) -> *mut std::ffi::c_void {
    match id {
        INTERFACE_PAPYRUS => {
            PAPYRUS_QUERIED.store(true, Ordering::SeqCst);
            &MOCK_PAPYRUS_INTERFACE as *const _ as *mut std::ffi::c_void
        }
        INTERFACE_SERIALIZATION => {
            SERIALIZATION_QUERIED.store(true, Ordering::SeqCst);
            std::ptr::null_mut()
        }
        _ => std::ptr::null_mut(),
    }
}

/// M91/M92 REGRESSION: SKSEPlugin_Load must query INTERFACE_PAPYRUS and register callback.
#[test]
fn test_papyrus_interface_registration_lifecycle() {
    PAPYRUS_QUERIED.store(false, Ordering::SeqCst);
    SERIALIZATION_QUERIED.store(false, Ordering::SeqCst);
    PAPYRUS_CALLBACK_REGISTERED.store(false, Ordering::SeqCst);

    let mock_skse = SKSEInterface {
        skse_version: 0x02020600,
        runtime_version: 0x01060280,
        editor_version: 0,
        is_editor: 0,
        query_interface: Some(mock_query_interface),
        get_plugin_handle: Some(mock_get_handle),
        get_release_index: Some(mock_get_release),
        get_plugin_info: None,
    };

    let loaded = unsafe { SKSEPlugin_Load(&mock_skse) };
    assert!(loaded);

    // Both serialization and Papyrus interfaces must be queried
    assert!(SERIALIZATION_QUERIED.load(Ordering::SeqCst));
    assert!(PAPYRUS_QUERIED.load(Ordering::SeqCst));

    // Callback must be registered and invoked
    assert!(PAPYRUS_CALLBACK_REGISTERED.load(Ordering::SeqCst));
    assert_eq!(get_registered_papyrus_function_count(), 5);
}

/// M92 REGRESSION: RegisterPapyrusFunctions must safely reject null and register all 5 functions.
#[test]
fn test_register_papyrus_functions_safety_and_binding() {
    // Null pointer is safely rejected without crash
    assert!(!unsafe { dogfish_integration::RegisterPapyrusFunctions(std::ptr::null_mut()) });

    // Non-null registry pointer registers all 5 functions
    let dummy_registry = 0xCAFE as *mut std::ffi::c_void;
    let result = unsafe { dogfish_integration::RegisterPapyrusFunctions(dummy_registry) };
    assert!(result);
    assert_eq!(get_registered_papyrus_function_count(), 5);
}

/// M92/M93 REGRESSION: 32-bit Papyrus VM FFI adapter signatures.
#[test]
fn test_papyrus_32bit_vm_adapter_signatures() {
    dogfish_integration::init_adapter(42);

    // Papyrus VM SInt32 is 4 bytes
    assert_eq!(std::mem::size_of::<i32>(), 4);

    // Test that the C-ABI VM adapter returns i32 (4 bytes), preventing 64-bit register corruption
    let treasury_res = dogfish_integration::Dogfish_GetTreasuryGold_C(std::ptr::null_mut());
    assert_eq!(std::mem::size_of_val(&treasury_res), 4);
    assert_eq!(treasury_res, 10000);

    // Test deposit adapter
    assert!(dogfish_integration::Dogfish_DepositGold_C(std::ptr::null_mut(), 1000));
    assert_eq!(dogfish_integration::Dogfish_GetTreasuryGold_C(std::ptr::null_mut()), 11000);

    // Test claim adapter
    assert_eq!(dogfish_integration::Dogfish_ClaimLocation_C(std::ptr::null_mut(), 0, true), 0);
    assert_eq!(dogfish_integration::Dogfish_GetHoldingOwner_C(std::ptr::null_mut(), 0), 0);
}
