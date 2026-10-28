# Dogfish Skyrim Integration: Minimal Vertical Slice

- **Component**: Skyrim Special Edition Native Runtime Adapter & Test Records
- **Milestone**: M60 — Minimal In-Game Vertical Slice
- **Phase**: Phase 15 (Skyrim Integration Probing)
- **Status**: IMPLEMENTED & TESTED

---

## 1. Architecture Overview

`dogfish-integration` provides a native C-ABI dynamic library (`cdylib`) implementing the SKSE64 plugin interface and linking directly to the standalone `dogfish-reconstruction` simulation crate.

```text
  Skyrim Special Edition Runtime
     │
     ▼ (Player activates banner in DogfishTestCell)
  DogfishClaimBannerScript.psc
     │
     ▼ (Dispatches native FFI call)
  Dogfish_ClaimLocation(location_id, has_staging_camp)
     │
     ▼ (SKSE C-ABI native entry point)
  dogfish-integration (cdylib)
     │
     ▼ (Atomic state transition & invariant check)
  dogfish-reconstruction::GameState::transfer_holding()
     │
     ▼ (Returns return code: 0 = Success)
  Skyrim UI / Engine Feedback
     │
     ▼ (Debug.Notification("Holding Claimed: Whiterun"))
  Player Screen Feedback
```

---

## 2. Exported Native C-ABI Functions

| Function Signature | Parameters | Return Values / Error Codes | Description |
| :--- | :--- | :--- | :--- |
| `SKSEPlugin_Query` | `skse`, `info` | `bool` | Negotiates plugin name and version with SKSE loader. |
| `SKSEPlugin_Load` | `skse` | `bool` | Initializes the simulation core and event sinks. |
| `Dogfish_ClaimLocation` | `u8 location_id`, `bool has_camp` | `0`: Success<br>`1`: Invalid ID<br>`2`: Not Adjacent<br>`3`: Already Owned<br>`99`: Uninitialized | Claims a holding; clears crime bounty; updates owner. |
| `Dogfish_GetHoldingOwner`| `u8 location_id` | `0`: Player<br>`1`: Imperial<br>`2`: Stormcloak<br>`3`: Neutral<br>`-1`: Error | Queries the holding's current owning faction. |
| `Dogfish_GetTreasuryGold`| None | `i64`: Treasury gold balance (`-1` on error) | Returns current national treasury balance. |
| `Dogfish_DepositGold` | `i64 amount` | `bool`: Success | Deposits gold into national treasury. |
| `Dogfish_AdvanceDay` | None | `u32`: New Day Number | Executes 24-hour economic & demographic loop. |

---

## 3. Isolated Test Records Specification (`dogfish_test.esp`)

To verify runtime interaction without entangling vanilla Civil War quests or exterior navmesh, the following minimal records are specified:

1. **Plugin Format**: ESL-flagged ESP (`ESP-FE`, load slot `0xFE`).
2. **Interior Test Cell**: `DogfishTestCell` (cloned from `qasmoke`).
   - Clean, flat interior test space with lighting.
   - Zero exterior LOD, navmesh generation, or weather conflicts.
3. **Interactive Test Banner**: `DogfishClaimBanner` (placed at `0, 0, 0` in `DogfishTestCell`).
   - Base Form: `Static` or `Activator` (using standard Imperial/Stormcloak banner mesh).
   - Attached Papyrus Script (`DogfishClaimBannerScript.psc`):
     ```papyrus
     Scriptname DogfishClaimBannerScript extends ObjectReference

     Int Property LocationID = 0 Auto ; Default: Whiterun

     Event OnActivate(ObjectReference akActionRef)
         if akActionRef == Game.GetPlayer()
             int result = Dogfish_ClaimLocation(LocationID, true)
             if result == 0
                 Debug.Notification("Dogfish: Claimed Holding " + LocationID)
             elseif result == 3
                 Debug.Notification("Dogfish: Holding is already claimed.")
             else
                 Debug.Notification("Dogfish: Failed to claim holding. Code: " + result)
             endif
         endif
     EndEvent

     ; Native Import
     Int Function Dogfish_ClaimLocation(Int locID, Bool hasCamp) Global Native
     ```
4. **Anchor Quest**: `DogfishAnchorQuest`.
   - Priority 50, Start Game Enabled.
   - Initializer for registration hooks.

---

## 4. Build & Test Instructions

### 4.1 Running Automated Unit Tests
From repository root:
```bash
cargo test --manifest-path integration/Cargo.toml --offline
```

### 4.2 Building the Native DLL (`cdylib`)
```bash
cargo build --manifest-path integration/Cargo.toml --release --offline
```
Output binary: `integration/target/release/dogfish_integration.dll` (Deploy to `Skyrim/Data/SKSE/Plugins/`).
