# Research Roadmap & Milestone Registry

## 1. Overview & Roadmap Status

This document tracks all 82 milestones of the *Conquest of Skyrim* specification recovery, probing, experimental reconstruction, and validation & repair program across 20 phases. It serves as the authoritative operational roadmap.

### Program Progression
- **Phases 1–8 (M01–M32)**: Initial specification recovery, architectural reverse engineering, and master specification synthesis (COMPLETE, commit `00323d9`).
- **Phases 9–11 (M33–M44)**: Evidence integrity audit, reproducible binary and script verification, numerical formula test suites, and empirical playtesting protocol (COMPLETE, commit `b935278`).
- **Phases 12–15 (M45–M62)**: Experimental reconstruction foundations, independent Rust simulation model, differential probing, and Skyrim runtime integration experiments (COMPLETE, commit `56305b1`).
- **Phases 16–20 (M63–M82)**: Reconstruction validation and repair program: execution baselines, contract traceability, adversarial defect reproduction, persistence and invariant repair, gameplay fidelity repair, native SKSE integration repair, and reconciled quality gate.

### Roadmap Summary

| Total Milestones | NOT STARTED | IN PROGRESS | BLOCKED | COMPLETE |
| :---: | :---: | :---: | :---: | :---: |
| 82 | 19 | 0 | 0 | 63 |

### Allowed Status Values
- **NOT STARTED**: Task has not yet begun.
- **IN PROGRESS**: Active research, probing, or reconstruction underway.
- **BLOCKED**: Execution halted by external dependencies or environmental constraints.
- **COMPLETE**: Deliverable created, evidence verified, review passed, and changes committed.

---

## 2. Master Milestone Registry

Dependencies below are conservative planning inputs, not blanket prerequisites for starting independent research. M01–M44 historical records and commits are preserved. Independent tasks in Phases 12–15 proceed whenever their specific inputs are available. Shared roadmap and configuration changes remain coordinator-owned.

The validation column records PASS, FAIL, BLOCKED, or INCONCLUSIVE once checked; Pending means no validation has run. COMPLETE follows `docs/execution-discipline.md`, including review/commit and explicit handling of limitations. Planned deliverable paths are created only when their milestone begins.

| ID | Title | Phase | Status | Dependencies | Assigned Agent | Deliverable | Validation | Blockers | Commit |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **M01** | Project Identity & History | Phase 1 | COMPLETE | None | Coordinator | `research/findings/project-overview.md` | PASS (Historical) | None | `808cba1` |
| **M02** | Documentation Recovery | Phase 1 | COMPLETE | None | Coordinator | `research/findings/documentation-inventory.md` | PASS (Historical) | None | `288d005` |
| **M03** | Comprehensive Feature Inventory | Phase 1 | COMPLETE | M01, M02 | Coordinator | `research/findings/feature-inventory.md` | PASS (Historical) | None | `2670636` |
| **M04** | Issues & Compatibility Inventory | Phase 1 | COMPLETE | M01, M02 | Coordinator | `research/findings/known-issues.md` | PASS (Historical) | None | `8a5462c` |
| **M05** | Distribution & Dependency Analysis | Phase 2 | COMPLETE | Phase 1 Gate | Coordinator | `research/findings/file-inventory.md` | PASS (Historical) | None | `606b2e7` |
| **M06** | Creation Engine Integration | Phase 2 | COMPLETE | M05 | Coordinator | `research/findings/skyrim-integration.md` | PASS (Historical) | None | `660fb67` |
| **M07** | Static Structure Recovery | Phase 2 | COMPLETE | M05, M06 | Coordinator | `research/findings/static-structure.md` | PASS (Historical) | None | `ef03ca4` |
| **M08** | Initialization & Runtime Lifecycle | Phase 2 | COMPLETE | M05, M06 | Coordinator | `research/findings/runtime-lifecycle.md` | PASS (Historical) | None | `90c2f86` |
| **M09** | Faction Creation | Phase 3 | COMPLETE | Phase 2 Gate | Coordinator | `research/findings/faction-creation.md` | PASS (Historical) | None | `dd5fa2a` |
| **M10** | Territory & Ownership | Phase 3 | COMPLETE | M09 | Coordinator | `research/findings/territory.md` | PASS (Historical) | None | `b7d1d82` |
| **M11** | Progression & Victory | Phase 3 | COMPLETE | M09, M10 | Coordinator | `research/findings/progression.md` | PASS (Historical) | None | `1ca2285` |
| **M12** | Player Interaction & Configuration | Phase 3 | COMPLETE | M09 | Coordinator | `research/findings/interactions.md` | PASS (Historical) | None | `47817a1` |
| **M13** | Army & Recruitment Systems | Phase 4 | COMPLETE | Phase 3 Gate | Coordinator | `research/findings/military-organization.md` | PASS (Historical) | None | `28be1d5` |
| **M14** | Orders, Movement & Deployment | Phase 4 | COMPLETE | M13 | Coordinator | `research/findings/military-orders.md` | PASS (Historical) | None | `d505daa` |
| **M15** | Combat Simulation | Phase 4 | COMPLETE | M13, M14 | Coordinator | `research/findings/battles.md` | PASS (Historical) | None | `c29f2ee` |
| **M16** | Siege Warfare | Phase 4 | COMPLETE | M14, M15 | Coordinator | `research/findings/sieges.md` | PASS (Historical) | None | `b939bde` |
| **M17** | Resource & Treasury Systems | Phase 5 | COMPLETE | Phase 3 Gate | Coordinator | `research/findings/economy.md` | PASS (Historical) | None | `a3308b5` |
| **M18** | Population & Employment | Phase 5 | COMPLETE | M17 | Coordinator | `research/findings/population.md` | PASS (Historical) | None | `7ee082a` |
| **M19** | Buildings & Infrastructure | Phase 5 | COMPLETE | M17, M18 | Coordinator | `research/findings/infrastructure.md` | PASS (Historical) | None | `da990e7` |
| **M20** | Technology & Research | Phase 5 | COMPLETE | M17, M19 | Coordinator | `research/findings/technology.md` | PASS (Historical) | None | `9792f7e` |
| **M21** | Government & Officials | Phase 6 | COMPLETE | Phase 4 & 5 Gates | Coordinator | `research/findings/government.md` | PASS (Historical) | None | `a6b8588` |
| **M22** | Jarls & Hold Administration | Phase 6 | COMPLETE | M21, M10 | Coordinator | `research/findings/jarls.md` | PASS (Historical) | None | `80da2db` |
| **M23** | Diplomacy | Phase 6 | COMPLETE | M21, M22 | Coordinator | `research/findings/diplomacy.md` | PASS (Historical) | None | `f1764aa` |
| **M24** | Law, Crime & Stability | Phase 6 | COMPLETE | M21, M22 | Coordinator | `research/findings/law-and-stability.md` | PASS (Historical) | None | `68eb1f4` |
| **M25** | Cross-System Dependency Recovery | Phase 7 | COMPLETE | Phase 6 Gate | Coordinator | `research/findings/system-dependencies.md` | PASS (Historical) | None | `d72bba4` |
| **M26** | Persistence & State Consistency | Phase 7 | COMPLETE | M25 | Coordinator | `research/findings/persistence.md` | PASS (Historical) | None | `a154283` |
| **M27** | Vanilla Skyrim Compatibility | Phase 7 | COMPLETE | M25 | Coordinator | `research/findings/vanilla-compatibility.md` | PASS (Historical) | None | `1515ca6` |
| **M28** | Failure Modes & Reliability | Phase 7 | COMPLETE | M25, M26, M27 | Coordinator | `research/findings/failure-modes.md` | PASS (Historical) | None | `62ccb3c` |
| **M29** | Behavioral Specification | Phase 8 | COMPLETE | Phase 7 Gate | Coordinator | `docs/behavioral-specification.md` | PASS (Historical) | None | `f38fb85` |
| **M30** | Recovered System Model | Phase 8 | COMPLETE | M29 | Coordinator | `docs/system-model.md` | PASS (Historical) | None | `6faf526` |
| **M31** | Completeness & Uncertainty Audit | Phase 8 | COMPLETE | M29, M30 | Coordinator | `docs/research-gaps.md` | PASS (Historical) | None | `cda0502` |
| **M32** | Final Recovered Specification | Phase 8 | COMPLETE | M29, M30, M31 | Coordinator | `docs/recovered-specification.md` | PASS (Historical) | None | `68a5915` |
| **M33** | Source Availability & Provenance Audit | Phase 9 | COMPLETE | None | Coordinator | `research/findings/source-provenance-audit.md` | PASS | None | `f0fe35c` |
| **M34** | Evidence Traceability Audit | Phase 9 | COMPLETE | M33 | Coordinator | `research/findings/evidence-audit.md` | PASS | None | `f426be7` |
| **M35** | Research Execution & Harness Compliance Audit | Phase 9 | COMPLETE | M33 | Coordinator | `research/findings/execution-audit.md` | PASS | None | `2c47d30` |
| **M36** | Confidence Reclassification | Phase 9 | COMPLETE | M33, M34, M35 | Coordinator | `research/findings/confidence-audit.md` | PASS | None | `1b2665c` |
| **M37** | Binary & Structural Verification | Phase 10 | COMPLETE | M33, Phase 9 Gate | Coordinator | `research/findings/verified-structure.md` | PASS | None | `7fad1ce` |
| **M38** | Papyrus Claim Verification | Phase 10 | COMPLETE | M37 | Coordinator | `research/findings/verified-papyrus-behavior.md` | PASS | None | `365b8a8` |
| **M39** | Numerical Mechanics & Formula Verification | Phase 10 | COMPLETE | M38 | Coordinator | `research/findings/verified-mechanics.md` | PASS | None | `eff2d0e` |
| **M40** | State, Persistence & Integration Verification | Phase 10 | COMPLETE | M38 | Coordinator | `research/findings/verified-state-and-integration.md` | PASS | None | `a343b8a` |
| **M41** | Controlled Playtesting Program | Phase 11 | COMPLETE | M36 | Coordinator | `research/playtesting/test-plan.md` | PASS | None | `971d64f` |
| **M42** | Static & Runtime Evidence Reconciliation | Phase 11 | COMPLETE | M39, M40, M41 | Coordinator | `research/findings/behavior-reconciliation.md` | PASS | None | `017828d` |
| **M43** | Adversarial Specification Audit | Phase 11 | COMPLETE | M36, M42 | Coordinator | `research/findings/adversarial-audit.md` | PASS | None | `49dc44d` |
| **M44** | Final Specification Integrity Gate | Phase 11 | COMPLETE | M43 | Coordinator | `docs/specification-integrity-report.md` | PASS | None | `f5c3164` |
| **M45** | Research Policy Transition | Phase 12 | COMPLETE | None | Coordinator | `guardrails.md`, `AGENTS.md` | PASS | None | `e40d1ff` |
| **M46** | Behavioral Contract Inventory | Phase 12 | COMPLETE | M45 | Coordinator | `docs/behavioral-contracts.md` | PASS | None | `6d13c5a` |
| **M47** | Reconstruction Validation Strategy | Phase 12 | COMPLETE | M45 | Coordinator | `docs/reconstruction-testing.md` | PASS | None | `d803cf2` |
| **M48** | Skyrim Integration Environment Survey | Phase 12 | COMPLETE | M45 | Coordinator | `docs/integration-survey.md` | PASS | None | `23fb296` |
| **M49** | Minimal Rust Simulation Core | Phase 13 | COMPLETE | M46, M47, Phase 12 Gate | Coordinator | `reconstruction/` | PASS | None | `ace699b` |
| **M50** | Faction & Territory Reconstruction | Phase 13 | COMPLETE | M49 | Coordinator | `reconstruction/` | PASS | None | `efe3437` |
| **M51** | Economy & Population Reconstruction | Phase 13 | COMPLETE | M49, M50 | Coordinator | `reconstruction/` | PASS | None | `e889494` |
| **M52** | Military & Warfare Reconstruction | Phase 13 | COMPLETE | M49, M50 | Coordinator | `reconstruction/` | PASS | None | `a055c1c` |
| **M53** | Government & Diplomacy Reconstruction | Phase 13 | COMPLETE | M50, M51, M52 | Coordinator | `reconstruction/` | PASS | None | `176b593` |
| **M54** | Persistence & Scenario Execution | Phase 13 | COMPLETE | M49–M53 | Coordinator | `reconstruction/` | PASS | None | `e127cb7` |
| **M55** | Differential Behavior Testing | Phase 14 | COMPLETE | M54, Phase 13 Gate | Coordinator | `research/findings/differential-testing.md` | PASS | None | `090ed46` |
| **M56** | Targeted Source Probing | Phase 14 | COMPLETE | M55 | Coordinator | `research/findings/targeted-source-probing.md` | PASS | None | `a2ca099` |
| **M57** | Reconstruction Corrections | Phase 14 | COMPLETE | M56 | Coordinator | `reconstruction/` | PASS | None | `947ffb8` |
| **M58** | Fidelity & Coverage Audit | Phase 14 | COMPLETE | M57 | Coordinator | `docs/reconstruction-coverage.md` | PASS | None | `877754b` |
| **M59** | Engine Boundary Mapping | Phase 15 | COMPLETE | M48, Phase 14 Gate | Coordinator | `docs/engine-boundary-map.md` | PASS | None | `dbba8a7` |
| **M60** | Minimal In-Game Vertical Slice | Phase 15 | COMPLETE | M59 | Coordinator | `integration/` | PASS | None | `449a469` |
| **M61** | Persistence & Compatibility Probe | Phase 15 | COMPLETE | M60 | Coordinator | `research/findings/integration-persistence-probe.md` | PASS | None | `bedb7a7` |
| **M62** | Reconstruction Feasibility Report | Phase 15 | COMPLETE | M58, M61 | Coordinator | `docs/reconstruction-feasibility.md` | PASS | None | `56305b1` |
| **M63** | Build & Test Baseline | Phase 16 | COMPLETE | M62, Phase 15 Gate | Coordinator | `research/investigations/build-and-test-baseline.md` | PASS | None | Pending |
| **M64** | Contract-to-Code Traceability Audit | Phase 16 | NOT STARTED | M63 | Unassigned | `research/investigations/contract-traceability-audit.md` | Pending | None | — |
| **M65** | Adversarial Regression Baseline | Phase 16 | NOT STARTED | M63, M64 | Unassigned | `research/investigations/adversarial-regression-baseline.md` | Pending | None | — |
| **M66** | Defect Registry & Repair Prioritization | Phase 16 | NOT STARTED | M63, M64, M65 | Unassigned | `research/investigations/defect-registry.md` | Pending | None | — |
| **M67** | Complete State Persistence | Phase 17 | NOT STARTED | M66, Phase 16 Gate | Unassigned | `reconstruction/src/persistence.rs` | Pending | None | — |
| **M68** | Serialization Safety & Data Validation | Phase 17 | NOT STARTED | M67 | Unassigned | `reconstruction/src/persistence.rs` | Pending | None | — |
| **M69** | Domain Invariants & Authorization | Phase 17 | NOT STARTED | M66, Phase 16 Gate | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M70** | Transactional Consistency & Resource Safety | Phase 17 | NOT STARTED | M69 | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M71** | Time, Randomness & Long-Running Simulation | Phase 17 | NOT STARTED | M67, M69, M70 | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M72** | Faction, World Model & Territory Fidelity | Phase 18 | NOT STARTED | Phase 17 Gate | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M73** | Economy, Population & Infrastructure Fidelity | Phase 18 | NOT STARTED | Phase 17 Gate | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M74** | Military, Strategic AI & Siege Fidelity | Phase 18 | NOT STARTED | Phase 17 Gate, M72 | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M75** | Government, Diplomacy, Law & Progression Fidelity | Phase 18 | NOT STARTED | Phase 17 Gate, M72 | Unassigned | `reconstruction/src/` | Pending | None | — |
| **M76** | Independent Differential Validation | Phase 18 | NOT STARTED | M72, M73, M74, M75 | Unassigned | `research/findings/independent-differential-validation.md` | Pending | None | — |
| **M77** | SKSE ABI & Plugin Loader Validation | Phase 19 | NOT STARTED | M63, Phase 17 Gate | Unassigned | `integration/` | Pending | None | — |
| **M78** | Papyrus Native Bridge & Test Plugin | Phase 19 | NOT STARTED | M77 | Unassigned | `integration/` | Pending | None | — |
| **M79** | SKSE Save/Load & State Synchronization | Phase 19 | NOT STARTED | M67, M68, M78 | Unassigned | `integration/` | Pending | None | — |
| **M80** | Real Skyrim Vertical Slice | Phase 19 | NOT STARTED | M78, M79 | Unassigned | `integration/` | Pending | None | — |
| **M81** | Fidelity, Coverage & Claims Reconciliation | Phase 20 | NOT STARTED | Phase 18 Gate, M76, M80 | Unassigned | `docs/reconstruction-coverage.md` | Pending | None | — |
| **M82** | Final Reconstruction Quality Gate | Phase 20 | NOT STARTED | M81 | Unassigned | `docs/reconstruction-repair-report.md` | Pending | None | — |

---

## 3. Phase Details & Validation Gates

### Phase 1: External Reconnaissance
- **M01: Project Identity and History** — `research/findings/project-overview.md`
- **M02: Documentation Recovery** — `research/findings/documentation-inventory.md`
- **M03: Comprehensive Feature Inventory** — `research/findings/feature-inventory.md`
- **M04: Issues and Compatibility Inventory** — `research/findings/known-issues.md`
- **Phase 1 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 2: Technical Reconnaissance
- **M05: Distribution and Dependency Analysis** — `research/findings/file-inventory.md`
- **M06: Creation Engine Integration** — `research/findings/skyrim-integration.md`
- **M07: Static Structure Recovery** — `research/findings/static-structure.md`
- **M08: Initialization and Runtime Lifecycle** — `research/findings/runtime-lifecycle.md`
- **Phase 2 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 3: Core Gameplay Specification
- **M09: Faction Creation** — `research/findings/faction-creation.md`
- **M10: Territory and Ownership** — `research/findings/territory.md`
- **M11: Progression and Victory** — `research/findings/progression.md`
- **M12: Player Interaction and Configuration** — `research/findings/interactions.md`
- **Phase 3 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 4: Military and Warfare
- **M13: Army and Recruitment Systems** — `research/findings/military-organization.md`
- **M14: Orders, Movement, and Deployment** — `research/findings/military-orders.md`
- **M15: Combat Simulation** — `research/findings/battles.md`
- **M16: Siege Warfare** — `research/findings/sieges.md`
- **Phase 4 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 5: Economy, Population, and Development
- **M17: Resource and Treasury Systems** — `research/findings/economy.md`
- **M18: Population and Employment** — `research/findings/population.md`
- **M19: Buildings and Infrastructure** — `research/findings/infrastructure.md`
- **M20: Technology and Research** — `research/findings/technology.md`
- **Phase 5 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 6: Politics, Diplomacy, and Administration
- **M21: Government and Officials** — `research/findings/government.md`
- **M22: Jarls and Hold Administration** — `research/findings/jarls.md`
- **M23: Diplomacy** — `research/findings/diplomacy.md`
- **M24: Law, Crime, and Stability** — `research/findings/law-and-stability.md`
- **Phase 6 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 7: Integration, Persistence, and Failure Analysis
- **M25: Cross-System Dependency Recovery** — `research/findings/system-dependencies.md`
- **M26: Persistence and State Consistency** — `research/findings/persistence.md`
- **M27: Vanilla Skyrim Compatibility** — `research/findings/vanilla-compatibility.md`
- **M28: Failure Modes and Reliability** — `research/findings/failure-modes.md`
- **Phase 7 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 8: Specification Reconstruction
- **M29: Behavioral Specification** — `docs/behavioral-specification.md`
- **M30: Recovered System Model** — `docs/system-model.md`
- **M31: Completeness and Uncertainty Audit** — `docs/research-gaps.md`
- **M32: Final Recovered Specification** — `docs/recovered-specification.md`
- **Phase 8 Gate Criteria**: **PASSED (Historical 2026-10-27)**.

### Phase 9: Evidence Integrity and Provenance
- **M33: Source Availability and Provenance Audit** — `research/findings/source-provenance-audit.md`
- **M34: Evidence Traceability Audit** — `research/findings/evidence-audit.md`
- **M35: Research Execution and Harness Compliance Audit** — `research/findings/execution-audit.md`
- **M36: Confidence Reclassification** — `research/findings/confidence-audit.md`
- **Phase 9 Gate Criteria**: **PASSED (2026-10-28)**.

### Phase 10: Reproducible Technical Verification
- **M37: Binary and Structural Verification** — `research/findings/verified-structure.md`
- **M38: Papyrus Claim Verification** — `research/findings/verified-papyrus-behavior.md`
- **M39: Numerical Mechanics and Formula Verification** — `research/findings/verified-mechanics.md`
- **M40: State, Persistence, and Engine Integration Verification** — `research/findings/verified-state-and-integration.md`
- **Phase 10 Gate Criteria**: **PASSED (2026-10-28)**.

### Phase 11: Empirical Validation and Final Recovery Audit
- **M41: Controlled Playtesting Program** — `research/playtesting/test-plan.md`
- **M42: Static and Runtime Evidence Reconciliation** — `research/findings/behavior-reconciliation.md`
- **M43: Adversarial Specification Audit** — `research/findings/adversarial-audit.md`
- **M44: Final Specification Integrity Gate** — `docs/specification-integrity-report.md`
- **Phase 11 Gate Criteria**: **PASSED (2026-10-28)**.

---

### Phase 12: Reconstruction Foundations
- **Objective**: Establish legal permissions, behavioral contract inventories, testing frameworks, and integration prerequisites for independent reconstruction.
- **M45: Research Policy Transition** — Update `AGENTS.md` and `guardrails.md` to permit independent behavioral reconstruction, mandatory source querying, and role separation while keeping IP protections intact.
  - *Deliverable*: `guardrails.md`, `AGENTS.md`
- **M46: Behavioral Contract Inventory** — Review recovered specification; reconcile 51 vs 55 feature count discrepancy; map inputs/outputs, difficulty, and integration dependencies into a formal reconstruction coverage matrix.
  - *Deliverable*: `docs/behavioral-contracts.md`
- **M47: Reconstruction Validation Strategy** — Define evaluation criteria for reconstructed systems (contract tests, state transition tests, numerical equivalence, differential testing, non-circular validation).
  - *Deliverable*: `docs/reconstruction-testing.md`
- **M48: Skyrim Integration Environment Survey** — Survey Creation Kit, ESP records, Papyrus, SKSE, native DLL loading, Rust interoperability, C++ bridges, and identify manual creation requirements.
  - *Deliverable*: `docs/integration-survey.md`
- **Phase 12 Gate Criteria**: **PASSED (2026-10-28)**. Reconstruction foundations verified: clean-room boundaries and 8-step protocol established (M45), full 55-feature behavioral contract matrix specified (M46), 7-tier non-circular validation strategy codified (M47), and Skyrim integration environment surveyed with verified host tooling (M48). All M45–M48 COMPLETE. Unblocks Phase 13 (M49).

### Phase 13: Independent System Reconstruction
- **Objective**: Build an independent, executable Rust simulation core modeling recovered strategic gameplay loops under deterministic test conditions.
- **M49: Minimal Rust Simulation Core** — Initialize `reconstruction/` Rust crate; establish domain state, explicit transitions, deterministic execution, and reproducible build commands.
  - *Deliverable*: `reconstruction/`
- **M50: Faction and Territory Reconstruction** — Reconstruct faction creation, identity proxies, territorial hierarchy (Hold Capitals 0..8, Forts, Villages), capture prerequisites, and ownership state transitions.
  - *Deliverable*: `reconstruction/`
- **M51: Economy and Population Reconstruction** — Reconstruct treasury, resource generation (Food, Metal, Wood, Docks), demographic labor allocation waterfall, tiered taxation, and facility upgrades (1..4).
  - *Deliverable*: `reconstruction/`
- **M52: Military and Warfare Reconstruction** — Reconstruct 10-division army management, Task Barrel order dispatching, recruitment, battle throttling, reinforcement waves, and siege mechanics.
  - *Deliverable*: `reconstruction/`
- **M53: Government and Diplomacy Reconstruction** — Reconstruct 7-member cabinet, Spymaster coups, judicial trials, Jarl fealty, bilateral non-aggression truces, and The Moot election loop.
  - *Deliverable*: `reconstruction/`
- **M54: Persistence and Scenario Execution** — Create standalone model serialization/deserialization, scenario execution harnesses, and cross-system consistency checks.
  - *Deliverable*: `reconstruction/`
- **Phase 13 Gate Criteria**: **PASSED (2026-10-28)**. Independent Rust simulation core (`reconstruction/`) successfully reproduces all major strategic gameplay loops (Faction declaration, 9-hold territorial expansion with atomic transfer, 4-stage demographic labor allocation, tiered taxation, facility construction, 10 field divisions, combat simulation, 5-stage siege operations with reinforcement waves, cabinet governance, judicial court hearings, truce diplomacy, and The Moot coronation) under deterministic test conditions (28/28 tests passing) with zero Skyrim engine dependencies. All M49–M54 COMPLETE. Unblocks Phase 14 (M55).

### Phase 14: Differential Probing and Gap Closure
- **Objective**: Execute differential comparisons between reconstructed models and original mod behavior, using discrepancies to drive targeted source probing.
- **M55: Differential Behavior Testing** — Construct comparative scenarios evaluating preconditions, state transitions, numerical outputs, and failure conditions against original mod findings.
  - *Deliverable*: `research/findings/differential-testing.md`
- **M56: Targeted Source Probing** — Deeply inspect original Papyrus code and ESP records for discovered gaps and discrepancies; update behavioral contracts.
  - *Deliverable*: `research/findings/targeted-source-probing.md`
- **M57: Reconstruction Corrections** — Correct independent Rust simulation using verified contracts; add regression tests for discovered discrepancies.
  - *Deliverable*: `reconstruction/`
- **M58: Fidelity and Coverage Audit** — Subsystem-by-subsystem evaluation measuring feature coverage, functional fidelity, behavioral fidelity, and integration dependencies with explicit denominators.
  - *Deliverable*: `docs/reconstruction-coverage.md`
- **Phase 14 Gate Criteria**: **PASSED (2026-10-28)**. Master coverage audit (`docs/reconstruction-coverage.md`) verified across all 55 features: 49 features reconstructed in standalone Rust (89.1%), 4 engine-tied integration features (7.3%), 2 preserved incomplete stubs (3.6%). Functional fidelity is 73.5% EXACT (36/49) and 26.5% COMPATIBLE (13/49) with 0% contradicted. All M55–M58 COMPLETE. Unblocks Phase 15 (M59).

### Phase 15: Skyrim Integration Probing
- **Objective**: Map integration boundaries, prototype a minimal in-game vertical slice, and assess full reconstruction feasibility.
- **M59: Engine Boundary Mapping** — Detail the exact boundary between simulation-owned state and Skyrim engine-owned state across Papyrus, SKSE, native forms, and manual CK assets.
  - *Deliverable*: `docs/engine-boundary-map.md`
- **M60: Minimal In-Game Vertical Slice** — Create one independent Skyrim integration experiment (e.g. claiming a single test location and reflecting ownership in the standalone model) with isolated test records.
  - *Deliverable*: `integration/`
- **M61: Persistence and Compatibility Probe** — Probe Skyrim save/load behavior, native state restoration, Papyrus state sync, and incomplete operation recovery using the vertical slice.
  - *Deliverable*: `research/findings/integration-persistence-probe.md`
- **M62: Reconstruction Feasibility Report** — Master evaluation synthesizing M45–M61 answering the 10 core feasibility questions and assessing technical viability of complete independent reconstruction.
  - *Deliverable*: `docs/reconstruction-feasibility.md`
- **Phase 15 Gate Criteria**: **PASSED (2026-10-28)**. Master feasibility report (`docs/reconstruction-feasibility.md`) delivers answers to the 10 core feasibility questions. Minimal path from independent simulation to Skyrim integration fully demonstrated via native SKSE C-ABI adapter (`integration/`, 3/3 tests passing), engine boundary mapping (`docs/engine-boundary-map.md`), and persistence compatibility probe (`research/findings/integration-persistence-probe.md`). All M59–M62 COMPLETE.

### Phase 16: Baseline Verification and Defect Reproduction
- **Objective**: Establish what actually works and create reproducible evidence for suspected failures.
- **M63: Build and Test Baseline** — Inspect both Rust crates (`reconstruction/`, `integration/`), verify toolchains, execute compilation, unit tests, clippy, formatting, and Python tests.
  - *Deliverable*: `research/investigations/build-and-test-baseline.md`
- **M64: Contract-to-Code Traceability Audit** — Audit reconstruction against behavioral contracts, specifications, original Papyrus sources, and ESP records; identify unsupported claims and incomplete stubs.
  - *Deliverable*: `research/investigations/contract-traceability-audit.md`
- **M65: Adversarial Regression Baseline** — Create tests designed to expose known weaknesses (save/load with active siege, facility upgrades in enemy holdings, invalid resource values, partial failures).
  - *Deliverable*: `research/investigations/adversarial-regression-baseline.md`
- **M66: Defect Registry and Repair Prioritization** — Consolidate findings into a prioritized, actionable defect registry (P0/P1/P2/P3).
  - *Deliverable*: `research/investigations/defect-registry.md`
- **Phase 16 Gate Criteria**: Pass when the existing reconstruction has a verified execution baseline and a concrete, reproducible repair backlog.

### Phase 17: Core Correctness and Persistence Repair
- **Objective**: Ensure that the standalone Rust model behaves consistently before attempting deeper engine integration.
- **M67: Complete State Persistence** — Audit and repair `persistence.rs` to serialize and restore all gameplay-critical state (RNG state, divisions, sieges, governors, diplomacy, cooldowns).
  - *Deliverable*: `reconstruction/src/persistence.rs`
- **M68: Serialization Safety and Data Validation** — Audit JSON implementation for string escaping, malformed input handling, numeric precision, and version compatibility.
  - *Deliverable*: `reconstruction/src/persistence.rs`
- **M69: Domain Invariants and Authorization** — Enforce minimum domain invariants across public state-mutating operations (ownership checks, prerequisites, negative checks).
  - *Deliverable*: `reconstruction/src/`
- **M70: Transactional Consistency and Resource Safety** — Audit multi-field operations for overflow, underflow, resource creation, and clean rollback on failure.
  - *Deliverable*: `reconstruction/src/`
- **M71: Time, Randomness, and Long-Running Simulation** — Test long-running multi-day simulations, tick boundaries, RNG continuation, and unbounded event growth.
  - *Deliverable*: `reconstruction/src/`
- **Phase 17 Gate Criteria**: Pass when the standalone Rust state model is internally consistent, persists completely, and rejects invalid operations.

### Phase 18: Gameplay Fidelity Repair
- **Objective**: Replace approximations and unsupported mechanics with behavior justified by original source analysis.
- **M72: Faction, World Model, and Territory Fidelity** — Verify world data, starting state, territorial transitions, and remove unsupported doctrine modifiers.
  - *Deliverable*: `reconstruction/src/`
- **M73: Economy, Population, and Infrastructure Fidelity** — Reconstruct exact verified formulas for taxation, production, docks, labor, population growth, and conscription.
  - *Deliverable*: `reconstruction/src/`
- **M74: Military, Strategic AI, and Siege Fidelity** — Enforce military prerequisites, realistic siege battle progression, casualty formulas, and AI targeting.
  - *Deliverable*: `reconstruction/src/`
- **M75: Government, Diplomacy, Law, and Progression Fidelity** — Source-back political transitions, truce negotiation math, court penalties, and Moot prerequisites.
  - *Deliverable*: `reconstruction/src/`
- **M76: Independent Differential Validation** — Execute differential testing with source-supported test vectors and explicit provenance; document known deviations.
  - *Deliverable*: `research/findings/independent-differential-validation.md`
- **Phase 18 Gate Criteria**: Pass when major strategic systems are supported by independent test evidence and unverified behavior remains visibly labeled.

### Phase 19: Native Skyrim Integration Repair
- **Objective**: Replace mock-tested integration claims with a correctly implemented and empirically tested engine bridge.
- **M77: SKSE ABI and Plugin Loader Validation** — Verify native plugin interface layouts, calling conventions, and export compatibility against actual SKSE headers.
  - *Deliverable*: `integration/`
- **M78: Papyrus Native Bridge and Test Plugin** — Implement missing Papyrus-to-native registration, function binding, and author the actual test plugin and scripts.
  - *Deliverable*: `integration/`
- **M79: SKSE Save/Load and State Synchronization** — Implement actual `SKSESerializationInterface` callbacks (Save, Load, Revert) and state synchronization.
  - *Deliverable*: `integration/`
- **M80: Real Skyrim Vertical Slice** — Perform genuine runtime integration test where an isolated Skyrim-side action updates Rust state and produces observable feedback.
  - *Deliverable*: `integration/`
- **Phase 19 Gate Criteria**: Pass only when the minimal Skyrim integration path has been demonstrated in an actual game session (record BLOCKED if manual testing unavailable).

### Phase 20: Final Validation and Documentation Repair
- **Objective**: Reconcile all fidelity claims and deliver the final quality-gated repair report.
- **M81: Fidelity, Coverage, and Claims Reconciliation** — Audit all reconstruction documentation; correct inflated claims; separate implemented, unit-tested, and runtime-tested coverage.
  - *Deliverable*: `docs/reconstruction-coverage.md`, `docs/milestones.md`
- **M82: Final Reconstruction Quality Gate** — Integrated review across M63–M81, executing all automated tests, attempting falsification, and authoring capstone repair report.
  - *Deliverable*: `docs/reconstruction-repair-report.md`
- **Phase 20 Gate Criteria**: Pass when the repair program produces a trustworthy reconstruction with clearly stated limitations.

---

## 4. Execution Dependency Graph & Parallel Groups

### Dependency Graph

```text
PHASE 12: RECONSTRUCTION FOUNDATIONS

M45 ──> M46 ──┐
     ├──> M47 ──┼──> Phase 12 Gate
     └──> M48 ──┘

PHASE 13: INDEPENDENT RECONSTRUCTION

Phase 12 Gate ──> M49 ──> M50 ──┬──> M51 ──┐
                                ├──> M52 ──┼──> M53 ──> M54 ──> Phase 13 Gate
                                └──────────┘

PHASE 14: DIFFERENTIAL PROBING & GAP CLOSURE

Phase 13 Gate ──> M55 ──> M56 ──> M57 ──> M58 ──> Phase 14 Gate

PHASE 15: SKYRIM INTEGRATION PROBING

M48 + Phase 14 Gate ──> M59 ──> M60 ──> M61 ──> M62 (Feasibility Report)

PHASE 16: BASELINE VERIFICATION & DEFECT REPRODUCTION

M62 + Phase 15 Gate ──> M63 ──┬──> M64 ──┬──> M66 ──> Phase 16 Gate
                              └──> M65 ──┘

PHASE 17: CORE CORRECTNESS & PERSISTENCE REPAIR

Phase 16 Gate ──┬──> M67 ──> M68 ──┐
                └──> M69 ──> M70 ──┼──> M71 ──> Phase 17 Gate

PHASE 18: GAMEPLAY FIDELITY REPAIR

Phase 17 Gate ──┬──> M72 ──┐
                ├──> M73 ──┼──> M76 ──> Phase 18 Gate
                ├──> M74 ──┤
                └──> M75 ──┘

PHASE 19: NATIVE SKYRIM INTEGRATION REPAIR

M63 + Phase 17 Gate ──> M77 ──> M78 ──> M79 ──> M80 ──> Phase 19 Gate

PHASE 20: FINAL VALIDATION & REPAIR GATE

Phase 18 Gate + Phase 19 Gate ──> M81 ──> M82 (Capstone Report) ──> Phase 20 Gate
```

### Workstream Tracks & Parallel Execution Groups

| Track | Primary Responsibilities | Target Outputs |
| :--- | :--- | :--- |
| **Track A: Reconstruction** | Independent Rust implementation, model unit tests, scenario execution, fidelity measurements. Works strictly from reviewed contracts. | `reconstruction/` |
| **Track B: Probing** | Original source inspection, primary evidence recovery, gap investigation, contract clarification. Never passes raw code as instructions. | `research/findings/` |
| **Track C: Integration** | Skyrim runtime interface investigation, SKSE/C++ bridge research, manual Creation Kit experiment procedures. | `docs/`, `integration/` |
| **Track D: Validation & Repair** | Defect reproduction, adversarial regression tests, invariant auditing, differential verification, claims reconciliation. | `research/investigations/`, `tests/` |

| Group | Milestones | Primary Focus | Coordination Notes |
| :--- | :--- | :--- | :--- |
| **Groups A–K** | M01–M44 | Historical Specification Recovery & Audits | Complete (`b935278`) |
| **Group L** | M45, M46, M47, M48 | Reconstruction Foundations | Complete (`c5f7748`) |
| **Group M** | M49, M50, M51, M52, M53, M54 | Independent System Reconstruction | Complete (`e127cb7`) |
| **Group N** | M55, M56, M57, M58 | Differential Probing & Gap Closure | Complete (`877754b`) |
| **Group O** | M59, M60, M61, M62 | Skyrim Integration Probing & Feasibility | Complete (`56305b1`) |
| **Group P** | M63, M64, M65, M66 | Baseline Verification & Defect Reproduction | Independent audits (M63-M65) feed registry (M66) |
| **Group Q** | M67, M68, M69, M70, M71 | Core Correctness & Persistence Repair | Persistence (M67-M68) & Invariants (M69-M71) |
| **Group R** | M72, M73, M74, M75, M76 | Gameplay Fidelity Repair | Subsystem fidelity with exclusive file ownership |
| **Group S** | M77, M78, M79, M80 | Native Skyrim Integration Repair | Sequential integration gates |
| **Group T** | M81, M82 | Final Validation & Documentation Repair | Capstone audit and quality gate |

---

## 5. Milestone Validation Gates (M01–M82)

| ID | Required validation before completion |
| :--- | :--- |
| M01 | Verify identity; document major releases, supported environments, evidence-backed development status, version distinctions, and source references. |
| M02 | Index significant available documentation with URLs/versions; distinguish primary/secondary sources; record conflicting descriptions and missing documentation. |
| M03 | Represent all major documented systems with stable feature identifiers; distinguish documented/technically identified/observed/disabled/incomplete/unverified status (overlap allowed); record known dependencies and unresolved features. |
| M04 | Categorize issues with provenance, affected versions where known, and available reproduction details; consolidate duplicates without losing sources; exclude unsupported root causes. |
| M05 | Inventory available package components, dependencies, version requirements, unknown formats, and acquisition provenance. |
| M06 | Identify relevant engine systems, verified integration points, external dependencies, and vanilla interaction surfaces; label inferences. |
| M07 | Map major identifiable components and evidence-traceable relationships; document external references and unknown components; never present inferred behavior as verified. |
| M08 | Document known lifecycle stages/entry points; investigate initialization; identify persistence boundaries and unresolved lifecycle behavior. |
| M09 | Reconstruct faction-creation workflow, inputs/outputs, and initial state; document dependencies and investigate important edge cases. |
| M10 | Document territory model, ownership transitions, capture prerequisites, and system effects; explicitly mark uncertainties. |
| M11 | Reconstruct major progression paths, requirements, branches/dependencies; investigate endgame conditions and document known progression failures. |
| M12 | Catalogue major interactions and access conditions, inputs/outcomes, configuration effects, and missing/ambiguous behavior. |
| M13 | Document army lifecycle, recruitment behavior, capacity constraints, resource dependencies, and commander relationships. |
| M14 | Inventory orders; document preconditions and expected effects; investigate transitions and identify known failures. |
| M15 | Reconstruct battle lifecycle/state transitions; investigate reinforcements; identify resolution conditions and cleanup; explicitly identify unknown formulas. |
| M16 | Reconstruct siege lifecycle/state transitions; characterize equipment effects and territorial consequences; investigate interruptions and failures. |
| M17 | Identify resources, map flows and production/consumption relationships; recover supported calculations and explicitly mark unknown formulas. |
| M18 | Characterize population/employment and relevant transitions; map economic dependencies and unknown calculations. |
| M19 | Inventory infrastructure; document construction lifecycle and resource/population dependencies; recover known effects and flag broken/incomplete features. |
| M20 | Inventory technologies and dependencies; characterize research lifecycle, unlock effects, and incomplete features. |
| M21 | Identify government roles; document appointments, delegated behavior, political dependencies, and unknown responsibilities. |
| M22 | Document Jarl lifecycle and hold relationships; investigate loyalty, verify rebellion availability, and identify major edge cases. |
| M23 | Inventory diplomatic actions, preconditions, outcomes, and state transitions; distinguish unsupported features from implemented ones. |
| M24 | Document legal interactions; investigate crime/stability relationships; characterize punishments, disabled systems, and failures. |
| M25 | Map major subsystem relationships and shared state where possible; document ordering constraints, investigate circular dependencies, and preserve contradictions. |
| M26 | Identify persistent-state categories; investigate relevant save/reload scenarios, known consistency failures, recovery, and remaining uncertainties. |
| M27 | Map compatibility surfaces, known conflicts, affected vanilla systems, relevant version/load-order dependencies; distinguish suspected from verified conflicts. |
| M28 | Document significant failure categories; separate symptoms/causes; record available reproductions, severity, known recovery, and unresolved causes. |
| M29 | Address the entire feature inventory and significant mechanics; exclude unsupported assumptions; preserve evidence, explicit unknowns, and contradictions. |
| M30 | Represent major domains consistently with verified findings; trace transitions/dependencies to evidence; label inferred internals; exclude speculative replacement architecture. |
| M31 | Account for every inventoried feature; audit uncertainties, contradictions, confidence classifications, missing evidence, and limitations. |
| M32 | Review M01–M31; verify feature/subsystem coverage, cross-system consistency, traceable technical claims, visible unknowns/contradictions, final review, Git state, and committed documentation. |
| M33 | Every claimed primary source identified; local availability checked; version and layout verified where possible; contradictions documented; unsupported inspection claims flagged; unavailable sources explicitly recorded; no original files modified. |
| M34 | All 32 previous milestones assessed; major technical claims traceable or flagged; unsupported CONFIRMED labels identified; missing evidence explicitly recorded; no invented citations or verification results. |
| M35 | Completion claims checked against available evidence; missing validation procedures identified; unsupported review claims flagged; important harness deviations documented; no previous work silently rewritten or deleted. |
| M36 | Unsupported confidence claims corrected; classification criteria applied consistently; significant disputed claims identified; changes traceable to previous classification; revalidation priorities established. |
| M37 | Structural metrics reproduced from actual files; tool versions recorded; input version established; commands or procedures documented; differences from previous findings recorded; relevant output retained with provenance; original mod untouched. |
| M38 | Important script references verified; referenced symbols checked against primary material; behavioral claims assessed; incorrect interpretations corrected; unverified code paths identified; reproducible inspection procedures documented. |
| M39 | Major numerical claims evaluated; exact formulas verified where possible; representative calculations reproduced; missing variables identified; incorrect formulas corrected; unrecoverable formulas marked UNKNOWN; validation procedures reproducible. |
| M40 | Major state ownership claims evaluated; relevant transitions traced; persistence claims assessed; vanilla integration claims verified where possible; unsupported architectural assumptions corrected; unresolved engine behavior documented. |
| M41 | Critical gameplay paths covered; experiments reproducible; evidence requirements clear; high-risk interactions prioritized; no playtesting results fabricated. |
| M42 | Available human observations reconciled; significant contradictions recorded; unverified runtime behavior remains clearly labeled; no unperformed experiments treated as evidence; unresolved test dependencies preserved. |
| M43 | Important claims challenged; major unsupported assertions addressed; contradictions identified; review findings classified by severity; material errors corrected or explicitly unresolved; no false independent-review claims. |
| M44 | M33–M43 reviewed; significant unsupported claims corrected; final documents consistent; evidence traceability audited; unresolved gaps preserved; validation results accurately reported; milestone statuses reconciled; adversarial review completed or fallback documented; changes committed. |
| M45 | Reconstruction explicitly permitted; mandatory source querying documented; original-code copying remains prohibited; reference isolation preserved; existing disciplines remain coherent; changes reviewed and committed. |
| M46 | Feature inventory reconciled (51 vs 55); every feature assigned a reconstruction status; major dependencies identified; missing contracts explicitly recorded; no unsupported fidelity assumptions. |
| M47 | Fidelity criteria established; test inputs and expected outcomes traceable; failure classifications defined; no circular validation; human-dependent tests distinguished from automated tests. |
| M48 | Relevant integration options identified; tooling requirements verified; important limitations documented; manual work identified; no integration approach selected without supporting evidence. |
| M49 | Rust crate builds cleanly; unit tests pass; state transitions are deterministic; no Skyrim dependency required; repository artifact discipline preserved. |
| M50 | Major ownership transitions tested; invalid transitions handled; state invariants enforced; behavior compared against available evidence; missing mechanics identified. |
| M51 | Recovered formulas independently checked; resource conservation and update invariants tested; boundary conditions covered; unknown mechanics identified; cross-system interactions exercised. |
| M52 | Army lifecycle demonstrated; battle state machine tested; reinforcement behavior covered; siege transitions represented; unresolved engine-dependent behavior isolated. |
| M53 | Major political transitions tested; cross-system dependencies covered; victory prerequisites represented; known incomplete features identified; unsupported mechanics excluded. |
| M54 | Equivalent states survive serialization; interrupted scenarios are tested; invalid states are detected; integration requirements documented; no claim of Skyrim persistence compatibility. |
| M55 | Comparison scenarios documented; available comparisons executed; differences recorded; no missing original observations fabricated; fidelity assessed per scenario. |
| M56 | High-priority gaps investigated; original source references recorded; previously unsupported assumptions corrected; remaining unknowns explicitly documented; findings incorporated into contracts. |
| M57 | Confirmed discrepancies addressed; regression tests added; existing behavior preserved or intentionally corrected; no unsupported implementation changes; all applicable tests pass. |
| M58 | Coverage matrix updated; fidelity evidence documented; important deviations identified; unknowns preserved; standalone reconstruction feasibility assessed. |
| M59 | Integration boundaries documented; engine ownership distinguished from simulation ownership; manual dependencies identified; required APIs verified; unsupported mechanisms flagged. |
| M60 | Integration builds or loads using selected mechanism; Skyrim-side event is received; reconstructed model updates; observable feedback produced; failure conditions documented; no original mod files modified; game execution verified where available. |
| M61 | Save/load procedure documented; available runtime tests executed; state consistency assessed; important compatibility risks identified; unverified engine behavior clearly labeled. |
| M62 | M45–M61 reviewed; technical results traceable; fidelity claims calibrated; integration evidence documented; manual work identified; remaining gaps prioritized; conclusions supported by experiments; final review completed; changes committed. |
| M63 | Commands recorded; actual results recorded; compiler warnings investigated; failed and skipped tests identified; environment limitations documented; no source changes necessary merely to make a report look clean. |
| M64 | All reconstructed features mapped; unsupported claims identified; missing implementations distinguished from incomplete implementations; critical gaps prioritized; no fidelity percentage accepted without evidence. |
| M65 | Critical suspected defects reproducible or explicitly inconclusive; regression tests committed; expected behavior independently justified; known failures retained as actionable work; no defective behavior silently normalized. |
| M66 | All known critical problems registered; defects assigned to repair milestones; unconfirmed suspicions clearly labeled; repair dependencies established; no unnecessary architectural rewrite proposed. |
| M67 | Full state equivalence demonstrated; active sieges survive save/load; RNG continuation is reproducible; no omitted gameplay-critical state; missing or incompatible save versions handled explicitly. |
| M68 | Valid strings round-trip; malformed input rejected; large integers retain precision; numeric conversions checked; unsupported save versions handled; fuzz or property-based testing used where useful; no panics on ordinary malformed input. |
| M69 | Enemy holdings cannot be managed without authorization; invalid operations do not mutate state; required prerequisites enforced; invariants exercised by negative tests; no unnecessary gameplay restrictions invented. |
| M70 | Failed operations preserve prior state; resource balances remain valid; no unauthorized resource creation; overflow and underflow handled; multi-step operations tested. |
| M71 | Deterministic replay established; time progression consistent; no duplicate daily updates; RNG state survives persistence; long-running simulations avoid overflow and uncontrolled memory growth; numerical behavior matches verified contracts where required. |
| M72 | World data verified against primary sources; starting state justified; territorial transitions corrected; unsupported modifiers removed or isolated; previously missing territory types assessed; relevant tests updated. |
| M73 | Major numerical formulas independently verified; relevant boundary conditions tested; cross-system resource flow checked; missing production mechanics documented; unsupported clamps or coefficients corrected; deterministic multi-day scenarios pass. |
| M74 | Military prerequisites enforced; invalid orders rejected; siege phase transitions justified; casualties and reinforcement rules tested; strategic AI formulas compared against original source; no instant unsupported victory paths; engine-dependent behavior explicitly isolated. |
| M75 | Major political paths tested; numerical diplomacy claims source-backed; Moot prerequisites verified; negative financial operations rejected; missing government mechanics identified; no unsupported political behavior classified EXACT. |
| M76 | Representative contracts compared; test vectors have explicit provenance; known deviations recorded; critical mismatches corrected; results reproducible; no unjustified EXACT classifications. |
| M77 | ABI checked against actual SKSE headers; Windows DLL produced using a supported toolchain; exports verified; loader compatibility assessed; unsupported runtime versions rejected appropriately; no in-game claims made from mock tests. |
| M78 | Papyrus registration implemented; native function signatures verified; required test records created or explicitly awaiting manual authoring; scripts compile where tooling exists; callback path tested to the available level; actual game execution remains a separate gate. |
| M79 | Callbacks implemented; state serialization integrated; save/load/revert behavior tested where possible; unsupported engine paths documented; no claims of guaranteed clean uninstallation; runtime-dependent results remain pending until tested. |
| M80 | Actual Skyrim runtime execution demonstrated; event received by the native implementation; state mutation confirmed; visible result confirmed; failure cases exercised; save/load behavior recorded separately. |
| M81 | Numerical coverage reproducible; unsupported claims corrected; documentation matches actual code; validation levels distinguished; remaining gaps prioritized; no historical records silently erased. |
| M82 | Critical defects addressed or explicitly blocked; all applicable automated tests pass; major fidelity claims traceable; persistence verified; native status accurately represented; runtime evidence supplied where required; documentation reconciled; final review performed; intended changes committed. |

