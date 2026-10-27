# Reference Inventory

This directory maintains the inventory of external research references, specifications, and source materials supporting the *Conquest of Skyrim* investigation.

Full reference management policy, acquisition criteria, and version pinning rules are defined in [`docs/references.md`](../docs/references.md).

## Directory Structure

- `sources/`: Reserved for external Git repositories (managed via Git submodules) and isolated reference code.
- `README.md`: Authoritative catalog of all acquired references.

## Active References

| Name | Type | Purpose | Revision/Version | License | Milestones |
| :--- | :--- | :--- | :--- | :--- | :--- |
| *(None)* | — | *No external references or submodules acquired during bootstrap* | — | — | — |

---

## Inventory Template

When references are acquired during future milestones, catalog them in the table above and record detailed provenance using the template below:

### `[Reference Name](<upstream-url>)`
- **Purpose**: Targeted research question or functional necessity.
- **Upstream Location**: Canonical URL.
- **Reference Type**: Git Submodule | Online Documentation | Technical Excerpt.
- **Revision / Version**: Pinned commit SHA, release tag, or retrieval date.
- **License / Terms**: License identifier and usage restrictions.
- **Relevant Milestones**: Target milestone identifiers (e.g., M01, M07).
- **Local Location**: Path under `references/sources/<name>` (if applicable).
