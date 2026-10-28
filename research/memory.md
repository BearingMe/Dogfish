# Research Memory Index

## Project Identity & Target Version
- **Status**: CONFIRMED
- **Finding**: *Conquest of Skyrim* was created by `galvatron304` (`galvatron530`), first released March 28, 2022 on Nexus Mods (#65628). Target primary research version is `v0.3.9` (released March 23, 2023). Official development concluded in 2024 when the author released project files to the community.
- **Evidence**: `research/findings/project-overview.md`
- **Related**: M01, M02, M05

## Documentation Landscape & Architectural Shift
- **Status**: CONFIRMED
- **Finding**: Documentation is decentralized across Nexus #65628, Reddit, Patreon, and Discord. In v0.3.x, the author moved operational management away from early MCM menus into native Papyrus dialogue trees with recruited officers (Steward for civil/diplomatic, Commander for military). Upgrading across major versions (0.1 -> 0.2 -> 0.3) strictly broke save game compatibility.
- **Evidence**: `research/findings/documentation-inventory.md`
- **Related**: M02, M03, M08, M12

## Comprehensive Feature Architecture
- **Status**: CONFIRMED
- **Finding**: *Conquest of Skyrim* v0.3.9 encompasses 51 individually tracked features across 18 strategic domains (`FEAT-FACT` through `FEAT-UI`). The core gameplay loop links Faction Creation -> Territory -> Outposts/Resources -> Military Recruitment/Armies -> War Declaration -> City Sieges -> Hold Jarl Appointments -> The Moot Coronation.
- **Evidence**: `research/findings/feature-inventory.md`
- **Related**: M03, M09-M24, M29

## Critical Stability Failure Modes & Incompatibilities
- **Status**: CONFIRMED
- **Finding**: *Conquest of Skyrim* has severe conflicts with vanilla Civil War quests (`CW01`/`CW02`/`Season Unending`), *Open Civil War*, and external follower frameworks (NFF/AFT). Major v0.3.9 defects include an internal hold-count mismatch blocking The Moot (`ISSUE-QST-01`), proximity trigger failures at Fort Amol (`ISSUE-QST-02`), and naked troop outfit evaluation delays (`ISSUE-GAME-01`).
- **Evidence**: `research/findings/known-issues.md`
- **Related**: M04, M08, M27, M28

## Physical Package Structure & Master Dependencies
- **Status**: CONFIRMED
- **Finding**: *Conquest of Skyrim* v0.3.9 distributes 3,779 loose files (521.93 MB uncompressed). Plugin `conquerskyrim.esp` is a Form Version 44 database with 22,690 records, requiring all official DLC masters (`Skyrim.esm`, `Update.esm`, `Dawnguard.esm`, `HearthFires.esm`, `Dragonborn.esm`). It bundles 1,539 compiled Papyrus scripts (`.pex`), 1,539 matching sources (`.psc`), UIlib v1 Scaleform menus, and custom xWMA audio for parade/battle drums.
- **Evidence**: `research/findings/file-inventory.md`
- **Related**: M05, M06, M07

## Creation Engine Integration Surfaces
- **Status**: CONFIRMED
- **Finding**: The mod hooks 47 Creation Engine record groups, prominently 238 quests (`QUST`), 1,275 globals (`GLOB`), 1,213 formlists (`FLST`), 1,126 actors (`NPC_`), 757 AI packages (`PACK`), and 183 factions (`FACT`). Core mechanics utilize `CS_NegotiationScript` (bilateral `WarScore` calculation engine), `CS_AttackCityScript` (`CS_BattleSize` throttling, catapults, battering rams), and `UIlib v1` Scaleform ModEvent messaging for text input and menu selection.
- **Evidence**: `research/findings/skyrim-integration.md`
- **Related**: M06, M07, M12, M15, M23

## Static Architecture & Controller Graph
- **Status**: CONFIRMED
- **Finding**: Static system structure is governed by 5 central quest controllers: `CS_GovScriptFunctions` (civil cabinet, thrones, executions), `CS_AttackPlanningNewScript` (invasion logistics, hold contiguity check), `CS_BattleTrackerScript` (boolean ownership arrays for cities, forts, camps), `CS_TransferOwnershipScript` (atomic marker/crime swapping across `AllHoldings`), and `CS_AttackCityScript` (siege lifecycle). Papyrus scripts consist primarily of 935 `TopicInfo` dialogue responses and 254 `Quest` state managers.
- **Evidence**: `research/findings/static-structure.md`
- **Related**: M07, M08, M09, M10, M14, M16

## Runtime Lifecycle & State Persistence
- **Status**: CONFIRMED
- **Finding**: Faction founding triggers via `CS_ReadStartBook.OnRead()` -> `CS_StartFactionMenu.MenuStart()` -> `SetStage(0)` on `GovQuest`. Background economy runs on 24-game-hour cycles (`RegisterForUpdateGameTime(24)`). On save reload, `CS_CheckVersion` executes internal version migrations (up to v0.3009, with a commented-out v0.4000 Jarl loyalty stub), and `CS_UpdateGovOutfitsOnLoadGame` refreshes appointed official outfits. Mid-game uninstallation corrupts hold ownership and leaves orphaned Papyrus threads.
- **Evidence**: `research/findings/runtime-lifecycle.md`
- **Related**: M08, M09, M17, M26, M28

## Faction Initialization Workflow & State Architecture
- **Status**: CONFIRMED
- **Finding**: Faction creation executes at `GovQuest` Stage 0: grants `BannerSpell` & `SettingsMenu`, opens Scaleform `UIlib_1` text prompts via `CS_SelectName` to rename proxy containers (`FactionNameChestObject` & `ChestObject`), teleports player to `BannerRoomTeleport` to swap 3D banners at scale 1.53 via `CS_ChangeBanners`, and initializes Skyrim holdings to Imperial (11) or Stormcloak (12) via `SetUpFactionOwners()`. Treasury starts at 0; player must recruit a Steward (free for friends, 100g/day for neutral NPCs), Second-in-Command (ASIC), and Blacksmith.
- **Evidence**: `research/findings/faction-creation.md`
- **Related**: M09, M10, M12, M13, M21

## Territorial Ownership & Holding State Machine
- **Status**: CONFIRMED
- **Finding**: Skyrim's territory is organized into a 4-tier hierarchy: 9 Hold Capitals (`citynum` 0..8, where 0..4 are major walled and 5..8 are minor open cities), Strategic Forts, Dependent Villages, and Frontier Outposts. State is dual-encoded: holding global variables use 1 (Player), 11 (Imperial), 12 (Stormcloak); native `CWOwner` keyword uses 69 for Player and `(newfaction - 10)` for vanilla factions. Transitions in `CS_TransferOwnershipScript` atomically clear local hold crime gold to 0, unseat Jarls to `CitizenFaction`, force enemy capital relocation (`ChooseRandomCapital()`), and enforce contiguous borders via `MessageNoHoldLink`.
- **Evidence**: `research/findings/territory.md`
- **Related**: M10, M11, M14, M16, M22, M24

## Campaign Progression & Endgame Victory Loop
- **Status**: CONFIRMED
- **Finding**: Progression spans 6 tiers: Outlaw Faction Founding -> Outpost Expansion -> Contiguous Hold Invasions -> Capital City Sieges -> The Moot (`CS_MeetingScript`) -> Royal Coronation (`CS_CoronationScript`). The Moot requires all 9 holds conquered; an appointed Jarl with >=60 loyalty nominates the player (otherwise self-nomination fallback). Jarls cast roll-call votes based on hold loyalty keyword; victory requires `MootVoteFor >= MootVoteAgainst`. Coronation stages Jarl/citizen kneeling animations, fealty oaths (+30 loyalty per hold), awards the High King/Queen crown, and sets `IsHighKing = 1`.
- **Evidence**: `research/findings/progression.md`
- **Related**: M11, M21, M22, M23, M29

## Player Interaction Surfaces & Control Modalities
- **Status**: CONFIRMED
- **Finding**: Player command is structured across four primary modalities: (1) Immersive NPC dialogue trees (935 `TopicInfo` scripts for Steward civil/taxes, Commander invasion/POWs, Blacksmith armory); (2) Interactive world activators (`CS_ManageHolding` on placed banners, thrones, chopping blocks); (3) Tactical war tables (`CS_MapManageMenuScript` checking contiguous adjacency `HoldsNotLinked` and troop superiority `MessageNoSuperiority`); and (4) Scaleform `UIlib v1` text/list prompts for custom naming. Configuration is governed by SkyUI MCM sliders (`CS_BattleSize`, `CS_BattleSpeed`) with an inventory `SettingsMenu` item fallback for gamepad/console users.
- **Evidence**: `research/findings/interactions.md`
- **Related**: M12, M03, M06, M09, M14, M17

## Military Organization & Unit Architecture
- **Status**: CONFIRMED
- **Finding**: The military system supports up to 10 operational field divisions (`MaxArmies = 10`), commanded by unique NPCs (free for friends, 100g/day for neutral NPCs; external follower frameworks corrupt AI packages). Unit rosters span standard infantry (`CountSoldier`, `CountWarrior`, `CountArcher`, `CountCrossbow`), 6 magic disciplines (Fire, Frost, Shock, Restoration, Conjuration, Alteration), 4 Dwemer automatons (Spider, Sphere, Ballista, Centurion), war beasts, and optional vampire hosts. Recruitment filters allow toggling playable races and genders via `CS_ChooseTroopRacesScript`. Equipment standardization uses a container-proxy pattern (`CS_CommanderOutfitScript`) across vanilla tiers or custom armor sets, sustained by daily food upkeep (`TroopCountFood`) and optional `AutoRecruitment`.
- **Evidence**: `research/findings/military-organization.md`
- **Related**: M13, M14, M15, M16, M20

## Military Dispatch & AI Package Anchor Architecture
- **Status**: CONFIRMED
- **Finding**: Military orders (Follow, Hold Patrol, Fort Garrison, City Garrison, Staging Camp, Hunt, Offensive Assault, Stand By) are dispatched via an invisible "Task Barrel" proxy pattern (`CS_PatrolHoldScript`) that alters AI package routines instantly without Papyrus re-evaluation loops. Transit duration between holds is computed via spatial distance markers (`DistanceMarker*`); off-screen arrivals automatically teleport commanders to assigned duty markers (`MoveTo`) to prevent wilderness grid stalls. Tactical 5x4 blocks and parade marches use relative offset AI packages.
- **Evidence**: `research/findings/military-orders.md`
- **Related**: M14, M15, M16, M27

## Combat Simulation & Reinforcement Wave Engine
- **Status**: CONFIRMED
- **Finding**: Combat engagements throttle concurrent in-cell actor spawns via `CS_BattleSize` (default: 40) to prevent engine memory crashes, while attrition scales by `CS_BattleSpeed` on `OnDying` events. Reinforcements spawn in coordinated waves at casualty milestones (20, 40, 60, 80, 100 defender deaths) rather than continuous trickles. When defeated, surviving enemies receive `RetreatFaction` AI packages to flee, and captives are converted to POWs via `PrisonerScript.PlayerTakePrisoner()`. Off-screen non-player battle resolution formulas remain unverified.
- **Evidence**: `research/findings/battles.md`
- **Related**: M15, M13, M14, M16, M28

## Siege Warfare Architecture & Capitulation Sequences
- **Status**: CONFIRMED
- **Finding**: Sieges execute through `CS_AttackCitySiegeScript` across 6 phases: (1) Precondition validation (land contiguity and military superiority); (2) Staging camp setup (`CampMarker1`/`CampMarker2` added to world map); (3) Outer gate assault with optional pincer flanking (`DoubleAttack`) and artillery (`CatapultsList`, `BatteringRamEnabled`); (4) Urban street combat balancing melee, ranged, healers, and buffers; (5) Palace breach into the seat of government triggering `JarlConfrontScene` and `JarlSurrenderScene`; and (6) Atomic holding transfer (`TransferOwnershipCity` to faction 69) and victory parade.
- **Evidence**: `research/findings/sieges.md`
- **Related**: M16, M10, M14, M15, M22, M27

## Regional Economic Simulation & Production Formulas
- **Status**: CONFIRMED
- **Finding**: Economy is simulated across 5 resources (Gold, Food, Metal, Wood, Trade Docks). Production is calculated via facility levels multiplied by base holding change (`FoodMax = FarmLevel * FoodChange`, `MetalMax = MineLevel * MetalChange`, etc.). Population growth evaluates `PopIncrease = random(0.3*Deficit, 0.5*Deficit)` where `Deficit = MaxJobs - 0.25*Pop`. Labor allocation strictly prioritizes Food first, Docks second, Wood third, Metal fourth. Major cities start at Pop 50 / FoodRequired 50 with distinct endowments (Whiterun is agricultural breadbasket with Farm 4/Yield 10; Markarth is mining capital with Mine 4/Yield 7; Solitude is trade capital with Docks 4/Yield 10).
- **Evidence**: `research/findings/economy.md`
- **Related**: M17, M18, M19, M21

## Demographic Data Model & Conscription Mathematics
- **Status**: CONFIRMED
- **Finding**: Population is tracked abstractly via location keywords (`Population`, `Unemployed`, `FoodRequired`, `CS_RecruitablePercentage`). Food requirement scales as `FoodRequired = Population + CS_NumGuards`. Conscription draws directly from the labor reserve (`RecruitableTroops = ceiling(RecruitablePercentage * Unemployed)`), deducting from both `Population` and `Unemployed`. Daily tax revenue scales with population and tier: Major Capitals yield `500*Pop + 250*Pop*TaxRate`, Minor Cities yield `300*Pop + 150*Pop*TaxRate`, and Villages yield `200*Pop + 100*Pop*TaxRate`. Local holdings store up to 5 surplus resource units before pooling into the national stockpile.
- **Evidence**: `research/findings/population.md`
- **Related**: M18, M13, M17, M19, M24

## Infrastructure Architecture & Construction Mechanics
- **Status**: CONFIRMED
- **Finding**: Holdings support 4-tier facility upgrades across 5 types: Farms (`FarmLevel`), Mines (`MineLevel`), Lumber Mills (`LumberMillLevel`), Docks (`DocksLevel`), and Barracks (`BarracksLevel`). Upgrades cost Treasury gold (`UpgradeCost`) and Wood reserves (`UpgradeCostWood`). Upgrading invokes `AddJobs()`, which immediately expands open jobs by the holding's base rate (`FoodChange`, `WoodChange`, `MetalChange`), raising the `MaxJobs` employment ceiling and driving subsequent population growth. Supported locations include all 9 hold capitals (`citynum 0..8`) and 6 major villages (`villagenum 9..14`: Riverwood, Rorikstead, Dragon Bridge, Ivarstead, Shor's Stone, Karthwasten).
- **Evidence**: `research/findings/infrastructure.md`
- **Related**: M19, M17, M18, M20

## Technology Progression & Research Equations
- **Status**: CONFIRMED
- **Finding**: Technology advances across 71 `PERK` records in 4 branches: Martial (10 levels for Soldier, Warrior, Archer, Crossbow, Guard), Arcane (Levels 1-5 for Destruction Fire/Frost/Shock), Dwemer Engineering (4 automaton tiers), and Dragon Mastery (`CS_SpeakToTamedDragonPerk`). Martial upgrade costs scale linearly: $\text{CostGold} = 5000 + (\text{Level}-1) \times 10000$ and $\text{CostMetal} = 50 + (\text{Level}-1) \times 100$, immediately incrementing `TroopPowerLevels` and unlocking tier gear in `UnlocksHeavy[]`/`UnlocksLight[]`.
- **Evidence**: `research/findings/technology.md`
- **Related**: M20, M13, M17, M19, M29

## Royal Cabinet & Judicial Administration Framework
- **Status**: CONFIRMED
- **Finding**: Sovereign government operates through a central cabinet (Steward for civil/treasury, Second-in-Command for warfare, Spymaster for covert spy networks and coups in `CS_IntelReportScript`, Blacksmith for armory supply, Court Mage for arcane wards, Court Bard for ceremonies, Executioner for capital trials) and provincial Jarls across the 9 holds. Judicial administration in `CS_CourtScript` allows convening court from royal thrones to judge seven legal categories (Theft, Assault, Murder, Speaking Out, Worship, Treason, Contraband) resulting in fines, dungeon imprisonment, or chopping block executions (`ExecutionQuest`).
- **Evidence**: `research/findings/government.md`
- **Related**: M21, M09, M22, M23, M24

## Jarlship Administration & Stability Mechanics
- **Status**: CONFIRMED
- **Finding**: Hold administration is delegated to appointed Jarls (`JarlAliases[0..8]`) in `CS_RecruitJarl`, dispatched via Story Manager event `CS_StoryJarlChooseGov`. Stability is tracked via dual keyword channels: `Loyalty` (public approval) and `JarlLoyalty` (ruler fealty). Negative modifiers include `NoFood`, `NoGarrison`, `NoCourt`, and high taxes; positive modifiers include `Parties` and `ArmiesDefending`. When hold `Loyalty` drops below 40, `ProtestQuest` initiates civilian demonstrations outside the palace; below 20 triggers `RebelDefendQuestStarted` armed secession. Deposed native Jarls are stripped of ruling packages and added to `CitizenFaction`.
- **Evidence**: `research/findings/jarls.md`
- **Related**: M22, M10, M11, M21, M24

## Diplomatic Framework & Truce Probability Formula
- **Status**: CONFIRMED
- **Finding**: Faction relations exist across four bilateral states: Neutral -> War <-> Truce -> Defeated. Formal joint-military alliances are UNIMPLEMENTED; diplomacy is restricted to non-aggression truces. In `CS_NegotiationScript`, treaty acceptance calculates: $\text{Chance} = 25\% \pm 25\% (\text{Relationship}) \pm 25\%/50\% (\text{Ideology}) \pm 25\% (\text{Army Size} < 60 \text{ vs } \ge 100)$. A successful truce temporarily removes `CWImperialFaction` or `CWSonsFaction` tags and halts AI invasion timers. POWs can be ransomed for gold via Commander dialogue or executed at the cost of diplomatic goodwill.
- **Evidence**: `research/findings/diplomacy.md`
- **Related**: M23, M11, M14, M21, M24

## Cross-System Dependency Graph & Execution Ordering
- **Status**: CONFIRMED
- **Finding**: The strategic simulation couples 10 core subsystems governed by shared globals (`CS_FactionTreasury`, `AllHoldings` 1/11/12, `CWOwner` 69, `Cities[]` boolean array, `WarScore`, `IsHighKing`). Daily 24h game-time updates execute in strict sequence: Production -> Domestic Sustenance (`Pop + Guards`) -> Military Upkeep (`TroopCountFood` + wages) -> 5-unit local reserve pooling -> Stability/Loyalty modifiers -> AI offensives. Circular feedback loops can trigger economic death spirals (Famine Spiral and Taxation/Mutiny Spiral).
- **Evidence**: `research/findings/system-dependencies.md`
- **Related**: M25, M10, M13, M17, M18, M22, M26

## Sovereign Legal System & Off-Screen Combat Equations
- **Status**: CONFIRMED
- **Finding**: Sovereign law operates across 7 crime categories (Theft, Assault, Murder, Speaking Out, Worship, Treason, Contraband) judged during throne court sessions (`CS_CourtScript`), sentencing offenders to fines, municipal hold jails (`CS_PrisonScript`), or chopping block decapitations (`ExecutionQuest`). In `CS_BountyScript`, off-screen detachment engagements simulate casualties using exact mathematical ratios: $\text{Chance} = \text{Troops} / \text{TroopCertain}$; if $\text{Chance} > \text{Roll}$, $\text{Dead} = \lfloor (1 - (\text{Chance} - \text{Roll})) \times \text{Troops} \rfloor$; if defeated, all dispatched troops are lost. Conquered holds zero out vanilla `CrimeFaction*` gold to establish independent jurisdiction.
- **Evidence**: `research/findings/law-and-stability.md`
- **Related**: M24, M15, M21, M22, M27

## Savegame Persistence & Self-Healing Architecture
- **Status**: CONFIRMED
- **Finding**: State persists across 1,275 globals, 238 running quests, and 88 reference aliases in `.ess` save files. Volatile 3D and string state is repaired on `OnPlayerLoadGame` via coordinated self-healing scripts: `CS_CheckVersion` (version migrations, timer restarts), `CS_UpdateFactionName` (container-proxy text re-attachment), `CS_UpdateGovOutfitsOnLoadGame` (Jarl/cabinet attire refresh), `CS_TroopEquipUpdateLoad` (armory list re-evaluation), and `CS_PlayerSurrenderLoadGame` (forces stage 100 to clear captive scene stalls). Saving mid-siege is highly vulnerable to barricade and wave counter desynchronization; mid-game uninstallation causes permanent hold broken states and orphaned Papyrus thread locks.
- **Evidence**: `research/findings/persistence.md`
- **Related**: M26, M08, M10, M12, M16, M28

## Vanilla Skyrim Master Overrides & Compatibility Invariants
- **Status**: CONFIRMED
- **Finding**: `conquerskyrim.esp` directly overrides 83 vanilla records, prominently `CWCampaignObj` (FormID `0x00026F45`), `CWChangeLocationScenes` (FormID `0x000D5176`), General Tullius (`0x0001336B`), Ulfric Stormcloak (`0x0010F9B9`), and 57 `LCTN` locations. A verified game-breaking conflict occurs if the player captures holds before resolving the Main Quest council summit *Season Unending* (`MQ302`) or *Dragon Rising* (`MQ104`), or if run alongside *Open Civil War* or *CWO*. Managing faction officers through third-party follower frameworks (AFT/NFF) breaks their `TaskBarrel` AI package stack.
- **Evidence**: `research/findings/vanilla-compatibility.md`
- **Related**: M27, M04, M06, M10, M14, M28

## Technical Failure Modes & Reliability Architecture
- **Status**: CONFIRMED
- **Finding**: Root-cause analysis categorizes 10 critical failure modes: (1) Siege multi-actor CTDs from Papyrus heap exhaustion (mitigated by setting `CS_BattleSize` to 25-30 and *SSE Engine Fixes*); (2) Navmesh assault freezes in exterior city overhauls (*JK's Skyrim*); (3) The Moot deadlock in `CS_MeetingScript.GetJarls()` if any hold lacks an appointed Jarl or has an invalid alias; (4) Fort Amol trigger freeze from out-of-order Eastmarch border init (patched by *Conquest of Skyrim Fixes* #119992); (5) Mid-siege save barricade desync; and (6) Irreparable save corruption upon mid-game uninstallation due to disabled vanilla markers and orphaned scripts.
- **Evidence**: `research/findings/failure-modes.md`
- **Related**: M28, M04, M08, M11, M15, M16, M26, M27

## Complete Behavioral Specification
- **Status**: CONFIRMED
- **Finding**: The complete behavioral specification addresses all 51 inventoried features across 18 strategic domains (`docs/behavioral-specification.md`). Core mechanisms specify: proxy container naming, 4-tier territory model (9 hold capitals with index numbers 0-8), 10 field armies with class scaling across mortal/arcane/Dwemer units, battle throttling via `CS_BattleSize`, 6-stage siege lifecycle with catapults and rams, mathematical production equations, demographic keyword allocation waterfall, 4-tier infrastructure upgrades, 7-member royal cabinet, Spymaster covert network and coups, judicial court hearings with 7 crime factions, bilateral non-aggression diplomacy, and The Moot election leading to High King coronation.
- **Evidence**: `docs/behavioral-specification.md`
- **Related**: M29, M03, M09-M24, M30, M32

## Recovered System Model & Architectural Topology
- **Status**: CONFIRMED
- **Finding**: The recovered system model (`docs/system-model.md`) synthesizes the investigated architecture across 7 subsystems (Governance Hub, Territorial Engine, Economic Engine, Military Command, Siege/Battle Simulator, Diplomatic Engine, Judicial System) bound to a domain ERD, 4 core state machines (Territorial Holdings, Bilateral Diplomacy, Combat Waves, Stability Cascades), and an event scheduling model (24h daily economic ticks, OnCellAttach dynamic streaming, OnDying attrition hooks, and Scaleform ModEvents). Engine integration directly overrides 83 master forms in `Skyrim.esm` (including `CWCampaignObj` `0x00026F45`, Tullius, Ulfric, and 57 locations).
- **Evidence**: `docs/system-model.md`
- **Related**: M30, M06, M07, M25, M27, M32

## Completeness & Uncertainty Audit
- **Status**: CONFIRMED
- **Finding**: Audit reconciles 100% of the 51 inventoried features (`docs/research-gaps.md`) across 94% Confirmed, 4% Observed, and 2% Inferred/Unknown confidence. Formally resolves key community contradictions (proves mutual exclusion with vanilla Civil War, proves bilateral truces rather than mutual defense alliances, proves Moot deadlock stems from unassigned Jarl aliases in `GetJarls()`). Uncovers an unreleased v0.4.0 developmental stub for `SetUpInitialJarlLoyalties()` commented out in `cs_checkversion.psc`.
- **Evidence**: `docs/research-gaps.md`
- **Related**: M31, M03, M28, M29, M32

## Final Recovered Specification Package
- **Status**: CONFIRMED
- **Finding**: The final recovered specification package (`docs/recovered-specification.md`) completes the 32-milestone research program. It consolidates all 20 required domains into an authoritative master document: 51-feature catalog, 6-tier progression loop from outlaw to High King coronation, 4-tier holding hierarchy, 10-division army architecture with Task Barrel AI anchors and battle size throttling, exact economic production/conscription formulas, 4-tier facility upgrades, 7-member cabinet with Spymaster covert coups and judicial trials, bilateral non-aggression truces, savegame persistence self-healing suite, 83 overridden vanilla records, and failure mode diagnosis.
- **Evidence**: `docs/recovered-specification.md`
- **Related**: M32, M01-M31

## Source Provenance & Physical Availability Audit
- **Status**: CONFIRMED
- **Finding**: Physical audit verifies that the original mod package is present on the research machine at `references/local/conquest-of-skyrim/Data/` (3,779 files, 547.28 MB). Plugin `conquerskyrim.esp` bears authentic mtime `2023-03-23 19:28:17` and `SHA256: 2fe454bf986d2238130ab96a69e1f04e6797e2b2fbb9b0aa82f10ffd15b596e8`. It contains an exact 1:1 match across all 1,539 compiled `.pex` scripts and 1,539 uncompiled `.psc` source scripts. The files are absent from GitHub exclusively due to intentional `.gitignore` exclusion complying with `guardrails.md`.
- **Evidence**: `research/findings/source-provenance-audit.md`
- **Related**: M33, M05, M34, M35, M37
