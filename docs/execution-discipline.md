# Agent Execution Discipline

## 1. Overview & Purpose

This project is a research-oriented specification recovery effort focused on *Conquest of Skyrim*. The goal is systematic understanding through evidence-backed investigation, not ad-hoc experimentation or hasty implementation.

To maintain research rigor and continuity across sessions, all agents must adhere to this execution workflow. The discipline is designed to support:

- **Long-running investigations**: Ensuring complex inquiries remain structured and auditable over time.
- **Multiple concurrent agents**: Enabling parallel tracks without data corruption, race conditions, or conflicting edits.
- **Interrupted sessions**: Allowing work to be resumed seamlessly by any agent at any time.
- **Evidence-backed research**: Grounding every conclusion in verifiable citations, logs, scripts, or playtest observations.
- **Incremental progress**: Taking small, verified steps rather than speculative leaps.
- **Independent validation**: Verifying findings rigorously prior to consolidation.
- **Small, traceable Git commits**: Maintaining clean, reversible version history via Conventional Commits.

*Note: Do not begin research milestones during infrastructure and discipline setup.*

---

## 2. Capability Detection & Fallback Matrix

Before beginning any task, determine which tools and capabilities are active in the current session.

- **Do not assume optional capabilities exist.**
- **Do not invent tool names, commands, or integrations.**
- **If an optional capability is unavailable, immediately apply its documented fallback.**
- **Do not spend significant time attempting to install or repair optional tools.**
- Check exposed tools, documented integrations, and command availability. A tool name or installed executable alone does not establish a working integration; treat unavailable or unverified optional capabilities as unavailable for the current task and record the fallback in its checkpoint or completion report.
- On resume, read the roadmap and the relevant checkpoint first. Detect capabilities again; do not rely on a prior session's tool availability.

### Fallback Matrix

| Capability | Preferred | Fallback |
| :--- | :--- | :--- |
| **Persistent knowledge** | AgentMemory | Repository Markdown (`research/memory.md`) |
| **Execution state** | Checkpoints | Repository checkpoint files (`research/checkpoints/`) |
| **Parallel execution** | Orca | Sequential execution |
| **Independent review** | `@adversary` | Structured self-review |
| **Version control** | Git | Report inability to complete |

### Version Control Requirement

Git is mandatory for achieving the full Definition of Done (DoD). If Git is unavailable in the execution environment:
1. Complete all possible research, validation, and documentation.
2. Formally report the task as blocked before attempting to finalize or commit.

---

## 3. Memory Discipline

### 3.1 Separate Knowledge From Progress

Information must be strictly categorized into two separate concerns: durable knowledge versus active execution state.

#### Knowledge (AgentMemory / `research/memory.md`)

Stores durable facts, discoveries, and verified domain intelligence that will remain useful across future milestones.

**What to store:**
- Verified technical characteristics and engine behaviors.
- Confirmed gameplay mechanics and reproduction results.
- Important Creation Engine and Papyrus constraints.
- Discovered mod dependencies and integration hooks.
- Reusable research conclusions and confirmed invariants.
- Known failure modes, engine limits, and breaking conditions.

**What NOT to store:**
- Temporary task progress or scratchpad notes.
- Speculation or unverified hypotheses presented as fact.
- Entire documents or large file copies.
- Raw investigation dumps or verbose logs.
- Redundant or duplicate observations.
- Information already easily retrievable from repository files without loss of context.

*AgentMemory complements repository documentation; it does not replace it. The repository remains the authoritative research record.*

#### Execution State (Checkpoints / `research/checkpoints/`)

Records the immediate operational state of active work to enable handoffs and recovery from interrupted sessions.

Every checkpoint must directly and concisely answer eight questions:
1. **What milestone or task is being executed?**
2. **What has been completed?**
3. **What remains?**
4. **What is currently blocked?**
5. **What files were modified?**
6. **What evidence has been collected?**
7. **What validation remains?**
8. **What should the next agent do?**

Checkpoints must be concise and actionable. Do not turn checkpoints into research reports.

### 3.2 Checkpoint Lifecycle

Create or update a checkpoint at these specific inflection points:
- Before beginning a substantial milestone or task.
- After completing a meaningful unit of work.
- Before delegating work to another agent.
- After receiving and integrating delegated results.
- Immediately when encountering an unexpected blocker.
- Before ending an incomplete or paused session.

*Do not checkpoint every trivial action. A checkpoint should enable a subsequent agent to resume work without needing to reconstruct the prior conversation history.*

### 3.3 AgentMemory Fallback (`research/memory.md`)

When AgentMemory is unavailable, record nonredundant persistent facts in `research/memory.md` (create the file only when needed). No memory write is required when the task produces no new durable knowledge or the repository already preserves it without loss of context. Store concise facts with citations to authoritative research documents.

The following is an illustrative format only, not an established finding; its evidence path is a future deliverable:

```markdown
## Faction Initialization

Status: Confirmed

Finding:
Faction creation initializes a persistent faction identity.

Evidence:
research/findings/faction-creation.md

Related:
M09
```

### 3.4 Checkpoint Fallback (`research/checkpoints/`)

When native execution checkpoints are unavailable, record execution state under `research/checkpoints/` (create directory only when needed). Maintain one active file per milestone or task (e.g., `research/checkpoints/m01.md`) answering the eight checkpoint questions. These concise Markdown records are execution state, not generated analysis artifacts. Each worker owns its task checkpoint; the coordinator owns shared roadmap updates. Close the checkpoint when work concludes, recording remaining limitations and a next action (or none).

---

## 4. Delegation & Parallel Execution

When multiple agents operate in parallel or tasks are delegated:

### 4.1 Explicit Task Boundaries
- The delegating coordinator must specify explicit task boundaries: exact scope, required inputs, owned files and artifact destinations, expected deliverables, validation criteria, and handoff/blocker reporting. Use Orca only when its documented integration is available; otherwise execute these bounded tasks sequentially.
- Delegated agents/workers must remain strictly within their assigned boundary. They must not refactor unrelated files, begin unassigned investigations, or expand scope.

### 4.2 File Ownership Rules
- Parallel execution requires strict file ownership to avoid merge conflicts and data overwrites.
- Each concurrent worker has exclusive write ownership over its assigned files or directories for the duration of the task.
- No two agents may edit the same file concurrently.
- Shared files (`AGENTS.md`, `docs/milestones.md`, `docs/glossary.md`, READMEs, `.gitmodules`, and consolidation documents) have one coordinating owner. Integrate worker results at agreed handoff points.
- Only the coordinator stages, commits, changes branches, or mutates shared Git/submodule metadata in a shared working tree. Workers report their changes and validation results without committing. Serialize Git mutations; inspect the shared index before staging and commit only owned, reviewed changes.
- Give workers distinct output paths, including run-specific names when concurrent or repeated runs could overwrite evidence.

---

## 5. Validation & Review Discipline

Research records may contain open questions, hypotheses, reported behavior, and contradictions when clearly labeled and sourced. Review their evidence and classification before committing; do not present them as verified conclusions.

### 5.1 Evidence Standards
- All claims must cite verifiable evidence: specific file paths, line numbers, script names, memory offsets, log entries, or playtesting reproduction logs.
- Classify substantive claims as **CONFIRMED** (authoritative technical evidence or verified direct inspection), **OBSERVED** (documented experiment or human playtest), **INFERRED** (supported reasoning, not directly established), **REPORTED** (source claim without independent verification), **UNKNOWN** (insufficient evidence), or **CONTRADICTED** (incompatible evidence). Preserve contradictions and identify the investigated version; do not merge incompatible releases.
- Significant investigations record milestone, questions, sources, method, evidence, observations, interpretation, uncertainties, validation results, and related findings. Keep raw evidence in `artifacts/`, investigation records in `research/investigations/`, human reports in `research/playtesting/`, and consolidated conclusions in `research/findings/`.
- Confidence classifications describe claims; PASS / FAIL / BLOCKED / INCONCLUSIVE in `docs/testing.md` describe validation outcomes. Neither substitutes for the other.

### 5.2 Independent Review & Adversary Fallback
- **Preferred Review**: Independent review via `@adversary` to test findings critically, search for counter-evidence, and detect logical gaps.
- **Fallback Review**: Structured self-review when `@adversary` is unavailable.
- **Adversary Fallback Integrity Rule**: When executing a structured self-review, the agent **must not impersonate or pretend to be an independent reviewer or `@adversary`**. The review must be explicitly identified as an internal structured self-review.
- Review the actual diff/deliverables against the task and milestone gate; challenge evidence, confidence labels, version applicability, cross-document consistency, failure cases, and filesystem/Git ownership. Correct material issues, rerun affected checks, and report the review method, unresolved limitations, and actual validation results in the checkpoint or completion report.

---

## 6. Definition of Done (DoD)

A milestone, task, or unit of work is considered **DONE** only when all of the following conditions are satisfied:

1. **Boundary Met**: The work satisfies all assigned objectives without unauthorized scope expansion.
2. **Evidence Documented**: All findings are backed by concrete evidence and linked to authoritative sources.
3. **State & Knowledge Recorded**: Update checkpoints at meaningful boundaries and close task state before completion. Store knowledge only when it meets the nonredundancy rules in section 3.
4. **Validation Completed**: Run applicable checks and record actual outcomes. Required gates must be satisfied. A reproduced defect may be a FAIL experiment yet valid research evidence; BLOCKED or INCONCLUSIVE results are not silently relabeled PASS. A milestone may pass with explicit limitations only when available evidence is reasonably exhausted and remaining gaps do not invalidate required conclusions; otherwise leave it BLOCKED or IN PROGRESS.
5. **Review Completed**: Reviewed via `@adversary` (or explicit, non-impersonated structured self-review); material review issues addressed.
6. **Git Committed**: Commit intended changes using Conventional Commits, then verify status. No task-owned changes or unexplained debris remain; preserve and report unrelated user/worker changes. A no-change review needs no empty commit. A delegated worker's result is ready for integration, not a completed milestone, until the coordinator validates, reviews, and commits it.

Update `docs/milestones.md` only for research milestones, not infrastructure tasks. Record a completed milestone's commit after it exists, in the next normal tracking commit; a temporary `pending commit` marker is allowed while finalizing. Never require a commit to contain its own hash or make empty commits solely to chase that hash.

*If Git is unavailable, perform all possible validation/review and report completion as BLOCKED at step 6, including any other unmet gates.*

---

## 7. Git Hygiene & Incremental Commits

- Always use **Conventional Commits** (`docs:`, `chore:`, `feat:`, `fix:`, `refactor:`, `test:`).
- Commit incrementally as logical units of work are completed and validated.
- Inspect the initial and final status and both staged/unstaged diffs. Stage explicit owned paths, not unrelated work. Remove only task-owned disposable files after checking ownership; never broadly reset or clean shared work.
- Do not commit broken utilities or misrepresented claims. Reviewed partial research and labeled hypotheses/unknowns may be committed as meaningful progress without marking the milestone COMPLETE.
