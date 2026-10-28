# Persistence & State Consistency Specification: Conquest of Skyrim

- **Milestone**: M26 — Persistence and State Consistency
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_checkversion.psc` (Version evaluation, migration handlers, timer re-registration)
  - `cs_updategovoutfitsonloadgame.psc` (Dynamic outfit refresh for cabinet and jarls)
  - `cs_troopequipupdateload.psc` (Armory and recruit equipment refresh)
  - `cs_playersurrenderloadgame.psc` (Surrender recovery and captive package clearing)
  - `cs_updatefactionname.psc` (Custom string proxy re-attachment)
- **Status**: CONFIRMED

---

## 1. Savegame Serialization Architecture

Skyrim Special Edition serializes persistent mod state into `.ess` binary save containers. In *Conquest of Skyrim*, persistent state is partitioned across four engine constructs:

```text
Creation Engine .ess Save Container
  │
  ├── 1. Global Variables (1,275 GLOB Records)
  │     ├── Faction Treasury (CS_FactionTreasury), WarScore, IsHighKing
  │     ├── Simulation Throttles: CS_BattleSize, CS_BattleSpeed
  │     └── Daily Hold Yields: CS_ResourceFoodPerDay*, TaxRate, FactionColor
  │
  ├── 2. Running Quest Stages & Variables (238 QUST Records)
  │     ├── Master state machines (GovQuest, CS_Commander, CWCampaignObj)
  │     └── Boolean ownership arrays: Cities[9], Forts[], Camps[] (CS_BattleTracker)
  │
  ├── 3. Dynamic ReferenceAliases (88 ReferenceAlias Scripts)
  │     ├── Executive Cabinet: Steward, ASIC, Blacksmith, CourtMage
  │     ├── Hold Governors: JarlAliases[0..8], StewardAliases[0..8]
  │     └── Active Division Rosters: Commanders[0..9]
  │
  └── 4. In-World Placed ObjectReferences
        ├── PlacedBanners FormList (Spawned with abForcePersist=true)
        └── Claimed Outpost Containers (FactionNameChestObject, ChestObject)
```

---

## 2. Reload Initialization & Self-Healing (`OnPlayerLoadGame`)

Because Skyrim does not persist volatile 3D models, animation states, or dynamic string tokens identically across reloads, the mod executes a coordinated suite of **self-healing scripts** on `Event OnPlayerLoadGame()`:

```text
Player Loads Save Game (OnPlayerLoadGame)
  │
  ├── CS_CheckVersion: Validates internal GovScriptFunctions.Version
  │     ├── If < 0.3: Displays modal error demanding clean save
  │     ├── If < 0.3007: Resets & re-registers TMImp / TMSons 24h GameTime timers
  │     └── If < 0.3009: Restarts CourtScript after 0.5s buffer
  │
  ├── CS_UpdateFactionName: Re-attaches custom text string to UI name proxies
  │
  ├── CS_UpdateGovOutfitsOnLoadGame: Re-equips Jarl and Cabinet regalia
  │
  ├── CS_TroopEquipUpdateLoad: Re-evaluates armory inventory lists
  │
  └── CS_PlayerSurrenderLoadGame: Sets stage 100 to abort captive scene freezes
```

---

## 3. Save/Reload Scenario Analysis & Failure Modes

### 3.1 Steady-State Reload (Peace / Routine Administration)
- **Outcome**: `STABLE`.
- **Behavior**: All global variables and holding ownerships restore cleanly. Daily economic timers resume their 24-hour cycle via `CS_CheckVersion`.

### 3.2 Reloading During an Active Siege Assault
- **Outcome**: `HIGH RISK / UNSTABLE`.
- **Vulnerability**:
  - Saving while armies are assaulting a city exterior or breaching a gate can cause Havok destructible barriers (battering ram gates) to reset to default closed positions without resetting quest stage objectives.
  - Spawning wave counters (`TotalDefenderDeaths`) can desynchronize, causing reinforcement waves to cease spawning and locking the invasion in an uncompletable state.
- **Rule**: Players should avoid saving mid-combat during an active city assault.

### 3.3 Reloading During Diplomatic Summits
- **Outcome**: `MODERATE RISK`.
- **Vulnerability**: `CS_NegotiationScript` relies on package evaluations (`EvaluateNegotiatingPackages()`). Reloading while seated at the neutral meeting table can occasionally break actor AI packages, causing Tullius or Ulfric to stand up and walk away.

### 3.4 Reloading During Player Surrender
- **Outcome**: `HEALED AUTOMATICALLY`.
- **Recovery**: `CS_PlayerSurrenderLoadGame` specifically hooks `OnPlayerLoadGame()` and forces `SetStage(100)` on the surrender controller, bypassing broken animation locks.

---

## 4. Uninstallation & Mod Removal Risks

Removing `conquerskyrim.esp` mid-playthrough results in **irreparable savegame corruption**:
1. **Orphaned Papyrus Threads**:
   - Over 1,500 active script instances remain embedded in the `.ess` save, continuously attempting to resolve deleted FormIDs, resulting in severe frame rate stutter and VM heap crashes.
2. **Permanent Hold Broken State**:
   - Conquered hold capitals have had vanilla Imperial/Stormcloak garrison markers and Jarls disabled by `CS_TransferOwnershipScript`.
   - Removing the mod does not restore vanilla markers; hold capitals remain permanently devoid of guards, unseating Skyrim's Civil War questline entirely.

---

## 5. Persistence Boundaries & Clean Save Invariants

| Component | Persistence Mechanism | Clean Save Required to Alter? |
| :--- | :--- | :---: |
| **Faction Identity** | Base dummy object renaming (`SetName`) | No (re-applied on load via `CS_UpdateFactionName`) |
| **Territory Ownership** | `AllHoldings` Globals + `CWOwner` Keyword Data | Yes (major structural changes break save state) |
| **Military Divisions** | `CS_Commander` Quest ReferenceAliases | Yes |
| **Economic Upgrades** | Location Keyword Data (`FarmLevel`, `MineLevel`) | No (persistent on Location forms) |
| **Major Mod Upgrades** | Internal version check (`GovScriptFunctions.Version`) | **YES** (Upgrades from v0.1/v0.2 to v0.3 require a new game) |
