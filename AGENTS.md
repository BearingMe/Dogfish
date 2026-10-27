# AI Agent Guide

## Project Identity & Purpose

**Dogfish** is an exploratory research project focused on *Conquest of Skyrim*, a Skyrim Special Edition mod by galvatron304 ([Nexus Mods #65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628)).

The project's purpose is to systematically investigate and understand the mod's strategic mechanics (faction management, territorial conquest, warfare, politics, and economy) and technical characteristics. This is strictly an exploratory research workspace, not an implementation project.

## Repository Structure

```text
├── AGENTS.md               # AI agent entry point and instructions
├── README.md               # Project overview and status
├── guardrails.md           # Research boundaries and rules (placeholder)
├── .gitignore              # Git ignore rules
├── scripts/                # Reusable research utilities
│   └── README.md
├── tests/                  # Automated validation and verification procedures
│   └── README.md
├── artifacts/              # Machine-generated outputs, dumps, and experiment data
│   ├── .gitignore
│   └── README.md
├── references/
│   ├── README.md           # Overview of reference materials
│   └── sources/            # Reserved for future reference repositories/sources
├── docs/
│   ├── milestones.md           # Research roadmap, milestone registry, and phase gates
│   ├── execution-discipline.md # Core execution, memory, and validation discipline
│   ├── testing.md              # Testing, script, and artifact management rules
│   ├── logging.md              # Logging standards, formats, and evidence capture
│   ├── references.md           # External reference acquisition, versioning, and isolation
│   └── glossary.md             # Domain and engine terminology placeholder
└── research/
    ├── findings/           # Consolidated, evidence-backed conclusions
    ├── investigations/     # Technical research topics and questions
    └── playtesting/        # Human gameplay observations and bug reports
```

## Organization of Research Material

Research is divided into two streams that feed into consolidated findings:

- **Technical Investigations (`research/investigations/`)**: Houses standalone technical research documents. Each investigation focuses on a specific question or subsystem (e.g., Papyrus scripting, dependencies, engine limits).
- **Playtesting Observations (`research/playtesting/`)**: Houses observations, reproduction steps for bugs, and empirical findings reported by human researchers during playtesting.
- **Consolidated Findings (`research/findings/`)**: Synthesizes verified conclusions supported by evidence from investigations or playtesting.
- **External References (`references/` and `references/sources/`)**: Stores source documentation, reference repositories, and external links.

## Execution Discipline

All agents must follow the full workflow defined in [`docs/execution-discipline.md`](./docs/execution-discipline.md). Key requirements:

- **Milestone Tracking**: Maintain all 32 milestones in [`docs/milestones.md`](./docs/milestones.md) through the `NOT STARTED`, `IN PROGRESS`, `BLOCKED`, `COMPLETE` lifecycle.
- **Capability Fallbacks**: Probe environment before use; apply documented fallbacks if optional tools (AgentMemory, Checkpoints, Orca, `@adversary`) are absent.
- **Separate Knowledge & Progress**: Store durable facts in knowledge storage (AgentMemory or `research/memory.md`); store operational progress in actionable checkpoints (`research/checkpoints/`).
- **Delegation & Concurrency**: Maintain explicit task boundaries and strict file ownership per worker.
- **Validation & Review**: Every finding requires concrete evidence. Use `@adversary` for independent review or explicitly labeled structured self-review (never impersonate independent review).
- **Definition of Done**: A task is complete only after validation passes, review is conducted, and changes are committed via Conventional Commits.

## Testing and Artifacts

Agents must adhere to the testing, script, and artifact rules in [`docs/testing.md`](./docs/testing.md). Specifically:

1. **Read `docs/testing.md`** before beginning any technical investigations or experiments.
2. **Use `scripts/`** for all persistent research utilities.
3. **Use `tests/`** for automated validation procedures.
4. **Use `artifacts/`** for all machine-generated outputs and dumps.
5. **Avoid competing output directories** (e.g., `output/`, `results/`, `dumps/`, `temp/`).
6. **Prefer existing tools** over authoring new scripts.
7. **Keep disposable experiments isolated** and clean up scratch files immediately.
8. **Preserve evidence provenance** by linking findings to specific artifact files and sources.
9. **Clean temporary files** before considering any investigation complete.
10. **Inspect Git status** before committing to ensure no unintended debris is tracked.

## Logging Discipline

Agents and scripts must adhere to [`docs/logging.md`](./docs/logging.md):

- **Standard Format**: Use `[TIMESTAMP][LEVEL][COMPONENT][MILESTONE] Message` with ISO 8601 UTC timestamps.
- **Severity Levels**: Use `DEBUG`, `INFO` (default), `WARN`, and `ERROR` appropriately; never silently suppress warnings.
- **Category Separation**: Separate operational execution logs, raw research evidence, and validated research findings.
- **Log Locations**: Direct all log files to `artifacts/` (e.g., `artifacts/M07/execution.log`); never write logs to root or `research/`.
- **External Engine Logs**: Extract concise, relevant excerpts with provenance (mod/game version, file, line) rather than committing multi-megabyte engine dumps.
- **Concurrency**: Assign separate log destinations to concurrent workers to avoid interleaved output.

## Reference Discipline

Agents must adhere to the reference management workflow in [`docs/references.md`](./docs/references.md). Specifically:

1. **Read `docs/references.md`** before acquiring external material.
2. **Keep persistent Git references** under `references/sources/`.
3. **Prefer pinned Git submodules** over ad-hoc clones.
4. **Verify upstream identity** and authority before acquisition.
5. **Document provenance and relevant licenses** for all materials.
6. **Treat reference repositories as read-only** research material.
7. **Avoid unnecessary downloads**, toolchains, or build dependencies.
8. **Keep generated outputs outside references** (route to `artifacts/`).
9. **Coordinate shared submodule modifications** through the coordinating agent.
10. **Update the reference inventory** in `references/README.md` when references change.

## Agent Guidelines & Tooling

- **Commit Style**: Always use Conventional Commits (`docs:`, `chore:`, `feat:`, `fix:`, etc.) for all git commits.
- **Reverse Engineering**: Probing with REA (Reverse Engineer Anything / reverse engineering everything) is allowed for inspecting and analyzing binaries, scripts, assets, and system behavior.

## Future Agent Instructions

Operational workflows and domain-specific agent instructions will be maintained in this file or added to `docs/` as the project develops.
