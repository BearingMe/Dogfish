# Research Scripts

This directory is the sole canonical location for all persistent, reusable research utilities and scripts.

## Purpose

Scripts in this directory automate technical tasks, such as:
- Plugin record inspection and parsing
- Metadata extraction
- Evidence normalization and validation
- Report and diagnostic generation

## Rules for Scripts

- **Single Location**: Reusable research utilities belong in `scripts/`; automated validation/test entry points belong in `tests/` and may call those utilities. Do not create secondary script folders (e.g., `tools/`, `utils/`, `scratch/`).
- **Design**: Scripts should have a single responsibility, accept explicit inputs, specify explicit output paths, avoid machine-specific paths, and operate read-only by default.
- **One-off Experiments**: Temporary disposable scripts must run in isolated temporary locations and be cleaned up after use. Promote them here only if they become reusable.

## Script Registry

### `verify_structure.py`
- **Purpose**: Reproducibly inspects and verifies package file counts, uncompressed size, script 1:1 matching, ESP header, master dependencies, top-level GRUP counts, and vanilla record overrides.
- **Required Inputs**: Path to mod Data directory (`--data-dir`, defaults to `references/local/conquest-of-skyrim/Data`).
- **Invocation**: `python -I scripts/verify_structure.py [--output artifacts/structure-audit.json]`
- **Generated Outputs**: Formatted console metrics and optional structured JSON report.
- **Dependencies**: Python standard library (`hashlib`, `struct`, `pathlib`, `json`).
- **Relevant Milestone**: M05, M06, M37.

### `verify_papyrus.py`
- **Purpose**: Programmatically audits Papyrus script class inheritance, property bindings, event handlers, and function signatures across the 11 core controller scripts.
- **Required Inputs**: Path to uncompiled script source directory (`--source-dir`, defaults to `references/local/conquest-of-skyrim/Data/scripts/source`).
- **Invocation**: `python -I scripts/verify_papyrus.py [--output artifacts/papyrus-audit.json]`
- **Generated Outputs**: Formatted console verification and optional JSON report.
- **Dependencies**: Python standard library (`re`, `json`, `pathlib`).
- **Relevant Milestone**: M07, M08, M38.

### `verify_formulas.py`
- **Purpose**: Implements and calculates representative test cases for all mathematical formulas recovered from Papyrus sources (taxation, facility production, population growth, conscription, technology upgrade costs, treaty acceptance probability, and off-screen detachment casualties).
- **Required Inputs**: None (self-contained verification utility).
- **Invocation**: `python -I scripts/verify_formulas.py`
- **Generated Outputs**: Formatted console metrics and assertion verifications.
- **Dependencies**: Python standard library (`math`).
- **Relevant Milestone**: M17, M18, M20, M23, M24, M39.

### `script_name.ext` *(Template)*
- **Purpose**: Short description of what the script does.
- **Required Inputs**: Arguments, files, or environment parameters needed.
- **Invocation**: Example command line invocation.
- **Generated Outputs**: Destination file paths (typically in `artifacts/`).
- **Dependencies**: Minimal runtime requirements (e.g., standard library, specific CLI tools).
- **Relevant Milestone**: Target milestone identifier (e.g., M01, M07).
