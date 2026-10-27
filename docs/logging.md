# Logging Discipline

## 1. Overview & Objectives

This document establishes logging practices for the *Conquest of Skyrim* specification recovery project. It complements:
- `docs/execution-discipline.md` (Execution, memory, delegation, review, and commits)
- `docs/testing.md` (Validation, scripts, tests, and artifact management)

The goal is to ensure all automated tools, research scripts, experiments, and playtesting logs produce consistent, traceable, and easily interpretable diagnostic output without introducing unnecessary complexity.

### Core Objectives
1. **Useful Diagnostic Information**: Clear, actionable messages explaining operational context and outcomes.
2. **Consistent Formatting**: Standardized structure across all project-owned utilities.
3. **Reproducibility**: Sufficient execution details to allow independent reproduction of results.
4. **Low Output Volume**: High signal-to-noise ratio; concise default console output.
5. **Clear Artifact Ownership**: All log files reside in designated locations under `artifacts/`.
6. **Minimal Overhead**: Simple formatting conventions without external logging frameworks, databases, or telemetry services.

---

## 2. Logging Categories

Logging is strictly separated into three categories:

```text
Execution
   ↓
Logs / Raw Evidence
   ↓
Investigation
   ↓
Validated Findings
```

### 2.1 Execution Logs
Operational diagnostics that describe what a script or tool executed:
- Script initialization, configuration, and discovered inputs
- Processing milestones and progress indicators
- Validation checks, recoverable warnings, and fatal errors
- Completion status and summary statistics

*Execution logs are operational records; they are not research findings.*

### 2.2 Research Evidence
Factual, primary-source output gathered during an investigation:
- Extracted plugin metadata and record dumps
- Filtered Papyrus or Creation Engine diagnostic messages
- Console outputs, stack traces, and reproduction logs
- Measured values, memory offsets, and observed behaviors

*Evidence must retain enough context to substantiate research conclusions. Do not treat every routine log line as evidence worth preserving.*

### 2.3 Research Findings
Interpreted, validated conclusions synthesized from evidence.
- Consolidated findings belong in `research/findings/`; investigations and playtesting retain their own records. Final synthesis belongs in the M29–M32 documents under `docs/`, with links back to supporting findings.
- Never store findings exclusively in log files or console transcripts.

---

## 3. Log Format

### 3.1 Standard Format

All project-owned scripts and utilities must format log entries as:

```text
[TIMESTAMP][LEVEL][COMPONENT][MILESTONE] Message
```

**Example:**
```text
[2026-10-27T14:30:00Z][INFO][PLUGIN][M07] Found 142 quest records
```

- **TIMESTAMP**: ISO 8601 UTC timestamp (`YYYY-MM-DDTHH:MM:SSZ`).
- **LEVEL**: Severity level (`DEBUG`, `INFO`, `WARN`, `ERROR`).
- **COMPONENT**: Short, stable uppercase identifier.
- **MILESTONE**: Target milestone identifier (e.g., `M01`, `M07`). Use `GENERAL` if no specific milestone applies.
- **Message**: Concise, informative summary of the event.

### 3.2 Severity Levels

| Level | Purpose | Default State | Examples |
| :--- | :--- | :--- | :--- |
| **DEBUG** | Detailed diagnostic information and internal variable states. | Disabled by default; enabled via `--debug` or `--verbose`. | Individual record parsing steps, low-level loop states, raw buffer dumps. |
| **INFO** | Meaningful execution milestones and state transitions. | Enabled (Default). | Starting operations, input discovery, producing artifacts, validation success. |
| **WARN** | Unexpected but recoverable conditions. | Enabled; must never be silently suppressed. | Missing optional metadata, unsupported minor record types, skipped optional inputs. |
| **ERROR** | Failed operations, broken invariants, or unrecoverable issues. | Enabled; accompanied by clear failure diagnostics. | Missing required dependencies, corrupt input files, validation failures. |

*Rule: Do not log expected or normal conditions as ERROR. Do not silently suppress warnings.*

---

## 4. Component Identification

Use short, consistent uppercase component identifiers. Standard examples include:

- `PLUGIN`: Skyrim `.esp` / `.esm` / `.esl` record inspection and parsing.
- `PAPYRUS`: Papyrus script decompilation, analysis, or log review.
- `SKSE`: SKSE plugin, DLL, or native interface analysis.
- `RECORDS`: Specific Creation Engine record types (e.g., QUEST, FACTION, NPC).
- `VALIDATION`: Automated document or evidence validation runs.
- `EVIDENCE`: Evidence normalization, filtering, and extraction.
- `PLAYTEST`: In-game playtesting and observation logging.
- `SCRIPT`: General research scripts and automation utilities.
- `SYSTEM`: Environment checks, capability probing, or workspace setup.

*Keep component names stable and intuitive. Do not build an elaborate taxonomy.*

---

## 5. Console Output Discipline

### 5.1 Default Behavior & Quiet Operation
- Scripts should run quietly at `INFO` level by default, outputting only high-level progress and final status.
- Avoid terminal spam; do not output thousands of lines to the console during routine batch operations.
- Support standard flags where appropriate:
  - `--verbose` / `--debug`: Enables detailed `DEBUG` logging.
  - `--quiet`: Suppresses informational logs, outputting only `WARN` and `ERROR`.

### 5.2 Stream Separation
- **`stdout`**: Requested data/results and help text; never mix diagnostic progress with machine-readable pipeline output.
- **`stderr`**: Operational log messages at all enabled levels, including `INFO` progress and `DEBUG`, plus `WARN` and `ERROR`. A documented log-file destination may also receive them.
- Logging ERROR does not replace a meaningful failure exit code or the validation classification required by `docs/testing.md`.

---

## 6. Log File Management & Storage

### 6.1 Log Destinations (`artifacts/`)
All persistent log files must be written under `artifacts/`:
- Group logs by milestone when applicable: `artifacts/M07/execution.log`
- General script logs: `artifacts/logs/script_name.log`
- Temporary experiment logs: written to isolated temporary files or `artifacts/tmp/` and cleaned up after validation.
- **Never write project-generated log files to the repository root, `research/`, `docs/`, or `scripts/`.** External tools/game logs may originate outside the repository; read them in place or preserve permitted excerpts under `artifacts/`. Do not reformat original logs to the project format.
- Create log directories only when needed and use run-specific names to avoid overwriting prior evidence. Redact credentials and unrelated personal information before retaining excerpts; preserve relevant context and document redaction.

### 6.2 Log Retention & Git Policy
- By default, all log files under `artifacts/` are ignored by Git via `artifacts/.gitignore`.
- Do not commit large, raw execution logs or diagnostic dumps.
- If a log excerpt constitutes critical evidence supporting a finding, extract the minimal necessary snippet into a concise evidence file or embed the quote in the corresponding research finding.

---

## 7. External & Game Engine Logs

Investigations often consume logs generated by external tools (Skyrim engine logs, `Papyrus.0.log`, SKSE plugin logs, crash logger outputs).

### Handling External Logs:
1. **Never commit raw game logs**: External engine logs are frequently multi-megabyte files containing unrelated engine noise.
2. **Extract with Provenance**: When extracting evidence from game logs, record:
   - Source log filename and path
   - Timestamp and game/mod versions
   - Exact relevant lines or stack traces
   - Context explaining the observed behavior
3. **Save Extracted Excerpts**: Store targeted excerpts under `artifacts/<MILESTONE>/` with clear documentation.

---

## 8. Concurrency & Parallel Execution

When multiple agents or parallel processes execute concurrently:
- **No Shared Log Files**: Concurrent agents or processes must never write to the same log file simultaneously to prevent race conditions and interleaved text.
- **Per-Task / Per-Agent Logs**: Each worker must log to its own distinct log file (e.g., `artifacts/M07/worker-1.log`, `artifacts/M07/worker-2.log`) or write to isolated console streams.
- **Coordinating Aggregation**: If consolidation is needed, the coordinating agent collates and summarizes worker outputs after execution completes.
