# Complete Behavioral Specification: Conquest of Skyrim

- **Document**: Functional & Behavioral Specification
- **Milestone**: M29 — Behavioral Specification
- **Subject**: *Conquest of Skyrim*
- **Investigated Baseline**: `v0.3.9` (March 23, 2023 release by `galvatron304`)
- **Status**: COMPLETE & EVIDENCE-BACKED

---

## 1. Architectural Scope & Method

This document provides the complete, authoritative behavioral specification for *Conquest of Skyrim* v0.3.9. Derived from static record analysis (`conquerskyrim.esp`), 1,539 Papyrus script sources, engine event models, and empirical playtesting logs, this specification details the observable inputs, preconditions, state machines, mathematical equations, and failure conditions across all **51 inventoried features** in the 18 strategic domains.

---

## 2. Specification by Strategic Domain

### 2.1 Domain 1: Faction Creation & Identity

#### `FEAT-FACT-01`: Faction Declaration
- **Purpose**: Establishes an independent sovereign faction outside the vanilla Imperial/Stormcloak conflict.
- **Entry Point**: `CS_ReadStartBook.OnRead()` or activating initial territory banner.
- **Preconditions**: Location cleared of hostile native actors; vanilla Civil War not active.
- **State Transition**: `FactionStarted = 1`; `GovQuest` advances to Stage 0.
- **Inputs & Outputs**: Grants player `BannerSpell` and `SettingsMenu` inventory item; sets up initial holding ownership globals (Imperial = 11, Stormcloak = 12).
- **Evidence & Confidence**: `cs_readstartbook.psc`, `qf_cs_factiongovernment_050095ca.psc`. **CONFIRMED**.

#### `FEAT-FACT-02`: Custom Faction & Army Naming
- **Purpose**: Dynamically renames the player's sovereign faction and army division prefixes.
- **Entry Point**: `CS_SelectName.ChangeName()` invoked during Stage 0 setup.
- **Mechanisms & Formulas**: Scaleform UIlib text input prompt (`ShowTextInput()`). Injects string into proxy container base records: `FactionNameChestObject.SetName(NewFactionName)` and `ChestObject.SetName(NewArmyName)`. Dynamic guard renaming executed via `NameChanger.ForceRefTo()` iterating `GuardFormLists[]`.
- **Evidence & Confidence**: `cs_selectname.psc`. **CONFIRMED**.

#### `FEAT-FACT-03`: Heraldry & Faction Banners
- **Purpose**: Replaces generic bandit and hold banners with custom heraldic emblems.
- **Mechanisms**: Spawns custom 3D banner meshes (`abForcePersist=true`) at scale 1.53 in conquered cities; activates matching `FactionColor` markers in claimed camps.
- **Evidence & Confidence**: `cs_changebanners.psc`. **CONFIRMED**.

#### `FEAT-FACT-04`: Government Doctrine Selection
- **Purpose**: Sets philosophical doctrine (Feudal, Republic, Military) altering tax yields and unrest rates.
- **Evidence & Confidence**: `research/findings/feature-inventory.md`. **REPORTED / UNVERIFIED**.

---

### 2.2 Domain 2: Territory & Ownership Hierarchy

#### `FEAT-TERR-01`: Outpost & Camp Claiming
- **Purpose**: Converts cleared caves, mines, and bandit camps into sovereign resource nodes.
- **Entry Point**: Casting `BannerSpell` in a cleared exterior/interior cell.
- **Mechanisms**: Spawns persistent banner with attached `CS_ManageHolding` script.
- **Evidence & Confidence**: `cs_manageholding.psc`. **CONFIRMED**.

#### `FEAT-TERR-02`: Fort Captures
- **Purpose**: Seizes major military fortresses across Skyrim.
- **Mechanisms**: `TransferOwnershipFort(fortnum, 1)` enables `FortMarkersPlayer`, sets `FortLoc.SetKeywordData(CWOwner, 69)`, and sets `AllHoldingsFort[fortnum] = 1`.
- **Failure Conditions**: Reconquest by enemy frees prisoners via `PrisonerScript.FreeAllFortPrisoners()`.
- **Evidence & Confidence**: `cs_transferownershipscript.psc`. **CONFIRMED**.

#### `FEAT-TERR-03`: Territorial Hierarchy
- **Specification**: 4-tier model: Hold Capitals (`citynum 0..8`) &rarr; Forts (`AllHoldingsFort`) &rarr; Dependent Villages (`villagenum 9..14`) &rarr; Frontier Camps (`PlayerCamps[]`).
- **Evidence & Confidence**: `cs_transferownershipscript.psc`, `cs_battletrackerscript.psc`. **CONFIRMED**.

#### `FEAT-TERR-04`: Camp Respawn Timers (v0.3.9)
- **Specification**: Evaluates cooldown timers during 24h update ticks, preventing defeated camps from instant respawn.
- **Evidence & Confidence**: v0.3.9 Changelog, `cs_checkversion.psc`. **CONFIRMED**.

---

### 2.3 Domain 3: Settlements & Hold Capitals

#### `FEAT-SETT-01`: Hold Capital Takeover
- **Purpose**: Overthrows hold governance and transitions capital palace to player control.
- **State Transition**: `TransferOwnershipCity(citynum, 69)` sets `CWOwner = 69`, `AllHoldings[citynum] = 1`, and zeroes crime gold.
- **Evidence & Confidence**: `cs_transferownershipscript.psc`. **CONFIRMED**.

#### `FEAT-SETT-02`: Hold Governor Appointment
- **Purpose**: Installs loyal NPC as Jarl to manage hold economy and garrison.
- **Mechanisms**: Dispatched via `CS_StoryJarlChooseGov` Story Manager event; binds `JarlAliases[citynum]`.
- **Evidence & Confidence**: `cs_recruitjarl.psc`. **CONFIRMED**.

#### `FEAT-SETT-03`: Settlement Garrisons
- **Specification**: Deploys defensive soldiers (`CS_NumGuards`); dynamically instantiated on `OnCellAttach()`.
- **Evidence & Confidence**: `cs_upgradeguardonattach.psc`. **CONFIRMED**.

---

### 2.4 Domain 4: Military Organization & Recruitment

#### `FEAT-MIL-01`: Officer Appointments
- **Specification**: Recruits unique NPCs as Commanders (0..9), Steward, and Blacksmith. Allies serve free; neutral NPCs require 100 gold/day wage.
- **Evidence & Confidence**: `cs_troopmanagemenuscript.psc`, `cs_govpositions.psc`. **CONFIRMED**.

#### `FEAT-MIL-02`: Troop Recruitment & Demographics
- **Specification**: Enlists soldiers drafted from `Unemployed` civilian pool ($\text{RecruitableTroops} = \lceil \text{RecruitablePercentage} \times \text{Unemployed} \rceil$). Demographic filters (`CS_ChooseTroopRacesScript`) toggle allowed playable races and genders.
- **Evidence & Confidence**: `cs_choosetroopracesscript.psc`, `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-MIL-03`: Army Division Scaling
- **Specification**: Supports up to 10 distinct field divisions (`MaxArmies = 10`), updated via 5 pairs of updaters (`Updater1..10`).
- **Evidence & Confidence**: `cs_troopmanagemenuscript.psc`. **CONFIRMED**.

#### `FEAT-MIL-04`: Equipment Standardization
- **Specification**: Uses container-proxy inventory (`CS_CommanderOutfitScript.OpenInventory(true)`) to assign standardized armor forms (Iron &rarr; Dragonbone or custom sets).
- **Evidence & Confidence**: `cs_commanderoutfitscript.psc`, `cs_troopequipmentupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-MIL-05`: Troop Training & Perk Trees
- **Specification**: Linear upgrade costs ($\text{CostGold} = 5000 + 10000(L-1)$, $\text{CostMetal} = 50 + 100(L-1)$) scale `TroopPowerLevels` and unlock tier armor lists across 71 `PERK` records.
- **Evidence & Confidence**: `cs_troopupgradescript.psc`. **CONFIRMED**.

---

### 2.5 Domain 5: Battles & Tactical Formations

#### `FEAT-BAT-01`: In-Cell Battle Throttling
- **Specification**: Capped by `CS_BattleSize` global (default: 40 actors) to prevent Papyrus heap crashes. Attrition scales by `CS_BattleSpeed` on `OnDying` events.
- **Evidence & Confidence**: `cs_attackcityscript.psc`. **CONFIRMED**.

#### `FEAT-BAT-02`: Tactical Formations (v0.3.9)
- **Specification**: Positions soldiers in rigid 5x4 blocks with locked relative offsets behind the division commander.
- **Evidence & Confidence**: `cs__pf_cs_attackcitysiegedrum_*.psc`. **CONFIRMED**.

#### `FEAT-BAT-03`: Victory Parades (v0.3.9)
- **Specification**: Scripted march through conquered capitals featuring drummers playing `drums of chaos.xwm`.
- **Evidence & Confidence**: `cs__sf_cs_coronationstewardst_*.psc`. **CONFIRMED**.

#### `FEAT-BAT-04`: Off-Screen Simulated Battles
- **Formula Recovered**:
  $$\text{ChanceOfVictory} = \frac{\text{TroopCounts}}{\text{TroopCountCertain}}$$
  $$\text{DeadTroops} = \lfloor (1 - (\text{Chance} - \text{Roll})) \times \text{TroopCounts} \rfloor$$
- **Evidence & Confidence**: `cs_bountyscript.psc`. **CONFIRMED**.

---

### 2.6 Domain 6: Siege Warfare & Breaching

#### `FEAT-SIEG-01`: Staging Encampments
- **Specification**: Deploys `CampMarker1` and `CampMarker2` outside city walls and registers them on the world map.
- **Evidence & Confidence**: `cs_attackcitysiegescript.psc`. **CONFIRMED**.

#### `FEAT-SIEG-02`: Siege Artillery & Battering Rams
- **Specification**: Functional catapult batteries (`CatapultsList`) and battering rams (`BatteringRamEnabled`) breaching gates.
- **Evidence & Confidence**: `cs_attackcitysiegescript.psc`. **CONFIRMED**.

#### `FEAT-SIEG-03`: Pincer Flanking (`DoubleAttack`)
- **Specification**: Splits division commanders equally between two entrance gates for simultaneous assaults.
- **Evidence & Confidence**: `cs_attackcitysiegescript.psc`. **CONFIRMED**.

#### `FEAT-SIEG-04`: Palace Breach & Capitulation
- **Specification**: Transitions battle into interior palace, triggering `JarlConfrontScene` and `JarlSurrenderScene`.
- **Evidence & Confidence**: `cs_attackcityscript.psc`. **CONFIRMED**.

---

### 2.7 Domain 7: Economy, Production & Treasury

#### `FEAT-ECON-01`: Central Treasury
- **Specification**: Gold stored in `CS_FactionTreasury`; supports withdrawal/deposit tiers (100 to 1,000,000 gold).
- **Evidence & Confidence**: `cs_economymenufunctions.psc`. **CONFIRMED**.

#### `FEAT-ECON-02`: Tiered Hold Taxation
- **Formulas Recovered**:
  - Major Walled Capitals (`0..4`): $\text{TreasuryPerDay} = 500 \cdot \text{Pop} + 250 \cdot \text{Pop} \cdot \text{TaxRate}$
  - Minor Open Capitals (`5..8`): $\text{TreasuryPerDay} = 300 \cdot \text{Pop} + 150 \cdot \text{Pop} \cdot \text{TaxRate}$
  - Villages (`9+`): $\text{TreasuryPerDay} = 200 \cdot \text{Pop} + 100 \cdot \text{Pop} \cdot \text{TaxRate}$
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-ECON-03`: Resource Extraction Nodes
- **Formulas Recovered**: Gross production evaluates facility levels and dock trade bonuses:
  $$\text{ResourceProduced} = 3 \times \Big( (\text{Level} \times \text{Change}) + \lfloor 0.3 \times \text{DocksChange} \times \text{DocksLevel} \rfloor - \text{JobsOpen} \Big)$$
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-ECON-04`: Daily Food Upkeep & Famine
- **Formula Recovered**: $\text{FoodRequired} = \text{Population} + \text{CS\_NumGuards}$. Armies consume `TroopCountFood`. Food deficits trigger civilian unrest and army desertions.
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

---

### 2.8 Domain 8: Population & Labor Allocation

#### `FEAT-POP-01`: Abstract Keyword Demographics
- **Specification**: Population tracked abstractly via location keywords (`Population`, `Unemployed`).
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-POP-02`: Labor Allocation Priority
- **Specification**: Strict waterfall: Priority 1 (Food) &rarr; Priority 2 (Docks) &rarr; Priority 3 (Wood) &rarr; Priority 4 (Metal).
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

#### `FEAT-POP-03`: Dynamic Growth
- **Formula**: If $\text{Pop} < \text{MaxJobs} + 0.25 \cdot \text{Pop}$, growth increases by $\text{random}(0.3 \cdot \text{Deficit}, 0.5 \cdot \text{Deficit})$.
- **Evidence & Confidence**: `cs_holdingeconomyupdaterscript.psc`. **CONFIRMED**.

---

### 2.9 Domain 9: Buildings & Infrastructure Upgrades

#### `FEAT-INFR-01`: 5 Facility Types & 4 Upgrade Tiers
- **Specification**: Farms (`FarmLevel`), Mines (`MineLevel`), Lumber Mills (`LumberMillLevel`), Docks (`DocksLevel`), Barracks (`BarracksLevel`) upgrade up to Level 4.
- **Evidence & Confidence**: `cs_upgradecityscript.psc`. **CONFIRMED**.

#### `FEAT-INFR-02`: Construction Costs & Job Creation
- **Specification**: Costs Treasury Gold (`UpgradeCost`) and Wood (`UpgradeCostWood`). Upgrading calls `AddJobs()`, immediately expanding employment capacity by `Change`.
- **Evidence & Confidence**: `cs_upgradecityscript.psc`. **CONFIRMED**.

---

### 2.10 Domain 10: Technology & Research Perk Trees

#### `FEAT-TECH-01`: Martial & Arcane Perks (71 PERK Records)
- **Specification**: 10 levels for Soldiers, Warriors, Archers, Crossbows, and Guards; 5 levels for Destruction Fire/Frost/Shock.
- **Evidence & Confidence**: `cs_troopupgradescript.psc`, `conquerskyrim.esp`. **CONFIRMED**.

#### `FEAT-TECH-02`: Dwemer Engineering & Dragon Mastery
- **Specification**: Unlocks Dwemer Spiders, Spheres, Ballistas, and Centurions; late-game perk `CS_SpeakToTamedDragonPerk` enables dragon dialogue and flight commands.
- **Evidence & Confidence**: `cs_armytroopdeath.psc`, `conquerskyrim.esp`. **CONFIRMED**.

---

### 2.11 Domain 11: Government & Cabinet Administration

#### `FEAT-GOV-01`: Seven-Member Central Cabinet
- **Specification**: Steward (domestic/taxes), Second-in-Command (war/POWs), Spymaster (intelligence/coups), Blacksmith (armory), Court Mage (arcane), Court Bard (ceremonies), Executioner (capital sentences).
- **Evidence & Confidence**: `cs_govpositions.psc`, `cs_govscriptfunctions.psc`. **CONFIRMED**.

#### `FEAT-GOV-02`: Covert Espionage & Coups (`CS_IntelReportScript`)
- **Specification**: Funds spy networks (`SpyNetworkGlobal`), monitors Jarl loyalty and line of succession (`LoSTitleTexts`), and sponsors internal palace coups (`CoupQuest`) to flip holds without sieges.
- **Evidence & Confidence**: `cs_intelreportscript.psc`. **CONFIRMED**.

#### `FEAT-GOV-03`: Judicial Throne Hearings (`CS_CourtScript`)
- **Specification**: Convenes court from thrones to judge 7 crime categories (Theft, Assault, Murder, Speaking Out, Worship, Treason, Contraband) resulting in fines, dungeon sentences, or executions.
- **Evidence & Confidence**: `cs_courtscript.psc`. **CONFIRMED**.

---

### 2.12 Domain 12: Jarls & Hold Administration

#### `FEAT-JARL-01`: Jarl Appointment & Deposition
- **Specification**: Appoints followers via `CS_StoryJarlChooseGov`; unseats native Jarls into `CitizenFaction`.
- **Evidence & Confidence**: `cs_recruitjarl.psc`, `cs_transferownershipscript.psc`. **CONFIRMED**.

#### `FEAT-JARL-02`: Dual-Layer Loyalty Mechanics
- **Specification**: Tracks citizen `Loyalty` vs ruler `JarlLoyalty`. Modifiers include `NoFood`, `NoGarrison`, `NoCourt`, `Parties`, `ArmiesDefending`.
- **Evidence & Confidence**: `cs_loyaltycheckscript.psc`. **CONFIRMED**.

#### `FEAT-JARL-03`: Protests & Secession Insurgency
- **Specification**: `Loyalty < 40` triggers civilian palace protests (`ProtestQuest`); `Loyalty < 20` triggers armed secession revolts (`RebelDefendQuestStarted`).
- **Evidence & Confidence**: `cs_loyaltycheckscript.psc`. **CONFIRMED**.

---

### 2.13 Domain 13: Diplomacy & Foreign Relations

#### `FEAT-DIPL-01`: Relational State Machine
- **Specification**: 4 bilateral states: Neutral &rarr; War &harr; Truce &rarr; Defeated. Joint-military alliances are un-implemented.
- **Evidence & Confidence**: `cs_negotiationscript.psc`. **CONFIRMED**.

#### `FEAT-DIPL-02`: Treaty Acceptance Formula
- **Formula Recovered**:
  $$\text{Chance} = 25\% \pm 25\% (\text{Relationship}) \pm 25\%/50\% (\text{Ideology}) \pm 25\% (\text{Army Leverage} < 60 \text{ vs } \ge 100)$$
- **Evidence & Confidence**: `cs_negotiationscript.psc`. **CONFIRMED**.

#### `FEAT-DIPL-03`: Truce Execution & POW Ransoms
- **Specification**: `MakeFriends()` removes `CWImperialFaction` or `CWSonsFaction` tags and halts AI invasion timers; POWs ransomed for gold or executed.
- **Evidence & Confidence**: `cs_negotiationscript.psc`, `cs_prisonerquestscript.psc`. **CONFIRMED**.

---

### 2.14 Domain 14: Law, Crime & Dungeons

#### `FEAT-LAW-01`: Autonomous Crime Jurisdiction
- **Specification**: Conquering holds executes `SetCrimeGold(0)` across `CrimeFactions[]`, detaching vanilla hold bounties.
- **Evidence & Confidence**: `cs_transferownershipscript.psc`. **CONFIRMED**.

#### `FEAT-LAW-02`: Municipal Jail System
- **Specification**: Dedicated prisoner arrays across all 9 hold capitals (`PrisonersMarkarth`..`PrisonersWinterhold`) and native Jarl captive arrays (`JarlPrisonersImp`/`Sons`).
- **Evidence & Confidence**: `cs_prisonscript.psc`. **CONFIRMED**.

#### `FEAT-LAW-03`: Capital Punishment
- **Specification**: Scripted decapitations on `ChoppingBlock` via `ExecutionQuest`.
- **Evidence & Confidence**: `cs_courtscript.psc`. **CONFIRMED**.

---

### 2.15 Domain 15: Stability & Public Order

#### `FEAT-STAB-01`: Public Order Multipliers
- **Specification**: Driven by `Harshness`, `TaxRate`, and `NoFood`.
- **Evidence & Confidence**: `cs_loyaltycheckscript.psc`. **CONFIRMED**.

#### `FEAT-STAB-02`: Secession Battles
- **Specification**: Hostile militia spawns inside hold capitals during armed revolts.
- **Evidence & Confidence**: `cs_loyaltycheckscript.psc`. **CONFIRMED**.

---

### 2.16 Domain 16: Player Progression Loop

#### `FEAT-PROG-01`: Six-Tier Title Advancement
- **Specification**: Progression spans Outlaw Rebel &rarr; Camp Lord &rarr; Hold Conqueror &rarr; Province Dominator &rarr; The Moot Nominee &rarr; Crowned High Monarch.
- **Evidence & Confidence**: `cs_meetingscript.psc`, `cs_coronationscript.psc`. **CONFIRMED**.

#### `FEAT-PROG-02`: Royal Court & Retinue
- **Specification**: Custom AI packages assigning royal bodyguards and palace housecarls.
- **Evidence & Confidence**: `cs_govscriptfunctions.psc`. **CONFIRMED**.

---

### 2.17 Domain 17: Victory Conditions & The Moot

#### `FEAT-VICT-01`: The Moot Convocation & Voting
- **Specification**: Requires all 9 holds conquered (`Cities[0..8] == true`). Jarls with `Loyalty >= 60` nominate the player. Roll-call voting resolves based on hold loyalty keyword; victory requires $\text{MootVoteFor} \ge \text{MootVoteAgainst}$.
- **Evidence & Confidence**: `cs_meetingscript.psc`. **CONFIRMED**.

#### `FEAT-VICT-02`: Coronation Ceremony
- **Specification**: Ceremonial assembly in palace: Jarls and citizens kneel via `IdleKneelingEnter` animations, Jarls deliver fealty oaths (+30 loyalty per hold), player equipped with `HighKingCrown`/`HighQueenCrown`, and `IsHighKing` set to 1.
- **Evidence & Confidence**: `cs_coronationscript.psc`. **CONFIRMED**.

---

### 2.18 Domain 18: User Interaction & Configuration

#### `FEAT-UI-01`: Scaleform UIlib Virtual Keyboard
- **Specification**: Invokes `UILIB_1_textInputOpen()` for dynamic faction and army naming.
- **Evidence & Confidence**: `cs_selectname.psc`. **CONFIRMED**.

#### `FEAT-UI-02`: Tactical War Map & MCM Configuration
- **Specification**: Map table manages division movement with contiguity check (`HoldsNotLinked`). SkyUI MCM configures `CS_BattleSize` and `CS_BattleSpeed` with `SettingsMenu` inventory fallback item.
- **Evidence & Confidence**: `cs_mapmanagemenuscript.psc`, `conquerskyrim.esp`. **CONFIRMED**.
