# Reference Inventory

This directory maintains the inventory of external research references, specifications, and source materials supporting the *Conquest of Skyrim* investigation.

Full reference management policy, acquisition criteria, and version pinning rules are defined in [`docs/references.md`](../docs/references.md).

## Directory Structure

- `sources/`: External Git repositories (managed via Git submodules) and isolated reference code.
- `README.md`: Authoritative catalog of all acquired references.

## Active References

| Name | Type | Purpose | Revision/Version | License | Milestones |
| :--- | :--- | :--- | :--- | :--- | :--- |
| [SkyUI-Community](https://github.com/doodlum/SkyUI-Community) | Git Submodule | UI framework, MCM Papyrus interfaces, and menu interaction reference | `b7a24a9` (v6.11 SE) | Open Source (GPLv3 derivation) | M05, M06, M12 |
| [skse64](https://github.com/ianpatt/skse64) | Git Submodule | Skyrim Script Extender (64-bit) core architecture, Papyrus native bindings, and engine hooks | `25b7235` (v2.3.1) | Custom Permissive / SKSE Team | M05, M06, M08 |

---

## Catalog Details

### [SkyUI-Community](https://github.com/doodlum/SkyUI-Community)
- **Purpose**: Reference for SkyUI interface architecture, Mod Configuration Menu (MCM) script implementations, ActionScript/Flash bindings, and UI integration hooks used by Skyrim mods including *Conquest of Skyrim*.
- **Upstream Location**: `https://github.com/doodlum/SkyUI-Community`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `b7a24a936647e815269e34f4899e4b09768eff29` (v6.11 SE, 2026-06-03)
- **License / Terms**: Open Source (GPLv3 upstream derivation)
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M12 (Player Interaction & Configuration)
- **Local Location**: `references/sources/SkyUI-Community`

### [skse64](https://github.com/ianpatt/skse64)
- **Purpose**: Authoritative reference for Skyrim Script Extender (64-bit) architecture, runtime memory addresses, native hooking patterns, Papyrus function extensions, and game version compatibility.
- **Upstream Location**: `https://github.com/ianpatt/skse64`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `25b72352adb6543fa6d0bd3795780672b2e238e0` (v2.3.1, 2026-09-06)
- **License / Terms**: Custom Permissive / SKSE Team (see `skse64_license.txt`)
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M08 (Runtime Lifecycle)
- **Local Location**: `references/sources/skse64`

---

## Inventory Template

When additional references are acquired during future milestones, catalog them in the table above and record detailed provenance using the template below:

### `[Reference Name](<upstream-url>)`
- **Purpose**: Targeted research question or functional necessity.
- **Upstream Location**: Canonical URL.
- **Reference Type**: Git Submodule | Online Documentation | Technical Excerpt.
- **Revision / Version**: Pinned commit SHA, release tag, or retrieval date.
- **License / Terms**: License identifier and usage restrictions.
- **Relevant Milestones**: Target milestone identifiers (e.g., M01, M07).
- **Local Location**: Path under `references/sources/<name>` (if applicable).
