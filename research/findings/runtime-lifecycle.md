# Initialization & Runtime Lifecycle: Conquest of Skyrim

- **Milestone**: M08 — Initialization and Runtime Lifecycle
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Evidence**: `cs_checkversion.psc`, `cs_readstartbook.psc`, `cs_startfactionmenu.psc`, `cs_govscriptfunctions.psc`, `cs_transferownershipscript.psc`
- **Status**: CONFIRMED

---

## 1. Lifecycle Overview

The runtime lifecycle of *Conquest of Skyrim* progresses through six distinct operational phases:

```text
1. Mod Activation & SEQ Pre-Cache
       │
2. Faction Founding & Activation (Book Read / Menu Event)
       │
3. Steady-State Background Execution (24h Daily GameTime Updates)
       │
4. Tactical Combat & Siege Invasions (Cell Attachment & Scene Triggers)
       │
5. Save Game Serialization & Load Verification (CS_CheckVersion / OnPlayerLoadGame)
       │
6. Endgame Resolution (The Moot / Coronation Ceremony)
```

---

## 2. Phase 1: Installation & Initialization Prerequisites

### 2.1 Load Order & Master Dependencies
- Loads after `Skyrim.esm`, `Update.esm`, `Dawnguard.esm`, `HearthFires.esm`, and `Dragonborn.esm`.
- Requires `SKSE64` and `SkyUI.esp` to be active.

### 2.2 SEQ Dialogue Pre-Caching (`conquerskyrim.seq`)
- Because the mod introduces Start-Game-Enabled quests with custom voiced dialogue, `SEQ/conquerskyrim.seq` (552 bytes) is loaded by the Skyrim engine during the main menu boot.
- *Function*: Prevents the vanilla Skyrim dialogue bug where Start-Game-Enabled quest dialogue fails to initialize until after saving and reloading.

---

## 3. Phase 2: Faction Founding & Activation Sequence

```text
Player reads Start Book (CS_ReadStartBook.psc)
       │
       ▼
Invokes CS_StartFactionMenu.MenuStart()
       │
       ▼
Displays Message Prompt (StartFactionMenu.show())
       │
       ▼
Executes SetStage(0) on GovQuest (CS_FactionGovernment)
       │
       ▼
Initializes Government Quest & Aliases (Steward, ASIC, Blacksmith)
       │
       ▼
Enables Banner Placement & Claiming Activators (CS_ManageAreaMenu)
```

1. **Trigger Mechanism**:
   - The player reads the in-game starter book (`CS_ReadStartBook extends ObjectReference`, listening on `Event OnRead()`).
   - `OnRead()` executes `(GovQuest as CS_StartFactionMenu).MenuStart()`.
2. **Faction Launch Prompt**:
   - `CS_StartFactionMenu` displays the primary message box `StartFactionMenu.show()`.
   - On confirmation (`Response == 0`), it executes `SetStage(0)` on `GovQuest`.
3. **State Initialization**:
   - `CS_FactionGovernment` transitions to Stage 0, spawning initial quest aliases and making the initial faction banner claimable.
   - Triggers the Scaleform ActionScript bridge (`UILIB_1_textInputOpen()`) allowing the player to name their faction.

---

## 4. Phase 3: Steady-State Background Execution

The mod avoids continuous high-frequency frame polling (`OnUpdate()` loops) to prevent Papyrus stack lag. Instead, background simulation uses two distinct timing mechanisms:

### 4.1 Daily Economic & Camp Recovery Timer (`RegisterForUpdateGameTime(24)`)
- Once active, master faction controllers register for 24-game-hour cycles:
  ```papyrus
  GovScriptFunctions.TMImp.RegisterForUpdateGameTime(24)
  GovScriptFunctions.TMSons.RegisterForUpdateGameTime(24)
  ```
- **Operations Executed Every 24 Game Hours**:
  - Iterates through `CS_AllHoldings` FormList.
  - Evaluates daily food yields (`CS_ResourceFoodPerDay*`) and deducts garrison troop rations.
  - Collects local hold taxes and deposits revenue into the `CS_FactionTreasury` global.
  - Evaluates defeated enemy camp recovery cooldown timers (introduced in v0.3.9 to prevent instant camp respawns).

### 4.2 Proximity Cell Attachment (`Event OnCellAttach()`)
- Garrison troops, banner statics, and fortification barricades are not kept continuously loaded in memory across all of Skyrim.
- Instead, attaching to an outpost or fort cell triggers `Event OnCellAttach()` on reference scripts (e.g., `cs_upgradeguardonattach.psc`, `cs_upgradetrooponattach.psc`), dynamically instantiating faction guards and evaluating current weapon/armor tiers.
- `Event OnCellDetach()` cleanly despawns transient actors, releasing actor buffers.

---

## 5. Phase 4: Save Serialization & `OnPlayerLoadGame` Maintenance

Analysis of `cs_checkversion.psc` confirms that *Conquest of Skyrim* maintains active save-game self-healing and migration handlers executed on `Event OnPlayerLoadGame()`:

```text
OnPlayerLoadGame()
   ├── CS_CheckVersion: Evaluates internal GovScriptFunctions.Version
   ├── CS_UpdateFactionName: Re-applies custom name string token to UI
   ├── CS_UpdateGovOutfitsOnLoadGame: Refreshes Jarl and Steward outfits
   ├── CS_TroopEquipUpdateLoad: Evaluates Blacksmith armory stock for recruits
   ├── CS_PlayerSurrenderLoadGame: Cleans captive scene packages
   └── CS_UpdateRacesOnLoad: Refreshes custom soldier race distribution
```

### 5.1 Internal Version Progression & Migration Handlers
`CS_CheckVersion` checks the internal floating-point `GovScriptFunctions.Version`:
- **`< 0.3`**: Blocks execution and displays a modal error message requiring a clean save:
  > *"Conquest of Skyrim: Older version detected. You need to use this update on a save that has never seen the mod..."*
- **`< 0.3001`**: Cleans player faction memberships from `TMImps.ArmyFaction` / `TMSons.ArmyFaction` if at war.
- **`< 0.3003`**: Calls `CourtScript.UpdateTo0Point3Point3()` to refresh court aliases.
- **`< 0.3005`**: Rebinds Second-in-Command (`ASIC`) to the Capital Housecarl alias.
- **`< 0.3007`**: Fails and shuts down vanilla `CWStartQuest` if running, and registers 24-hour game-time updates for Imperial and Stormcloak faction engines.
- **`< 0.3008`**: Invokes `CheckAllHoldings()` if either enemy faction has been eliminated.
- **`< 0.3009`**: Stops and restarts `CourtScript` after a 0.5-second buffer.
- **`< 0.4000` (Commented Out in Source)**: Contains unreleased stub for `GovScriptFunctions.LoyaltyScript.SetUpInitialJarlLoyalties()`, confirming the planned v0.4.0 Jarl Loyalty system.

---

## 6. Phase 5: Shutdown & Uninstallation Implications

*Conquest of Skyrim* cannot be safely uninstalled mid-playthrough after declaring a faction or capturing territory:
1. **Holding State Desynchronization**: Conquered hold palaces and forts have vanilla garrison markers disabled and vanilla Jarls unseated via `CS_TransferOwnershipScript`. Removing the mod leaves these locations permanently unoccupied and prevents vanilla Civil War quest stages from advancing.
2. **Orphaned Papyrus Threads**: Over 1,500 active script instances and 238 quests are serialized into the Skyrim `.ess` save file. Removing the plugin leaves orphaned scripts attempting to resolve missing FormIDs, resulting in severe save bloat and Papyrus VM thread lockups.

---

## 7. Lifecycle Boundaries & State Invariants

| Lifecycle Boundary | State Invariant | Failure Mode if Broken |
| :--- | :--- | :--- |
| **Faction Activation** | Requires clean or non-conflicting Civil War state (`CWStartQuest`). | Starting mid-Civil War locks hold capital palace doors. |
| **Daily Economic Tick** | Runs once every 24 game hours via `RegisterForUpdateGameTime(24)`. | Time-skipping mods (`set timescale`) can accelerate or freeze daily tax cycles. |
| **Cell Loading** | Dynamic actor outfit assignment refreshed on `OnCellAttach()`. | Engine outfit refresh lag causes temporary "naked troops" until cell reload. |
| **Save Load Migration** | `CS_CheckVersion` validates migration state up to internal version 0.3009. | Upgrading from v0.1/v0.2 throws modal abort message. |
| **Endgame The Moot** | Requires all 9 hold booleans in `CS_BattleTrackerScript.Cities[]` to be true. | Logic check bug in v0.3.9 requires console stage advance or community patch #119992. |
