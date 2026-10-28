# Research Roadmap & Milestone Registry

## 1. Overview & Roadmap Status

This document tracks all 44 milestones of the *Conquest of Skyrim* specification recovery project across 11 phases. It serves as the authoritative operational roadmap for the research program.

The original 32-milestone program produced comprehensive documentation (M01–M32, closed historically in commit `00323d9`). A subsequent repository audit identified critical evidence-integrity concerns, specifically the need to systematically verify all claims against primary sources, audit evidence traceability, eliminate confidence inflation, and establish reproducible technical inspection.

**Program Extension Principle**:
- **Preserve M01–M32**: Existing historical completion records, deliverables, and commits are preserved.
- **Add M33–M44**: 12 new milestones across 3 phases (Phase 9: Evidence Integrity & Provenance, Phase 10: Reproducible Technical Verification, Phase 11: Empirical Validation & Final Recovery Audit).
- **Audit & Revalidation**: Findings from Phase 9 will audit previous claims without silently rewriting history, establishing clear revalidation priorities for Phases 10 and 11.

### Roadmap Summary

| Total Milestones | NOT STARTED | IN PROGRESS | BLOCKED | COMPLETE |
| :---: | :---: | :---: | :---: | :---: |
| 44 | 12 | 0 | 0 | 32 |

### Allowed Status Values
- **NOT STARTED**: Research has not yet begun.
- **IN PROGRESS**: Active investigation or playtesting underway.
- **BLOCKED**: Execution halted by external dependencies or environmental constraints.
- **COMPLETE**: Deliverable created, evidence verified, review passed, and changes committed.

---

## 2. Master Milestone Registry

Dependencies below are conservative planning inputs, not blanket prerequisites for starting independent research. M01–M32 historical records are preserved. For Phase 9 (M33–M36), independent audit investigations can proceed concurrently; integrate their results before closing the Phase 9 gate.

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
| **M33** | Source Availability & Provenance Audit | Phase 9 | NOT STARTED | None | Unassigned | `research/findings/source-provenance-audit.md` | Pending | None | — |
| **M34** | Evidence Traceability Audit | Phase 9 | NOT STARTED | M33 | Unassigned | `research/findings/evidence-audit.md` | Pending | None | — |
| **M35** | Research Execution & Harness Compliance Audit | Phase 9 | NOT STARTED | M33 | Unassigned | `research/findings/execution-audit.md` | Pending | None | — |
| **M36** | Confidence Reclassification | Phase 9 | NOT STARTED | M33, M34, M35 | Unassigned | `research/findings/confidence-audit.md` | Pending | None | — |
| **M37** | Binary & Structural Verification | Phase 10 | NOT STARTED | M33, Phase 9 Gate | Unassigned | `research/findings/verified-structure.md` | Pending | None | — |
| **M38** | Papyrus Claim Verification | Phase 10 | NOT STARTED | M37 | Unassigned | `research/findings/verified-papyrus-behavior.md` | Pending | None | — |
| **M39** | Numerical Mechanics & Formula Verification | Phase 10 | NOT STARTED | M38 | Unassigned | `research/findings/verified-mechanics.md` | Pending | None | — |
| **M40** | State, Persistence & Integration Verification | Phase 10 | NOT STARTED | M38 | Unassigned | `research/findings/verified-state-and-integration.md` | Pending | None | — |
| **M41** | Controlled Playtesting Program | Phase 11 | NOT STARTED | M36 | Unassigned | `research/playtesting/test-plan.md` | Pending | None | — |
| **M42** | Static & Runtime Evidence Reconciliation | Phase 11 | NOT STARTED | M39, M40, M41 | Unassigned | `research/findings/behavior-reconciliation.md` | Pending | None | — |
| **M43** | Adversarial Specification Audit | Phase 11 | NOT STARTED | M36, M42 | Unassigned | `research/findings/adversarial-audit.md` | Pending | None | — |
| **M44** | Final Specification Integrity Gate | Phase 11 | NOT STARTED | M43 | Unassigned | `docs/specification-integrity-report.md` | Pending | None | — |

---

## 3. Phase Details & Validation Gates

### Phase 1: External Reconnaissance
- **M01: Project Identity and History** — Author, contributors, release chronology, Skyrim versions, distribution channels, known forks.
  - *Deliverable*: `research/findings/project-overview.md`
- **M02: Documentation Recovery** — Nexus description, installation docs, changelogs, author statements, community guides.
  - *Deliverable*: `research/findings/documentation-inventory.md`
- **M03: Comprehensive Feature Inventory** — Structured catalogue of all advertised/documented features across all domains.
  - *Deliverable*: `research/findings/feature-inventory.md`
- **M04: Issues and Compatibility Inventory** — Documented bugs, crashes, quest breakages, compatibility issues, and author warnings.
  - *Deliverable*: `research/findings/known-issues.md`
- **Phase 1 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Primary investigation version established (v0.3.9); documentation sources indexed; feature inventory comprehensive; known issues categorized; research gaps identified.

### Phase 2: Technical Reconnaissance
- **M05: Distribution and Dependency Analysis** — Package layout, plugin records, scripts, assets, required/optional dependencies.
  - *Deliverable*: `research/findings/file-inventory.md`
- **M06: Creation Engine Integration** — Engine hooks (quests, factions, locations, NPCs, AI packages, global variables, Papyrus/SKSE).
  - *Deliverable*: `research/findings/skyrim-integration.md`
- **M07: Static Structure Recovery** — Plugin record relationships, script attachment, event registrations, data dependencies.
  - *Deliverable*: `research/findings/static-structure.md`
- **M08: Initialization and Runtime Lifecycle** — Quest startup, background updates, save/load behavior, shutdown, recovery.
  - *Deliverable*: `research/findings/runtime-lifecycle.md`
- **Phase 2 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Technical surface sufficiently mapped to support focused subsystem investigation.

### Phase 3: Core Gameplay Specification
- **M09: Faction Creation** — Workflow, prerequisites, initial resources, customization, failure behavior, persistence.
  - *Deliverable*: `research/findings/faction-creation.md`
- **M10: Territory and Ownership** — Hierarchy, holds, settlements, forts, capture requirements, ownership transitions.
  - *Deliverable*: `research/findings/territory.md`
- **M11: Progression and Victory** — Progression stages, unlock loops, political progression, The Moot, High King status, victory conditions.
  - *Deliverable*: `research/findings/progression.md`
- **M12: Player Interaction and Configuration** — Menus, dialogue, commands, configuration surfaces (MCM only if established), status reporting, error feedback.
  - *Deliverable*: `research/findings/interactions.md`
- **Phase 3 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Complete observable core loop described from faction initialization through conquest and endgame.

### Phase 4: Military and Warfare
- **M13: Army and Recruitment Systems** — Recruitment sources, troop types, costs, capacity limits, commanders, equipment.
  - *Deliverable*: `research/findings/military-organization.md`
- **M14: Orders, Movement, and Deployment** — Command hierarchy, movement logic, patrols, garrisons, camps, order transitions.
  - *Deliverable*: `research/findings/military-orders.md`
- **M15: Combat Simulation** — Battle initialization, participant selection, casualties, retreat, victory/defeat thresholds, post-battle cleanup.
  - *Deliverable*: `research/findings/battles.md`
- **M16: Siege Warfare** — Siege initiation, assault mechanics, defender behavior, garrison integration, settlement transfer.
  - *Deliverable*: `research/findings/sieges.md`
- **Phase 4 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Complete military lifecycle documented across recruitment, division hierarchy, spatial movement, combat simulation, and siege capitulation.

### Phase 5: Economy, Population, and Development
- **M17: Resource and Treasury Systems** — Resource types, production, storage, trade, treasury flows, taxes, shortages, update rates.
  - *Deliverable*: `research/findings/economy.md`
- **M18: Population and Employment** — Population models, growth, labor requirements, conscription, settlement differences.
  - *Deliverable*: `research/findings/population.md`
- **M19: Buildings and Infrastructure** — Construction options, prerequisites, costs, production facilities, settlement upgrades.
  - *Deliverable*: `research/findings/infrastructure.md`
- **M20: Technology and Research** — Categories, prerequisites, research progression, persistent upgrades, limitations.
  - *Deliverable*: `research/findings/technology.md`
- **Phase 5 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Full economic loop recovered: Population -> Labor -> Production -> Resources -> Development and Resources -> Military Capacity -> Expansion.

### Phase 6: Politics, Diplomacy, and Administration
- **M21: Government and Officials** — Administrative hierarchy, official appointments, authority, delegation, removal.
  - *Deliverable*: `research/findings/government.md`
- **M22: Jarls and Hold Administration** — Jarl appointments, hold ownership, loyalty, rebellions, interaction with vanilla Jarls.
  - *Deliverable*: `research/findings/jarls.md`
- **M23: Diplomacy** — Diplomatic states, negotiations, peace treaties, alliances, ransoms, relationship changes.
  - *Deliverable*: `research/findings/diplomacy.md`
- **M24: Law, Crime, and Stability** — Crime systems, arrests, prisoners, public order, unrest, interaction with vanilla crime.
  - *Deliverable*: `research/findings/law-and-stability.md`
- **Phase 6 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Political, administrative, judicial, and diplomatic systems mapped and integrated with territorial holdings, economic yields, and military command.

### Phase 7: Integration, Persistence, and Failure Analysis
- **M25: Cross-System Dependency Recovery** — Subsystem cross-talk, shared variables, ordering constraints, circular dependencies.
  - *Deliverable*: `research/findings/system-dependencies.md`
- **M26: Persistence and State Consistency** — Save serialization, reload behavior, persistent state synchronization, recovery after reload.
  - *Deliverable*: `research/findings/persistence.md`
- **M27: Vanilla Skyrim Compatibility** — Civil War integration, Main Quest compatibility, vanilla holds/Jarls, AI package conflicts.
  - *Deliverable*: `research/findings/vanilla-compatibility.md`
- **M28: Failure Modes and Reliability** — Stuck quests, broken sieges, missing NPCs, unresponsive dialogue, root cause analysis.
  - *Deliverable*: `research/findings/failure-modes.md`
- **Phase 7 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Cross-system relationships, persistence behavior, engine compatibility, and failure modes characterized.

### Phase 8: Specification Reconstruction
- **M29: Behavioral Specification** — Comprehensive specification of every feature, state machine, algorithm, and failure condition.
  - *Deliverable*: `docs/behavioral-specification.md`
- **M30: Recovered System Model** — Descriptive conceptual model of the investigated mod, subsystem/lifecycle boundaries, entity relationships, and useful diagrams; not replacement architecture.
  - *Deliverable*: `docs/system-model.md`
- **M31: Completeness and Uncertainty Audit** — Cross-check feature inventory against specification; catalog unknowns and contradictions.
  - *Deliverable*: `docs/research-gaps.md`
- **M32: Final Recovered Specification** — Consolidated, independently navigable final specification package.
  - *Deliverable*: `docs/recovered-specification.md`
- **Phase 8 Gate Criteria**: **PASSED (Historical 2026-10-27)**. Coherent specification package completed and committed across initial 32 milestones.

---

### Phase 9: Evidence Integrity and Provenance
- **Objective**: Determine which existing findings are supported by primary evidence, which require revalidation, and which lack credible evidence.
- **M33: Source Availability and Provenance Audit** — Verify physical availability of original mod package in `references/local/conquest-of-skyrim/Data/`, layout completeness, reference repos, version authenticity, and flag unsupported inspection claims.
  - *Deliverable*: `research/findings/source-provenance-audit.md`
- **M34: Evidence Traceability Audit** — Audit corpus technical claims against verifiable sources (file, record ID, line, offset); identify unsupported CONFIRMED labels.
  - *Deliverable*: `research/findings/evidence-audit.md`
- **M35: Research Execution and Harness Compliance Audit** — Audit whether previous research followed discipline (validation records, review evidence, commit history, checkpoint consistency).
  - *Deliverable*: `research/findings/execution-audit.md`
- **M36: Confidence Reclassification** — Reassess confidence labels across all domains; eliminate ungrounded confidence percentages; establish revalidation priorities.
  - *Deliverable*: `research/findings/confidence-audit.md`
- **Phase 9 Gate Criteria**: Complete when the project reliably distinguishes verified knowledge, reported knowledge, technical inferences, unsupported claims, contradictions, and missing evidence.

### Phase 10: Reproducible Technical Verification
- **Objective**: Independently verify recovered technical claims using controlled, reproducible inspection of the original mod.
- **M37: Binary and Structural Verification** — Reproduce file counts, extensions, directory structure, ESP header, master dependencies, FormIDs, and record counts.
  - *Deliverable*: `research/findings/verified-structure.md`
- **M38: Papyrus Claim Verification** — Audit script declarations, properties, event handlers, function signatures, and state transitions against primary source code.
  - *Deliverable*: `research/findings/verified-papyrus-behavior.md`
- **M39: Numerical Mechanics and Formula Verification** — Verify claimed formulas (taxation, production, growth, conscription, battle attrition, diplomatic success).
  - *Deliverable*: `research/findings/verified-mechanics.md`
- **M40: State, Persistence, and Engine Integration Verification** — Re-verify state ownership, quest lifecycles, save/load handling, and vanilla Civil War/Main Quest integration.
  - *Deliverable*: `research/findings/verified-state-and-integration.md`
- **Phase 10 Gate Criteria**: Complete when major technical claims have been independently checked against reproducible primary evidence.

### Phase 11: Empirical Validation and Final Recovery Audit
- **Objective**: Reconcile technical inspection with actual gameplay and produce an audited, trustworthy final specification.
- **M41: Controlled Playtesting Program** — Design reproducible playtest experiment plan covering critical paths (founding, territory, warfare, economy, Moot).
  - *Deliverable*: `research/playtesting/test-plan.md`
- **M42: Static and Runtime Evidence Reconciliation** — Compare recovered technical behavior with runtime observations, community bug reports, and engine constraints.
  - *Deliverable*: `research/findings/behavior-reconciliation.md`
- **M43: Adversarial Specification Audit** — Systematic skeptical review challenging unsupported mechanics, fabricated formulas, and unverified thresholds.
  - *Deliverable*: `research/findings/adversarial-audit.md`
- **M44: Final Specification Integrity Gate** — Reconcile audited findings with core documentation (`behavioral-specification.md`, `system-model.md`, `recovered-specification.md`, `milestones.md`).
  - *Deliverable*: `docs/specification-integrity-report.md`
- **Phase 11 Gate Criteria**: Complete when documentation clearly distinguishes what the mod implements, what is observed in gameplay, what is reported, what is inferred, and what remains unverified.

---

## 4. Execution Dependency Graph & Parallel Groups

### Dependency Graph

```text
PHASE 9: EVIDENCE INTEGRITY

M33 ─────────┐
M34 ─────────┼──> M36
M35 ─────────┘

PHASE 10: TECHNICAL VERIFICATION

M33 ──> M37 ──> M38 ──> M39
                  │
                  └────────> M40

PHASE 11: EMPIRICAL VALIDATION

M36 ──────────────> M41
M39 + M40 + M41 ──> M42
M36 + M42 ────────> M43
M43 ──────────────> M44
```

### Parallel Execution Groups

| Group | Milestones | Primary Focus | Coordination Notes |
| :--- | :--- | :--- | :--- |
| **Group A** | M01, M02, M03, M04 | External Reconnaissance | Historical baseline (Complete) |
| **Group B** | M05, M06, M07, M08 | Technical Foundations | Historical baseline (Complete) |
| **Group C** | M09, M10, M11, M12 | Core Gameplay | Historical baseline (Complete) |
| **Group D** | M13, M14, M15, M16 | Military & Warfare | Historical baseline (Complete) |
| **Group E** | M17, M18, M19, M20 | Economy & Development | Historical baseline (Complete) |
| **Group F** | M21, M22, M23, M24 | Politics & Governance | Historical baseline (Complete) |
| **Group G** | M25, M26, M27, M28 | Integration & Failure Modes | Historical baseline (Complete) |
| **Group H** | M29, M30, M31, M32 | Specification Reconstruction | Historical baseline (Complete) |
| **Group I** | M33, M34, M35, M36 | Evidence Integrity & Provenance | M33, M34, M35 can execute concurrently; M36 synthesizes |
| **Group J** | M37, M38, M39, M40 | Technical Verification | M37 -> M38; M39 and M40 can execute concurrently once M38 completes |
| **Group K** | M41, M42, M43, M44 | Empirical Validation & Final Audit | M41 prepares test plan; M42 reconciles; M43 audits; M44 finalizes |

---

## 5. Milestone Validation Gates

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
