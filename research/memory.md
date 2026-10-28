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
