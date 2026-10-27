# Testing and Artifact Discipline

## 1. Overview & Repository Structure

This document establishes the testing, validation, script management, and artifact handling standards for the *Conquest of Skyrim* research project.

The workspace will encompass static analysis, script inspection, engine experiments, and human playtesting. To maintain reproducibility, traceability, and filesystem order across long-running investigations, all agents must adhere to strict filesystem boundaries.

### Canonical Locations

```text
├── scripts/            # Reusable research utilities and inspection tools
│   └── README.md       # Script registry and usage instructions
├── tests/              # Automated validation and verification procedures
│   └── README.md       # Test catalog and execution notes
├── artifacts/          # Machine-generated outputs, dumps, and experiment data
│   ├── .gitignore      # Ignores generated artifacts by default
│   └── README.md       # Retention categories and artifact rules
├── docs/               # Architecture, discipline, and domain documentation
│   ├── execution-discipline.md
│   └── testing.md
└── research/           # Human-readable research records and synthesis
    ├── findings/       # Consolidated, evidence-backed conclusions
    ├── investigations/ # Individual technical research investigations
    └── playtesting/    # Human gameplay logs and empirical bug reports
```

- **No Competing Directories**: Do not create redundant directories such as `tools/`, `utils/`, `output/`, `results/`, `dumps/`, `generated/`, `build/`, `tmp/`, or `scratch/`.
- **No Premature Placeholders**: Do not create placeholder scripts or tests before there is actual code to run.

---

## 2. Testing Principles

### 2.1 Evidence-Driven Validation

Every technical investigation or research document must define how its conclusions are validated. Results must be classified using one of four unambiguous outcomes:

- **PASS**: The stated validation criteria and expected behaviors were fully satisfied with verifiable evidence.
- **FAIL**: The observed result contradicted expected behavior, an invariant was violated, or an assertion failed.
- **BLOCKED**: Execution or validation could not proceed due to missing preconditions, external dependencies, or environmental limitations.
- **INCONCLUSIVE**: Execution occurred, but the gathered evidence was insufficient to definitively confirm or refute the hypothesis.

*Rule: Never report PASS merely because a script exited with status code 0. Validation requires confirming that the evidence actually supports the conclusion.*

### 2.2 Research Validation

When reviewing or finalizing research documents, verify:
1. **Source Attribution**: All technical assertions and claims cite specific sources (file, record ID, line, offset, or playtest log).
2. **Evidence Consistency**: Cited evidence directly and logically supports the stated conclusion.
3. **Version Specificity**: Mod version, game version, and environment details are explicitly recorded.
4. **Preserved Contradictions**: Conflicting observations are documented rather than smoothed over or discarded.
5. **Explicit Inferences**: Inferences, hypotheses, and deductions are explicitly labeled as such, distinct from observed facts.
6. **Milestone Completeness**: The core research questions of the milestone are addressed.
7. **Documented Unknowns**: Gaps in knowledge or unverified areas are explicitly listed.
8. **Reference Integrity**: All cross-references and links to local files and artifacts resolve correctly.

*Automate validation checks where practical, but avoid building elaborate testing frameworks for simple Markdown notes.*

### 2.3 Technical Validation

For technical experiments and static analysis tasks, the documentation must capture:
- **Objective**: Specific question or hypothesis being tested.
- **Environment**: Tooling versions, operating system, and runtime context.
- **Inputs**: Exact files, arguments, or data fed into the test.
- **Commands**: Exact commands executed to reproduce the result.
- **Expected Results**: Anticipated behavior or output based on theory.
- **Actual Results**: Measured, recorded outcomes.
- **Generated Artifacts**: Output file paths located in `artifacts/`.
- **Interpretation**: Meaning of the result in the context of the research question.
- **Limitations**: Factors that qualify the conclusion or constrain generalizability.

*Prefer deterministic, repeatable procedures. Do not confuse successful execution with successful investigation.*

### 2.4 Human Playtesting Validation

Human playtesting logs in `research/playtesting/` must include:
- **Test ID**: Unique identifier for the test session.
- **Related Milestone**: Target milestone (e.g., M02, M14).
- **Skyrim Version**: Exact build number (e.g., Skyrim SE 1.5.97, 1.6.640).
- **Mod Version**: Exact version of *Conquest of Skyrim* tested.
- **Relevant Load Order**: Active plugins affecting the test area.
- **Save State**: Starting save state description (e.g., new game, clean save post-conquest).
- **Preconditions**: In-game state prior to action (faction relations, active quests, player level).
- **Reproduction Steps**: Sequential, detailed steps taken by the player.
- **Expected Behavior**: What the mod or game was supposed to do.
- **Observed Behavior**: What actually occurred in-game.
- **Result Classification**: PASS, FAIL, BLOCKED, or INCONCLUSIVE.
- **Supporting Evidence**: Screenshots, Papyrus logs, save files, or diagnostic dumps.

*Fields that are genuinely irrelevant to a specific test may be omitted, but any missing critical data must be noted. Human observations must never be fabricated, simulated, or inferred by AI agents.*

---

## 3. Script Discipline

### 3.1 Scripts Have One Home (`scripts/`)

All persistent, reusable scripts belong in `scripts/`.
- Do not scatter scripts across the repository root, research folders, docs, or artifact directories.
- Do not create alternative script locations (`tools/`, `utils/`, `helpers/`, `experiments/`, `scratch/`, `temp_scripts/`, `scripts_old/`, `backup_scripts/`).

### 3.2 Prefer Existing Tools

Before writing a new script:
1. Verify whether an existing script in `scripts/` can be reused or extended.
2. Check whether an installed CLI tool (e.g., ripgrep, git, python standard tools, REA tools) can perform the check directly.
3. Determine whether a concise, read-only one-liner is sufficient.
4. Write a script only when it delivers distinct, repeatable value.

*Do not write a script merely because an interpreter is available. Choose the simplest suitable mechanism.*

### 3.3 One-Off and Disposable Scripts

When conducting rapid exploratory experiments:
- Execute one-off scripts in an isolated temporary location.
- Never place disposable scripts directly into root or research directories.
- Ensure outputs are directed strictly into `artifacts/` or temporary folders.
- Clean up disposable scripts and temporary scratch files immediately after the experiment concludes.
- If a one-off experiment proves repeatedly useful, refactor and promote it into `scripts/` with proper documentation.

### 3.4 Persistent Script Quality

Every script in `scripts/` must:
- Have a clear, descriptive filename indicating its function.
- Focus on a single responsibility.
- Accept explicit inputs via arguments or standard interfaces.
- Write outputs to explicit destinations in `artifacts/`.
- Avoid hardcoded machine-specific or user-specific paths.
- Operate read-only by default (never modify tracked repository files without an explicit flag).
- Return meaningful exit codes (0 for success, non-zero for errors).
- Report errors clearly to stderr.
- Rely on minimal external dependencies.

### 3.5 Script Documentation

All reusable scripts must be cataloged in `scripts/README.md` with:
- Purpose
- Required inputs
- Invocation example
- Generated outputs
- Dependencies
- Relevant milestone

### 3.6 Language & Runtime Discipline

Do not enforce an arbitrary single programming language prematurely. Python, PowerShell, Bash, or Node may be used where appropriate based on:
- Compatibility with target tools (e.g., REA, Bethesda file parsers).
- Maintenance overhead and standard library availability.
- Environment repeatability.

*Do not introduce third-party package managers, virtual environments, or heavy dependency trees unless an active investigation demonstrably requires them.*

---

## 4. Artifact Discipline

### 4.1 Artifacts Have One Home (`artifacts/`)

All machine-generated files, parsed data, diagnostic dumps, and intermediate outputs belong in `artifacts/`.
- Competing output directories (e.g., `output/`, `outputs/`, `results/`, `dumps/`, `generated/`, `build/`, `tmp/`, `temp/`, `backups/`) are strictly prohibited.
- `research/` directories must contain only interpreted, human-readable markdown documentation, never raw generated dumps.

### 4.2 Artifact Ownership & Metadata

Every retained artifact must have:
- A clear producing task, investigation, or script.
- An identified purpose.
- Traceable source data.
- A defined retention classification.

*Do not retain generated files simply because a script created them.*

### 4.3 Artifact Organization

- Group artifacts by milestone when applicable (e.g., `artifacts/M07/plugin-records.json`).
- Subdirectories must be created only when actual artifacts exist to populate them. Do not pre-create empty milestone directories.

### 4.4 Retention Categories

Classify every generated artifact:
1. **Temporary**: Intermediate, regenerable files. Deleted immediately after validation concludes.
2. **Reproducible**: Outputs deterministically regenerable via tracked scripts and documented inputs. Maintained locally, Git-ignored, and regenerated when needed.
3. **Evidence**: Essential outputs required to substantiate a documented research finding. Retained intentionally with provenance. Prefer concise, targeted excerpts over massive binary or text dumps.

### 4.5 Git Policy

- All files in `artifacts/` are ignored by default via `artifacts/.gitignore`.
- Do not commit temporary caches, raw debug dumps, duplicate logs, or large binaries.
- Small, essential, verified evidence files may be tracked in Git only with clear justification.
- Never use broad force-add (`git add -f artifacts/`) commands.

### 4.6 Artifact Cleanup Procedure

At the conclusion of each investigation:
1. Identify all files produced during the task.
2. Delete all temporary intermediate files.
3. Retain verified evidence in the proper milestone path under `artifacts/`.
4. Ensure retained evidence has documented provenance.
5. Check for accidental file creation outside `artifacts/`.
6. Inspect `git status` to ensure only intended changes remain.

---

## 5. Filesystem Discipline

### 5.1 No Directory Proliferation
Do not create new directories to house one or two isolated files. Use the existing canonical structure (`scripts/`, `tests/`, `artifacts/`, `docs/`, `research/`, `references/`). New directories require a demonstrated structural necessity.

### 5.2 No Backup Proliferation
Do not create manual backup copies of files (`file_old.md`, `file_backup.py`, `analysis_v2_final.md`). Use Git for version history.

### 5.3 No Untracked Debris
Before completing any task, inspect the repository with `git status`. Ensure no orphan scratchpads, temporary dumps, or untracked debris remain in the working tree.

### 5.4 Respect Reference Sources
The `references/` directory and `references/sources/` subdirectories house external materials and upstream sources. Never output generated artifacts, modify reference repos, or run ad-hoc scripts inside reference source directories.

---

## 6. Test Organization

### 6.1 Test Naming Conventions
Use descriptive, intent-revealing filenames:
- Good: `validate_research_links.py`, `test_record_parser.py`, `verify_glossary_refs.py`
- Prohibited: `test1.py`, `check.py`, `experiment_new.py`, `temp_test.py`

### 6.2 Test Scope & Determinism
Tests must validate a specific invariant or data consistency rule. They must run deterministically, require no undocumented local machine state, and remain isolated from unrelated tests.

### 6.3 Test Data
Small, reusable test fixtures should be located alongside the corresponding test in `tests/`. Large input sets belong outside the source tree. Generated test outputs must be directed to `artifacts/` or an isolated temporary directory.

### 6.4 Transparent Failure Reporting
A failing test must clearly output:
- Tested invariant or component
- Expected outcome vs. actual outcome
- Exact failing input or file path
- Location of diagnostic artifacts

*Never suppress test errors or silently skip assertions to simulate a passing result.*

---

## 7. Investigation & Validation Lifecycle

For every technical research milestone, follow these seven steps:

1. **Plan**: Identify the research question, required inputs, validation method, necessary scripts, and expected artifacts.
2. **Execute**: Run the investigation using existing tools and scripts where possible. Confine outputs strictly to `artifacts/`.
3. **Validate**: Execute applicable tests and verification procedures. Classify outcomes as PASS, FAIL, BLOCKED, or INCONCLUSIVE.
4. **Inspect**: Check the filesystem for unexpected directories, untracked debris, or modified references.
5. **Clean**: Remove disposable intermediate files while preserving verified evidence.
6. **Review**: Follow the review protocol in `docs/execution-discipline.md` (independent `@adversary` review or labeled structured self-review).
7. **Commit**: Verify the working tree is clean and commit changes using Conventional Commits.
