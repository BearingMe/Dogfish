# Reference Inventory

This directory maintains the inventory of external research references, specifications, and source materials supporting the *Conquest of Skyrim* investigation.

Full reference management policy, acquisition criteria, and version pinning rules are defined in [`docs/references.md`](../docs/references.md).

## Directory Structure

- `sources/`: External Git repositories (managed via Git submodules) and isolated reference code. Project-chosen reference directory names use **lowercased-with-hyphen naming** (`kebab-case`); upstream contents are not renamed.
- `local/`: Manually supplied, non-Git reference material. Acquisition READMEs and ignore rules are tracked; original mod contents are ignored. `Data/` is currently MISSING (checked 2026-10-27); recheck before dependent work.
- `README.md`: Authoritative catalog of all acquired references.

## Active References

| Name | Type | Purpose | Revision/Version | License | Milestones |
| :--- | :--- | :--- | :--- | :--- | :--- |
| [skyui-community](https://github.com/doodlum/SkyUI-Community) | Git Submodule | UI/MCM reference; mod usage not yet verified | `b7a24a9` (`v6.11-2-gb7a24a9`) | Project-wide license unverified | M05, M06, M12 |
| [skse64](https://github.com/ianpatt/skse64) | Git Submodule | Script Extender bindings and engine interface reference | `25b7235` (`v2.3.1-3-g25b7235`) | Restrictions documented; full reuse terms unverified | M05, M06, M08 |
| [conquest-of-skyrim](./local/conquest-of-skyrim/README.md) | Local Reference (MISSING) | Primary research subject; user supplies original files | Target v0.3.9 (March 2023, task-supplied metadata) | Permissions unverified; no redistribution | All (M01–M32) |

---

## Catalog Details

### [skyui-community](https://github.com/doodlum/SkyUI-Community)
- **Purpose**: UI/MCM and ActionScript interface reference for future investigation. Its specific use by *Conquest of Skyrim* remains unverified; the reference is not proof of a dependency.
- **Upstream Location**: `https://github.com/doodlum/SkyUI-Community`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `b7a24a936647e815269e34f4899e4b09768eff29` (commit date 2026-06-03); `git describe` reports `v6.11-2-gb7a24a9`, not the exact v6.11 release.
- **Acquisition**: 2026-10-27, explicitly requested upstream; the existing checkout was pinned for reference, not selected as a verified mod-compatible release.
- **License / Terms**: Project-wide license unverified. No root license was identified; bundled tool/library licenses do not establish a license for the whole project. Do not assume GPLv3 applies to all content.
- **Nested Reference**: Already initialized `external/BethesdaCMakeModules` at `72fb6a3d67ef8974850aea7805030caaac4a807f`, from `https://github.com/Exit-9B/BethesdaCMakeModules.git` (upstream-declared build dependency, not needed for present research). Its `LICENSE` is present but not evaluated here. Leave it unchanged; do not initialize further dependencies by default.
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M12 (Player Interaction & Configuration)
- **Local Location**: `references/sources/skyui-community`

### [skse64](https://github.com/ianpatt/skse64)
- **Purpose**: Authoritative reference for Skyrim Script Extender (64-bit) architecture, runtime memory addresses, native hooking patterns, Papyrus function extensions, and game version compatibility.
- **Upstream Location**: `https://github.com/ianpatt/skse64`
- **Reference Type**: Git Submodule
- **Revision / Version**: Commit `25b72352adb6543fa6d0bd3795780672b2e238e0` (commit date 2026-09-06); `git describe` reports `v2.3.1-3-g25b7235`, not the exact v2.3.1 release.
- **Acquisition**: 2026-10-27, explicitly requested upstream; the existing checkout was pinned for reference, not selected as a verified mod-compatible release.
- **License / Terms**: `skse64_license.txt` explicitly excludes the Skyrim Online/Skyrim Together team from use. This note does not establish a general permissive grant; full reuse terms remain unverified.
- **Relevant Milestones**: M05 (Dependencies), M06 (Creation Engine Integration), M08 (Runtime Lifecycle)
- **Local Location**: `references/sources/skse64`

### [conquest-of-skyrim](./local/conquest-of-skyrim/README.md)
- **Purpose**: Primary target mod of the research program and specification recovery effort.
- **Author**: galvatron304
- **Upstream Location**: `https://www.nexusmods.com/skyrimspecialedition/mods/65628`
- **Reference Type**: Local Reference (User-Supplied)
- **Revision / Version**: Target version 0.3.9 ("Conquest of Skyrim - WIP Version", March 2023), supplied by the acquisition task, not independently verified against a local package.
- **Distribution / Acquisition**: 7z archive, manually downloaded/extracted by the user only. Acquisition pending; no local `Data/` exists as of 2026-10-27. Follow the linked acquisition guide.
- **License / Terms**: Author permissions not yet verified. Public availability is not a reuse grant; do not download, commit, or redistribute original files as an agent.
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
