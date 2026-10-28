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
