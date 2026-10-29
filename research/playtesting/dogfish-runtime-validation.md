# Dogfish In-Game Runtime Validation Protocol

- **Document**: Empirical Skyrim Runtime Verification Plan & Observation Log
- **Milestone**: M97 — In-Game Runtime Validation
- **Phase**: Phase 25 (Persistence Hardening and Real Runtime Validation)
- **Status**: **BLOCKED (Awaiting Human In-Game Execution)**
- **Test Protocol Version**: 1.0.0
- **Target Plugin**: `dogfish_test.esp` (ESL-flagged Form Version 44)
- **Target Native DLL**: `dogfish_integration.dll` (v0.1.0, 215 KB)

---

## 1. Executive Summary & Epistemological Status

In strict adherence to Rule 1.4 (*"No False Completion"*):
> *Never mark a required runtime milestone COMPLETE solely because code compiles, a DLL exports functions, or unit tests pass. Runtime validation requires actual Skyrim execution. When manual game testing is unavailable, record BLOCKED.*

Because this project operates in an automated headless development environment without an active graphical display running `SkyrimSE.exe`, Milestone M97 is formally recorded as **`BLOCKED (Awaiting Human In-Game Execution)`**.

Zero game execution is fabricated. This document establishes the exact 15-step empirical verification sequence and log collection schema for a human researcher to execute on a Windows gaming workstation.

---

## 2. Fifteen-Step Runtime Verification Sequence

| Step | Action | Expected In-Engine Observation | Pass/Fail Criteria |
| :---: | :--- | :--- | :--- |
| **1** | Launch Skyrim SE via `skse64_loader.exe` | Game launches to main menu without CTD. | SKSE loader attaches cleanly. |
| **2** | Check `skse64.log` | `loading plugin DogfishIntegration` reported. | Plugin loads with exit code success. |
| **3** | Check `Papyrus.0.log` | `RegisterPapyrusFunctions` executed. | Native functions bound to VM. |
| **4** | Open console (`~`), type `coc DogfishTestCell` | Player teleports into interior test room. | Interior cell renders cleanly. |
| **5** | Approach `DogfishClaimBanner` activator | Activation prompt appears (`E) Activate Banner`). | Activator reference is interactive. |
| **6** | Press `E` to activate banner | `Dogfish_ClaimLocation_C(0, true)` invoked. | FFI boundary crossed. |
| **7** | Check simulation state mutation | Whiterun holding ownership transferred to Player. | Native state updated atomically. |
| **8** | Return code received by Papyrus | Return code `0` returned to script. | Function returns successfully. |
| **9** | Confirm visible on-screen feedback | Notification: `"Dogfish: Claimed Holding 0 successfully!"`. | Visual feedback observed on HUD. |
| **10** | Open console, type `save DogfishTestSave` | Savegame created in `Documents/.../Saves/`. | `Dogfish_SaveCallback` writes `'STAT'` chunk. |
| **11** | Verify companion co-save file | `DogfishTestSave.skse` created alongside `.ess`. | Co-save file size > 500 bytes. |
| **12** | Load the savegame (`load DogfishTestSave`) | Game loads; `Dogfish_LoadCallback` invoked. | Native state restored from `'STAT'` chunk. |
| **13** | Activate banner a second time | Notification: `"Dogfish: Holding 0 is already claimed..."`. | State persistence confirmed in game. |
| **14** | Trigger invalid operation (claim non-adjacent hold) | Notification: `"Dogfish: Claim failed. Error code: 2"`. | Safe error code handling confirmed. |
| **15** | Main menu / New game test | `Dogfish_RevertCallback` resets state. | State reset to defaults verified. |

---

## 3. Required Observation Data Collection Schema

When executed by a human researcher, the following empirical data must be recorded in this document:

```text
[EXECUTION REPORT TEMPLATE]
Date of Test: YYYY-MM-DD
Skyrim SE Version: (e.g. 1.6.640.0)
SKSE64 Version: (e.g. 2.2.3)
Test Plugin Version: 1.0 (dogfish_test.esp)
Active Mod Manager: (MO2 / Vortex / Manual)
GPU / Graphics Driver: (e.g. RTX 4070, Driver 560.xx)

OBSERVED LOGS:
- skse64.log excerpt: [paste here]
- Papyrus.0.log excerpt: [paste here]

VERIFICATION RESULTS:
- Step 1-3 (Loading & Registration): [PASS / FAIL]
- Step 4-9 (Activation & Feedback): [PASS / FAIL]
- Step 10-13 (Co-Save Persistence): [PASS / FAIL]
- Step 14-15 (Error Handling & Revert): [PASS / FAIL]

PHOTOGRAPHIC / VIDEO EVIDENCE:
- Screenshot of HUD notification: [attached image path]
- Screenshot of Saves directory (.ess + .skse): [attached image path]
```

---

## 4. Milestone Verdict

Milestone M97 remains **`BLOCKED`** pending the execution of this protocol by a human player on a configured Skyrim installation.
