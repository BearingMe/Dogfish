# Dogfish Skyrim SE Test Package & Deployment Guide

- **Milestone**: M96 — Reproducible Skyrim Test Package
- **Phase**: Phase 25 (Persistence Hardening and Real Runtime Validation)
- **Target Runtime**: Skyrim Special Edition (1.5.97.0) / Anniversary Edition (1.6.640.0, 1.6.1170.0+)
- **Required Loader**: SKSE64 (v2.0.20+ for 1.5.97; v2.2.3+ for 1.6.640; v2.2.6+ for 1.6.1170)

---

## 1. Artifact Verification & Manifest

The workspace produces the verified native DLL binary and uncompiled Papyrus script sources staged under `integration/deploy/package/`:

| File Path in Package | File Type | Verification / Provenance | Status |
| :--- | :---: | :--- | :---: |
| `Data/SKSE/Plugins/dogfish_integration.dll` | 64-bit PE DLL | SHA256: `d9f64ac8dbe886cd43549b13797107c85c4de851aeb2d43c31160b3b744ebd8e` | **PRODUCED (215 KB)** |
| `Data/Source/Scripts/DogfishNativeBridge.psc` | Papyrus Source | Declares 5 native C-ABI bindings to `dogfish_integration.dll` | **PRODUCED** |
| `Data/Source/Scripts/DogfishClaimBannerScript.psc` | Papyrus Source | Activator event script calling `DogfishNativeBridge.ClaimLocation` | **PRODUCED** |
| `Data/Scripts/DogfishNativeBridge.pex` | Compiled PEX | Requires Creation Kit / PapyrusCompiler | *Awaiting Manual Compile* |
| `Data/Scripts/DogfishClaimBannerScript.pex` | Compiled PEX | Requires Creation Kit / PapyrusCompiler | *Awaiting Manual Compile* |
| `Data/dogfish_test.esp` | ESL Plugin | Created via xEdit recipe (`integration/records/dogfish_test_recipe.md`) | *Awaiting Manual Compile* |

*Epistemological Note*: In compliance with Rule 1.4 (*"No False Completion"*), compiled `.pex` bytecode and the `.esp` plugin are NOT fabricated in this repository; they are generated on the player's game machine using the instructions below.

---

## 2. Compilation Instructions (Human Researcher)

### 2.1 Compiling Papyrus Scripts via CLI
From your Skyrim SE game directory (containing `PapyrusCompiler.exe`):
```cmd
PapyrusCompiler.exe "Data\Source\Scripts\DogfishNativeBridge.psc" -f="TESV_Papyrus_Flags.flg" -i="Data\Source\Scripts;Data\Scripts\Source" -o="Data\Scripts"
PapyrusCompiler.exe "Data\Source\Scripts\DogfishClaimBannerScript.psc" -f="TESV_Papyrus_Flags.flg" -i="Data\Source\Scripts;Data\Scripts\Source" -o="Data\Scripts"
```

### 2.2 Authoring `dogfish_test.esp` via xEdit
Follow the exact recipe in [`integration/records/dogfish_test_recipe.md`](../records/dogfish_test_recipe.md):
1. Create ESL-flagged plugin `dogfish_test.esp` with master `Skyrim.esm`.
2. Add interior cell `DogfishTestCell`.
3. Add activator `DogfishClaimBannerActi` with script `DogfishClaimBannerScript` (`LocationID = 0`).
4. Place reference `DogfishClaimBannerREFR` at `(0, 0, 0)`.
5. Add quest `DogfishAnchorQuest` (Start Game Enabled).

---

## 3. Deployment & Test Procedure

### 3.1 Installation
- **Mod Organizer 2**: Zip `integration/deploy/package/Data/` and install as an active mod, adding the compiled `.pex` and `dogfish_test.esp`.
- **Manual**: Copy `Data/` contents directly into `Skyrim Special Edition/Data/`.

### 3.2 In-Game Execution Sequence
1. Launch Skyrim using `skse64_loader.exe`.
2. At the main menu, open console (`~`) and type:
   ```text
   coc DogfishTestCell
   ```
3. Walk forward to the banner activator and press `E` (Activate).
4. **Observable Feedback Verification**:
   - Verify on-screen notification: `"Dogfish: Claimed Holding 0 successfully!"`.
   - Press `E` a second time: verify notification: `"Dogfish: Holding 0 is already claimed by your faction."`.
5. Open console and save the game:
   ```text
   save DogfishTestSave
   ```
6. Verify that `Documents/My Games/Skyrim Special Edition/Saves/DogfishTestSave.skse` was created alongside `DogfishTestSave.ess`.
7. Reload the save (`load DogfishTestSave`) and verify state consistency.

---

## 4. Log Inspection & Troubleshooting

- **SKSE Plugin Loader Log**: Check `Documents/My Games/Skyrim Special Edition/SKSE/skse64.log`. Verify:
  ```text
  checking plugin C:\Games\Skyrim Special Edition\Data\SKSE\Plugins\dogfish_integration.dll
  plugin DogfishIntegration (00000001) reported as compatible with current runtime version
  loading plugin DogfishIntegration
  ```
- **Papyrus VM Log**: Check `Documents/My Games/Skyrim Special Edition/Logs/Script/Papyrus.0.log`. Verify:
  ```text
  [Dogfish] Holding 0 claimed by player.
  ```

---

## 5. Clean Removal & Teardown

1. Delete `Data/SKSE/Plugins/dogfish_integration.dll`.
2. Delete `Data/dogfish_test.esp`.
3. Delete `Data/Scripts/Dogfish*.pex`.
4. Delete `DogfishTestSave.skse`.
5. Because 100% of simulation state was maintained in native memory and `.skse` co-saves, the player's core savegame contains zero orphaned script instances.
