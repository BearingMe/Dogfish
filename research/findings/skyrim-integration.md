# Creation Engine Integration: Conquest of Skyrim

- **Milestone**: M06 — Creation Engine Integration
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Plugin Analyzed**: `conquerskyrim.esp` (Form Version 44, 22,690 records)
- **Status**: CONFIRMED

---

## 1. Overview of Creation Engine Architecture

*Conquest of Skyrim* integrates with the Bethesda Creation Engine not through native C++ DLL hooks, but via deep utilization of core Skyrim Special Edition engine constructs:
1. **Event-Driven Quest Controllers**: Large `QUST` records managing persistent aliases, hold hierarchies, and siege state machines.
2. **Fine-Grained Global State**: 1,275 `GLOB` variables maintaining daily resource equations, tax rates, and diplomatic war scores.
3. **Dynamic AI Packages**: 757 `PACK` records governing army marching formations, patrol routes, hold guards, and surrender behaviors.
4. **Hybrid Dialogue & Scaleform UI**: 689 KB of `DIAL` branches paired with Scaleform (`UIlib v1`) for text entry and dynamic list selection.

---

## 2. Key Record Group Quantitative Analysis

Binary group parsing of `conquerskyrim.esp` reveals 47 distinct top-level GRUP types. Key architectural groups:

| Signature | Record Type | Exact Count | Data Size | Architectural Purpose |
| :--- | :--- | :---: | :---: | :--- |
| `QUST` | Quest Controllers | 238 | 972 KB | Master campaign quests, officer managers, invasion planners, and siege state controllers. |
| `WRLD` | Worldspace Edits | — | 860 KB | Exterior camp placements, siege staging areas, banner markers, and destructible barriers. |
| `DIAL` | Dialogue Topics | — | 689 KB | Custom dialogue trees for Steward, Commander, Blacksmith, Jarls, and soldiers. |
| `PACK` | AI Packages | 757 | 472 KB | Tactical 5x4 formations, 2-column parades, hold patrols, guard duties, and prisoner trailing. |
| `NPC_` | Actor Templates | 1,126 | 394 KB | Officer templates, generic faction recruits, archers, cavalry, and custom court bards. |
| `MESG` | UI Message Boxes | 513 | 298 KB | Banner prompts, capitulation notices, status alerts, and tax adjustment dialogs. |
| `CELL` | Cells (Int/Ext) | — | 244 KB | Palace interiors (Castle Dour, Understone Keep, Palace of the Kings), dungeons, and fort barracks. |
| `LCTN` | Location Records | 168 | 162 KB | Custom and overridden Skyrim locations (holds, redoubts, forts, and outposts). |
| `FLST` | Form Lists | 1,213 | 145 KB | Master arrays: `CS_AllHoldings`, `CS_CityHoldings`, `CS_GarrisonSpawnPoints`, `AttackMarkers`. |
| `SCEN` | Scripted Scenes | 75 | 116 KB | Scripted court hearings, jarl confrontations, peace negotiation summits, and coronations. |
| `GLOB` | Global Variables | 1,275 | 93 KB | Per-settlement daily food rates, faction treasury, war score, battle speed, and battle size. |
| `ACTI` | Activators | — | 92 KB | Interactive war maps, placeable banners, supply chests, and recruitment lecterns. |
| `FACT` | Factions | 183 | 22 KB | Faction relationships, ranks, crime jurisdictions, and military division groups (`CS_PlayerArmyFactionGroup*`). |
| `PERK` | Perks | 71 | 20 KB | Troop combat upgrades (Warrior, Archer, Mage perks) and player authority perks. |
| `KYWD` | Keywords | 223 | 13 KB | Story Manager transfer tags (`CS_StoryTransferHoldingCity`), hold levels, and troop counts. |

---

## 3. Verified Creation Engine Integration Surfaces

### 3.1 Quests & Story Manager (`QUST`, `SMQN`)
- **Master Controllers**:
  - `CWCampaignObj`: Intercepts and overrides vanilla Civil War campaign objectives.
  - `CS_Commander`: Master quest managing all active army divisions, troop movements, and POW registers.
  - `CS_FactionGovernment`: Civil administration quest tracking tax rates, laws, and official appointments.
  - `CS_ManageAreaMenu`: Dynamic location interaction quest binding banners to outpost menus.
- **Story Manager Integration**:
  - Employs `Keyword Property CS_StoryTransferHoldingCity` and `CS_StoryTransferHoldingFort` to trigger event-driven quest nodes when a city or fort changes ownership.

### 3.2 Faction System & Crime Mechanics (`FACT`)
- **Independent Sovereignty**:
  - Player forces are encapsulated under `PlayerArmyFaction` and `PlayerFaction`.
  - Enemy forces are dynamically mapped to `EnemyTroopFaction` during invasions.
- **Crime & Jurisdiction Overrides**:
  - Contains custom crime factions (e.g., `CS_CrimeTalosWorship`, `CS_CrimeThief`).
  - Holds reference array `FormList Property CrimeFactions Auto` to unhook vanilla hold crime factions (`CrimeFactionWhiterun`, `CrimeFactionHaafingar`) upon city conquest, resolving bounties.
- **POW System**:
  - Uses `POWFaction` and `CS_PrisonerFortFaction` with `PrisonerCuffs` armor to transition hostile NPCs into non-hostile captive actors.

### 3.3 Location & Holding Framework (`LCTN`, `FLST`)
- **Holding Registers**:
  - Master FormList `CS_AllHoldings` indexes all conquerable Skyrim locations.
  - Sub-lists `CS_CityHoldings`, `AttackMarkers1`, `AttackMarkers2`, and `JarlMarkers` specify exact coordinate references for siege staging, battering ram approaches, and palace surrender confrontations.
- **Vanilla Location Edits**:
  - Modifies 168 `LCTN` records, linking vanilla holds (e.g., `MarkarthUnderstoneKeepLocation`, `SolitudeCastleDourLocation`) to `CS_TransferOwnershipScript`.

### 3.4 Diplomacy & War Score Engine (`CS_NegotiationScript`)
Direct inspection of `cs_negotiationscript.psc` confirms a dedicated diplomatic war-score subsystem:
- Tracks bilateral war momentum:
  - `GlobalVariable Property WarScore Auto`
  - `GlobalVariable Property WarScoreImp Auto` (Imperial front)
  - `GlobalVariable Property WarScoreSons Auto` (Stormcloak front)
- Diplomatic state globals:
  - `TruceImp` / `TruceSons`: Boolean globals controlling active hostilities.
  - `Reparations`: Gold compensation negotiated during peace summits.
  - `HostProposal`, `HostOffer`, `GuestCounterOffer`, `FinalDecision`: Scaleform-driven dialogue negotiation parameters.

### 3.5 Warfare & Siege Controllers (`CS_AttackCityScript`)
- **Engine Simulation Parameters**:
  - `CS_BattleSize`: Global variable regulating maximum simultaneous combatant actor spawns to prevent engine crash limits.
  - `CS_BattleSpeed`: Global variable regulating AI aggression and reinforcement waves.
- **Siege Equipment Integration**:
  - `FormList Property CatapultsList Auto` & `CatapultOpsList Auto`: Manages dynamic activation of siege artillery.
  - `bool Property BatteringRamEnabled Auto`: Controls exterior gate assault mechanics.
- **Audio & Ambiance Hooks**:
  - Leverages vanilla `MusicType Property MUSCombatCivilWar Auto` alongside custom `Sound Property BattleDrumsSound Auto` and `Armor Property WarDrums Auto`.

---

## 4. Papyrus Virtual Machine & Runtime Event Model

Analysis of the 1,539 Papyrus scripts reveals primary runtime integration hooks:

```text
Game Engine Events
  ├── Event OnCellAttach()         -> Spawns banners, garrison guards on cell enter
  ├── Event OnCellDetach()         -> Despawns transient actors to free memory
  ├── Event OnUpdateGameTime()     -> Daily economic tick (food, taxes, camp recovery)
  ├── Event OnDying(Actor akKiller)-> Updates army casualty counts & POW triggers
  ├── Event OnActivate()           -> War banner & map interaction menus
  └── Event OnTriggerEnter()       -> Proximity detection for battle / ambush starts

Mod Event Communication
  ├── RegisterForModEvent()        -> Binds Papyrus to UIlib Scaleform menu events
  └── SendModEvent()               -> Sends menu responses back to quest scripts
```

- **Daily Economic Ticks**:
  - Uses `RegisterForUpdateGameTime(24.0)` rather than high-frequency polling. Daily updates iterate over `CS_AllHoldings` to calculate food consumption and deposit hold tax revenue.
- **UIlib Scaleform Integration**:
  - Direct invocations of `UILIB_1_textInputOpen()` and `UILIB_1_listMenuOpen()` allow custom player text strings to pass from ActionScript directly into Papyrus string variables.

---

## 5. Interaction Surface with Vanilla Skyrim Actors

| Vanilla Surface | Interaction Mechanism | Risk / Failure Mode |
| :--- | :--- | :--- |
| **Imperial Legion (`CWImperialFaction`, Tullius)** | Scripted faction hostility; negotiation summits in `CS_NegotiationScript`. | High if vanilla Civil War questline is active concurrently. |
| **Stormcloaks (`CWSonsFaction`, Ulfric)** | Scripted faction hostility; surrender dialogue in `CS_AttackCityScript`. | High if vanilla Civil War questline is active concurrently. |
| **Vanilla Jarls** | Replaced via `JarlConfrontScene` and `JarlSurrenderScene`. Native Jarls moved or imprisoned. | Incompatibility if Jarl is marked Essential by an unresolved vanilla quest. |
| **Hold Crime Factions** | Faction ownership cleared and replaced with player crime rules. | Lingering vanilla bounties cause guard hostility if transfer fails to fire. |
| **Skyrim Dragons (DLC2)** | Dragon ally scripts (`DLC2TameDragonFollowPlayerOnGround`, `cs__qf_cs_dragonjoinfaction`). | Dragon recruitment available as late-game faction perk. |

---

## 6. Identified Inferences vs. Confirmed Implementations

- **CONFIRMED**: Full war score, daily resource tracking, Scaleform text input, custom drummer AI, and Jarl confrontation scenes are directly verified via record signatures and Papyrus source code.
- **INFERRED**: Off-screen simulated battle calculation appears to evaluate `TotalTroopCount` and `CS_BattleSize` globals rather than running complex individual actor stat simulations. (Subject to detailed verification in Phase 4).
