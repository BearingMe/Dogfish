# Research Roadmap & Milestone Registry

## 1. Overview & Roadmap Status

This document tracks all 62 milestones of the *Conquest of Skyrim* specification recovery, probing, and experimental reconstruction program across 15 phases. It serves as the authoritative operational roadmap.

### Program Progression
- **Phases 1–8 (M01–M32)**: Initial specification recovery, architectural reverse engineering, and master specification synthesis (COMPLETE, commit `00323d9`).
- **Phases 9–11 (M33–M44)**: Evidence integrity audit, reproducible binary and script verification, numerical formula test suites, and empirical playtesting protocol (COMPLETE, commit `b935278`).
- **Phases 12–15 (M45–M62)**: Experimental reconstruction foundations, independent Rust simulation model, differential probing, and Skyrim runtime integration experiments.

### Roadmap Summary

| Total Milestones | NOT STARTED | IN PROGRESS | BLOCKED | COMPLETE |
| :---: | :---: | :---: | :---: | :---: |
| 62 | 14 | 0 | 0 | 48 |

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
| **M49** | Minimal Rust Simulation Core | Phase 13 | NOT STARTED | M46, M47, Phase 12 Gate | Unassigned | `reconstruction/` | Pending | None | — |
| **M50** | Faction & Territory Reconstruction | Phase 13 | NOT STARTED | M49 | Unassigned | `reconstruction/` | Pending | None | — |
| **M51** | Economy & Population Reconstruction | Phase 13 | NOT STARTED | M49, M50 | Unassigned | `reconstruction/` | Pending | None | — |
| **M52** | Military & Warfare Reconstruction | Phase 13 | NOT STARTED | M49, M50 | Unassigned | `reconstruction/` | Pending | None | — |
| **M53** | Government & Diplomacy Reconstruction | Phase 13 | NOT STARTED | M50, M51, M52 | Unassigned | `reconstruction/` | Pending | None | — |
| **M54** | Persistence & Scenario Execution | Phase 13 | NOT STARTED | M49–M53 | Unassigned | `reconstruction/` | Pending | None | — |
| **M55** | Differential Behavior Testing | Phase 14 | NOT STARTED | M54, Phase 13 Gate | Unassigned | `research/findings/differential-testing.md` | Pending | None | — |
| **M56** | Targeted Source Probing | Phase 14 | NOT STARTED | M55 | Unassigned | `research/findings/targeted-source-probing.md` | Pending | None | — |
| **M57** | Reconstruction Corrections | Phase 14 | NOT STARTED | M56 | Unassigned | `reconstruction/` | Pending | None | — |
| **M58** | Fidelity & Coverage Audit | Phase 14 | NOT STARTED | M57 | Unassigned | `docs/reconstruction-coverage.md` | Pending | None | — |
| **M59** | Engine Boundary Mapping | Phase 15 | NOT STARTED | M48, Phase 14 Gate | Unassigned | `docs/engine-boundary-map.md` | Pending | None | — |
| **M60** | Minimal In-Game Vertical Slice | Phase 15 | NOT STARTED | M59 | Unassigned | `integration/` | Pending | None | — |
| **M61** | Persistence & Compatibility Probe | Phase 15 | NOT STARTED | M60 | Unassigned | `research/findings/integration-persistence-probe.md` | Pending | None | — |
| **M62** | Reconstruction Feasibility Report | Phase 15 | NOT STARTED | M58, M61 | Unassigned | `docs/reconstruction-feasibility.md` | Pending | None | — |

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
- **Phase 12 Gate Criteria**: Pass when reconstruction can begin with clear behavioral contracts, legal boundaries, validation criteria, and a preliminary understanding of integration constraints.

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
- **Phase 13 Gate Criteria**: Pass when the independently written simulation can reproduce the major strategic gameplay loops under deterministic test conditions without Skyrim engine dependencies.

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
- **Phase 14 Gate Criteria**: Pass when the project can explain which major systems have been faithfully reconstructed, which remain approximate, and what evidence is missing.

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
- **Phase 15 Gate Criteria**: Pass when the project has demonstrated, or clearly identified blockers to demonstrating, a minimal path from independent simulation to Skyrim integration.

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
```

### Workstream Tracks & Parallel Execution Groups

| Track | Primary Responsibilities | Target Outputs |
| :--- | :--- | :--- |
| **Track A: Reconstruction** | Independent Rust implementation, model unit tests, scenario execution, fidelity measurements. Works strictly from reviewed contracts. | `reconstruction/` |
| **Track B: Probing** | Original source inspection, primary evidence recovery, gap investigation, contract clarification. Never passes raw code as instructions. | `research/findings/` |
| **Track C: Integration** | Skyrim runtime interface investigation, SKSE/C++ bridge research, manual Creation Kit experiment procedures. | `docs/`, `integration/` |

| Group | Milestones | Primary Focus | Coordination Notes |
| :--- | :--- | :--- | :--- |
| **Groups A–K** | M01–M44 | Historical Specification Recovery & Audits | Complete (`b935278`) |
| **Group L** | M45, M46, M47, M48 | Reconstruction Foundations | M45 unblocks M46, M47, M48 in parallel |
| **Group M** | M49, M50, M51, M52, M53, M54 | Independent System Reconstruction | Sequential domain modeling in Rust |
| **Group N** | M55, M56, M57, M58 | Differential Probing & Gap Closure | Comparative testing and contract refinement |
| **Group O** | M59, M60, M61, M62 | Skyrim Integration Probing & Feasibility | Vertical slice prototyping and final report |

---

## 5. Milestone Validation Gates (M01–M62)

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
