# Project Guardrails & Clean-Room Reconstruction Boundaries

## 1. Objective & Scope Transition

This project has progressed from initial specification recovery into **Experimental Reconstruction, Probing, and Skyrim Integration** centered on *Conquest of Skyrim* v0.3.9 (by `galvatron304`).

The current mission is threefold:
1. **Reconstruction**: Determine whether the recovered specification can produce a faithful, independently authored, and executable behavioral model.
2. **Probing**: Discover missing information through targeted, evidence-driven investigation of the original implementation and controlled experiments.
3. **Integration**: Determine how reconstructed systems can interact with Skyrim's runtime through minimal experimental adapters.

The goal is to establish technical feasibility through executable experiments, measurable behavioral equivalence, and progressively deeper integration—not to immediately produce a complete replacement mod.

---

## 2. Permitted and Prohibited Activities

### 2.1 Permitted Activities
- **Independent Behavioral Reconstruction**: Authoring original simulation code, domain state models, and state machines based on recovered behavioral specifications.
- **Executable Reference Models**: Building standalone simulation tools (preferring Rust for deterministic domain models).
- **Standalone Simulations**: Simulating economic equations, battle attrition, and territory ownership in isolated test harnesses.
- **Experimental Engine Adapters**: Prototyping minimal interfaces connecting independent simulation logic to Skyrim runtimes (SKSE, Papyrus, C++ bridges).
- **Minimal Skyrim Integration Prototypes**: Controlled in-game vertical slices (e.g., claiming a single test location).
- **Automated Behavioral Comparison**: Differential testing comparing reconstructed simulation outputs against original mod behavior.
- **Source Inspection for Research**: Systematic static analysis of original plugin records and script sources.
- **Controlled Technical Experiments**: Reproducible test scripts and empirical playtesting protocols.

### 2.2 Prohibited Activities
- **No Verbatim Code Copying**: Never copy decompiled Papyrus script sources, functions, or fragments verbatim into project code.
- **No Direct Translation**: Do not perform mechanical line-by-line translation of original Papyrus code into another language.
- **No Asset Redistribution**: Original 3D meshes, textures, music, voice recordings, and sound archives belong to the original author and Bethesda Game Studios. They must never be tracked in Git, committed to public remotes, or redistributed.
- **No Reference Modification**: Original mod files in `references/local/conquest-of-skyrim/Data/` and submodules in `references/sources/` must remain strictly read-only, immutable, and Git-ignored.
- **No Unverified Claims**: Never present inferred behavior as verified; classify claims according to the established dual-axis framework.
- **No Proprietary Code Reuse**: Do not treat original source code as a reusable implementation library without appropriate permissions.

---

## 3. Mandatory Source Querying

Consulting the actual original source code is **REQUIRED** when investigating implementation-dependent behavior.

**Primary Source Location**: `references/local/conquest-of-skyrim/Data/`
- Papyrus source files (`scripts/source/*.psc`)
- Compiled Papyrus bytecode (`scripts/*.pex`)
- Plugin records, forms, and keywords (`conquerskyrim.esp`)
- Quest definitions and stage fragments
- AI packages and reference aliases

### 3.1 Eight-Step Source Query Protocol
When any reconstruction task depends on original mod behavior:
1. **Identify the relevant specification claim** from existing research.
2. **Locate the supporting original source** (script file, line number, or plugin record).
3. **Inspect the relevant implementation**, including surrounding conditions, call sites, and state transitions.
4. **Extract the observable behavior and constraints** into independent notes.
5. **Document findings independently** in research deliverables.
6. **Establish an implementation-independent behavioral contract** defining inputs, preconditions, state transitions, outputs, and invariants.
7. **Implement that contract** without copying or directly translating source code.
8. **Validate the reconstruction** against expected outcomes derived from the contract.

*Rule: A source filename, property declaration, or function signature alone is insufficient evidence of behavior. Always inspect actual logic, conditions, and state transitions. If primary sources are unavailable, mark dependent work BLOCKED.*

---

## 4. Clean-Room Separation

To prevent inadvertent copyright infringement and ensure independent authorship, work is conceptually partitioned into two distinct responsibilities:

### 4.1 The Researcher Role
- Inspects original implementation material (`references/local/conquest-of-skyrim/Data/`).
- Analyzes raw Papyrus bytecode, plugin records, and engine hooks.
- Recovers state transitions, input/output contracts, and numerical formulas.
- Documents edge cases and identifies technical unknowns.
- **Output**: Implementation-independent behavioral contracts and research findings.
- *Strict Invariant*: Never writes implementation code; never passes raw source-code bodies as implementation instructions.

### 4.2 The Implementer Role
- Works strictly from recovered behavioral contracts, specifications, and independent expected values.
- Independently designs data structures, state representations, and control flows.
- Authors original code (preferring Rust for simulation logic).
- Validates behavior using automated test assertions.
- Reports ambiguities back for additional probing when specifications are incomplete.
- *Strict Invariant*: Never inspects original mod source code directly while authoring implementations.

### 4.3 Execution in Single-Agent & Multi-Agent Contexts
- When multiple agents operate concurrently (via Orca or subagents), strict role separation between Researcher and Implementer agents must be maintained.
- When operating in a single-agent context, the agent must separate research and implementation into **explicit, sequential stages**: complete and review the behavioral contract first before writing any implementation code.
- *Disclaimer*: A single-agent source-informed implementation must be accurately described as stage-separated, not as a formally independent multi-party clean-room process. Clean-room procedures mitigate copying risks but do not substitute for legal compliance.

---

## 5. Attribution and Reference Isolation

- **Attribution**: Full attribution must be maintained for `galvatron304` (`galvatron530`) and upstream projects (`skse64`, `skyui-community`).
- **Filesystem Isolation**:
  - Original mod files remain strictly under `references/local/conquest-of-skyrim/Data/` (Git-ignored).
  - External Git references remain under `references/sources/` (submodules).
  - Reconstructed code belongs under `reconstruction/`.
  - Research findings belong under `research/findings/`.
  - Reusable utilities belong under `scripts/`; automated test suites belong under `tests/`.
  - Generated build outputs belong under `artifacts/` (Git-ignored).
