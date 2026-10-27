# Reference Management Discipline

## 1. Overview & Objectives

This document establishes the acquisition, organization, versioning, isolation, and maintenance discipline for external references within the *Conquest of Skyrim* specification recovery project.

Research into complex Skyrim systems requires consulting external source materials, including:
- Skyrim Script Extender (SKSE) source code and headers
- CommonLibSSE and reverse-engineered engine interfaces
- Official and community Creation Engine and Papyrus documentation
- Skyrim plugin analysis and inspection tools
- Open-source Skyrim mod implementations
- Original mod author notes and changelogs

The objective is to maintain a small, reproducible, well-documented, and strictly isolated collection of external references without cloning arbitrary repositories or accumulating unmanaged dependencies.

*Note: Do not acquire references or clone repositories during bootstrap.*

---

## 2. Document Responsibilities

- **`docs/references.md`** (this document): Defines policy, acquisition criteria, version pinning, provenance, isolation boundaries, and validation rules.
- **`references/README.md`**: Maintains the active inventory cataloging all acquired references, their purpose, upstream URLs, versions, licenses, and target milestones.
- **`AGENTS.md`**: Provides a concise summary and operational entry point directing agents to this discipline.

---

## 3. Reference Categories

External materials fall into three distinct categories:

### 3.1 Git Repositories
- **Examples**: SKSE source repositories, CommonLibSSE forks, open-source modding utilities, relevant open-source Skyrim mod scripts.
- **Canonical Location**: `references/sources/<repository-name>/`
- **Management**: Prefer **Git submodules** for repositories intended to remain part of the project's permanent, reproducible reference set. Do not use unmanaged clones as permanent references.
- **Read-Only**: External repositories are strictly read-only. Never modify tracked files or generate build artifacts inside reference submodules.

### 3.2 Online Documentation
- **Examples**: Official documentation wikis, technical articles, Nexus mod descriptions, GitHub documentation pages, Creation Engine guides, author statements.
- **Handling**: Record URLs, titles, access dates, relevant versions, and specific section references in the inventory or research documents.
- **Snapshots**: Do not mirror whole websites. Preserve only small, targeted excerpts or text snapshots when necessary for permanence or offline reference, citing source provenance.

### 3.3 Local Technical Material
- **Examples**: Plugin record dumps, Papyrus diagnostic logs, tool-generated analysis reports, extracted metadata.
- **Handling**: These are research artifacts, not source references. Manage them under `artifacts/` according to `docs/testing.md`.
- **Prohibition**: Never place binary mod archives (`.zip`, `.7z`), plugin files (`.esp`, `.esm`), BSAs, or raw diagnostic dumps inside `references/sources/`.

---

## 4. Acquisition Discipline

### 4.1 Justification Before Acquisition
Before adding any reference to the workspace, the researcher or agent must answer seven questions:
1. **What specific research question does this reference answer?**
2. **Which milestone or task requires it?**
3. **What concrete information is expected from it?**
4. **Is an existing reference or public documentation sufficient?**
5. **Is a permanent local copy or submodule truly necessary?**
6. **What is the acquisition and maintenance cost?**
7. **Are there licensing, copyright, or access restrictions?**

*Rule: Do not acquire repositories simply because they are related to Skyrim or modding. Every reference must have a demonstrated research justification.*

### 4.2 Hierarchy of Authoritative Sources
When seeking technical truth, prioritize sources in the following order:
1. **Official project repositories** (e.g., SKSE upstream, official engine headers)
2. **Official technical documentation** (e.g., Creation Kit wiki, Bethesda technical specs)
3. **Original author publications** (e.g., *Conquest of Skyrim* release notes, source scripts)
4. **Maintained community documentation** (e.g., UESP, well-vetted developer wikis)
5. **Established open-source implementations** (e.g., CommonLibSSE plugins)
6. **Community discussions and secondary explanations** (forums, Discord logs, Reddit)

*Authority does not guarantee correctness. Always record observed discrepancies, version differences, and contradictory claims.*

### 4.3 Mandatory Version Pinning
To ensure scientific reproducibility across long-running research:
- **Pin to Exact Revisions**: Every Git repository must be checked out to a specific, immutable commit hash or release tag. Never track floating branch heads (`main`, `master`, `HEAD`).
- **Record Version Details**: For documentation or software releases, record the exact semantic version (e.g., SKSE 2.0.20, Skyrim SE 1.5.97 / 1.6.640).
- **Document Snapshot Dates**: For online web documentation without formal versioning, record the exact retrieval date (ISO 8601).

---

## 5. Provenance and Licensing

Every reference recorded in `references/README.md` must document:
- **Reference Name**: Formal name of the project or library.
- **Purpose**: Why it was added and how it serves the research.
- **Upstream Location**: Canonical URL or repository clone URL.
- **Reference Type**: Git submodule, documentation link, or excerpt.
- **Version / Revision**: Exact commit hash, release tag, or retrieval date.
- **License / Terms**: License identifier (e.g., MIT, GPLv3, CC-BY, proprietary / author-restricted) and any usage constraints.
- **Relevant Milestones**: Associated research milestones (e.g., M01, M05).
- **Local Path**: Relative path within the repository (e.g., `references/sources/commonlib-sse`).

---

## 6. Source Isolation & Storage Discipline

1. **Strict Filesystem Isolation**: All external code repositories reside strictly under `references/sources/`.
2. **Read-Only Invariant**: External sources are immutable references. Never commit project code, research notes, or experimental outputs into external source directories.
3. **No Binary Blobs**: Do not commit mod archives, compiled game DLLs, or binary assets into `references/`. Large binary assets belong outside Git, governed by `.gitignore`.
4. **Clean Workspace**: Ensure that inspecting external references does not leave untracked compilation files or caches (`.o`, `.pyc`, build folders) in reference trees.

---

## 7. Parallel Acquisition & Coordination

When multiple agents or parallel tasks execute concurrently:
- **Centralized Inventory Updates**: Reference additions must be coordinated. The coordinating agent manages submodule initialization and updates `references/README.md`.
- **No Concurrent Checkouts**: Parallel subagents must not independently clone or modify Git submodules to avoid checkout collisions, lockfile conflicts, or duplicated storage.
- **Pre-Check Existing References**: Agents must inspect `references/README.md` before requesting or acquiring any new source material.
