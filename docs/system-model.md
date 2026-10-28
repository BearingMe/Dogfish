# Recovered System Model: Conquest of Skyrim

- **Document**: Descriptive Architecture & Conceptual Model
- **Milestone**: M30 — Recovered System Model
- **Subject**: *Conquest of Skyrim*
- **Investigated Baseline**: `v0.3.9` (March 23, 2023 release by `galvatron304`)
- **Status**: COMPLETE & VERIFIED

---

## 1. System Decomposition & Component Architecture

*Conquest of Skyrim* is structured as a hub-and-spoke operational engine hosted within the Skyrim Special Edition runtime. The architecture decouples into seven primary operational subsystems:

```text
                                 ┌─────────────────────────────────┐
                                 │       Scaleform UI Bridge       │
                                 │  (UIlib v1 Flash Menus & Input) │
                                 └───────────────┬─────────────────┘
                                                 │ ModEvents
                                                 ▼
┌──────────────────────────┐     ┌─────────────────────────────────┐     ┌──────────────────────────┐
│   Territorial Engine     │◄───►│    Central Governance Hub       │◄───►│     Economic Engine      │
│  (Holding & Fort Swaps)  │     │  (CS_GovScriptFunctions.psc)    │     │(Production & Demographics│
└────────────┬─────────────┘     └───────────────┬─────────────────┘     └────────────┬─────────────┘
             │                                   │                                    │
             │ Adjacency Data                    │ Treaty Summits                     │ Food & Wages
             ▼                                   ▼                                    ▼
┌──────────────────────────┐     ┌─────────────────────────────────┐     ┌──────────────────────────┐
│     Military Command     │◄───►│       Diplomatic Engine         │     │     Judicial System      │
│ (10 Armies & TaskBarrels)│     │     (CS_NegotiationScript)      │     │  (CS_CourtScript & Jails)│
└────────────┬─────────────┘     └─────────────────────────────────┘     └──────────────────────────┘
             │
             ▼
┌──────────────────────────┐
│ Siege & Battle Simulator │
│(Wave Spawns & Throttling)│
└──────────────────────────┘
```

### Subsystem Responsibilities
1. **Central Governance Hub (`CS_GovScriptFunctions`)**: Master state coordinator. Owns executive cabinet aliases (Steward, General, Blacksmith, Mage), throne activators, royal capital coordinates, and version migration loops.
2. **Territorial Engine (`CS_TransferOwnershipScript`, `CS_BattleTrackerScript`)**: Owns holding state arrays (`Cities[]`, `Forts[]`, `Camps[]`), 3D banner swaps at scale 1.53, and jurisdiction resets.
3. **Economic Engine (`CS_HoldingEconomyUpdaterScript`, `CS_EconomyVariableFunctions`)**: Simulates 24-game-hour production cycles, facility levels (1..4), demographic labor allocation, and tiered taxation.
4. **Military Command (`CS_CommanderMovingScript`, `CS_PatrolHoldScript`)**: Manages up to 10 army divisions using the Task Barrel AI proxy pattern, hold distance transit calculations, and demographic recruitment filters.
5. **Siege & Battle Simulator (`CS_AttackCityScript`, `CS_AttackCitySiegeScript`)**: Executes multi-stage city breaches, battering ram operations, catapult batteries, and wave reinforcements throttled by `CS_BattleSize`.
6. **Diplomatic Engine (`CS_NegotiationScript`)**: Models bilateral war scores, summit negotiations, and non-aggression truces with Tullius and Ulfric.
7. **Judicial System (`CS_CourtScript`, `CS_PrisonScript`)**: Evaluates seven crime categories, hold jail confinement, and chopping block executions.

---

## 2. Domain Entity-Relationship Model (ERD)

The domain model connects the persistent entities operating in the worldspace:

```text
 [Monarch: Player] 1 ──── 1 [Faction Identity: FactionNameChestObject]
       │
       ├──── 1..* [Cabinet Officers: Steward, ASIC, Blacksmith, Mage, Spymaster]
       │
       ├──── 1..10 [Army Divisions: Commanders[0..9]]
       │              │
       │              ├──── 1..* [Soldier Units: Infantry, Archers, Mages, Dwemer, Beasts]
       │              │
       │              └──── 1 [Standardized Armory Form: CustomOutfit]
       │
       └──── 1..* [Conquered Holdings: AllHoldings]
                      │
                      ├──── 1 [Hold Capital: citynum 0..8]
                      │        │
                      │        ├──── 0..1 [Appointed Jarl: JarlAliases[citynum]]
                      │        ├──── 0..1 [Municipal Palace Throne]
                      │        └──── 0..* [Dungeon Prisoners: Prisoners[citynum]]
                      │
                      ├──── 0..* [Dependent Villages: villagenum 9..14]
                      │
                      ├──── 0..* [Strategic Fortresses: AllHoldingsFort]
                      │
                      └──── 1..5 [Productive Facilities: Farm, Mine, Mill, Docks, Barracks]
                                    │
                                    └──── 1..4 [Facility Level: 1..4]
```

---

## 3. Core State Machines

### 3.1 Territorial Holding Lifecycle
```text
[ Unowned / Bandit Held ]
          │ Player clears cell & plants banner (BannerSpell)
          ▼
[ Claimed Frontier Outpost ] (Global = 1, CWOwner = 69)
          │ Contiguous border established & army dispatched
          ▼
[ Active Staging Camp ] (CampMarker1/2 added to map)
          │ Siege assault initiated
          ▼
[ Contested War Zone ] (BattleTracker.Cities[i] = true)
          │ Street defenders routed & palace breached
          ▼
[ Conquered Sovereign Holding ]
          │ Governor appointed -> Crime reset -> Production active
          ▼
[ Stable Integrated Hold ] ────(Loyalty < 20)────► [ Armed Secession Revolt ]
```

### 3.2 Diplomatic Relationship State Machine
```text
                 ┌───────────────┐
                 │    Neutral    │
                 └───────┬───────┘
                         │ Declaration of War (Steward letter)
                         ▼
                 ┌───────────────┐
      ┌─────────►│   Hostility   │◄────────┐
      │          └───────┬───────┘         │
      │ Summit           │ Truce Accepted  │ Truce Violated
      │ Failed           ▼ (MakeFriends)   │ (BreakTruce)
      │          ┌───────────────┐         │
      └──────────┤ Bilateral     ├─────────┘
                 │ Truce         │
                 └───────┬───────┘
                         │ All 9 Holds Conquered
                         ▼
                 ┌───────────────┐
                 │   Defeated    │
                 └───────────────┘
```

---

## 4. Data & Resource Flow Model

```text
[ Civilian Population ]
         │
         ├───────────────────────(Conscription)────────────────────────┐
         │                                                             │
         ▼                                                             ▼
[ Unemployed Labor ]                                           [ Military Recruits ]
         │                                                             │
         ├──────► Agricultural Farms ──────► Daily Food Produced        │
         │                                         │                   │
         ├──────► Maritime Docks     ──────► Trade Docks Yield         │
         │                                         │                   │
         ├──────► Lumber Mills       ──────► Raw Timber Stockpile      │
         │                                         │                   │
         └──────► Mining Outposts    ──────► Raw Metal Stockpile ──────┤
                                                   │                   │
                                                   ▼                   ▼
                                          [ Faction Treasury ] ──► [ Army Upkeep ]
                                                   │             (Wages & Rations)
                                                   ▼
                                       [ Upgrades & Research ]
                                       (Facilities & PERK Tiers)
```

---

## 5. Event Distribution & Scheduling Topology

*Conquest of Skyrim* utilizes event-driven distribution rather than frame-by-frame polling:

1. **Daily Game-Time Clock (`RegisterForUpdateGameTime(24.0)`)**:
   - Master economic evaluation tick.
   - Computes gross production, civilian consumption, military upkeep, and holding loyalty adjustments.
2. **Cell Attachment Hooks (`Event OnCellAttach()`)**:
   - Spatially triggered when the player enters a location cell.
   - Dynamically evaluates dynamic equipment forms, instantiates stationed guards, and spawns 3D banner meshes.
   - `OnCellDetach()` unloads transient actors to free memory.
3. **Combat Death Signals (`Event OnDying()`)**:
   - Attaches to combatants via `CS_AttackCityTroopDeathTrigger`.
   - Decrements division troop totals by `CS_BattleSpeed` and checks reinforcement wave milestones.
4. **Scaleform Asynchronous Bridge (`RegisterForModEvent()`)**:
   - Captures UIlib text input strings and list selections asynchronously, preventing engine freezing.
5. **Story Manager Nodes (`SMQN`)**:
   - Injects holding transfer and appointment events directly into Skyrim's quest engine (`CS_StoryTransferHoldingCity`, `CS_StoryJarlChooseGov`).

---

## 6. Persistence & Serialization Boundaries

| Layer | Storage Destination | Volatility & Restoration Mechanism |
| :--- | :--- | :--- |
| **Numerical State** | `.ess` Save Container (1,275 Globals) | Persistent; holds treasury, war scores, daily rates, and battle throttles. |
| **Quest Progression** | `.ess` Save Container (238 Quests) | Persistent; running quest stage integers and active objectives. |
| **Actor Pointers** | `.ess` Save Container (88 ReferenceAliases) | Persistent; FormIDs linked to commanders, stewards, jarls, and prisoners. |
| **Volatile 3D Meshes** | Game Memory Heap | Transient; 3D models and anims reconstructed upon cell load. |
| **Custom String Text** | Proxy Base Containers (`SetName`) | Re-propagated on save load via `CS_UpdateFactionName.OnPlayerLoadGame()`. |
| **Dynamic Outfits** | Actor Outfits | Refreshed on load via `CS_UpdateGovOutfitsOnLoadGame` and `CS_TroopEquipUpdateLoad`. |

---

## 7. Engine Integration Topology & Overridden Master Nodes

```text
Skyrim.esm Native Root
  ├── QUST: CWCampaignObj (0x00026F45)       <-- Overridden to hijack civil war objectives
  ├── SMQN: CWChangeLocationScenes (0x000D5176)<-- Overridden to trigger custom hold transfers
  ├── NPC_: General Tullius (0x0001336B)      <-- Injected with negotiation & truce dialogue
  ├── NPC_: Ulfric Stormcloak (0x0010F9B9)   <-- Injected with negotiation & truce dialogue
  ├── LCTN: 57 Hold & Fort Locations         <-- Injected with CWOwner keyword hooks (69)
  └── WRLD: Tamriel Exterior (0x0000003C)    <-- Added staging camps & siege apparatus
```

---

## 8. System Invariants & Architectural Constraints

1. **Territorial Contiguity Invariant**: Armies cannot invade non-adjacent holds (`HoldsNotLinked`).
2. **Actor Memory Throttling**: Concurrent in-cell combatants are strictly clamped to `CS_BattleSize` (default: 40) to prevent Papyrus heap crashes.
3. **Single Faction Gate**: `FactionStarted` acts as a one-way gate (0 &rarr; 1), preventing multi-faction desync.
4. **Clean Save Invariant**: Upgrades across major architectural boundaries (v0.1/v0.2 &rarr; v0.3) require a new save due to Papyrus script instance serialization.
5. **Irrecoverable Removal**: Mid-game uninstallation permanently breaks hold palace markers and leaves orphaned Papyrus threads.
