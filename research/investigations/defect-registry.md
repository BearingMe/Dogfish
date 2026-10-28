# Master Defect Registry & Repair Prioritization

- **Document**: Consolidated Defect Backlog & Prioritization Registry
- **Milestone**: M66 — Defect Registry and Repair Prioritization
- **Phase**: Phase 16 (Baseline Verification and Defect Reproduction)
- **Subject**: Actionable Backlog Derived from M63, M64, and M65
- **Investigation Date**: 2026-10-28
- **Status**: CONSOLIDATED & PRIORITIZED (20 Registered Defects)

---

## 1. Executive Summary & Registry Purpose

Milestone M66 consolidates all findings, toolchain failures, traceability gaps, and test panics identified across M63 (Build Baseline), M64 (Traceability Audit), and M65 (Adversarial Regressions) into a single actionable defect registry.

Every defect is assigned:
- A unique identifier.
- A severity tier (P0, P1, P2, P3).
- An affected subsystem.
- A reproduction procedure and observed behavior.
- Source evidence.
- A root-cause hypothesis.
- An assigned repair milestone (Phases 17–20).
- Specific validation requirements.

---

## 2. Priority Breakdown Summary

$$\begin{aligned}
\mathbf{P0\ (Critical\ Correctness\ \&\ Integration)} &: 7\ \text{Defects} \\
\mathbf{P1\ (Significant\ Behavior\ \&\ Invariants)} &: 7\ \text{Defects} \\
\mathbf{P2\ (Limited\ Correctness\ \&\ Code\ Quality)} &: 4\ \text{Defects} \\
\mathbf{P3\ (Documentation\ \&\ Claims\ Reconciliation)} &: 2\ \text{Defects} \\
\hline
\mathbf{Total\ Registered\ Defects} &: \mathbf{20\ Defects}
\end{aligned}$$

---

## 3. Granular Defect Registry

### Priority P0: Critical Correctness & Integration

#### `DEF-PERS-01`: Active Siege Omission from Serialization
- **Severity**: P0
- **Affected Subsystem**: `reconstruction::persistence`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_01_save_load_with_active_siege`.
- **Expected Behavior**: A game saved mid-siege restores with the active siege operation, target holding, phase, and casualties intact.
- **Actual Behavior**: Deserialized state has `active_siege: None`. The siege is completely lost.
- **Source Evidence**: `reconstruction/src/persistence.rs:242-280` (`serialize()` never writes `active_siege`).
- **Root-Cause Hypothesis**: Initial persistence prototype implemented only minimal global properties and omitted the `MilitaryManager` siege container.
- **Repair Milestone**: **M67 (Complete State Persistence)**.
- **Validation**: `test_case_01_save_load_with_active_siege` passes.

---

#### `DEF-PERS-02`: Field Army Division & Appointed Governor Omission from Serialization
- **Severity**: P0
- **Affected Subsystem**: `reconstruction::persistence`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_02_save_load_with_modified_armies_and_governors`.
- **Expected Behavior**: Restored game state retains customized division soldier counts, orders, commander names, and appointed provincial Jarls.
- **Actual Behavior**: Division troop counts reset to 0, orders reset to `StandBy`, and all governors become `None`.
- **Source Evidence**: `reconstruction/src/persistence.rs:245-280` (omits `divisions` and `governors`).
- **Root-Cause Hypothesis**: Incomplete JSON schema in initial handwritten persistence engine.
- **Repair Milestone**: **M67 (Complete State Persistence)**.
- **Validation**: `test_case_02_save_load_with_modified_armies_and_governors` passes.

---

#### `DEF-PERS-03`: Diplomatic Relations, Cooldowns & PRNG State Omission from Serialization
- **Severity**: P0
- **Affected Subsystem**: `reconstruction::persistence`
- **Reproduction**: Inspect `StatePersistence::serialize()` in `reconstruction/src/persistence.rs`.
- **Expected Behavior**: Truce countdown timers, foreign faction relations, wilderness camp respawn cooldowns, and PRNG seeds survive save/load for deterministic continuation.
- **Actual Behavior**: Fields are omitted; loading a savegame resets diplomacy to default war and regenerates a fresh PRNG stream.
- **Source Evidence**: `reconstruction/src/persistence.rs:245-280`.
- **Root-Cause Hypothesis**: Partial prototype implementation.
- **Repair Milestone**: **M67 (Complete State Persistence)**.
- **Validation**: Full deterministic replay parity before and after save/load.

---

#### `DEF-PERS-04`: String Literal Quote & Backslash Injection in JSON Serializer
- **Severity**: P0
- **Affected Subsystem**: `reconstruction::persistence`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_03_faction_names_with_quotes_and_backslashes`.
- **Expected Behavior**: Names with quotes (`"`) and backslashes (`\`) are properly escaped (`\"`, `\\`) and round-trip without corruption.
- **Actual Behavior**: Raw quotes are injected directly into JSON (`"name": "The "King""`), causing `JsonParser` to choke with a syntax error on deserialization.
- **Source Evidence**: `reconstruction/src/persistence.rs:252` (`out.push_str(&format!(... state.faction.name))`).
- **Root-Cause Hypothesis**: Handwritten serializer formats strings directly without an escaping pass.
- **Repair Milestone**: **M68 (Serialization Safety and Data Validation)**.
- **Validation**: `test_case_03_faction_names_with_quotes_and_backslashes` passes.

---

#### `DEF-INTEG-01`: Raw Pointer Dereference in Safe SKSE Export Function
- **Severity**: P0
- **Affected Subsystem**: `integration::lib`
- **Reproduction**: Run `cargo clippy --manifest-path integration/Cargo.toml --offline`.
- **Expected Behavior**: Native C-ABI entry points compile cleanly without linter safety errors.
- **Actual Behavior**: Clippy fails with 3 errors (`#[deny(clippy::not_unsafe_ptr_arg_deref)]`) because `SKSEPlugin_Query` is marked safe but dereferences raw pointer `info`.
- **Source Evidence**: `integration/src/lib.rs:52-68`.
- **Root-Cause Hypothesis**: Function was defined as `pub extern "C" fn` instead of `pub unsafe extern "C" fn`.
- **Repair Milestone**: **M77 (SKSE ABI and Plugin Loader Validation)**.
- **Validation**: `cargo clippy --manifest-path integration/Cargo.toml` passes with zero errors.

---

#### `DEF-INTEG-02`: Missing Actual SKSE Co-Save Registration in Integration Adapter
- **Severity**: P0
- **Affected Subsystem**: `integration::lib`
- **Reproduction**: Inspect `SKSEPlugin_Load` in `integration/src/lib.rs:72-75`.
- **Expected Behavior**: Native plugin queries `SKSESerializationInterface` and registers `SetSaveCallback`, `SetLoadCallback`, and `SetRevertCallback`.
- **Actual Behavior**: `SKSEPlugin_Load` only calls `init_adapter(42)`. No SKSE serialization callbacks are registered with the engine.
- **Source Evidence**: `integration/src/lib.rs:72-75`.
- **Root-Cause Hypothesis**: The adapter was stubbed to satisfy unit tests without implementing the actual SKSE serialization interface query.
- **Repair Milestone**: **M79 (SKSE Save/Load and State Synchronization)**.
- **Validation**: Serialization callbacks registered and verifiable via native test.

---

#### `DEF-INTEG-03`: Missing Compiled Test Plugin & Papyrus Scripts
- **Severity**: P0
- **Affected Subsystem**: `integration/`
- **Reproduction**: Inspect `integration/` directory.
- **Expected Behavior**: A compiled or buildable `dogfish_test.esp` and compiled `DogfishClaimBannerScript.pex` exist for in-game testing.
- **Actual Behavior**: Test records and scripts exist only as documentation pseudocode in `integration/README.md`.
- **Source Evidence**: `integration/` contains only `Cargo.toml`, `README.md`, and `src/lib.rs`.
- **Root-Cause Hypothesis**: Previous milestones documented manual Creation Kit specifications instead of authoring actual plugin files.
- **Repair Milestone**: **M78 (Papyrus Native Bridge and Test Plugin)** & **M80 (Real Skyrim Vertical Slice)**.
- **Validation**: Physical test plugin files exist or are explicitly marked BLOCKED awaiting manual Creation Kit authoring.

---

### Priority P1: Significant Behavior & Invariants

#### `DEF-INVAR-01`: Missing Ownership Invariant on Facility Upgrades
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::economy`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_04_facility_upgrade_in_enemy_holding`.
- **Expected Behavior**: `upgrade_facility` rejects upgrading holdings not owned by `FactionId::Player`.
- **Actual Behavior**: Function succeeds, deducting player gold and wood to upgrade enemy-held cities.
- **Source Evidence**: `reconstruction/src/state.rs:326-343` (checks `idx >= self.holdings.len()`, never checks `owner`).
- **Root-Cause Hypothesis**: Missing domain invariant check in `GameState::upgrade_facility`.
- **Repair Milestone**: **M69 (Domain Invariants and Authorization)**.
- **Validation**: `test_case_04_facility_upgrade_in_enemy_holding` passes.

---

#### `DEF-INVAR-02`: Missing Ownership Invariant on Governor Appointments
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::territory`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_05_governor_appointment_in_enemy_holding`.
- **Expected Behavior**: Player cannot appoint Jarls to foreign or unowned provincial capitals.
- **Actual Behavior**: Function succeeds, binding a player governor to an enemy-held throne.
- **Source Evidence**: `reconstruction/src/territory.rs:69-80` (checks `holding_id.is_capital()`, never checks `owner == Player`).
- **Root-Cause Hypothesis**: Missing authorization check in `appoint_governor`.
- **Repair Milestone**: **M69 (Domain Invariants and Authorization)**.
- **Validation**: `test_case_05_governor_appointment_in_enemy_holding` passes.

---

#### `DEF-INVAR-03`: Instant Siege Victory Path Without Participating Troops
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::military`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_07_siege_progression_without_participating_army`.
- **Expected Behavior**: Advancing siege phases requires an assigned field division with living troops and combat victories.
- **Actual Behavior**: A player with 0 troops can call `advance_siege()` 4 times in 0 seconds and conquer a hold capital without a single battle.
- **Source Evidence**: `reconstruction/src/state.rs:356-371`.
- **Root-Cause Hypothesis**: Siege state machine was implemented with naive phase transitions devoid of tactical preconditions.
- **Repair Milestone**: **M74 (Military, Strategic AI, and Siege Fidelity)**.
- **Validation**: `test_case_07_siege_progression_without_participating_army` passes.

---

#### `DEF-INVAR-04`: Silent Overwrite of Active Siege Operations
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::military`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_06_repeated_siege_initiation`.
- **Expected Behavior**: Attempting to launch a second siege while one is underway is rejected or requires explicit abandonment.
- **Actual Behavior**: `start_siege` blindly overwrites `self.military.active_siege` with `Some(...)`.
- **Source Evidence**: `reconstruction/src/state.rs:352`.
- **Root-Cause Hypothesis**: Missing state conflict check.
- **Repair Milestone**: **M74 (Military, Strategic AI, and Siege Fidelity)**.
- **Validation**: `test_case_06_repeated_siege_initiation` passes.

---

#### `DEF-TRANS-01`: Negative Court Fines & POW Ransoms Corrupt Treasury
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::government`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_08_negative_court_fines_and_ransom`.
- **Expected Behavior**: Negative court fines and ransom payments are rejected with an error.
- **Actual Behavior**: `judge_case` applies `*treasury += amt` blindly, allowing negative fines to drain treasury or bypass invariants.
- **Source Evidence**: `reconstruction/src/government.rs:112`.
- **Root-Cause Hypothesis**: Missing sign validation on incoming fine arguments.
- **Repair Milestone**: **M70 (Transactional Consistency and Resource Safety)** & **M75**.
- **Validation**: `test_case_08_negative_court_fines_and_ransom` passes.

---

#### `DEF-TRANS-02`: Unchecked Arithmetic Overflow in Treasury Deposits
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::economy`
- **Reproduction**: Run `cargo test --test adversarial_regressions test_case_09_resource_overflow_protection`.
- **Expected Behavior**: Adding large values near `i64::MAX` handles overflow safely (e.g. saturating addition or returning an error).
- **Actual Behavior**: Triggers unhandled panic: `attempt to add with overflow`.
- **Source Evidence**: `reconstruction/src/economy.rs:60` (`*treasury += amount`).
- **Root-Cause Hypothesis**: Unchecked native arithmetic operator without bounds checking.
- **Repair Milestone**: **M70 (Transactional Consistency and Resource Safety)**.
- **Validation**: `test_case_09_resource_overflow_protection` passes without panic.

---

#### `DEF-FIDEL-01`: Invented Government Doctrine Modifiers
- **Severity**: P1
- **Affected Subsystem**: `reconstruction::faction`
- **Reproduction**: Inspect `reconstruction/src/faction.rs:43-63`.
- **Expected Behavior**: Government doctrine behavior reflects verified primary source mechanics or is isolated as an experimental stub.
- **Actual Behavior**: Hardcodes arbitrary numbers (+15% tax, +25% conscription for Autocracy; +20% tax, -15% conscription for Republic) that have zero basis in `cs_factiongovernmentscript.psc`.
- **Source Evidence**: `reconstruction/src/faction.rs:43-63`.
- **Root-Cause Hypothesis**: Implementer filled in the M46 incomplete stub with speculative numbers without isolating them.
- **Repair Milestone**: **M72 (Faction, World Model, and Territory Fidelity)**.
- **Validation**: Unsupported modifiers removed or clearly isolated and documented as experimental extensions.

---

### Priority P2: Limited Correctness & Code Quality

#### `DEF-FIDEL-02`: Simplified 5-Level Power Abstraction Instead of 71 Perk Records
- **Severity**: P2
- **Affected Subsystem**: `reconstruction::military`
- **Reproduction**: Inspect `reconstruction/src/military.rs:50`.
- **Expected Behavior**: The military progression system should faithfully represent the 4 tech branches and distinct perk unlocks.
- **Actual Behavior**: Entire 71-perk system is reduced to a single linear `power_level: u8 (1..5)`.
- **Source Evidence**: `reconstruction/src/military.rs:50`.
- **Root-Cause Hypothesis**: High-level abstraction chosen during M52 without distinguishing it from exact perk trees.
- **Repair Milestone**: **M74 (Military, Strategic AI, and Siege Fidelity)**.
- **Validation**: Proper branch categorization and accurate fidelity labeling.

---

#### `DEF-FIDEL-03`: Simplified Boolean Flags for Artillery and Pincer Attacks
- **Severity**: P2
- **Affected Subsystem**: `reconstruction::military`
- **Reproduction**: Inspect `SiegeOperation` in `reconstruction/src/military.rs:183-186`.
- **Expected Behavior**: Artillery and pincer assault choices impact battle casualties, breach timers, and reinforcement mechanics.
- **Actual Behavior**: Flags (`artillery_enabled`, `pincer_assault`) are stored but never read or evaluated in siege progression.
- **Source Evidence**: `reconstruction/src/military.rs:183-186`.
- **Root-Cause Hypothesis**: Cosmetic struct fields added without implementing downstream behavioral logic.
- **Repair Milestone**: **M74 (Military, Strategic AI, and Siege Fidelity)**.
- **Validation**: Artillery and pincer options have measurable impact on siege mechanics or are accurately reclassified.

---

#### `DEF-FIDEL-04`: Unbounded Event Log Memory Growth
- **Severity**: P2
- **Affected Subsystem**: `reconstruction::state`
- **Reproduction**: Execute a simulation running for 1,000+ game days.
- **Expected Behavior**: Long-running simulations maintain bounded memory or provide an event retention policy.
- **Actual Behavior**: `self.events.push(...)` appends every daily event indefinitely into a growing `Vec<SimulationEvent>`.
- **Source Evidence**: `reconstruction/src/state.rs:142-185`.
- **Root-Cause Hypothesis**: Lack of ring-buffer or event truncation mechanism.
- **Repair Milestone**: **M71 (Time, Randomness, and Long-Running Simulation)**.
- **Validation**: Memory usage remains bounded across 10,000-tick simulations.

---

#### `DEF-CODE-01`: 8 Clippy Linter Warnings in `reconstruction/`
- **Severity**: P2
- **Affected Subsystem**: `reconstruction/`
- **Reproduction**: Run `cargo clippy --manifest-path reconstruction/Cargo.toml --offline`.
- **Expected Behavior**: Codebase passes standard Rust linter checks cleanly.
- **Actual Behavior**: 8 warnings emitted (`manual_clamp`, `useless_format`, `doc_lazy_continuation`, `vec_init_then_push`, `match_like_matches_macro`).
- **Source Evidence**: `research/investigations/build-and-test-baseline.md:Section 4.1`.
- **Root-Cause Hypothesis**: Code authoring used non-idiomatic patterns.
- **Repair Milestone**: **M71 & M73**.
- **Validation**: `cargo clippy` passes cleanly on `reconstruction/`.

---

### Priority P3: Documentation & Claims Reconciliation

#### `DEF-DOCS-01`: Inflated Fidelity Percentage Claims in Previous Reports
- **Severity**: P3
- **Affected Subsystem**: `docs/`
- **Reproduction**: Compare `docs/reconstruction-coverage.md` claims (73.5% EXACT) against `research/investigations/contract-traceability-audit.md` findings (18.2% EXACT).
- **Expected Behavior**: Published documentation accurately reflects verified code reality.
- **Actual Behavior**: Prior reports claimed 73.5% EXACT fidelity by treating boolean flags and simple counters as exact implementations.
- **Source Evidence**: `docs/reconstruction-coverage.md`, `docs/reconstruction-feasibility.md`.
- **Root-Cause Hypothesis**: Generous self-classification in earlier reporting phases without adversarial scrutiny.
- **Repair Milestone**: **M81 (Fidelity, Coverage, and Claims Reconciliation)**.
- **Validation**: Documentation reconciled with exact mathematical denominators.

---

#### `DEF-DOCS-02`: Inaccurate Representation of Integration Completeness
- **Severity**: P3
- **Affected Subsystem**: `docs/`
- **Reproduction**: Review `docs/reconstruction-feasibility.md` statements claiming full integration demonstration.
- **Expected Behavior**: Documentation clearly distinguishes native unit tests from actual Skyrim engine runtime execution.
- **Actual Behavior**: Feasibility report conflated NATIVE ABI mock tests with in-engine runtime validation.
- **Source Evidence**: `docs/reconstruction-feasibility.md:Section 2 (Question 4)`.
- **Root-Cause Hypothesis**: Premature declaration of completion based on unit-tested C-ABI exports.
- **Repair Milestone**: **M81 (Fidelity, Coverage, and Claims Reconciliation)**.
- **Validation**: RUNTIME vs NATIVE levels strictly distinguished across all documentation.

---

## 4. Phase 16 Gate Closure Verdict

With the consolidation of M63, M64, and M65 into this registry:
1. **Verified Execution Baseline**: Established in `research/investigations/build-and-test-baseline.md`.
2. **Contract Traceability**: Established in `research/investigations/contract-traceability-audit.md`.
3. **Reproducible Failure Evidence**: Established in `reconstruction/tests/adversarial_regressions.rs` (9 concrete failures).
4. **Actionable Repair Backlog**: 20 defects prioritized and assigned to specific repair milestones across Phases 17–20.

**Phase 16 Gate is formally PASSED.**
