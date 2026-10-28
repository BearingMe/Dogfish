# Military Orders, Movement & Deployment: Conquest of Skyrim

- **Milestone**: M14 — Orders, Movement, and Deployment
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_commandermovingscript.psc` (`StartCommanderMoving`, `CommanderArrival`, `StopCommanderMoving`)
  - `cs_patrolholdscript.psc` (`TaskBarrel` AI package architecture, `StartPatrol`, `FollowPlayer`)
  - `cs_troopmanagemenuscript.psc` (Hold distance markers, transit time calculation)
  - `cs_attackplanningnewscript.psc` (`InvasionTarget`, `SelectedCommanders`, `HoldsNotLinked`)
- **Status**: CONFIRMED

---

## 1. Command Hierarchy & Dispatch Model

Military command flows strictly from sovereign authority down to individual combatants:

```text
Player (Supreme Commander)
  │
  ├── High Commander / Second-in-Command (ASIC)
  │     └── Global War Room: Invasion dispatch, truce enforcement, prisoner transport
  │
  └── Field Division Commanders (Commanders[0] through Commanders[9])
        │
        ├── Movement Dispatch: CS_CommanderMovingScript (Transit & Destination Targets)
        ├── Routine Assignment: CS_PatrolHoldScript (Task Barrel AI Package System)
        └── Combat Execution: Stationed soldiers, archers, and support mages
```

---

## 2. Order Inventory & Operational Modalities

*Conquest of Skyrim* provides eight distinct operational order types:

| Order Type | Target / Location | Script Trigger / Anchor | Operational Behavior |
| :--- | :--- | :--- | :--- |
| **Follow Player** | Dynamic Player Coordinates | `TaskBarrelFollow` | Division follows player across Skyrim; teleports 120 units behind player if unloaded. |
| **Hold Patrol** | Hold Territory | `CommandersPatrollingHolds` | Roving patrols along road networks and perimeter boundaries within the designated hold. |
| **Fort Garrison** | Conquered Fort | `TaskBarrelsDefendFort` | Stationed defense; manning walls, gates, and interior barracks. Suppresses local unrest. |
| **City Garrison** | Hold Capital | `TaskBarrelsDefendCity` | Urban peacekeeping; replaces vanilla city guards with faction-standardized soldiers. |
| **Forward Camp** | Frontier Checkpoint | `TaskBarrelsAttackCamp` | Staging camp established outside enemy hold borders prior to initiating major sieges. |
| **Resource Hunt** | Wilderness Nodes | `TaskBarrelHunt` | Deploys detachments to harvest game and forage, providing passive daily food stockpiles. |
| **Offensive Assault** | Enemy Capital / Fort | `TaskBarrelsAttackCity` | Converges multiple army divisions on the target location to initiate siege warfare. |
| **Stand By / Idle** | Headquarters / Barracks | `TaskBarrelNoOrder` | Division enters low-cost idle sandbox behavior; minimizes CPU pathing overhead. |

---

## 3. The "Task Barrel" AI Package Anchor Pattern

To manipulate Skyrim's rigid AI package stack without recompiling complex package conditions for each individual NPC, the author engineered the **Task Barrel proxy pattern**:

```text
Player issues order in Dialogue / War Map
       │
       ▼
Script assigns Commander to specific Task Barrel (CS_PatrolHoldScript)
  [TaskBarrelFollow | TaskBarrelHunt | TaskBarrelsAttackCity | TaskBarrelsDefendFort]
       │
       ▼
Creation Engine AI Package stack evaluates target reference alias
       │
       ▼
Commander and attached division soldiers execute target package instantly
```

- **Mechanism**: Invisible activator markers ("Task Barrels") are placed in worldspaces or carried as reference aliases.
- **Package Binding**: AI packages on commanders evaluate target aliases pointing to these barrels. When a script moves an alias to a different Task Barrel (e.g., from `TaskBarrelNoOrder` to `TaskBarrelsAttackCity`), the actor's package stack immediately updates without requiring package re-evaluation loops.

---

## 4. Movement, Transit & Arrival Simulation

### 4.1 Spatial Transit Distance Calculation
Movement across Skyrim is not instantaneous. Transit duration is calculated based on Euclidean distance between hold capital spatial markers:
- **Spatial Markers**: `DistanceMarkerMarkarth`, `DistanceMarkerRiften`, `DistanceMarkerSolitude`, `DistanceMarkerWhiterun`, `DistanceMarkerWindhelm`, `DistanceMarkerDawnstar`, `DistanceMarkerFalkreath`, `DistanceMarkerMorthal`, `DistanceMarkerWinterhold`.
- **Duration Tracking**:
  - `MovingArrivalTime`: Timestamp when the army began transit.
  - `MovingArrivalTimeRemaining`: Integer countdown representing in-game travel hours.
  - Allows off-screen armies to march across the province in a realistic, synchronized timeframe.

### 4.2 Seamless Arrival & Unloaded Cell Teleportation (`CommanderArrival`)
When an army arrives at its destination:
- **3D Loaded Check**:
  - If the player is in the same cell (`Commander.Is3DLoaded() == true`), the army physically marches to their assigned duty markers.
  - If the cell is unloaded (`Commander.Is3DLoaded() == false`), the script executes an atomic coordinate shift:
    ```papyrus
    Commander.MoveTo(FortCommanders.Positions[ArmyScript.MovingArrivalHoldNum])
    ```
- **Engine Invariant**: This unloaded teleportation prevents armies from getting permanently stuck in unloaded wilderness grids or broken navmesh corridors across distant Skyrim cells.

---

## 5. Tactical Battle Formations (v0.3.9)

In version 0.3.9, the author introduced structured field battle formations:
- **5x4 Block Formations**:
  - Attached soldiers execute package fragments (`cs__pf_cs_attackcitysiegedrum_*.psc`) locking their relative coordinate offsets behind the division commander.
- **Parade Formations**:
  - Two-column marching configurations flanked by drummers equipped with `CS_WarDrum` armor items, executing synchronized `IdleApplaud` and cadence walk packages.

---

## 6. Preconditions, Failure Conditions & Edge Cases

1. **Territory Contiguity Failure (`HoldsNotLinked`)**:
   - Armies cannot be dispatched to attack holds that do not share a land border with a player-controlled holding. The war table cancels the order and outputs `HoldsNotLinked`.
2. **Military Inferiority Warning (`MessageNoSuperiority`)**:
   - If an army division attempts an assault against a fortress with an overwhelming enemy defender count, the script prompts a superiority warning.
3. **Follower Mod Collision**:
   - If an external mod (AFT, NFF, EFF) attempts to manage a division commander, it forcibly replaces the `TaskBarrel` package with a vanilla follower package, permanently breaking hold patrols and army deployment.
