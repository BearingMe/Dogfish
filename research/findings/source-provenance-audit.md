# Source Availability & Provenance Audit: Conquest of Skyrim

- **Document**: Primary Source Provenance Audit
- **Milestone**: M33 — Source Availability and Provenance Audit
- **Phase**: Phase 9 (Evidence Integrity and Provenance)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & VERIFIED

---

## 1. Executive Summary & Core Finding

A critical question was raised during repository review:
> *Why were local mod files reported as absent in earlier documentation (and why are they absent from GitHub), yet technical milestones (M05–M32) claim direct binary and script inspection?*

### The Verdict:
1. **Physical Local Presence**: The original mod package **is physically present and complete** on the local research machine at `references/local/conquest-of-skyrim/Data/`.
2. **Authentic Provenance**: The primary plugin `conquerskyrim.esp` bears an authentic author build timestamp of **March 23, 2023, 19:28:17 UTC** and cryptographic hash `SHA256: 2fe454bf986d2238130ab96a69e1f04e6797e2b2fbb9b0aa82f10ffd15b596e8`.
3. **Absence from GitHub Explained**: The mod files are absent from GitHub **solely because they are intentionally Git-ignored** via `references/local/conquest-of-skyrim/.gitignore` in strict compliance with the clean-room guardrails (`guardrails.md`), which forbid committing proprietary game binaries and assets.
4. **Historical Absence Resolved**: During Phase 1 (M01–M04), the files were indeed `MISSING` on disk. The files arrived locally at **2026-10-27 20:23:00 UTC**, at which point `guardrails.md` was established, and Phase 2 technical inspection began.
5. **Reproducibility Risk**: Any external researcher cloning from GitHub will find `Data/` missing by default and must follow `references/local/conquest-of-skyrim/README.md` to restore local physical files for replication.

---

## 2. Local Source Inventory & Cryptographic Identification

### 2.1 Primary Plugin (`conquerskyrim.esp`)
- **Path**: `references/local/conquest-of-skyrim/Data/conquerskyrim.esp`
- **File Size**: `4,991,035` bytes
- **File Modification Time (mtime)**: `2023-03-23 19:28:17` (matches official Nexus Mods release time)
- **File Creation Time (ctime)**: `2026-10-27 20:23:00` (arrival on research machine)
- **SHA-256 Digest**: `2fe454bf986d2238130ab96a69e1f04e6797e2b2fbb9b0aa82f10ffd15b596e8`
- **Header Structure**: `TES4` record, Form Version `44`, HEDR Version `1.70`, Next Object ID `0x00D3D879`, Stated Record Count `22,690`.
- **Master Files Declared (MAST)**:
  1. `Skyrim.esm`
  2. `Update.esm`
  3. `Dawnguard.esm`
  4. `HearthFires.esm`
  5. `Dragonborn.esm`

### 2.2 Package Completeness Audit
The extracted directory contains 3,779 files across 7 subdirectories (547,282,791 uncompressed bytes):

| Directory | Subdirs | Files | Primary Formats | Completeness Assessment |
| :--- | :---: | :---: | :--- | :--- |
| `Data/` (Root) | 7 | 1 | `.esp` (1) | **Complete**. Primary plugin present. |
| `Data/scripts/` | 1 | 1,539 | `.pex` (1,539) | **Complete**. Compiled Papyrus bytecode. |
| `Data/scripts/source/` | 0 | 1,539 | `.psc` (1,539) | **Complete**. Uncompiled human-readable Papyrus source. |
| `Data/meshes/` | 24 | 402 | `.nif` (402) | **Complete**. 167 FaceGen meshes, 235 custom static/furniture meshes. |
| `Data/textures/` | 39 | 290 | `.dds` (290) | **Complete**. 167 FaceGen tintmasks, 64 custom textures, 59 Incaendo banners. |
| `Data/interface/` | 1 | 3 | `.swf` (3) | **Complete**. Scaleform UIlib v1 menus (`buttonArt`, `ListMenu`, `TextInputMenu`). |
| `Data/music/` | 2 | 1 | `.xwm` (1) | **Complete**. Custom parade track (`drums of chaos.xwm`). |
| `Data/sound/` | 3 | 3 | `.xwm` (3) | **Complete**. Custom SFX (`parade_drums`, `battle_drums`, `war_drum`). |
| `Data/SEQ/` | 0 | 1 | `.seq` (1) | **Complete**. Dialogue pre-cache table (`conquerskyrim.seq`, 552 bytes). |

### 2.3 Script Source Correspondence (1:1 Ratio)
An exhaustive scan of `Data/scripts/` and `Data/scripts/source/` confirms:
- **Compiled Scripts (`.pex`)**: Exactly `1,539` files.
- **Source Scripts (`.psc`)**: Exactly `1,539` files.
- **Stem Mapping**: Exactly `1,539 / 1,539` stems match 1:1. Zero compiled scripts lack source code; zero source files lack compiled counterparts.

---

## 3. Reference Repositories Audit (Git Submodules)

Both external reference repositories are initialized, clean, and checked out to exact revisions:

| Submodule | Local Path | Pinned Commit SHA | Tag / Version | Upstream URL | Status |
| :--- | :--- | :--- | :--- | :--- | :---: |
| **`skse64`** | `references/sources/skse64` | `25b72352adb6543fa6d0bd3795780672b2e238e0` | `v2.3.1-3-g25b7235` | `https://github.com/ianpatt/skse64` | **Clean & Valid** |
| **`skyui-community`** | `references/sources/skyui-community` | `b7a24a936647e815269e34f4899e4b09768eff29` | `v6.11-2-gb7a24a9` | `https://github.com/doodlum/SkyUI-Community` | **Clean & Valid** |

---

## 4. Documentation vs. Inspection Contradictions

| Topic | Documented Claim in Repository | Physical Reality on Disk | Audit Assessment |
| :--- | :--- | :--- | :--- |
| **"Local mod is MISSING"** | Recorded in `references/README.md` and initial checkpoints during setup. | Files were placed into `Data/` at 2026-10-27 20:23 UTC and remained present throughout M05–M32. | **Timeline Discrepancy**. The note in `references/README.md` was authored during bootstrap before files were supplied, and was not refreshed after file placement. |
| **"No source code in package"** | Speculative assumption in early modding discussions that only `.pex` existed. | Full `.psc` source code is physically bundled under `Data/scripts/source/`. | **Confirmed Present**. Author intentionally bundled all source code in the WIP release. |
| **"Missing on GitHub"** | Cloned repositories on GitHub show empty `Data/`. | Files are excluded by `.gitignore` rule `/*`. | **Expected & Compliant**. Guardrail compliance prevents asset redistribution; not an evidence failure. |

---

## 5. Provenance Verdict & Replication Protocol

- **Authenticity**: The investigated package is confirmed to be the genuine **Conquest of Skyrim v0.3.9** (released March 23, 2023 by `galvatron304`).
- **No Simulation**: All record counts (22,690 records, 47 GRUP types) and script traces cited in M05–M32 originate from real, measured data on disk.
- **Replication Instruction for External Auditors**:
  1. Clone `BearingMe/Dogfish`.
  2. Download `Conquest of Skyrim - WIP Version` v0.3.9 from Nexus Mods (#65628).
  3. Extract contents into `references/local/conquest-of-skyrim/Data/`.
  4. Verify `conquerskyrim.esp` matches `SHA256: 2fe454bf986d2238130ab96a69e1f04e6797e2b2fbb9b0aa82f10ffd15b596e8`.
