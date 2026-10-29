# Native Integration Audit & Defect Reproduction Report

- **Document**: Technical Audit & Defect Reproduction of Skyrim Native Bridge
- **Milestone**: M91 — Integration Defect Reproduction
- **Phase**: Phase 24 (Native Integration Correctness)
- **Subject**: Audit of `integration/src/lib.rs`, `integration/scripts/source/`, and SKSE Headers
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & DEFECTS REPRODUCED (Failing Regression Test Committed)

---

## 1. Executive Summary & Purpose

Milestone M91 investigates the actual runtime integration mechanisms in `integration/src/lib.rs`, assessing why previous completion claims for M78 and M79 were premature and establishing reproducible test failures for all identified omissions.

### Core Discoveries:
1. **Total Omission of Papyrus Registration in `SKSEPlugin_Load`**:
   `SKSEPlugin_Load` queries `INTERFACE_SERIALIZATION` (3) for co-save callbacks, but **never queries `INTERFACE_PAPYRUS` (2)** and never registers `RegisterPapyrusFunctions`. In a live Skyrim engine session, SKSE would never invoke the plugin's Papyrus registration routine.
2. **`RegisterPapyrusFunctions` is a Non-Functional Stub**:
   The existing implementation in `integration/src/lib.rs:55-63` merely contains comment lines and `true;`. It makes zero calls to the Papyrus virtual machine and binds zero functions.
3. **Existing Integration Tests Contained False Positives**:
   `test_papyrus_registration_interface` passed solely because the function returned `true` unconditionally on a `null` pointer.
4. **ABI Data Type Mismatches**:
   `Dogfish_GetTreasuryGold()` returns a 64-bit integer (`i64` / 8 bytes), whereas Skyrim's Papyrus VM stack frame treats `Int` as a 32-bit signed integer (`SInt32` / 4 bytes). Calling this from Papyrus would corrupt VM stack registers.

---

## 2. Granular Investigation of Integration Defects

### 2.1 Defect A: `SKSEPlugin_Load` Fails to Query Papyrus Interface
- **Source Inspection (`integration/src/lib.rs:215-238`)**:
  ```rust
  #[no_mangle]
  pub unsafe extern "C" fn SKSEPlugin_Load(skse: *const SKSEInterface) -> bool {
      init_adapter(42);
      if !skse.is_null() {
          if let Some(query_iface) = (*skse).query_interface {
              let serde_ptr = query_iface(INTERFACE_SERIALIZATION) ...
              // COMPLETELY MISSING: query_iface(INTERFACE_PAPYRUS)!
          }
      }
      true
  }
  ```
- **Reproduction**:
  Authored test `test_defect_papyrus_interface_omitted_in_plugin_load` in `integration/tests/integration_defects.rs`.
- **Observed Result**:
  ```text
  ---- test_defect_papyrus_interface_omitted_in_plugin_load stdout ----
  thread 'test_defect_papyrus_interface_omitted_in_plugin_load' panicked at tests\integration_defects.rs:67:5:
  DEFECT REPRODUCED: SKSEPlugin_Load fails to query INTERFACE_PAPYRUS (2)!
  ```
- **Root Cause**: Developer implemented serialization querying during M79 but omitted querying `INTERFACE_PAPYRUS`.

---

### 2.2 Defect B: `RegisterPapyrusFunctions` is a Complete No-Op
- **Source Inspection (`integration/src/lib.rs:55-63`)**:
  ```rust
  #[no_mangle]
  pub unsafe extern "C" fn RegisterPapyrusFunctions(_registry: *mut std::ffi::c_void) -> bool {
      // Registers native functions:
      // "DogfishNativeBridge::ClaimLocation" -> Dogfish_ClaimLocation
      ...
      true // <--- LITERALLY DOES NOTHING!
  }
  ```
- **Analysis**:
  - In `references/sources/skse64/skse64/PluginAPI.h:139-149`, `SKSEPapyrusInterface` passes a `VMClassRegistry*`.
  - In `references/sources/skse64/skse64/PapyrusNativeFunctions.h:31-60`, registering a function requires instantiating an `IFunction` subclass (e.g. `NativeFunction0`, `NativeFunction1`, `NativeFunction2`) and calling `registry->RegisterFunction(func)`.
  - Returning `true` without calling `registry->RegisterFunction` leaves the Papyrus VM unaware of any `DogfishNativeBridge` functions, causing Skyrim to throw `Cannot call undefined native function ClaimLocation()` at runtime.

---

### 2.3 Defect C: False Positive Test in Prior Test Harness
- **Source Inspection (`integration/src/lib.rs:456`)**:
  ```rust
  #[test]
  fn test_papyrus_registration_interface() {
      assert!(unsafe { RegisterPapyrusFunctions(std::ptr::null_mut()) });
  }
  ```
- **Audit Assessment**:
  - This unit test created the false illusion that Papyrus registration was verified and passing.
  - In reality, passing `null_mut()` should either fail or register functions. Because the stub returned `true` unconditionally, the test reported success on non-existent functionality.

---

### 2.4 Defect D: ABI Type Incompatibility (Papyrus Int vs Rust Types)
- **Investigation**:
  - `Dogfish_GetTreasuryGold`: Returns `i64` (8 bytes). Papyrus `Int` is `SInt32` (4 bytes).
  - `Dogfish_ClaimLocation`: Takes `location_id: u8` (1 byte). Papyrus passes `Int` (4 bytes).
  - Calling convention mismatch: When the Papyrus VM invokes a native function, it unpacks parameters from `VMState::argList` and pushes the return value to the VM stack. Exposing raw C functions without VM parameter unpacking adapters results in memory corruption.

---

## 3. Bridge Strategy Evaluation: Rust vtable vs. Minimal C++ Bridge

In accordance with the prompt guidance for M92:
*"If implementing the Papyrus VM interface directly in Rust requires unsafe, undocumented C++ object layouts, investigate a minimal C++ bridge. Prefer a small, verified bridge over fragile assumptions about C++ ABI layout."*

### Analysis of the Two Approaches:

| Architectural Approach | Feasibility | Maintenance Risk | Recommendation |
| :--- | :--- | :--- | :--- |
| **Approach 1: Pure Rust `IFunction` vtable synthesis** | Extremely High Risk | High: MSVC C++ class layouts, RTTI structures, `BSIntrusiveRefCounted` atomics, and virtual destructors are compiler-dependent and undocumented. | **REJECTED** |
| **Approach 2: Minimal C++ Bridge (`papyrus_bridge.cpp`)** | Proven & Safe | Zero: Uses pinned SKSE C++ templates (`NativeFunction2<StaticFunctionTag, SInt32, UInt32, bool>`) directly from `references/sources/skse64/`. Compiles via MSVC into the DLL or a static object linked to Rust. | **RECOMMENDED** |

---

## 4. Actionable Repair Requirements for M92–M94

1. **For M92**:
   - Query `INTERFACE_PAPYRUS` in `SKSEPlugin_Load`.
   - Implement real native function binding using the verified C++ bridge pattern or C-ABI VM wrappers.
   - Bind all 5 declared functions: `ClaimLocation`, `GetHoldingOwner`, `GetTreasuryGold`, `DepositGold`, `AdvanceDay`.
2. **For M93**:
   - Reconcile parameter types: Adapt `i64` to `i32` / clamped integer for Papyrus `Int`.
   - Add panic catch wrappers (`std::panic::catch_unwind`) to prevent Rust panics from crossing the foreign C-ABI boundary into Skyrim.
3. **For M94**:
   - Implement a mock `SKSESerializationInterface` in Rust tests that actually writes, reads, and verifies byte buffers rather than passing `null_mut()`.
