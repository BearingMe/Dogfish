# Reference Inventory

This directory maintains the inventory of external research references, specifications, and source materials supporting the *Conquest of Skyrim* investigation.

Full reference management policy, acquisition criteria, and version pinning rules are defined in [`docs/references.md`](../docs/references.md).

## Directory Structure

- `sources/`: Reserved for external Git repositories (managed via Git submodules) and isolated reference code.
- `README.md`: Authoritative catalog of all acquired references.

## Active References

*No external references or submodules have been acquired during the bootstrap phase.*

---

## Inventory Schema

When references are acquired during future milestones, document each entry using the following schema:

| Field | Description |
| :--- | :--- |
| **Reference Name** | Formal project, library, or document name |
| **Purpose** | Targeted research question or functional necessity |
| **Upstream Location** | Canonical repository URL or web documentation address |
| **Reference Type** | Git Submodule, Online Documentation, or Technical Excerpt |
| **Version / Revision** | Exact commit SHA, release tag, or retrieval timestamp |
| **License / Terms** | Upstream license (e.g., MIT, GPL, Proprietary) and restrictions |
| **Relevant Milestones**| Associated milestones (e.g., M01, M07) |
| **Local Location** | Local path under `references/sources/` or `references/` |

### Entry Template

```markdown
### [Reference Name]
- **Purpose**: [Brief explanation of research utility]
- **Upstream Location**: [URL]
- **Reference Type**: [Git Submodule | Online Documentation | Excerpt]
- **Version / Revision**: [Commit hash or version tag]
- **License / Terms**: [License identifier and usage terms]
- **Relevant Milestones**: [Milestone IDs]
- **Local Location**: `references/sources/[name]`
```
