# Skyrim Engine Boundary Mapping & Interface Specification

- **Document**: Technical Boundary Specification & State Division Map
- **Milestone**: M59 — Engine Boundary Mapping
- **Phase**: Phase 15 (Skyrim Integration Probing)
- **Subject**: *Conquest of Skyrim* Runtime Boundary vs. `dogfish-reconstruction`
- **Investigation Date**: 2026-10-28
- **Status**: SPECIFIED & APPROVED

---

## 1. Executive Summary & Boundary Architecture

Milestone M59 establishes the precise architectural boundary between the independently reconstructed Rust simulation model (`reconstruction/`) and the Skyrim Special Edition engine runtime (Creation Engine 64-bit).

```text
  ┌────────────────────────────────────────┐         ┌────────────────────────────────────────┐
  │         SKYRIM SPECIAL EDITION         │         │         DOGFISH SIMULATION             │
  │            (Creation Engine)           │         │         (Independent Rust)             │
  ├────────────────────────────────────────┤         ├────────────────────────────────────────┤
  │ • 3D Meshes, Textures, Shaders, LOD    │         │ • 15 Provincial Holdings State Model   │
  │ • Havok Physics, Ragdolls, Barricades  │         │ • Regional Tiered Taxation Formulas    │
  │ • Real-Time Cell Grid (uGridsToLoad=5) │         │ • Demographic Labor Allocation (4-Tier)│
  │ • Navmesh (NAVM), Pathfinding, AI Stack│◄───────►│ • 10-Division Roster & Military Upkeep │
  │ • Scaleform Menus, Audio (xWMA/fuz)    │         │ • 7-Member Cabinet & Spymaster Coups   │
  │ • Papyrus VM Execution Slices (1.2ms)  │         │ • Bilateral Diplomacy & Truce Timers   │
  │ • Vanilla Savegame Container (.ess)    │         │ • The Moot Convocation & Election      │
  │ • FormIDs, Records (QUST, CELL, REFR)  │         │ • Decoupled State Persistence (JSON)   │
  └────────────────────────────────────────┘         └────────────────────────────────────────┘
                       ▲                                                  ▲
                       └──────────────────┬───────────────────────────────┘
                                          │
                         ┌────────────────┴────────────────┐
                         │      INTEGRATION ADAPTER        │
                         │    (SKSE Native cdylib / FFI)   │
                         ├─────────────────────────────────┤
                         │ • SKSEPlugin_Query / Load C-ABI │
                         │ • SKSESerializationInterface    │
                         │ • BSTEventSink<TESCombatEvent>  │
                         │ • Papyrus Native Bridge / Event │
                         └─────────────────────────────────┘
```

---

## 2. Explicit State Ownership Division

### 2.1 Simulation-Owned State
The following systems are owned exclusively by the deterministic Rust simulation model:
1. **Sovereign Identity & Governance**: Faction title, army nomenclature, heraldic banner theme, political doctrines (`Autocracy`, `Feudalism`, `Republic`, `Meritocracy`).
2. **Territorial State Machine**: Global holding ownership across all 15 holdings (9 hold capitals, 6 major villages), spatial contiguity validation, and camp respawn cooldowns.
3. **Macroeconomics & Demographics**: National treasury gold, pooled resource stockpiles (Food, Metal, Wood, Trade Docks), 5 facility levels (1..4), population, unemployment reserve, and conscription rates.
4. **Military Architecture**: Roster of 10 field divisions, troop specialization classes, power levels (1..5), task orders, and off-screen detachment combat resolution.
5. **Cabinet, Judiciary & The Moot**: Ministerial appointments (7 roles), dungeon prisoner tallies, bilateral treaty truces, provincial Jarl loyalty, and High King election status.
6. **State Persistence**: Serialized decoupled state files (`saves/dogfish_state.json`), independent of Skyrim `.ess` binaries.

### 2.2 Skyrim Engine-Owned State
The following systems are owned exclusively by the Creation Engine:
1. **3D World & Spatial Rendering**: Terrain heightmaps, worldspaces (`Tamriel`), cell coordinates, LOD meshes, lighting, and occlusion culling.
2. **Actor Lifecycle & Physics**: Skeletal meshes (`NIF`), Havok collision geometries, destructible barrier meshes, ragdoll physics, and hit detection.
3. **Pathfinding & AI Packages**: Navigation meshes (`NAVM`), actor wander/patrol routines, travel packages, and real-time combat threat evaluation.
4. **User Interface & Audio**: Scaleform SWF HUD components, dialogue trees (`DIAL`/`INFO`), message boxes, localized voice files, and music triggers.
5. **Savegame Containers**: Core `.ess` savegame serialization, player coordinates, inventory containers, and engine global variables.

---

## 3. Cross-Boundary Communication Interfaces

Communication across the engine boundary is structured into strictly typed inward triggers and outward signals:

### 3.1 Inward Triggers (Skyrim Engine &rarr; Simulation Core)

| Engine Event Source | Trigger Mechanism | Simulation Core API | Handled Outcome |
| :--- | :--- | :--- | :--- |
| **Claim Activator Interaction** | `OnActivate` event on in-world banner activator | `GameState::transfer_holding()` | Validates adjacency; transfers holding atomically; clears crime gold. |
| **Actor Death / Combat Wave** | `BSTEventSink<RE::TESDeathEvent>` | `SiegeOperation::record_defender_casualty()` | Increments casualty counter; checks 20/40/60/80/100 reinforcement wave milestones. |
| **Throne Judicial Hearing** | Player sits on Jarl throne activator | `GameState::judge_court_case()` | Hand down sentence; collects fine or updates dungeon count. |
| **Covert Coup Trigger** | Dialogue choice with Chief Spymaster | `GameState::attempt_covert_coup()` | Bribes garrison; transfers capital without siege combat. |
| **In-Game 24h Time Tick** | `OnUpdateGameTime` or SKSE timer | `GameState::tick_day()` | Advances daily economic, demographic, and diplomatic simulation loop. |
| **Savegame Save / Load** | `SKSESerializationInterface` callbacks | `StatePersistence::serialize` / `deserialize` | Synchronizes decoupled state file with active Skyrim save file. |

### 3.2 Outward Signals (Simulation Core &rarr; Skyrim Engine)

| Simulation Output | Delivery Mechanism | Engine-Side Action | Visible In-Game Outcome |
| :--- | :--- | :--- | :--- |
| **Holding Captured** | SKSE Native Bridge Call | `SetCrimeGold(0)`, swap banner mesh | Crime cleared; faction colors displayed in holding. |
| **Reinforcement Wave** | SKSE Task Queue / Actor Spawner | Dispatch spawn queue for 20 reserve actors | Reinforcement wave rushes gate/streets. |
| **Treasury Mutation** | Papyrus Global Sync / Menu Update | `UpdateCurrentInstanceGlobal(TreasuryGold)` | Updated gold displayed in management menus. |
| **Notification Message**| Engine Notification Dispatch | `Debug.Notification(message)` | Upper-left notification banner displayed. |
| **High King Crowned** | Quest Stage Advancement | Stage set on `CoronationQuest`; equip crown | Crown awarded; victory fanfare played. |

---

## 4. Native Engine & Script APIs

The integration adapter utilizes standard SKSE64 native interfaces verified in `references/sources/skse64`:

```cpp
// SKSE Native Plugin C-ABI Interface (Verified in skse64/PluginAPI.h)
extern "C" {
    __declspec(dllexport) bool SKSEPlugin_Query(const SKSEInterface* skse, PluginInfo* info) {
        info->infoVersion = PluginInfo::kInfoVersion;
        info->name = "DogfishIntegration";
        info->version = 1;
        return true;
    }

    __declspec(dllexport) bool SKSEPlugin_Load(const SKSEInterface* skse) {
        // 1. Initialize Address Library
        // 2. Register SKSE Serialization Callbacks (Save, Load, Revert)
        // 3. Register Event Sinks (Death, Combat, LocationChange)
        // 4. Initialize Dogfish Simulation Core
        return true;
    }
}
```

---

## 5. Manual Creation Kit & xEdit Asset Requirements

To execute the minimal in-game vertical slice (M60), the following isolated records are required:

1. **Plugin Format**: ESL-flagged ESP (`ESP-FE`, slot `0xFE`). Consumes zero master plugin slots.
2. **Isolated Interior Test Cell (`DogfishTestCell`)**:
   - Cloned from vanilla `qasmoke`.
   - *Rationale*: Completely bypasses exterior worldspace navmesh generation, worldspace LOD recalculations, and vanilla Civil War script conflicts.
3. **Interactive Test Banner (`DogfishClaimBannerREFR`)**:
   - Simple static reference placed in `DogfishTestCell` with an attached trigger script dispatching the claim event to the native adapter.
4. **Anchor Quest (`DogfishAnchorQuest`)**:
   - Flagged *Start Game Enabled* to register native event listeners upon game startup.

---

## 6. High-Risk Integration Traps & Mitigations

| Identified Risk / Engine Trap | Root Cause in Original Mod | Dogfish Mitigation Strategy |
| :--- | :--- | :--- |
| **Split-Brain Desynchronization** | Sequential non-transactional script execution in `cs_transferownershipscript.psc` | Atomic state transition in Rust core prior to engine reflection. |
| **Havok Mid-Siege Barrier Desync** | Zero `OnPlayerLoadGame` handlers in `CS_AttackCityScript` | Decoupled persistence serializes exact combat phase independently of Havok physics. |
| **The Moot Convocation Deadlock** | Unhandled null Jarl actor references in `cs_meetingscript.psc:GetJarls()` | Invariant check requires all 8 provincial Jarls appointed before convocation. |
| **Savegame Bloat & Script Stalls** | 1,539 Papyrus scripts serialized directly inside `.ess` save files | 100% of strategic domain state maintained in native memory and decoupled external storage. |
| **Papyrus Thread Starvation** | Asynchronous Papyrus tasks throttled to 1.2ms engine time slices | All heavy mathematical calculations executed instantaneously in native Rust code. |
