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

When native AgentMemory is unavailable, record persistent facts in `research/memory.md` (create the file only when needed). Store concise facts with citations to authoritative research documents:

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

When native checkpoints are unavailable, record execution state under `research/checkpoints/` (create directory only when needed). Maintain one active file per milestone or task (e.g., `research/checkpoints/milestone-XXX.md`) answering the eight checkpoint questions.

---

## 4. Delegation & Parallel Execution

When multiple agents operate in parallel or tasks are delegated:

### 4.1 Explicit Task Boundaries
- The delegating coordinator must specify explicit task boundaries: exact scope, required inputs, expected deliverables, and validation criteria.
- Delegated agents/workers must remain strictly within their assigned boundary. They must not refactor unrelated files, begin unassigned investigations, or expand scope.

### 4.2 File Ownership Rules
- Parallel execution requires strict file ownership to avoid merge conflicts and data overwrites.
- Each concurrent worker has exclusive write ownership over its assigned files or directories for the duration of the task.
- No two agents may edit the same file concurrently.
- Shared index files (such as `docs/glossary.md`, `README.md`, or consolidation documents) must be modified only by the coordinating agent after workers conclude.

---

## 5. Validation & Review Discipline

No research conclusion may enter the repository without verification and review.

### 5.1 Evidence Standards
- All claims must cite verifiable evidence: specific file paths, line numbers, script names, memory offsets, log entries, or playtesting reproduction logs.
- Unverified assumptions must be explicitly labeled as hypotheses or open questions.

### 5.2 Independent Review & Adversary Fallback
- **Preferred Review**: Independent review via `@adversary` to test findings critically, search for counter-evidence, and detect logical gaps.
- **Fallback Review**: Structured self-review when `@adversary` is unavailable.
- **Adversary Fallback Integrity Rule**: When executing a structured self-review, the agent **must not impersonate or pretend to be an independent reviewer or `@adversary`**. The review must be explicitly identified as an internal structured self-review, systematically evaluating the work against requirements, edge cases, and evidence standards.

---

## 6. Definition of Done (DoD)

A milestone, task, or unit of work is considered **DONE** only when all of the following conditions are satisfied:

1. **Boundary Met**: The work satisfies all assigned objectives without unauthorized scope expansion.
2. **Evidence Documented**: All findings are backed by concrete evidence and linked to authoritative sources.
3. **State & Knowledge Recorded**: Durable facts are stored in knowledge memory; ongoing task state is checkpointed.
4. **Validation Completed**: All verification criteria and checks pass without errors.
5. **Review Completed**: Reviewed via `@adversary` (or explicit, non-impersonated structured self-review).
6. **Git Committed**: Changes are staged and committed using Conventional Commits with a clean working tree.

*If Git is unavailable, complete items 1–5, then report the task as blocked at step 6.*

---

## 7. Git Hygiene & Incremental Commits

- Always use **Conventional Commits** (`docs:`, `chore:`, `feat:`, `fix:`, `refactor:`, `test:`).
- Commit incrementally as logical units of work are completed and validated.
- Ensure the working tree is clean and uncommitted scratch files are removed.
- Never commit broken states, half-finished hypotheses, or unverified claims.
