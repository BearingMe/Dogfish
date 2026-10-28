# Research Roadmap & Milestone Registry

## 1. Overview & Roadmap Status

This document tracks all 32 milestones of the *Conquest of Skyrim* specification recovery project. It serves as the authoritative operational roadmap for the research program.

### Roadmap Summary

| Total Milestones | NOT STARTED | IN PROGRESS | BLOCKED | COMPLETE |
| :---: | :---: | :---: | :---: | :---: |
| 32 | 8 | 0 | 0 | 24 |

### Allowed Status Values
- **NOT STARTED**: Research has not yet begun.
- **IN PROGRESS**: Active investigation or playtesting underway.
- **BLOCKED**: Execution halted by external dependencies or environmental constraints.
- **COMPLETE**: Deliverable created, evidence verified, review passed, and changes committed.

---

## 2. Master Milestone Registry

Dependencies below are conservative planning inputs, not blanket prerequisites for starting independent research. M01–M04 public-documentation work can run concurrently; integrate their results before closing the Phase 1 gate. Later milestones may begin when their specific evidence/prerequisites are available; record deviations and do not impose artificial waits on unrelated human playtesting. Shared roadmap/reference changes remain coordinator-owned.

Phase 1 (External Reconnaissance), Phase 2 (Technical Reconnaissance), Phase 3 (Core Gameplay Specification), Phase 4 (Military and Warfare), Phase 5 (Economy, Population, and Development), and Phase 6 (Politics, Diplomacy, and Administration) are COMPLETE (M01–M24 validated). Target version is confirmed as v0.3.9 (March 23, 2023 release by galvatron304). Local mod files are verified as AVAILABLE in `references/local/conquest-of-skyrim/Data/` and clean-room guardrails are established in `guardrails.md`. Phase 7 Integration, Persistence, and Failure Analysis is unlocked.

The validation column records PASS, FAIL, BLOCKED, or INCONCLUSIVE once checked; Pending means no validation has run. COMPLETE follows `docs/execution-discipline.md`, including review/commit and explicit handling of limitations. Planned deliverable paths below are intentionally absent until their milestone begins.

| ID | Title | Phase | Status | Dependencies | Assigned Agent | Deliverable | Validation | Blockers | Commit |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **M01** | Project Identity & History | Phase 1 | COMPLETE | None | Coordinator | `research/findings/project-overview.md` | PASS | None | `808cba1` |
| **M02** | Documentation Recovery | Phase 1 | COMPLETE | None | Coordinator | `research/findings/documentation-inventory.md` | PASS | None | `288d005` |
| **M03** | Comprehensive Feature Inventory | Phase 1 | COMPLETE | M01, M02 | Coordinator | `research/findings/feature-inventory.md` | PASS | None | `2670636` |
| **M04** | Issues & Compatibility Inventory | Phase 1 | COMPLETE | M01, M02 | Coordinator | `research/findings/known-issues.md` | PASS | None | `8a5462c` |
| **M05** | Distribution & Dependency Analysis | Phase 2 | COMPLETE | Phase 1 Gate | Coordinator | `research/findings/file-inventory.md` | PASS | None | `606b2e7` |
| **M06** | Creation Engine Integration | Phase 2 | COMPLETE | M05 | Coordinator | `research/findings/skyrim-integration.md` | PASS | None | `660fb67` |
| **M07** | Static Structure Recovery | Phase 2 | COMPLETE | M05, M06 | Coordinator | `research/findings/static-structure.md` | PASS | None | `ef03ca4` |
| **M08** | Initialization & Runtime Lifecycle | Phase 2 | COMPLETE | M05, M06 | Coordinator | `research/findings/runtime-lifecycle.md` | PASS | None | `90c2f86` |
| **M09** | Faction Creation | Phase 3 | COMPLETE | Phase 2 Gate | Coordinator | `research/findings/faction-creation.md` | PASS | None | `dd5fa2a` |
| **M10** | Territory & Ownership | Phase 3 | COMPLETE | M09 | Coordinator | `research/findings/territory.md` | PASS | None | `b7d1d82` |
| **M11** | Progression & Victory | Phase 3 | COMPLETE | M09, M10 | Coordinator | `research/findings/progression.md` | PASS | None | `1ca2285` |
| **M12** | Player Interaction & Configuration | Phase 3 | COMPLETE | M09 | Coordinator | `research/findings/interactions.md` | PASS | None | `47817a1` |
| **M13** | Army & Recruitment Systems | Phase 4 | COMPLETE | Phase 3 Gate | Coordinator | `research/findings/military-organization.md` | PASS | None | `28be1d5` |
| **M14** | Orders, Movement & Deployment | Phase 4 | COMPLETE | M13 | Coordinator | `research/findings/military-orders.md` | PASS | None | `d505daa` |
| **M15** | Combat Simulation | Phase 4 | COMPLETE | M13, M14 | Coordinator | `research/findings/battles.md` | PASS | None | `c29f2ee` |
| **M16** | Siege Warfare | Phase 4 | COMPLETE | M14, M15 | Coordinator | `research/findings/sieges.md` | PASS | None | `b939bde` |
| **M17** | Resource & Treasury Systems | Phase 5 | COMPLETE | Phase 3 Gate | Coordinator | `research/findings/economy.md` | PASS | None | `a3308b5` |
| **M18** | Population & Employment | Phase 5 | COMPLETE | M17 | Coordinator | `research/findings/population.md` | PASS | None | `7ee082a` |
| **M19** | Buildings & Infrastructure | Phase 5 | COMPLETE | M17, M18 | Coordinator | `research/findings/infrastructure.md` | PASS | None | `da990e7` |
| **M20** | Technology & Research | Phase 5 | COMPLETE | M17, M19 | Coordinator | `research/findings/technology.md` | PASS | None | `9792f7e` |
| **M21** | Government & Officials | Phase 6 | COMPLETE | Phase 4 & 5 Gates | Coordinator | `research/findings/government.md` | PASS | None | `a6b8588` |
| **M22** | Jarls & Hold Administration | Phase 6 | COMPLETE | M21, M10 | Coordinator | `research/findings/jarls.md` | PASS | None | `80da2db` |
| **M23** | Diplomacy | Phase 6 | COMPLETE | M21, M22 | Coordinator | `research/findings/diplomacy.md` | PASS | None | `f1764aa` |
| **M24** | Law, Crime & Stability | Phase 6 | COMPLETE | M21, M22 | Coordinator | `research/findings/law-and-stability.md` | PASS | None | `68eb1f4` |
| **M25** | Cross-System Dependency Recovery | Phase 7 | NOT STARTED | Phase 6 Gate | Unassigned | `research/findings/system-dependencies.md` | Pending | None | — |
| **M26** | Persistence & State Consistency | Phase 7 | NOT STARTED | M25 | Unassigned | `research/findings/persistence.md` | Pending | None | — |
| **M27** | Vanilla Skyrim Compatibility | Phase 7 | NOT STARTED | M25 | Unassigned | `research/findings/vanilla-compatibility.md` | Pending | None | — |
| **M28** | Failure Modes & Reliability | Phase 7 | NOT STARTED | M25, M26, M27 | Unassigned | `research/findings/failure-modes.md` | Pending | None | — |
| **M29** | Behavioral Specification | Phase 8 | NOT STARTED | Phase 7 Gate | Unassigned | `docs/behavioral-specification.md` | Pending | None | — |
| **M30** | Recovered System Model | Phase 8 | NOT STARTED | M29 | Unassigned | `docs/system-model.md` | Pending | None | — |
| **M31** | Completeness & Uncertainty Audit | Phase 8 | NOT STARTED | M29, M30 | Unassigned | `docs/research-gaps.md` | Pending | None | — |
| **M32** | Final Recovered Specification | Phase 8 | NOT STARTED | M29, M30, M31 | Unassigned | `docs/recovered-specification.md` | Pending | None | — |

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
- **Phase 1 Gate Criteria**: **PASSED (2026-10-27)**. Primary investigation version established (v0.3.9); documentation sources indexed; feature inventory comprehensive (51 features); known issues categorized; research gaps identified.

### Phase 2: Technical Reconnaissance
- **M05: Distribution and Dependency Analysis** — Package layout, plugin records, scripts, assets, required/optional dependencies.
  - *Deliverable*: `research/findings/file-inventory.md`
- **M06: Creation Engine Integration** — Engine hooks (quests, factions, locations, NPCs, AI packages, global variables, Papyrus/SKSE).
  - *Deliverable*: `research/findings/skyrim-integration.md`
- **M07: Static Structure Recovery** — Plugin record relationships, script attachment, event registrations, data dependencies.
  - *Deliverable*: `research/findings/static-structure.md`
- **M08: Initialization and Runtime Lifecycle** — Quest startup, background updates, save/load behavior, shutdown, recovery.
  - *Deliverable*: `research/findings/runtime-lifecycle.md`
- **Phase 2 Gate Criteria**: **PASSED (2026-10-27)**. Technical surface sufficiently mapped across distribution, engine integration, static controller architecture, and runtime lifecycle. Subsystem investigation unlocked.

### Phase 3: Core Gameplay Specification
- **M09: Faction Creation** — Workflow, prerequisites, initial resources, customization, failure behavior, persistence.
  - *Deliverable*: `research/findings/faction-creation.md`
- **M10: Territory and Ownership** — Hierarchy, holds, settlements, forts, capture requirements, ownership transitions.
  - *Deliverable*: `research/findings/territory.md`
- **M11: Progression and Victory** — Progression stages, unlock loops, political progression, The Moot, High King status, victory conditions.
  - *Deliverable*: `research/findings/progression.md`
- **M12: Player Interaction and Configuration** — Menus, dialogue, commands, configuration surfaces (MCM only if established), status reporting, error feedback.
  - *Deliverable*: `research/findings/interactions.md`
- **Phase 3 Gate Criteria**: **PASSED (2026-10-27)**. Complete observable core loop reconstructed from faction initialization through conquest, progression, and player interaction channels.

### Phase 4: Military and Warfare
- **M13: Army and Recruitment Systems** — Recruitment sources, troop types, costs, capacity limits, commanders, equipment.
  - *Deliverable*: `research/findings/military-organization.md`
- **M14: Orders, Movement, and Deployment** — Command hierarchy, movement logic, patrols, garrisons, camps, order transitions.
  - *Deliverable*: `research/findings/military-orders.md`
- **M15: Combat Simulation** — Battle initialization, participant selection, casualties, retreat, victory/defeat thresholds, post-battle cleanup.
  - *Deliverable*: `research/findings/battles.md`
- **M16: Siege Warfare** — Siege initiation, assault mechanics, defender behavior, garrison integration, settlement transfer.
  - *Deliverable*: `research/findings/sieges.md`
- **Phase 4 Gate Criteria**: **PASSED (2026-10-27)**. Complete military lifecycle documented across recruitment, division hierarchy, spatial movement, combat simulation, and siege capitulation.

### Phase 5: Economy, Population, and Development
- **M17: Resource and Treasury Systems** — Resource types, production, storage, trade, treasury flows, taxes, shortages, update rates.
  - *Deliverable*: `research/findings/economy.md`
- **M18: Population and Employment** — Population models, growth, labor requirements, conscription, settlement differences.
  - *Deliverable*: `research/findings/population.md`
- **M19: Buildings and Infrastructure** — Construction options, prerequisites, costs, production facilities, settlement upgrades.
  - *Deliverable*: `research/findings/infrastructure.md`
- **M20: Technology and Research** — Categories, prerequisites, research progression, persistent upgrades, limitations.
  - *Deliverable*: `research/findings/technology.md`
- **Phase 5 Gate Criteria**: **PASSED (2026-10-27)**. Full economic loop recovered: Population -> Labor -> Production -> Resources -> Development and Resources -> Military Capacity -> Expansion.

### Phase 6: Politics, Diplomacy, and Administration
- **M21: Government and Officials** — Administrative hierarchy, official appointments, authority, delegation, removal.
  - *Deliverable*: `research/findings/government.md`
- **M22: Jarls and Hold Administration** — Jarl appointments, hold ownership, loyalty, rebellions, interaction with vanilla Jarls.
  - *Deliverable*: `research/findings/jarls.md`
- **M23: Diplomacy** — Diplomatic states, negotiations, peace treaties, alliances, ransoms, relationship changes.
  - *Deliverable*: `research/findings/diplomacy.md`
- **M24: Law, Crime, and Stability** — Crime systems, arrests, prisoners, public order, unrest, interaction with vanilla crime.
  - *Deliverable*: `research/findings/law-and-stability.md`
- **Phase 6 Gate Criteria**: **PASSED (2026-10-27)**. Political, administrative, judicial, and diplomatic systems mapped and integrated with territorial holdings, economic yields, and military command.

### Phase 7: Integration, Persistence, and Failure Analysis
- **M25: Cross-System Dependency Recovery** — Subsystem cross-talk, shared variables, ordering constraints, circular dependencies.
  - *Deliverable*: `research/findings/system-dependencies.md`
- **M26: Persistence and State Consistency** — Save serialization, reload behavior, persistent state synchronization, recovery after reload.
  - *Deliverable*: `research/findings/persistence.md`
- **M27: Vanilla Skyrim Compatibility** — Civil War integration, Main Quest compatibility, vanilla holds/Jarls, AI package conflicts.
  - *Deliverable*: `research/findings/vanilla-compatibility.md`
- **M28: Failure Modes and Reliability** — Stuck quests, broken sieges, missing NPCs, unresponsive dialogue, root cause analysis.
  - *Deliverable*: `research/findings/failure-modes.md`
- **Phase 7 Gate Criteria**: Cross-system relationships, persistence behavior, engine compatibility, and failure modes characterized.

### Phase 8: Specification Reconstruction
- **M29: Behavioral Specification** — Comprehensive specification of every feature, state machine, algorithm, and failure condition.
  - *Deliverable*: `docs/behavioral-specification.md`
- **M30: Recovered System Model** — Descriptive conceptual model of the investigated mod, subsystem/lifecycle boundaries, entity relationships, and useful diagrams; not replacement architecture.
  - *Deliverable*: `docs/system-model.md`
- **M31: Completeness and Uncertainty Audit** — Cross-check feature inventory against specification; catalog unknowns and contradictions.
  - *Deliverable*: `docs/research-gaps.md`
- **M32: Final Recovered Specification** — Consolidated, independently navigable final specification package.
  - *Deliverable*: `docs/recovered-specification.md`
- **Phase 8 Gate Criteria**: Coherent, evidence-backed, fully validated recovered specification package completed and committed.

---

## 4. Parallel Execution Groups

| Group | Milestones | Primary Focus | Coordination Notes |
| :--- | :--- | :--- | :--- |
| **Group A** | M01, M02, M03, M04 | External Reconnaissance | Independent public documentation inspection |
| **Group B** | M05, M06, M07, M08 | Technical Foundations | Coordinate shared source acquisition under `references/sources/` |
| **Group C** | M09, M10, M11, M12 | Core Gameplay | Focus on observable core mechanics |
| **Group D** | M13, M14, M15, M16 | Military & Warfare | State transitions, recruitment, and simulation |
| **Group E** | M17, M18, M19, M20 | Economy & Development | Resource equations, production, and technology |
| **Group F** | M21, M22, M23, M24 | Politics & Governance | Hold administration, diplomacy, and law |
| **Group G** | M25, M26, M27, M28 | Integration & Failure Modes | Cross-cutting interactions, persistence, and engine risks |
| **Group H** | M29, M30, M31, M32 | Specification Reconstruction | Coordinated synthesis and comprehensive audit; M32 integrates reviewed predecessors |

## 5. Milestone Validation Gates

These gates restore the mission's per-milestone acceptance criteria; phase summaries alone do not establish completion. Apply the evidence classifications and limitations rules in `docs/execution-discipline.md`. A question may be answered as unsupported/unknown only with evidence of the investigation and an explanation; do not assume every listed feature exists.

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

A phase gate summarizes validated milestones; it does not require fabricating inaccessible internals. Missing evidence that invalidates a required conclusion keeps the milestone open/blocked. Synthesis must remain understandable without reading raw logs, preserve version distinctions, and reference subsystem findings rather than merely concatenate them.
