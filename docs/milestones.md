# Research Roadmap & Milestone Registry

## 1. Overview & Roadmap Status

This document tracks all 32 milestones of the *Conquest of Skyrim* specification recovery project. It serves as the authoritative operational roadmap for the research program.

### Roadmap Summary

| Total Milestones | NOT STARTED | IN PROGRESS | BLOCKED | COMPLETE |
| :---: | :---: | :---: | :---: | :---: |
| 32 | 32 | 0 | 0 | 0 |

### Allowed Status Values
- **NOT STARTED**: Research has not yet begun.
- **IN PROGRESS**: Active investigation or playtesting underway.
- **BLOCKED**: Execution halted by external dependencies or environmental constraints.
- **COMPLETE**: Deliverable created, evidence verified, review passed, and changes committed.

---

## 2. Master Milestone Registry

| ID | Title | Phase | Status | Dependencies | Assigned Agent | Deliverable | Validation | Blockers | Commit |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **M01** | Project Identity & History | Phase 1 | NOT STARTED | None | Unassigned | `research/findings/project-overview.md` | Pending | None | — |
| **M02** | Documentation Recovery | Phase 1 | NOT STARTED | None | Unassigned | `research/findings/documentation-inventory.md` | Pending | None | — |
| **M03** | Comprehensive Feature Inventory | Phase 1 | NOT STARTED | M01, M02 | Unassigned | `research/findings/feature-inventory.md` | Pending | None | — |
| **M04** | Issues & Compatibility Inventory | Phase 1 | NOT STARTED | M01, M02 | Unassigned | `research/findings/known-issues.md` | Pending | None | — |
| **M05** | Distribution & Dependency Analysis | Phase 2 | NOT STARTED | Phase 1 Gate | Unassigned | `research/findings/file-inventory.md` | Pending | None | — |
| **M06** | Creation Engine Integration | Phase 2 | NOT STARTED | M05 | Unassigned | `research/findings/skyrim-integration.md` | Pending | None | — |
| **M07** | Static Structure Recovery | Phase 2 | NOT STARTED | M05, M06 | Unassigned | `research/findings/static-structure.md` | Pending | None | — |
| **M08** | Initialization & Runtime Lifecycle | Phase 2 | NOT STARTED | M05, M06 | Unassigned | `research/findings/runtime-lifecycle.md` | Pending | None | — |
| **M09** | Faction Creation | Phase 3 | NOT STARTED | Phase 2 Gate | Unassigned | `research/findings/faction-creation.md` | Pending | None | — |
| **M10** | Territory & Ownership | Phase 3 | NOT STARTED | M09 | Unassigned | `research/findings/territory.md` | Pending | None | — |
| **M11** | Progression & Victory | Phase 3 | NOT STARTED | M09, M10 | Unassigned | `research/findings/progression.md` | Pending | None | — |
| **M12** | Player Interaction & Configuration | Phase 3 | NOT STARTED | M09 | Unassigned | `research/findings/interactions.md` | Pending | None | — |
| **M13** | Army & Recruitment Systems | Phase 4 | NOT STARTED | Phase 3 Gate | Unassigned | `research/findings/military-organization.md` | Pending | None | — |
| **M14** | Orders, Movement & Deployment | Phase 4 | NOT STARTED | M13 | Unassigned | `research/findings/military-orders.md` | Pending | None | — |
| **M15** | Combat Simulation | Phase 4 | NOT STARTED | M13, M14 | Unassigned | `research/findings/battles.md` | Pending | None | — |
| **M16** | Siege Warfare | Phase 4 | NOT STARTED | M14, M15 | Unassigned | `research/findings/sieges.md` | Pending | None | — |
| **M17** | Resource & Treasury Systems | Phase 5 | NOT STARTED | Phase 3 Gate | Unassigned | `research/findings/economy.md` | Pending | None | — |
| **M18** | Population & Employment | Phase 5 | NOT STARTED | M17 | Unassigned | `research/findings/population.md` | Pending | None | — |
| **M19** | Buildings & Infrastructure | Phase 5 | NOT STARTED | M17, M18 | Unassigned | `research/findings/infrastructure.md` | Pending | None | — |
| **M20** | Technology & Research | Phase 5 | NOT STARTED | M17, M19 | Unassigned | `research/findings/technology.md` | Pending | None | — |
| **M21** | Government & Officials | Phase 6 | NOT STARTED | Phase 4 & 5 Gates | Unassigned | `research/findings/government.md` | Pending | None | — |
| **M22** | Jarls & Hold Administration | Phase 6 | NOT STARTED | M21, M10 | Unassigned | `research/findings/jarls.md` | Pending | None | — |
| **M23** | Diplomacy | Phase 6 | NOT STARTED | M21, M22 | Unassigned | `research/findings/diplomacy.md` | Pending | None | — |
| **M24** | Law, Crime & Stability | Phase 6 | NOT STARTED | M21, M22 | Unassigned | `research/findings/law-and-stability.md` | Pending | None | — |
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
- **Phase 1 Gate Criteria**: Primary investigation version established; documentation sources indexed; feature inventory comprehensive; known issues categorized; research gaps identified.

### Phase 2: Technical Reconnaissance
- **M05: Distribution and Dependency Analysis** — Package layout, plugin records, scripts, assets, required/optional dependencies.
  - *Deliverable*: `research/findings/file-inventory.md`
- **M06: Creation Engine Integration** — Engine hooks (quests, factions, locations, NPCs, AI packages, global variables, Papyrus/SKSE).
  - *Deliverable*: `research/findings/skyrim-integration.md`
- **M07: Static Structure Recovery** — Plugin record relationships, script attachment, event registrations, data dependencies.
  - *Deliverable*: `research/findings/static-structure.md`
- **M08: Initialization and Runtime Lifecycle** — Quest startup, background updates, save/load behavior, shutdown, recovery.
  - *Deliverable*: `research/findings/runtime-lifecycle.md`
- **Phase 2 Gate Criteria**: Technical surface sufficiently mapped to support focused subsystem investigation.

### Phase 3: Core Gameplay Specification
- **M09: Faction Creation** — Workflow, prerequisites, initial resources, customization, failure behavior, persistence.
  - *Deliverable*: `research/findings/faction-creation.md`
- **M10: Territory and Ownership** — Hierarchy, holds, settlements, forts, capture requirements, ownership transitions.
  - *Deliverable*: `research/findings/territory.md`
- **M11: Progression and Victory** — Progression stages, unlock loops, political progression, The Moot, High King status, victory conditions.
  - *Deliverable*: `research/findings/progression.md`
- **M12: Player Interaction and Configuration** — Menus, dialogue, commands, MCM settings, status reporting, error feedback.
  - *Deliverable*: `research/findings/interactions.md`
- **Phase 3 Gate Criteria**: Complete observable core loop described from faction initialization through conquest and endgame.

### Phase 4: Military and Warfare
- **M13: Army and Recruitment Systems** — Recruitment sources, troop types, costs, capacity limits, commanders, equipment.
  - *Deliverable*: `research/findings/military-organization.md`
- **M14: Orders, Movement, and Deployment** — Command hierarchy, movement logic, patrols, garrisons, camps, order transitions.
  - *Deliverable*: `research/findings/military-orders.md`
- **M15: Combat Simulation** — Battle initialization, participant selection, casualties, retreat, victory/defeat thresholds, post-battle cleanup.
  - *Deliverable*: `research/findings/battles.md`
- **M16: Siege Warfare** — Siege initiation, assault mechanics, defender behavior, garrison integration, settlement transfer.
  - *Deliverable*: `research/findings/sieges.md`
- **Phase 4 Gate Criteria**: Complete military lifecycle documented (Recruitment → Organization → Deployment → Combat → Resolution → Recovery).

### Phase 5: Economy, Population, and Development
- **M17: Resource and Treasury Systems** — Resource types, production, storage, trade, treasury flows, taxes, shortages, update rates.
  - *Deliverable*: `research/findings/economy.md`
- **M18: Population and Employment** — Population models, growth, labor requirements, conscription, settlement differences.
  - *Deliverable*: `research/findings/population.md`
- **M19: Buildings and Infrastructure** — Construction options, prerequisites, costs, production facilities, settlement upgrades.
  - *Deliverable*: `research/findings/infrastructure.md`
- **M20: Technology and Research** — Categories, prerequisites, research progression, persistent upgrades, limitations.
  - *Deliverable*: `research/findings/technology.md`
- **Phase 5 Gate Criteria**: Economic loops mapped (Population → Labor → Production → Resources → Development & Resources → Military Capacity → Expansion).

### Phase 6: Politics, Diplomacy, and Administration
- **M21: Government and Officials** — Administrative hierarchy, official appointments, authority, delegation, removal.
  - *Deliverable*: `research/findings/government.md`
- **M22: Jarls and Hold Administration** — Jarl appointments, hold ownership, loyalty, rebellions, interaction with vanilla Jarls.
  - *Deliverable*: `research/findings/jarls.md`
- **M23: Diplomacy** — Diplomatic states, negotiations, peace treaties, alliances, ransoms, relationship changes.
  - *Deliverable*: `research/findings/diplomacy.md`
- **M24: Law, Crime, and Stability** — Crime systems, arrests, prisoners, public order, unrest, interaction with vanilla crime.
  - *Deliverable*: `research/findings/law-and-stability.md`
- **Phase 6 Gate Criteria**: Political and administrative systems connected to territorial control, economy, and military systems.

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
- **M30: Recovered System Model** — Descriptive conceptual model, architecture boundaries, entity relationships, and diagrams.
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
| **Group H** | M29, M30, M31, M32 | Specification Reconstruction | Sequential synthesis and comprehensive audit |
