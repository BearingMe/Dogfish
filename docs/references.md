# Reference Management Discipline

## 1. Overview & Objectives

This document establishes the acquisition, organization, versioning, isolation, and maintenance discipline for external research references within the *Conquest of Skyrim* specification recovery project.

External references may include:
- Skyrim Script Extender (SKSE) source repositories and headers
- CommonLibSSE and related reverse-engineering projects
- Creation Engine documentation and technical specifications
- Papyrus documentation, compilers, and analysis tooling
- Skyrim plugin inspection and record parsing tools
- Open-source Skyrim mod implementations
- Technical articles, specifications, and author statements
- Original mod documentation and changelogs
- Public repositories relevant to the investigation

The objective is to maintain a small, reproducible, well-documented, and strictly isolated collection of useful references.

*Infrastructure-only tasks do not authorize acquisition or research milestones. Acquire references only for an authorized, justified research task.*

---

## 2. Document Responsibilities

- **`docs/references.md`** (this document): Defines reference acquisition, provenance, versioning, isolation, maintenance, and validation rules.
- **`references/README.md`**: Maintains the actual compact reference inventory identifying reference name, purpose, upstream location, reference type, version/revision, license/usage restrictions, relevant milestones, and local location.
- **`AGENTS.md`**: Provides a concise entry point directing agents to this discipline.

---

## 3. Reference Categories

Distinguish three categories of external materials:

### 3.1 Git Repositories
- **Examples**: SKSE source repositories, CommonLibSSE projects, open-source modding tools, relevant open-source Skyrim mods.
- **Canonical Location**: Persistent Git repositories belong strictly under `references/sources/<repository-name>/`.
- **Submodules**: Prefer **Git submodules** for repositories intended to remain part of the project's reproducible reference collection. Do not use ordinary Git clones as permanent references without a specific reason.

### 3.2 Online Documentation
- **Examples**: Official documentation, technical articles, Nexus mod descriptions, GitHub documentation, Creation Engine references, author statements, community technical explanations.
- **Handling**: Do not download entire websites by default. Prefer recording URLs, titles, versions, and relevant sections. Preserve small local excerpts or snapshots only when necessary and permitted. Do not mirror documentation merely for convenience.

### 3.3 Local Technical Material
- **Examples**: Skyrim plugin metadata, installed mod manifests, Papyrus diagnostic logs, tool-generated reports, extracted record information.
- **Handling**: These are not automatically Git references. Use the existing testing and artifact discipline (`docs/testing.md`) for generated material.
- **Prohibition**: Do not place binary mod distributions, extracted archives, or diagnostic dumps inside `references/sources/`.
- **Original mod exception**: User-supplied original files belong only in `references/local/conquest-of-skyrim/Data/`, outside Git, following the [local acquisition guide](../references/local/conquest-of-skyrim/README.md). Agents do not download/extract or redistribute the mod. Validate availability read-only; availability alone does not authorize implementation inspection while guardrails are unfinished. Generated analysis still belongs in `artifacts/`.

---

## 4. Reference Acquisition

### 4.1 Justify Before Acquiring
Before adding a reference, establish:
1. What research question does it answer?
2. Which milestone requires it?
3. What information is expected?
4. Is an existing reference sufficient?
5. Is a local copy necessary?
6. What is the approximate acquisition cost?
7. Are there relevant licensing or access restrictions?

Do not acquire repositories simply because they are related to Skyrim. Every persistent reference must have a clear research purpose.

### 4.2 Prefer Authoritative Sources
Prioritize sources in the following order:
1. Official project repositories.
2. Official technical documentation.
3. Original author publications.
4. Maintained community documentation.
5. Established open-source implementations.
6. Community discussions and secondary explanations.

Authority does not automatically guarantee correctness. Record relevant version differences and conflicting evidence.

### 4.3 Verify Identity
Before acquiring a repository:
- Verify its upstream URL.
- Verify the repository owner.
- Check whether it is official, maintained, forked, or archived.
- Inspect its stated purpose.
- Check the relevant version.
- Identify its license.
- Confirm its relevance.

Do not assume similarly named repositories are equivalent. Do not invent repository URLs.

### 4.4 Minimize Downloads
Prefer lightweight inspection before cloning:
- Read the README.
- Inspect repository structure.
- Review relevant source files online.
- Examine release history.
- Check documentation.
- Verify license information.

Clone only when local inspection provides meaningful value. Do not download releases, binaries, SDKs, or build dependencies merely to inspect source code.

---

## 5. Git Submodule Discipline

### 5.1 Canonical Location
All persistent external Git repositories belong under:

`references/sources/<repository-name>/`

**Naming Rule**: Project-chosen reference directory names directly under `references/sources/` (and `references/local/`) use lowercase letters/digits separated by single hyphens (`kebab-case`, e.g., `skyui-community`, `skse64`, `commonlib-sse`). Do not rename upstream files or nested submodules to impose this rule; their original layout must remain intact.

Do not create competing directories such as `external/`, `vendor/`, `third_party/`, `cloned/`, `github/`, `repositories/`, or `dependencies/`. Do not place Git repositories in `scripts/`, `tests/`, `docs/`, or `artifacts/`.

### 5.2 Adding Submodules
Use standard Git submodule operations:

```bash
git submodule add <verified-upstream-url> references/sources/<name>
```

Do not use placeholder URLs as executable commands. Verify the actual upstream before running the command. Git manages submodule metadata through `.gitmodules` and tracks the selected submodule commit. Do not manually simulate submodules by copying repository contents.

### 5.3 Revision Pinning
Every submodule must use a known revision. Record:
- Repository URL.
- Commit SHA.
- Relevant release or tag, if available.
- Acquisition date.
- Reason for selecting the revision.

Prefer stable releases when appropriate. Use development revisions only when they contain specifically required information. Do not automatically track the latest upstream branch. Do not update references merely because newer commits exist.

### 5.4 Reproducibility
Restore first-level references at their recorded commits from the project root, without automatically acquiring nested dependencies:

```bash
git submodule update --init -- references/sources/skyui-community references/sources/skse64
```

For a new workspace, clone the project normally using its verified URL, then run the targeted command above. This project currently has no configured remote; `<project-url>` is a template, not an established address.

Only when nested dependencies are justified and approved, the standard recursive alternatives are:

```bash
git submodule update --init --recursive
git clone --recurse-submodules <project-url>
```

Never execute placeholder URLs. Verify pinned commits are obtainable when acquisition/restoration is authorized; otherwise report that remote availability was not tested. Tags are descriptive; the exact commit SHA is the reproducibility key, even when a checkout is ahead of a tag. Do not assume the default branch represents the researched revision.

### 5.5 Submodule Updates
Update a submodule only when:
- A milestone requires newer information.
- The current revision contains a relevant defect.
- A newer version materially changes the investigation.
- The existing revision is unavailable or unsuitable.

Before updating:
1. Identify the current revision.
2. Identify the proposed revision.
3. Review relevant upstream changes.
4. Determine which findings may be affected.
5. Update the submodule.
6. Validate the new revision.
7. Update the reference inventory.
8. Commit the change.

Do not run bulk updates without a demonstrated need.

### 5.6 Submodule Isolation
Treat reference repositories as read-only research material. Do not:
- Modify upstream source files.
- Commit changes inside reference repositories.
- Create experimental branches.
- Generate build outputs inside references.
- Store research notes inside references.
- Place temporary scripts inside references.
- Modify upstream configuration for convenience.

If experimentation requires modifying a reference, create a separate isolated working copy outside the reference tree. Document its purpose and remove it when no longer required. The original reference must remain unchanged.

### 5.7 Dirty Submodules
Before committing, inspect submodule status. A dirty submodule must be investigated. Do not blindly reset or clean it. Determine whether changes are accidental, generated, preexisting, produced by another agent, or intentionally experimental. Never discard unknown changes without verifying ownership.

### 5.8 Removing References
A reference may be removed when:
- It is no longer relevant.
- Its information is duplicated by a better source.
- It cannot be used under applicable restrictions.
- Its maintenance cost exceeds its research value.

Before removal:
1. Identify dependent research documents.
2. Preserve necessary provenance.
3. Confirm no active investigation depends on it.
4. Remove it using appropriate Git operations.
5. Update the reference inventory.
6. Validate repository consistency.

Do not delete reference directories manually without considering Git submodule metadata.

---

## 6. Non-Git References

### 6.1 Documentation References
For online documentation, record:
- Title.
- Author or organization.
- URL.
- Relevant version.
- Access date.
- Relevant sections.
- Associated milestones.

Do not copy entire articles into the repository. Prefer linking to the original material.

### 6.2 Version-Specific Documentation
Skyrim modding documentation may differ across:
- Skyrim Special Edition vs. Anniversary Edition.
- Skyrim runtime versions (e.g., 1.5.97, 1.6.640, 1.6.1170).
- SKSE releases.
- Creation Kit versions.
- Papyrus environments.
- Modding tool versions.

Record the relevant version whenever it affects technical conclusions. Do not silently apply documentation from one runtime version to another.

### 6.3 Archived Sources
When original documentation is unavailable, archived copies may be useful. Record: original URL, archive URL, archive date, known differences, and reliability limitations. Do not treat an archived page as evidence of the current state of a project.

### 6.4 Community Sources
Community discussions can reveal undocumented behavior, known defects, compatibility problems, workarounds, implementation constraints, and version-specific observations. Distinguish community reports from verified technical facts. Preserve attribution and source context.

---

## 7. Provenance Discipline

Every significant reference must have traceable provenance. At minimum, record:
- Identity.
- Origin.
- Version or revision.
- Purpose.
- Relevant milestone.
- Access or acquisition date.

For Git repositories, record the exact commit SHA. For documentation, record the relevant version and URL. For local artifacts, follow `docs/testing.md`. Do not create a separate provenance database; use the Markdown inventory in `references/README.md`.

---

## 8. Licensing and Permissions

Before acquiring or using external material:
1. Identify the stated license or usage terms.
2. Record relevant restrictions.
3. Distinguish publicly accessible material from material licensed for reuse.
4. Identify uncertainty when permissions are unclear.
5. Escalate unresolved restrictions before actions that depend on permission.

Public availability does not imply unrestricted reuse. Do not assume every GitHub repository is open source. A repository without a license must not be treated as permissively licensed. Do not automatically copy external source code, assets, or documentation into project-owned files.

The project's detailed legal and clean-room boundaries will be defined separately in `guardrails.md`; do not invent them in a maintenance task. While it remains a placeholder, research is limited to public documentation and ordinary reference discovery; defer inspection or transformation of the original mod's implementation.

---

## 9. Research Usage

### 9.1 References Are Evidence Sources
External repositories help answer research questions. They do not automatically establish requirements for our recovered specification. Distinguish:
- What a reference implements.
- What the original mod implements.
- What Skyrim supports.
- What we infer from their relationship.

Do not treat behavior found in SKSE or another mod as proof that *Conquest of Skyrim* uses the same mechanism.

### 9.2 Traceability
When an investigation uses a reference, record reference identifier, relevant file or documentation section, revision, supported claim, and any limitations. Prefer precise references over vague statements (e.g., avoid "According to SKSE...").

### 9.3 Avoid Reference Contamination
Do not mix external source code with project-owned scripts. Do not create project files by copying large portions of reference implementations. Keep reference-derived technical observations distinguishable from independently established findings. Follow clean-room boundaries.

---

## 10. Parallel Agent Discipline

Multiple agents may inspect references concurrently:
- **Shared Read Access**: Agents may read the same pinned references. They must not modify them.
- **Acquisition Ownership**: Only the coordinating agent should add, update, or remove persistent submodules. Delegated agents may recommend references (including repository identity, verified upstream URL, purpose, milestone, expected value, known restrictions).
- **Avoid Concurrent Git Mutations**: Do not allow multiple agents to modify `.gitmodules`, submodule revisions, or the reference inventory concurrently. Serialize these operations. Parallel investigation is encouraged; parallel mutation of shared Git metadata is not.
- **Reference Consistency**: All agents should use the same pinned revisions for a given investigation unless version comparison is explicitly required. Record deviations; do not silently switch versions.

---

## 11. Storage Discipline

External repositories can consume substantial disk space. Before cloning:
- Estimate repository size when practical.
- Check whether shallow cloning is appropriate.
- Consider whether sparse checkout is sufficient.
- Determine whether full history is necessary.
- Avoid downloading binary releases unnecessarily.

Do not install toolchains or build reference projects merely to inspect their source. Do not download nested dependencies unless required. When using shallow or partial clones, document limitations affecting reproducibility or historical analysis. Do not automatically initialize nested submodules unless needed. Preserve the ability to restore the intended reference state.

---

## 12. Reference Inventory

Maintain `references/README.md` as a compact inventory using Markdown links and standard fields:
- Name
- Type
- Purpose
- Revision/Version
- License
- Milestones
- Upstream URL

Do not create a separate metadata file for every reference. Do not introduce YAML, JSON, TOML, or a database merely to catalogue references.

---

## 13. Integration With Existing Documents

The repository uses:
- **`docs/execution-discipline.md`**: Controls task execution, delegation, memory, checkpoints, review, and commits.
- **`docs/testing.md`**: Controls validation, scripts, generated artifacts, and cleanup.
- **`docs/logging.md`**: Controls diagnostic output, log formatting, and retention.
- **`docs/references.md`** (this document): Controls external source acquisition, provenance, versioning, and isolation.
- **`guardrails.md`**: Will define legal, licensing, clean-room, and research boundaries; currently an unfinished placeholder, not established permission.

Cross-reference these documents without duplicating their contents.
