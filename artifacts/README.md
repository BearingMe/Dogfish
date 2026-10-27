# Artifacts

This directory is the sole canonical location for all machine-generated outputs, intermediate analysis data, and technical experiment results produced during research.

## Purpose

Artifacts are outputs, not source code or permanent documentation. Examples include:
- Extracted metadata and parsed plugin records
- Intermediate analysis results and dumps
- Generated diagnostic reports and logs
- Temporary experiment outputs
- Machine-generated evidence files

## Retention Categories

1. **Temporary**: Intermediate outputs that can be freely regenerated. Removed after validation.
2. **Reproducible**: Outputs generated deterministically from documented inputs and scripts. Retained locally (Git-ignored) or regenerated as needed.
3. **Evidence**: Essential outputs required to substantiate research findings. Preserved intentionally with documented provenance and cross-referenced from research documents.

## Organization & Git Policy

- **Milestone Grouping**: Subdirectories (e.g., `artifacts/M07/`) should be created only when artifacts actually exist.
- **Ignored by Default**: All files in `artifacts/` are ignored by `.gitignore` by default. Only small, essential, verified evidence files may be tracked when justified.
