# Research Guardrails & Clean-Room Boundaries

## 1. Objective & Scope

This project is an **independent exploratory research and specification recovery initiative** centered on *Conquest of Skyrim* (v0.3.9 by `galvatron304`). 

The sole objective is to recover a coherent, evidence-backed technical and behavioral specification of the mod's systems. This document establishes the mandatory clean-room boundaries, legal guardrails, and research invariants governing all human researchers and AI agents.

---

## 2. Core Clean-Room Principles

### 2.1 Specification Recovery, Not Implementation
- **Descriptive, Not Generative**: The recovered specification describes what exists, how systems interact, and what failure modes occur.
- **No Replacement Mod**: Do not implement replacement mod scripts, construct playable gameplay systems, or build architectural foundations for an eventual clone.
- **No Speculative Design**: Do not "improve" or redesign mechanics; analyze and document the original implementation as observed.

### 2.2 Intellectual Property & Asset Protection
- **Zero Asset Redistribution**: Original 3D meshes, textures, music, voice recordings (ElevenLabs/SKVA), and compiled sound archives belong exclusively to the original author and Bethesda Game Studios. They must never be tracked in Git, committed to public remotes, or distributed.
- **No Verbatim Code Copying**: Do not copy decompiled Papyrus script sources verbatim into project files. Research findings must express system logic as abstract state machines, behavioral descriptions, data flows, and pseudocode specifications.
- **Immutable Local References**: Original mod files reside strictly in `references/local/conquest-of-skyrim/Data/` and must remain completely untouched, read-only, and Git-ignored.

---

## 3. Permitted Research Activities

1. **Read-Only Inspection**:
   - Examining plugin headers, record structures, quest stages, and dialogue trees in `conquerskyrim.esp`.
   - Analyzing compiled Papyrus (`.pex`) script structures to extract state transitions, event handlers, and API interactions.
   - Inspecting folder hierarchies, record counts, and file manifests.
2. **Technical Tooling**:
   - Using reverse-engineering and inspection tools (REA, Bethesda plugin inspectors, Papyrus disassemblers) in read-only mode.
   - Extracting minimal, targeted metadata and diagnostic traces into `artifacts/`.
3. **Synthesis & Documentation**:
   - Synthesizing findings into structured Markdown under `research/findings/`.
   - Recording primary facts into `research/memory.md`.
   - Cataloging external source documentation in `references/README.md`.

---

## 4. Prohibited Activities

- **Modifying Reference Data**: Never edit, patch, overwrite, or re-save files inside `references/local/conquest-of-skyrim/Data/` or `references/sources/`.
- **Committing Proprietary Binaries**: Never use `git add -f` to track `.esp`, `.esm`, `.pex`, `.bsa`, `.7z`, or asset files.
- **Executing Unknown Code**: Never execute native binary executables from mod archives.
- **Polluting Reference Trees**: Never place generated logs, test outputs, or scratchpad scripts inside reference directories.
- **Fabricating Observations**: Never invent engine mechanisms, assume unverified behavior, or simulate playtest results.

---

## 5. Evidence Standards & Attribution

- All technical conclusions must cite verifiable sources (plugin record signatures, script names, variable identifiers, playtest logs, or documentation entries).
- Full attribution must be maintained for `galvatron304` (`galvatron530`) and upstream libraries (`skse64`, `skyui-community`).
- Any unverified assumption must be explicitly classified as `INFERRED` or `UNKNOWN` rather than presented as established fact.
