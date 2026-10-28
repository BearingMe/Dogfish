# Behavioral Contract Inventory & Reconstruction Matrix

- **Document**: Reconstruction Coverage Matrix & Contract Specifications
- **Milestone**: M46 — Behavioral Contract Inventory
- **Phase**: Phase 12 (Reconstruction Foundations)
- **Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Status**: RECONCILED & SPECIFIED ($N = 55$ Features)

---

## 1. Resolution of the 51 vs. 55 Feature Discrepancy

A discrepancy was identified between early research documentation citing "51 features" and later audit scripts extracting 55 features.

### Mathematical Re-Audit:
Summing the individual feature entries across all 18 strategic domains in `research/findings/feature-inventory.md`:

$$\begin{aligned}
N = &\ 4\ (\text{FACT}) + 4\ (\text{TERR}) + 3\ (\text{SETT}) + 5\ (\text{MIL}) + 4\ (\text{BAT}) + 4\ (\text{SIEG}) \\
    &+ 4\ (\text{ECON}) + 3\ (\text{POP}) + 2\ (\text{INFR}) + 2\ (\text{TECH}) + 3\ (\text{GOV}) + 2\ (\text{JARL}) \\
    &+ 3\ (\text{DIPL}) + 3\ (\text{LAW}) + 2\ (\text{STAB}) + 2\ (\text{PROG}) + 3\ (\text{VICT}) + 2\ (\text{UI}) \\
  = &\ \mathbf{55\ Unique\ Features}
\end{aligned}$$

- **Discrepancy Cause**: Early drafting in Milestone M03 summarized the feature catalogue as "51 features" before the final addition of `FEAT-TERR-04` (camp respawn cooldowns), `FEAT-SETT-03` (settlement garrisons), and separate entries for `FEAT-VICT-02` / `03`.
- **Authoritative Baseline**: Exactly **55 features** exist in the inventory. All 55 features are accounted for below.

---

## 2. Behavioral Contract Methodology

A **Behavioral Contract** defines the observable requirements of a subsystem without dictating internal implementation details:
- **Inputs**: Data passed into the system (commands, gold, resources, coordinates).
- **Preconditions**: Invariants that must hold before execution (adjacency, disposition, military superiority).
- **State Transitions**: Formal mutations of the domain state model.
- **Outputs & Side Effects**: Returned values, modified registries, and outward signals.
- **Invariants**: Rules that must never be violated (e.g., non-negative treasury, capacity caps).

*Implementation Principle*: The Rust simulation does not mimic Papyrus class structures or dummy container proxies. It implements equivalent behavioral contracts using idiomatic, type-safe Rust data structures.

---

## 3. Reconstruction Coverage Matrix ($N = 55$)

| Feature ID | Feature Name | Primary Source Code Reference | Inputs & Outputs | Calibrated Confidence | Reconstruction Difficulty | Integration Dependency | Missing Information / Gaps |
| :--- | :--- | :--- | :--- | :---: | :---: | :--- | :--- |
| `FEAT-FACT-01` | Faction Declaration | `cs_readstartbook.psc`, `qf_cs_factiongovernment_*.psc` | In: Read event; Out: `FactionStarted=1`, Stage 0 | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-FACT-02` | Custom Naming | `cs_selectname.psc`, `UIlib_1.psc` | In: Name strings; Out: Faction/Army strings | CONFIRMED IN CODE | Low | Scaleform UI in Skyrim | Native string tokens |
| `FEAT-FACT-03` | Heraldry & Banners | `cs_changebanners.psc` | In: FactionColor; Out: Spawned banner references | CONFIRMED IN CODE | Moderate | NIF meshes, world markers | Visual only |
| `FEAT-FACT-04` | Government Doctrine | `cs_factiongovernmentscript.psc` | In: Doctrine selection; Out: Modifier variables | INCOMPLETE STUB | Low | None (Standalone state) | Exact yield multipliers unverified |
| `FEAT-TERR-01` | Outpost Claiming | `cs_manageholding.psc` | In: Banner activation; Out: Location global = 1 | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-TERR-02` | Fort Captures | `cs_transferownershipscript.psc` | In: Fort victory; Out: `AllHoldingsFort[i] = 1` | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-TERR-03` | Territory Hierarchy | `cs_transferownershipscript.psc` | In: Holding queries; Out: 4-tier classification | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-TERR-04` | Camp Respawn Timers | `cs_checkversion.psc`, `cs_tmscript.psc` | In: Camp clear; Out: 24h recovery cooldown | CONFIRMED IN CODE | Low | None (Standalone state) | Cooldown duration bounds |
| `FEAT-SETT-01` | Capital Takeover | `cs_transferownershipscript.psc` | In: Siege victory; Out: `AllHoldings[i]=1`, `CWOwner=69` | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-SETT-02` | Governor Appointment | `cs_recruitjarl.psc` | In: Appoint citizen; Out: `JarlAliases[i]` assigned | CONFIRMED IN CODE | Low | Story Manager in Skyrim | None |
| `FEAT-SETT-03` | Settlement Garrisons | `cs_upgradeguardonattach.psc` | In: Station troops; Out: `CS_NumGuards` updated | CONFIRMED IN CODE | Low | OnCellAttach in Skyrim | None |
| `FEAT-MIL-01` | Officer Recruitment | `cs_govpositions.psc`, `cs_troopmanagemenu*.psc` | In: Hire NPC; Out: Bound officer alias, wage logic | CONFIRMED IN CODE | Moderate | AI Packages in Skyrim | None |
| `FEAT-MIL-02` | Troop Recruitment | `cs_choosetroopracesscript.psc`, `cs_holdingeconomyupdaterscript.psc` | In: Draft order; Out: Recruits drafted from Unemployed | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-MIL-03` | 10 Field Armies | `cs_troopmanagemenuscript.psc` | In: Create division; Out: Division 0..9 active | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-MIL-04` | Armory Outfitting | `cs_commanderoutfitscript.psc` | In: Armor tier; Out: Standardized outfit forms | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-MIL-05` | Troop Training | `cs_troopupgradescript.psc` | In: Gold + Metal; Out: `TroopPowerLevels += 1` | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-BAT-01` | Battle Throttling | `cs_attackcityscript.psc` | In: Spawn loop; Out: In-cell actors clamped to BattleSize | CONFIRMED IN CODE | High | Actor spawning in Skyrim | None |
| `FEAT-BAT-02` | 5x4 Formations | `cs__pf_cs_attackcitysiegedrum_*.psc` | In: Formation command; Out: Locked relative AI offsets | CONFIRMED IN CODE | High | AI Package offsets in Skyrim | None |
| `FEAT-BAT-03` | Victory Parades | `cs_coronationscript.psc` | In: Parade order; Out: Synchronized drum cadence march | CONFIRMED IN CODE | Moderate | xWMA audio, anims in Skyrim | None |
| `FEAT-BAT-04` | Simulated Battles | `cs_bountyscript.psc`, `cs_tmscript.psc` | In: Troop ratios, roll; Out: Victory bool, casualties | CONFIRMED / UNKNOWN | High | None (Standalone state) | General AI vs AI formula unknown |
| `FEAT-SIEG-01` | Staging Camps | `cs_attackcitysiegescript.psc` | In: Target city; Out: `CampMarker1/2` enabled | CONFIRMED IN CODE | Low | Map markers in Skyrim | None |
| `FEAT-SIEG-02` | Siege Artillery & Rams | `cs_attackcitysiegescript.psc` | In: Requisition gear; Out: Catapults, rams enabled | CONFIRMED IN CODE | High | Havok physics in Skyrim | None |
| `FEAT-SIEG-03` | Pincer Assaults | `cs_attackcitysiegescript.psc` | In: DoubleAttack=true; Out: Split division deployment | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-SIEG-04` | Palace Capitulation | `cs_attackcityscript.psc` | In: Breach palace; Out: `JarlSurrenderScene` | CONFIRMED IN CODE | Moderate | Scripted scenes in Skyrim | None |
| `FEAT-ECON-01` | Faction Treasury | `cs_economymenufunctions.psc` | In: Deposit/withdraw; Out: `CS_FactionTreasury` balance | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-ECON-02` | Tiered Taxation | `cs_holdingeconomyupdaterscript.psc` | In: Pop, TaxRate, Tier; Out: Exact daily revenue | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-ECON-03` | Resource Production | `cs_holdingeconomyupdaterscript.psc` | In: Facility level, change, docks; Out: Daily units | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-ECON-04` | Daily Food Upkeep | `cs_holdingeconomyupdaterscript.psc` | In: Pop + Guards + Armies; Out: Food offset | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-POP-01` | Keyword Demographics | `cs_holdingeconomyupdaterscript.psc` | In: Holding query; Out: `Population`, `Unemployed` | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-POP-02` | Labor Allocation | `cs_holdingeconomyupdaterscript.psc` | In: Unemployed labor; Out: Waterfall: Food->Docks->Wood->Metal | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-POP-03` | Conscription Drain | `cs_holdingeconomyupdaterscript.psc` | In: Enlist; Out: `ceil(Rate * Unemployed)` deducted | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-INFR-01` | 5 Facility Types | `cs_upgradecityscript.psc` | In: Farm, Mine, Mill, Docks, Barracks; Out: Level 1..4 | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-INFR-02` | Construction Costs | `cs_upgradecityscript.psc` | In: Gold + Wood; Out: Level increment, `AddJobs()` | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-TECH-01` | 71 Perk Trees | `cs_troopupgradescript.psc`, ESP | In: Gold + Metal; Out: Perk ranks, stat scaling | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-TECH-02` | Civil Research | `cs_upgradecityscript.psc` | In: Upgrade facility; Out: Location keyword level | INCOMPLETE STUB | Low | None (Standalone state) | No separate perk tree exists |
| `FEAT-GOV-01` | The Steward | `cs_govpositions.psc` | In: Appoint; Out: Treasury, taxes, and emissaries active | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-GOV-02` | The Commander | `cs_govpositions.psc` | In: Appoint; Out: Military campaigns and POWs active | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-GOV-03` | The Blacksmith | `cs_govpositions.psc` | In: Appoint; Out: Standardized armory outfitting active | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-JARL-01` | Jarl Replacement | `cs_recruitjarl.psc` | In: Appoint; Out: Story Manager event, palace throne bound | CONFIRMED IN CODE | Low | Story Manager in Skyrim | None |
| `FEAT-JARL-02` | Dual Loyalty Tracking | `cs_loyaltycheckscript.psc` | In: Daily conditions; Out: `Loyalty` and `JarlLoyalty` | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-DIPL-01` | War Declaration | `cs_factiongovernmentscript.psc` | In: Declare; Out: `AtWarPlayer = true`, AI timers active | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-DIPL-02` | Truces & Summits | `cs_negotiationscript.psc` | In: Treaty terms; Out: CalculateSuccess() roll, truce | CONFIRMED IN CODE | Moderate | None (Standalone state) | None |
| `FEAT-DIPL-03` | POW Ransoms | `cs_prisonerquestscript.psc` | In: Ransom prisoner; Out: Gold added to treasury | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-LAW-01` | Autonomous Jurisdiction | `cs_transferownershipscript.psc` | In: City conquest; Out: `SetCrimeGold(0)` | CONFIRMED IN CODE | Low | Crime factions in Skyrim | None |
| `FEAT-LAW-02` | Judicial Hearings | `cs_courtscript.psc` | In: Sit on throne; Out: Cases judged, fines/executions | CONFIRMED IN CODE | Moderate | Scripted scenes in Skyrim | None |
| `FEAT-LAW-03` | Municipal Dungeons | `cs_prisonscript.psc` | In: Sentence jail; Out: Prisoner alias assigned | CONFIRMED IN CODE | Moderate | Dungeons in Skyrim | None |
| `FEAT-STAB-01` | Public Order Metrics | `cs_loyaltycheckscript.psc` | In: Food, taxes, garrison; Out: Hold `Loyalty` | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-STAB-02` | Protests & Secession | `cs_loyaltycheckscript.psc` | In: Loyalty < 40 / < 20; Out: `ProtestQuest` / `RebelDefend` | CONFIRMED IN CODE | Moderate | In-world spawns in Skyrim | None |
| `FEAT-PROG-01` | 6-Tier Title Loop | `cs_meetingscript.psc`, `cs_coronationscript.psc` | In: Conquest progress; Out: Title & authority progression | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-PROG-02` | Royal Court Retinue | `cs_govscriptfunctions.psc` | In: Capital takeover; Out: Bodyguards, throne markers | CONFIRMED IN CODE | Low | AI Packages in Skyrim | None |
| `FEAT-VICT-01` | Conquering 9 Holds | `cs_battletrackerscript.psc` | In: Hold conquests; Out: `Cities[0..8] == true` | CONFIRMED IN CODE | Low | None (Standalone state) | None |
| `FEAT-VICT-02` | The Moot Convocation | `cs_meetingscript.psc` | In: Convene; Out: 8 appointed Jarls vote, majority wins | CONFIRMED IN CODE | Moderate | Scripted scenes in Skyrim | Jarl aliasing deadlock bug |
| `FEAT-VICT-03` | Coronation Ceremony | `cs_coronationscript.psc` | In: Moot victory; Out: Crown awarded, `IsHighKing = 1` | CONFIRMED IN CODE | Moderate | Anims, crown items in Skyrim | None |
| `FEAT-UI-01` | Contextual Dialogue | 935 `TopicInfo` scripts | In: Dialogue choices; Out: Dispatched script functions | CONFIRMED IN CODE | High | Skyrim Dialogue Trees | None |
| `FEAT-UI-02` | MCM & Settings Menu | SkyUI MCM, `SettingsMenu` | In: Player config; Out: `CS_BattleSize`, `CS_BattleSpeed` | CONFIRMED IN CODE | Low | SkyUI / SKSE in Skyrim | None |

---

## 4. Standalone Rust Reconstruction Scope (Phases 13)

Of the 55 features:
- **48 Features** are fully implementable in the standalone Rust simulation model (`reconstruction/`), requiring zero Skyrim engine code.
- **3 Features** contain verified stubs or unknown heuristics (`FEAT-FACT-04`, `FEAT-TECH-02`, and general AI battle math in `FEAT-BAT-04`). They will be modeled with explicit configuration parameters and documented fallback behavior.
- **4 Features** are engine-tied interaction layers (`FEAT-UI-01`, `FEAT-UI-02`, 3D banner spawning in `FEAT-FACT-03`, and victory parade animations in `FEAT-BAT-03`), which belong to the integration track (Phase 15).

---

## 5. Subsystem Contract Specifications (Rust Targets)

### 5.1 Territorial Contract (`reconstruction::territory`)
- **Types**: `HoldingId` (0..14), `HoldingTier` (CapitalMajor, CapitalMinor, Village, Fort, Camp), `FactionId` (Player, Imperial, Stormcloak, Neutral).
- **Invariants**:
  - `HoldingId 0..8` represent the 9 hold capitals.
  - Ownership transitions must verify spatial contiguity (`is_adjacent(origin, target)`).
  - Transitioning ownership updates `holding_faction` and resets local crime gold.

### 5.2 Economic Contract (`reconstruction::economy`)
- **Types**: `Treasury` (i64 gold), `ResourceStockpile` (Food, Metal, Wood, Docks), `FacilityLevel` (1..4).
- **Formulas**:
  - $\text{Tax}(pop, rate, tier)$ evaluates exact 500/250, 300/150, 200/100 ratios.
  - $\text{Production}(level, change, docks, open\_jobs)$ evaluates verified formula.
  - Labor allocation strictly resolves Food &rarr; Docks &rarr; Wood &rarr; Metal.

### 5.3 Military Contract (`reconstruction::military`)
- **Types**: `Division` (0..9), `TroopClass` (Infantry, ShockWarrior, Archer, Crossbow, Battlemage, Automaton, Beast), `Order` (Follow, Patrol, Garrison, Camp, Hunt, Assault, StandBy).
- **Invariants**:
  - Maximum 10 divisions (`MaxArmies = 10`).
  - Upgrade costs evaluate $\text{Gold} = 5000 + 10000(L-1)$ and $\text{Metal} = 50 + 100(L-1)$.
  - Combat resolution evaluates wave reinforcement triggers at 20, 40, 60, 80, 100 casualties.
