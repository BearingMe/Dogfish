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

When adding a reusable script, document it below using this format:

### `script_name.ext` *(Template)*
- **Purpose**: Short description of what the script does.
- **Required Inputs**: Arguments, files, or environment parameters needed.
- **Invocation**: Example command line invocation.
- **Generated Outputs**: Destination file paths (typically in `artifacts/`).
- **Dependencies**: Minimal runtime requirements (e.g., standard library, specific CLI tools).
- **Relevant Milestone**: Target milestone identifier (e.g., M01, M07).
