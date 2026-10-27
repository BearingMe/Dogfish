# Reference Inventory

This directory maintains the inventory of external research references, specifications, and source materials supporting the *Conquest of Skyrim* investigation.

Full reference management policy, acquisition criteria, and version pinning rules are defined in [`docs/references.md`](../docs/references.md).

## Directory Structure

- `sources/`: External Git repositories (managed via Git submodules) and isolated reference code. Directories must follow **lowercased-with-hyphen naming** (`kebab-case`).
- `local/`: Manually supplied, non-Git reference materials (e.g., original mod distribution). Untracked in Git.
- `README.md`: Authoritative catalog of all acquired references.

## Active References

| Name | Type | Purpose | Revision/Version | License | Milestones |
| :--- | :--- | :--- | :--- | :--- | :--- |
| [skyui-community](https://github.com/doodlum/SkyUI-Community) | Git Submodule | UI framework, MCM Papyrus interfaces, and menu interaction reference | `b7a24a9` (v6.11 SE) | Open Source (GPLv3 derivation) | M05, M06, M12 |
| [skse64](https://github.com/ianpatt/skse64) | Git Submodule | Skyrim Script Extender (64-bit) core architecture, Papyrus native bindings, and engine hooks | `25b7235` (v2.3.1) | Custom Permissive / SKSE Team | M05, M06, M08 |
| [conquest-of-skyrim](./local/conquest-of-skyrim/README.md) | Local Reference | Primary subject of investigation and specification recovery | v0.3.9 (March 2023) | Proprietary / galvatron304 | All (M01–M32) |

---

## Catalog Details

### [skyui-community](https://github.com/doodlum/SkyUI-Community)
- **Purpose**: Reference for SkyUI interface architecture, Mod Configuration Menu (MCM) script implementations, ActionScript/Flash bindings, and UI integration hooks used by Skyrim mods including *Conquest of Skyrim*.
- **Upstream Location**: `https://github.com/doodlum/SkyUI-Community`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `b7a24a936647e815269e34f4899e4b09768eff29` (v6.11 SE, 2026-06-03)
- **License / Terms**: Open Source (GPLv3 upstream derivation)
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M12 (Player Interaction & Configuration)
- **Local Location**: `references/sources/skyui-community`

### [skse64](https://github.com/ianpatt/skse64)
- **Purpose**: Authoritative reference for Skyrim Script Extender (64-bit) architecture, runtime memory addresses, native hooking patterns, Papyrus function extensions, and game version compatibility.
- **Upstream Location**: `https://github.com/ianpatt/skse64`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `25b72352adb6543fa6d0bd3795780672b2e238e0` (v2.3.1, 2026-09-06)
- **License / Terms**: Custom Permissive / SKSE Team (see `skse64_license.txt`)
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M08 (Runtime Lifecycle)
- **Local Location**: `references/sources/skse64`

### [conquest-of-skyrim](./local/conquest-of-skyrim/README.md)
- **Purpose**: Primary target mod of the research program and specification recovery effort.
- **Author**: galvatron304
- **Upstream Location**: `https://www.nexusmods.com/skyrimspecialedition/mods/65628`
- **Reference Type**: Local Reference (User-Supplied)
- **Revision / Version**: Version 0.3.9 ("Conquest of Skyrim - WIP Version", March 2023 release)
- **Acquisition Method**: Manual download from Nexus Mods by user; placed into `references/local/conquest-of-skyrim/Data/`
- **License / Terms**: Proprietary mod distribution / author permissions (Nexus Mods standard). Untracked in Git.
- **Relevant Milestones**: All milestones (M01 through M32)
- **Local Location**: `references/local/conquest-of-skyrim/Data/` (Git-ignored)

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
- **Local Location**: Path under `references/sources/<name>` or `references/local/<name>` (if applicable).
