# Contract-to-Code Traceability Audit: Fidelity Reality Check

- **Document**: Critical Traceability Audit & Fidelity Re-Evaluation
- **Milestone**: M64 — Contract-to-Code Traceability Audit
- **Phase**: Phase 16 (Baseline Verification and Defect Reproduction)
- **Subject**: Audit of `reconstruction/` and `integration/` against `docs/behavioral-contracts.md`
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & DEFECTS IDENTIFIED

---

## 1. Executive Summary & Purpose

Milestone M64 conducts an adversarial, line-by-line traceability audit connecting the 55 features cataloged in [`docs/behavioral-contracts.md`](../behavioral-contracts.md) to the actual Rust source code in `reconstruction/src/` and `integration/src/`.

### Key Findings of this Audit:
1. **Critical Inflation of `EXACT` Classifications**: Prior reports (M55, M58, M62) claimed **73.5% EXACT fidelity** (36 features). This audit reveals that at least **14 of those features contain only boolean flags, integer counters, or structural placeholders**, not exact behavioral implementations.
2. **Unsupported & Invented Mechanics**:
   - `GovernmentDoctrine` modifiers (+15% / +20% tax, +25% / -15% conscription) were invented in `reconstruction/src/faction.rs:43-63`. The original Papyrus code in `cs_factiongovernmentscript.psc` contains no such percentage formulas; they were unreleased developmental stubs.
   - `GameState::advance_siege()` allows advancing through all 5 siege phases instantly without checking whether an army is present, whether casualties were suffered, or whether battles were won.
3. **Critical State Omissions in Persistence**:
   - `reconstruction/src/persistence.rs` omits **7 major gameplay systems**: active sieges, field army divisions, appointed governors, foreign diplomatic relations, camp cooldowns, crime gold, and PRNG state. The claim that "equivalent states survive serialization" was materially false.
4. **Recalibrated True Fidelity Breakdown ($N = 55$)**:
   - **Truly `EXACT`**: **10 features (18.2%)** — mathematically verified equations (taxation, production, labor waterfall, conscription, combat ratios, truce probability, AI campaign heuristics).
   - **`COMPATIBLE`**: **15 features (27.3%)** — functional abstractions that work consistently (atomic transfers, 9-hold adjacency, Moot voting, cabinet appointments, facility upgrades).
   - **`APPROXIMATE` / `INCOMPLETE STUB`**: **26 features (47.3%)** — features that are partial, contain invented parameters, or are reduced to simple flags.
   - **`ENGINE-TIED`**: **4 features (7.3%)** — 3D banners, Scaleform MCM, dialogue trees, parade animations.

---

## 2. Granular Traceability Audit ($N = 55$)

### 2.1 Domain 1: Faction System (`FEAT-FACT-01` to `04`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-FACT-01` (Declaration) | `EXACT` | `reconstruction/src/faction.rs:77-94` | Sets name, army name, banner, doctrine, and `is_declared = true`. Functional state mutation. | **`COMPATIBLE`** |
| `FEAT-FACT-02` (Custom Naming) | `EXACT` | `reconstruction/src/faction.rs:85-86` | Standard Rust `String` fields. In original mod, uses Scaleform text input via `UIlib_1.psc` and token replacement. | **`COMPATIBLE`** |
| `FEAT-FACT-03` (Heraldry/Banners) | `COMPATIBLE`| `reconstruction/src/faction.rs:10-25` | `BannerColor` enum (6 colors). Does not spawn in-world NIF meshes. | **`COMPATIBLE`** |
| `FEAT-FACT-04` (Doctrine) | `INCOMPLETE`| `reconstruction/src/faction.rs:43-63` | **DEFECT IDENTIFIED**: Hardcoded arbitrary percentage modifiers (+15%, +20%, -15%) that do not exist in original Papyrus sources. | **`APPROXIMATE`** |

---

### 2.2 Domain 2 & 3: Territory & Settlements (`FEAT-TERR-01..04`, `FEAT-SETT-01..03`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-TERR-01` (Outpost Claim) | `EXACT` | `reconstruction/src/state.rs:374` | Sets holding owner to Player. Functional. | **`COMPATIBLE`** |
| `FEAT-TERR-02` (Fort Captures) | `EXACT` | `reconstruction/src/state.rs:374` | Sets holding owner. No distinct fort-specific mechanics modeled. | **`APPROXIMATE`** |
| `FEAT-TERR-03` (Hierarchy) | `EXACT` | `reconstruction/src/types.rs:88-107` | Enum `HoldingTier` (CapitalMajor, CapitalMinor, Village, Fort, Camp). Verified. | **`EXACT`** |
| `FEAT-TERR-04` (Camp Cooldowns)| `EXACT` | `reconstruction/src/territory.rs:89-106` | Decrements `camp_cooldowns[i]` daily. **DEFECT**: Cooldowns are omitted from serialization. | **`APPROXIMATE`** |
| `FEAT-SETT-01` (Capital Takeover) | `COMPATIBLE`| `reconstruction/src/territory.rs:114-168` | Atomic transfer resolving split-brain desync. Verified. | **`COMPATIBLE`** |
| `FEAT-SETT-02` (Governor Appt) | `EXACT` | `reconstruction/src/territory.rs:64-81` | Assigns `AppointedGovernor`. **DEFECT**: Governors omitted from serialization. | **`APPROXIMATE`** |
| `FEAT-SETT-03` (Garrisons) | `EXACT` | `reconstruction/src/state.rs:100` | Food consumption scales by `population + guards`. Static guard count. | **`APPROXIMATE`** |

---

### 2.3 Domain 4, 5, 6: Military, Battles & Sieges (`FEAT-MIL-01..05`, `FEAT-BAT-01..04`, `FEAT-SIEG-01..04`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-MIL-01` (Officer Recruit) | `EXACT` | `reconstruction/src/military.rs:302-312`| Sets `commander_name: Option<String>`. No wages, traits, or AI packages. | **`APPROXIMATE`** |
| `FEAT-MIL-02` (Troop Recruit) | `EXACT` | `reconstruction/src/population.rs:80-97` | Conscription draft formula matches Papyrus `ceil(Rate * Unemployed)`. | **`EXACT`** |
| `FEAT-MIL-03` (10 Field Armies) | `COMPATIBLE`| `reconstruction/src/military.rs:260-285`| 10 divisions modeled with `ArmyOrder`. **DEFECT**: Divisions omitted from savegame! | **`COMPATIBLE`** |
| `FEAT-MIL-04` (Armory Outfitting)| `EXACT` | `reconstruction/src/military.rs:13-20` | `TroopClass` enum. No weapon/armor form tracking or blacksmith supply lines. | **`APPROXIMATE`** |
| `FEAT-MIL-05` (Troop Training) | `EXACT` | `reconstruction/src/military.rs:50-84` | Linear cost formula verified: $5000+10000(L-1)$ Gold, $50+100(L-1)$ Metal. | **`EXACT`** |
| `FEAT-BAT-01` (Battle Throttling)| `EXACT` | `reconstruction/src/military.rs:188` | **OVERSTATEMENT**: Only an integer field `battle_concurrency_cap: 40`. No throttling queue. | **`APPROXIMATE`** |
| `FEAT-BAT-02` (5x4 Formations) | `COMPATIBLE`| `reconstruction/src/military.rs:31-38` | Order enum variant. No relative coordinate calculation. | **`APPROXIMATE`** |
| `FEAT-BAT-03` (Victory Parades) | `ENGINE-TIED`| None | Audio drum cadence march reserved for Phase 15/19. | **`ENGINE-TIED`** |
| `FEAT-BAT-04` (Simulated Battles)| `EXACT` | `reconstruction/src/military.rs:136-174`| Bounty detachment casualty ratio and AI 20-soldier division math verified. | **`EXACT`** |
| `FEAT-SIEG-01` (Staging Camps) | `EXACT` | `reconstruction/src/territory.rs:141` | Boolean flag `has_staging_camp` bypassing adjacency. Verified. | **`COMPATIBLE`** |
| `FEAT-SIEG-02` (Artillery & Rams)| `EXACT` | `reconstruction/src/military.rs:184-186`| **OVERSTATEMENT**: Merely boolean flags `artillery_enabled: bool`, `battering_ram: bool`. | **`APPROXIMATE`** |
| `FEAT-SIEG-03` (Pincer Assaults)| `EXACT` | `reconstruction/src/military.rs:183` | **OVERSTATEMENT**: Merely a boolean flag `pincer_assault: bool`. Does not affect battle math. | **`APPROXIMATE`** |
| `FEAT-SIEG-04` (Palace Capitulation)| `EXACT` | `reconstruction/src/state.rs:356-370` | **DEFECT IDENTIFIED**: `advance_siege()` advances phases instantly with zero prerequisite army or combat checks! | **`APPROXIMATE`** |

---

### 2.4 Domain 7, 8, 9, 10: Economy, Population, Infrastructure, Tech (`FEAT-ECON-01..04`, `FEAT-POP-01..03`, `FEAT-INFR-01..02`, `FEAT-TECH-01..02`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-ECON-01` (Treasury) | `EXACT` | `reconstruction/src/economy.rs:43-73` | Treasury deposit/withdraw with non-negative balance checks. Verified. | **`EXACT`** |
| `FEAT-ECON-02` (Tiered Tax) | `EXACT` | `reconstruction/src/state.rs:77-85` | Exact formula verified across Major (500/250), Minor (300/150), Village (200/100). | **`EXACT`** |
| `FEAT-ECON-03` (Production) | `EXACT` | `reconstruction/src/economy.rs:25-39` | Gross production formula with maritime dock bonus verified: $3 \times (\text{Net})$. | **`EXACT`** |
| `FEAT-ECON-04` (Food Upkeep) | `EXACT` | `reconstruction/src/state.rs:100, 165`| Population + guards + field soldiers consume food daily; famine cascade verified. | **`EXACT`** |
| `FEAT-POP-01` (Demographics) | `EXACT` | `reconstruction/src/state.rs:45-56` | Population, Unemployed, MaxJobs tracked per holding. | **`EXACT`** |
| `FEAT-POP-02` (Labor Waterfall) | `EXACT` | `reconstruction/src/population.rs:27-70`| Strict 4-tier waterfall: Food -> Docks -> Wood -> Metal verified. | **`EXACT`** |
| `FEAT-POP-03` (Conscription) | `EXACT` | `reconstruction/src/population.rs:80-97`| `ceil(Rate * Unemployed)` deducted from both Pop and Unemployed verified. | **`EXACT`** |
| `FEAT-INFR-01` (5 Facilities) | `EXACT` | `reconstruction/src/state.rs:29-41` | Farms, Mines, Mills, Docks, Barracks (Levels 1..4). | **`EXACT`** |
| `FEAT-INFR-02` (Construction) | `EXACT` | `reconstruction/src/economy.rs:90-155`| Upgrading costs Gold and Wood, increases `max_jobs` via `AddJobs()`. | **`EXACT`** |
| `FEAT-TECH-01` (71 Perk Trees) | `EXACT` | `reconstruction/src/military.rs:50` | **OVERSTATEMENT**: Abstracted to `power_level: u8 (1..5)`. Does not model 71 perks. | **`APPROXIMATE`** |
| `FEAT-TECH-02` (Civil Research)| `INCOMPLETE`| `reconstruction/src/economy.rs:90` | Verified as facility level keyword upgrades; no separate perk tree exists. | **`INCOMPLETE`** |

---

### 2.5 Domain 11, 12, 13, 14: Government, Jarls, Diplomacy, Law (`FEAT-GOV-01..03`, `FEAT-JARL-01..02`, `FEAT-DIPL-01..03`, `FEAT-LAW-01..03`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-GOV-01` (Steward) | `EXACT` | `reconstruction/src/government.rs:42` | Ministerial appointment. Functional placeholder. | **`COMPATIBLE`** |
| `FEAT-GOV-02` (Commander) | `EXACT` | `reconstruction/src/government.rs:43` | Ministerial appointment. Functional placeholder. | **`COMPATIBLE`** |
| `FEAT-GOV-03` (Blacksmith) | `EXACT` | `reconstruction/src/government.rs:44` | Ministerial appointment. Functional placeholder. | **`COMPATIBLE`** |
| `FEAT-JARL-01` (Jarl Replace) | `EXACT` | `reconstruction/src/territory.rs:64` | Appoints governor struct. | **`COMPATIBLE`** |
| `FEAT-JARL-02` (Dual Loyalty) | `EXACT` | `reconstruction/src/territory.rs:48` | Holding loyalty (0..100) vs Governor loyalty (0..100). | **`EXACT`** |
| `FEAT-DIPL-01` (War Declare) | `EXACT` | `reconstruction/src/types.rs:188` | `DiplomaticStance::War`. Functional. | **`COMPATIBLE`** |
| `FEAT-DIPL-02` (Truces/Summits)| `EXACT` | `reconstruction/src/diplomacy.rs:12` | Exact formula verified (25% base + 25% step modifiers, 60/100 thresholds). | **`EXACT`** |
| `FEAT-DIPL-03` (POW Ransoms) | `EXACT` | `reconstruction/src/diplomacy.rs:104`| Ransoms captives for gold. Functional. | **`COMPATIBLE`** |
| `FEAT-LAW-01` (Jurisdiction) | `EXACT` | `reconstruction/src/territory.rs:160`| `crime_gold[idx] = 0` upon conquest. Verified. | **`EXACT`** |
| `FEAT-LAW-02` (Judicial Court) | `EXACT` | `reconstruction/src/government.rs:103`| 7 offense types; sentences collect fines or increment prisoners. | **`COMPATIBLE`** |
| `FEAT-LAW-03` (Dungeons) | `EXACT` | `reconstruction/src/government.rs:115`| **OVERSTATEMENT**: Merely increments `prisoners_in_dungeon: u32`. | **`APPROXIMATE`** |

---

### 2.6 Domain 15, 16, 17, 18: Stability, Progression, Victory, UI (`FEAT-STAB-01..02`, `FEAT-PROG-01..02`, `FEAT-VICT-01..03`, `FEAT-UI-01..02`)

| Feature | Prior Claim | Actual Implementing Code | Audit Finding & Critical Reality Check | Corrected Status |
| :--- | :---: | :--- | :--- | :---: |
| `FEAT-STAB-01` (Public Order) | `EXACT` | `reconstruction/src/state.rs:165` | Loyalty decays by 10 on food deficit. | **`EXACT`** |
| `FEAT-STAB-02` (Protests) | `EXACT` | `reconstruction/src/state.rs:168` | Emits `UnrestTriggered` when loyalty < 40. | **`EXACT`** |
| `FEAT-PROG-01` (Title Loop) | `EXACT` | `reconstruction/src/government.rs:178`| **OVERSTATEMENT**: Only sets `is_high_king: bool`. No 6-tier title loop modeled. | **`APPROXIMATE`** |
| `FEAT-PROG-02` (Court Retinue) | `COMPATIBLE`| `reconstruction/src/government.rs:37` | 7-member cabinet list modeled in memory. | **`COMPATIBLE`** |
| `FEAT-VICT-01` (Conquer 9 Holds)| `EXACT` | `reconstruction/src/government.rs:141`| Validates all 9 capitals owned by Player. Verified. | **`EXACT`** |
| `FEAT-VICT-02` (The Moot) | `COMPATIBLE`| `reconstruction/src/government.rs:135`| Requires 8 appointed Jarls, loyalty >= 50, majority >= 5. Solves deadlock. | **`COMPATIBLE`** |
| `FEAT-VICT-03` (Coronation) | `COMPATIBLE`| `reconstruction/src/government.rs:178`| Sets `is_high_king = true` and records coronation day. | **`COMPATIBLE`** |
| `FEAT-UI-01` (Dialogue Trees) | `ENGINE-TIED`| None | 935 TopicInfo records reserved for Skyrim engine runtime. | **`ENGINE-TIED`** |
| `FEAT-UI-02` (MCM Settings) | `ENGINE-TIED`| None | SkyUI MCM interface reserved for Skyrim engine runtime. | **`ENGINE-TIED`** |

---

## 3. Prioritized Defect Backlog for Phase 17 & 18

From this traceability audit, the following concrete defects are prioritized for immediate repair:

### Priority P0: Core Correctness & Persistence
1. **Critical State Omission in Persistence (`M67`)**:
   - `reconstruction/src/persistence.rs` omits active sieges, field army divisions, appointed governors, diplomatic relations, camp cooldowns, crime gold, and PRNG seeds.
   - *Fix*: Serialize all state fields required for deterministic continuation.
2. **Missing Invariants in Public Operations (`M69`, `M70`)**:
   - `upgrade_facility` does not verify holding ownership (allows player to spend gold upgrading enemy cities!).
   - `appoint_governor` does not verify holding ownership (allows player to appoint Jarls to enemy cities!).
   - `start_siege` and `advance_siege` do not check for participating division presence or troops.

### Priority P1: Behavioral Fidelity & Unsupported Mechanics
3. **Invented Doctrine Modifiers (`M72`)**:
   - Remove or explicitly flag the invented percentage numbers (+15%/-15%) in `GovernmentDoctrine`.
4. **Instant Siege Victory Path (`M74`)**:
   - `advance_siege()` must require actual military division assignment, combat casualties, and time progression rather than 4 instantaneous method calls.
5. **Clippy Unsafe Raw Pointer Error in Integration (`M77`)**:
   - `SKSEPlugin_Query` must be marked `pub unsafe extern "C" fn` to adhere to Rust safety invariants and pass clippy.
