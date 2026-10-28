# State, Persistence & Engine Integration Verification: Conquest of Skyrim

- **Document**: Technical Verification of State Boundaries & Failure Modes
- **Milestone**: M40 — State, Persistence, and Engine Integration Verification
- **Phase**: Phase 10 (Reproducible Technical Verification)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Primary Source Evidence**: `cs_transferownershipscript.psc`, `cs_attackcityscript.psc`, `cs_checkversion.psc`, `cs_bountyscript.psc`, `conquerskyrim.esp` FormID overrides
- **Status**: VERIFIED & REPRODUCIBLE

---

## 1. Technical Boundary: Creation Engine State vs. Mod State

A core architectural risk in Skyrim overhaul development is managing the boundary between the engine's native runtime state and the mod's Papyrus-managed state. 

In *Conquest of Skyrim*, these two layers are loosely coupled across three primary interfaces:

```text
       ┌─────────────────────────────────────────────────────────────┐
       │               Creation Engine Native State                  │
       │  (Actor AI Package Stacks, Havok Physics, CWOwner Keywords) │
       └──────────────────────────────▲──────────────────────────────┘
                                      │
                   Porous Interface   │ (No Transactions / No Rollbacks)
                                      ▼
       ┌─────────────────────────────────────────────────────────────┐
       │                 Mod Papyrus State Machine                   │
       │(AllHoldings Globals, Cities[] Booleans, ReferenceAliases)   │
       └─────────────────────────────────────────────────────────────┘
```

---

## 2. The Fallacy of "Atomic" Territory Transfers (`CS_TransferOwnershipScript`)

Earlier research documentation referred to `TransferOwnershipCity` as an "atomic transfer". Direct static code analysis proves this is **architecturally inaccurate**:

### 2.1 Non-Transactional Procedural Execution
`TransferOwnershipCity(int citynum, int newfaction, bool keepjarl=false)` executes 60+ sequential script operations with **zero transaction isolation and zero rollback mechanism**:

```text
Operation 1: (CrimeFactions.GetAt(citynum) as Faction).SetCrimeGold(0)
Operation 2: CityLoc.SetKeywordData(CWOwner, 69)           <-- Native Engine Keyword Updated
Operation 3: While loop disables Imperial/Stormcloak guard markers
Operation 4: While loop forces deposed rulers into CitizenFaction
Operation 5: TMImperial.ChooseRandomCapital()
Operation 6: (AllHoldings.GetAt(citynum) as GlobalVariable).SetValue(1) <-- Mod Global Updated
```

### 2.2 The Split-Brain State Vulnerability
Because `CityLoc.SetKeywordData(CWOwner, 69)` is executed at line 102, but `(AllHoldings.GetAt(citynum) as GlobalVariable).SetValue(1)` is not executed until line 147:
- If a Papyrus thread crash, cell unload, or save-game event interrupts the script between lines 102 and 147:
  - The Creation Engine's location system considers the city owned by the Player (`CWOwner = 69`).
  - But the mod's economic and invasion manager still reads `AllHoldings` as Imperial (`11`) or Stormcloak (`12`).
- **Consequence**: The holding enters a permanent "split-brain" state where tax scripts fail to deposit gold, enemy armies attack a city they believe they already own, and The Moot cannot convene.

---

## 3. Absence of Reload Recovery in Active Sieges (`CS_AttackCityScript`)

Static audit of `cs_attackcityscript.psc` reveals that the master siege controller contains **zero `Event OnPlayerLoadGame()` handlers**:
1. **Volatile Havok Physics vs. Serialized Papyrus Arrays**:
   - The battering ram destruction sequence relies on Havok movable statics and destructible barrier states.
   - When saving mid-assault, Papyrus serializes `TroopsRemainingDefenders` and `AttackersInitial`.
   - However, cell reload causes Skyrim's engine to re-evaluate the physical 3D worldspace from base forms, frequently resetting gate barricades into their default locked state while the quest script believes the gates are breached.
2. **Spawn Wave Desynchronization**:
   - Reinforcements trigger at exact casualty numbers (`20, 40, 60, 80, 100`). If an interrupted reload resets local actor references without recalculating `TotalDefenderDeaths`, wave progression halts permanently, locking the siege in combat limbo.

---

## 4. Save/Reload Self-Healing Boundaries

While combat quests lack reload recovery, general civil state is maintained by active self-healing routines in `cs_checkversion.psc`:
- **Timer Re-Registration**:
  - Daily 24h game-time updates can stall if serialized timer IDs are dropped during save transitions. Lines 40–50 of `cs_checkversion.psc` explicitly unregister and re-register:
    ```papyrus
    GovScriptFunctions.TMImp.UnRegisterForUpdateGameTime()
    GovScriptFunctions.TMImp.RegisterForUpdateGameTime(24)
    ```
- **Surrender Deadlock Reset**:
  - `cs_playersurrenderloadgame.psc` forcibly invokes `SetStage(100)` on load to ensure the player character is not permanently trapped in a surrendered ragdoll or captive package.

---

## 5. Vanilla Engine Overrides & Breakage Mechanics

### 5.1 `CWCampaignObj` (`0x00026F45`) Hijacking
- `conquerskyrim.esp` directly edits the master quest controlling the vanilla Civil War.
- **Engine Invariant**: If a player attempts to run *Conquest of Skyrim* alongside the vanilla Civil War, both quest engines listen to the same location-change events (`CWChangeLocationScenes`), firing contradictory alias assignments and locking palace doors.

### 5.2 Follower Framework Package Overrides
- Faction commanders rely on the `TaskBarrel` proxy pattern in `cs_patrolholdscript.psc`.
- Importing commanders into external frameworks (AFT, NFF) strips their `TaskBarrel` package assignment and replaces it with standard follower sandboxing, permanently breaking army marching orders.

---

## 6. Summary of Architectural Truths

| Investigated Feature | Nominal Claim | Verified Technical Reality in Code |
| :--- | :--- | :--- |
| **Territory Transfer** | "Atomic transaction" | Sequential procedural script loop; vulnerable to split-brain desync if interrupted. |
| **Mid-Combat Save** | "Persistent battle state" | High vulnerability; zero reload recovery handlers in `CS_AttackCityScript`. |
| **Civil War Mod Compatibility**| "Compatible with care" | Structurally incompatible; directly overrides `CWCampaignObj` `0x00026F45`. |
| **Commander Movement** | "Dynamic AI pathfinding" | Invisible Task Barrel reference alias targeting; unloaded cell coordinate shift (`MoveTo`). |
