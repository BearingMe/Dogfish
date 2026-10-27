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
- **Ignored by Default**: Generated files are ignored unless an exact path is explicitly approved for tracking.

## Retained Project Artwork

- **`dogfish-og-cover.png`**: User-approved repository cover and README banner, 1280 × 640 pixels. Generated from [`docs/dogfish-og-cover.svg`](../docs/dogfish-og-cover.svg) using installed headless Chromium; fonts use Cinzel, Inter, and JetBrains Mono with the SVG's fallbacks. Intentionally tracked through one exact ignore exception for public repository presentation, not research evidence. The SVG remains the editable source. Upload the PNG under GitHub repository Settings → General → Social preview to set the link-sharing image; committing it does not set that repository setting.
