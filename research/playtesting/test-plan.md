# Controlled Human Playtesting Program: Conquest of Skyrim

- **Document**: Empirical Verification Test Plan
- **Milestone**: M41 — Controlled Playtesting Program
- **Phase**: Phase 11 (Empirical Validation and Final Recovery Audit)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Author**: Coordinator Agent
- **Execution Responsibility**: Human Researcher / Playtester
- **Status**: ACTIVE TEST PLAN (No results fabricated)

---

## 1. Playtesting Strategy & Principles

This document establishes the structured human playtesting protocol for *Conquest of Skyrim* v0.3.9. 

In strict adherence to `docs/execution-discipline.md` and `docs/testing.md`:
1. **Zero Fabrication**: AI agents must never fabricate gameplay logs, simulate test outputs, or claim empirical verification without human playtest reports.
2. **High-Information Density**: Focuses on 12 critical, high-risk operational experiments rather than hundreds of trivial checks.
3. **Reproducibility**: Every experiment defines explicit starting preconditions, save state requirements, sequential player actions, expected outcomes, and verifiable evidence standards.

---

## 2. Test Execution & Reporting Protocol

When executing an experiment:
1. Launch Skyrim SE/AE with `SKSE64` and `SkyUI`.
2. Load the specified clean baseline save.
3. Execute the exact numbered actions.
4. Record screenshots, save files, and Papyrus log excerpts (`Papyrus.0.log`).
5. Classify the outcome as:
   - **`PASS`**: Observed in-game behavior matches verified script expectations.
   - **`FAIL`**: Behavior contradicts script expectations or an unhandled crash occurs.
   - **`BLOCKED`**: Prerequisites or engine state prevent completing actions.
   - **`INCONCLUSIVE`**: Test executed, but evidence was insufficient to verify the invariant.
6. File the completed log in `research/playtesting/` using template `EXP-XXX-report.md`.

---

## 3. High-Priority Experiment Specifications

### `EXP-FACT-01`: Faction Declaration & Scaleform Naming
- **Related Milestone**: M09 (Faction Creation), M12 (Player Interaction)
- **Objective**: Verify that reading the starter book initializes `GovQuest` Stage 0, opens `UIlib v1` virtual keyboard, successfully applies custom name strings to container proxies, and teleports the player to the customization chamber.
- **Preconditions**: Mod installed on clean save; character outside combat in an exterior cell.
- **Required Save State**: Clean save (Level 1–5, vanilla Civil War unstarted).
- **Player Actions**:
  1. Open inventory and read `Conquest of Skyrim - Starter Book`.
  2. Select option 0 ("Declare Faction") in the message prompt.
  3. Enter custom string `"Dreadguard"` in the Faction Name prompt.
  4. Enter custom string `"Dread"` in the Army Prefix prompt.
- **Expected Observations**:
  - Player receives `BannerSpell` and `SettingsMenu` inventory items.
  - Player is teleported into `BannerRoomTeleport` interior cell.
  - Dialogue token `<Alias=FactionName>` displays `"Dreadguard"`.
- **Evidence Requirements**: Screenshot of naming prompt; savegame inspect showing `FactionStarted == 1`.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-TERR-01`: Outpost Claiming & Banner Mesh Placement
- **Related Milestone**: M10 (Territory & Ownership)
- **Objective**: Verify that casting `BannerSpell` in a cleared bandit camp claims the location, places a 3D banner at scale 1.53, and attaches `CS_ManageHolding`.
- **Preconditions**: Eligible bandit redoubt (e.g., Silent Moons Camp or Halted Stream Camp) cleared of all hostile bandits.
- **Required Save State**: Post-`EXP-FACT-01` active faction save.
- **Player Actions**:
  1. Travel to cleared camp.
  2. Cast `BannerSpell` at the central camp marker.
  3. Activate the newly spawned banner.
- **Expected Observations**:
  - Banner spawns with chosen heraldry emblem.
  - Activating banner opens `CS_ManageHolding` menu displaying holding type and resource yields.
  - Location keyword `CWOwner` updates to `69`.
- **Evidence Requirements**: Screenshot of placed banner and holding menu.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-MIL-01`: Commander Appointment & Armory Standardization
- **Related Milestone**: M13 (Military Organization)
- **Objective**: Verify that recruiting an ally NPC binds custom AI packages, verifies wage mechanics, and equips troops via the container proxy.
- **Preconditions**: Faction founded; player possesses 1 follower (high disposition) and 1 neutral NPC.
- **Required Save State**: Active faction save with claimed base.
- **Player Actions**:
  1. Speak to follower (e.g., Lydia or Faendal); select *"I want you to lead an army division"*.
  2. Speak to neutral mercenary (e.g., Jenassa); select the same option.
  3. Speak to Faction Blacksmith; deposit 10 Iron Swords and 10 Iron Cuirasses into the armory container proxy.
- **Expected Observations**:
  - Follower serves free of charge; mercenary demands 100 gold/day wage.
  - Both NPCs receive `FollowScoutCommander` package and division assignment.
  - Newly enlisted recruits spawn wearing Iron armor without "naked troop" glitches.
- **Evidence Requirements**: Dialogue confirmation screenshots; actor inventory inspection.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-ORD-01`: Task Barrel AI Package Transitions & Patrols
- **Related Milestone**: M14 (Military Orders)
- **Objective**: Verify that issuing a "Patrol Hold" order transfers the commander to `TaskBarrelsDefendHold` and causes the division to march to hold borders.
- **Preconditions**: Division 1 active with assigned commander.
- **Required Save State**: Active faction save in Whiterun hold.
- **Player Actions**:
  1. Speak to Division 1 Commander.
  2. Order: *"Patrol Whiterun Hold"*.
  3. Observe commander navigation along main road network.
- **Expected Observations**:
  - Commander immediately stops sandbox idle and begins road patrol route.
  - Off-screen fast travel does not trap the army in unloaded wilderness cells.
- **Evidence Requirements**: Screenshot of marching division; world map confirmation.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-BAT-01`: In-Cell Combat Simulation & Casualty Waves
- **Related Milestone**: M15 (Combat Simulation)
- **Objective**: Verify that combat engagements throttle concurrent actor spawns to `CS_BattleSize` and spawn reinforcements at 20-casualty milestones.
- **Preconditions**: Player army deployed against hostile Imperial/Stormcloak field detachment.
- **Required Save State**: Staged save prior to field skirmish.
- **Player Actions**:
  1. Engage enemy force with `CS_BattleSize` set to 30 in MCM.
  2. Defeat 20 enemy combatants.
- **Expected Observations**:
  - Total active combatant actors in cell does not exceed 30.
  - Upon reaching 20 defender deaths, a reinforcement horn sounds and wave 2 spawns at the perimeter.
  - Surviving enemies receive `RetreatFaction` AI packages upon army collapse.
- **Evidence Requirements**: `Papyrus.0.log` trace; screenshot of reinforcement wave spawn.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-SIEG-01`: City Siege Breaching & Battering Ram Execution
- **Related Milestone**: M16 (Siege Warfare)
- **Objective**: Verify that forward staging camps spawn outside city gates, catapult batteries fire, and battering rams breach outer barriers.
- **Preconditions**: Army possesses military superiority over target minor capital (e.g., Falkreath).
- **Required Save State**: Active invasion planned via `CS_AttackPlanningNewScript`.
- **Player Actions**:
  1. Travel to `CampMarker1` outside Falkreath.
  2. Initiate siege assault via Commander dialogue.
  3. Advance to the outer gate barrier with the assault squad.
- **Expected Observations**:
  - Destructible barricade breaks under battering ram attack.
  - Defenders retreat into Jarl's Longhouse upon exterior collapse.
  - Entering palace initiates `JarlConfrontScene`.
- **Evidence Requirements**: Screenshots of staging camp and broken gate barricade.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-ECON-01`: Daily Economic Tick & Tiered Taxation
- **Related Milestone**: M17 (Resource Systems), M18 (Population & Taxes)
- **Objective**: Verify that daily 24h game-time updates credit exact mathematical tax revenue to `CS_FactionTreasury`.
- **Preconditions**: Whiterun conquered (`holdingnum = 3`, `Pop = 50`, `TaxRate = 2`).
- **Required Save State**: Sovereign holding active with known treasury baseline.
- **Player Actions**:
  1. Record current `CS_FactionTreasury` gold via Steward dialogue.
  2. Wait 24 consecutive in-game hours using the Wait menu (`T`).
  3. Check treasury balance immediately after the daily update tick.
- **Expected Observations**:
  - Treasury increases by exactly 50,000 gold ($500 \cdot 50 + 250 \cdot 50 \cdot 2$) minus standing army wages.
  - Food reserves decrement by $\text{FoodRequired} = \text{Population} + \text{Guards}$.
- **Evidence Requirements**: Pre/post treasury dialogue balance screenshots.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-GOV-01`: Palace Throne Court Hearings & Capital Sentences
- **Related Milestone**: M21 (Government), M24 (Law, Crime & Stability)
- **Objective**: Verify that sitting on the royal throne initiates `CS_CourtScript` hearings and chopping block executions.
- **Preconditions**: Conquered hold capital with active throne activator (`Thrones[CapitalGlobal]`).
- **Required Save State**: Sovereign capital with at least 1 detained prisoner.
- **Player Actions**:
  1. Activate palace throne and sit down.
  2. Steward presents criminal case (`CS_CourtScript`).
  3. Sentence offender to execution.
- **Expected Observations**:
  - `ExecutionQuest` begins; guards escort prisoner to palace chopping block.
  - Executioner plays decapitation animation on `ChoppingBlock`.
  - Hold `Loyalty` keyword updates with `Harshness` modifier.
- **Evidence Requirements**: Screenshot of court hearing and execution scene.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-DIPL-01`: Diplomatic Truce Negotiations
- **Related Milestone**: M23 (Diplomacy)
- **Objective**: Verify that diplomatic letters summon Tullius or Ulfric to a treaty table and `MakeFriends()` halts hostilities.
- **Preconditions**: Active war state with Imperial Legion; player army $\ge 100$ troops.
- **Required Save State**: High-leverage sovereign save.
- **Player Actions**:
  1. Direct Steward to send peace letter to General Tullius.
  2. Travel to designated neutral meeting table.
  3. Select Pro-Imperial or Neutral concession terms.
- **Expected Observations**:
  - Tullius accepts truce based on $\ge 100$ troop leverage ($+25\%$).
  - `TruceImp` global set to `1`.
  - Imperial patrols on roads become non-hostile (`CWImperialFaction` removed).
- **Evidence Requirements**: Screenshots of summit seating and non-hostile Imperial patrol.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-SAV-01`: Mid-Siege Save & Reload Barricade Integrity
- **Related Milestone**: M26 (Persistence), M28 (Failure Modes)
- **Objective**: Verify whether saving mid-assault reproduces the suspected Havok gate desynchronization.
- **Preconditions**: Active outdoor city siege in progress; outer barricade partially damaged.
- **Required Save State**: Temporary test save created during active combat.
- **Player Actions**:
  1. Save game mid-assault while battering ram is active.
  2. Exit to main menu and reload test save.
- **Expected Observations**:
  - Check whether destructible barricade resets to closed state or wave counters stall.
- **Evidence Requirements**: Save file; comparative screenshots before/after reload.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-CW-01`: Vanilla Civil War Active Quest Collision
- **Related Milestone**: M27 (Vanilla Compatibility)
- **Objective**: Verify that founding a faction mid-Civil War causes dialogue locks or unseated Jarl invulnerability.
- **Preconditions**: Player character active in vanilla quest `CW01B` or `CWSiege`.
- **Required Save State**: Active vanilla Civil War save.
- **Player Actions**:
  1. Attempt to claim an outpost and declare faction.
  2. Travel to Dragonsreach and attempt to converse with Jarl Balgruuf.
- **Expected Observations**:
  - Confirm whether palace doors lock or dialogue fails to advance due to `CWCampaignObj` collision.
- **Evidence Requirements**: Quest journal screenshot; `Papyrus.0.log` quest error dump.
- **Result Classification**: `[PENDING EXECUTION]`

---

### `EXP-VICT-01`: The Moot Convocation & High King Coronation
- **Related Milestone**: M11 (Progression & Victory), M22 (Jarls)
- **Objective**: Verify that conquering 9 holds with appointed Jarls triggers The Moot, roll-call voting, and royal coronation.
- **Preconditions**: All 9 hold capitals conquered; each hold has an appointed Jarl in `GovFaction`.
- **Required Save State**: Endgame 9-hold sovereign save.
- **Player Actions**:
  1. Speak to Steward: select *"Convene The Moot"*.
  2. Travel to capital palace throne room.
  3. Observe Jarl nomination and roll-call voting tally.
  4. Advance to Coronation Day.
- **Expected Observations**:
  - High loyalty Jarl delivers nomination speech.
  - Votes resolve with message notifications: `"X Votes For, Y Votes Against"`.
  - Coronation sequence executes: Jarls and citizens kneel; player receives `HighKingCrown`; `IsHighKing` global set to `1`.
- **Evidence Requirements**: Screenshots of The Moot assembly and coronation kneeling scene.
- **Result Classification**: `[PENDING EXECUTION]`
