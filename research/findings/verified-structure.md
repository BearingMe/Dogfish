# Binary & Structural Verification: Conquest of Skyrim

- **Document**: Reproducible Technical Verification Report
- **Milestone**: M37 — Binary and Structural Verification
- **Phase**: Phase 10 (Reproducible Technical Verification)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Verification Script**: `scripts/verify_structure.py`
- **Automated Test Suite**: `tests/test_structure.py` (6/6 tests passing)
- **Status**: VERIFIED & REPRODUCIBLE

---

## 1. Verification Scope & Execution Environment

In accordance with Phase 10 standards and the M35 compliance remediation plan, all structural and binary claims from M05–M08 were independently re-measured using the automated, reproducible verification utility `scripts/verify_structure.py`.

### Environment & Tool Metrics
- **Python Runtime**: Python 3.13 (Isolated Execution `python -I`)
- **Plugin Target**: `references/local/conquest-of-skyrim/Data/conquerskyrim.esp`
- **Execution Command**: `python -I scripts/verify_structure.py`
- **Automated Test Assertion**: `python -I tests/test_structure.py`

---

## 2. Package & Filesystem Measurements

Independent programmatic scanning of the extracted package directory yields the following verified metrics:

| Metric | Measured Value | Historical Claim (M05) | Discrepancy / Variance | Verification Status |
| :--- | :---: | :---: | :---: | :---: |
| **Total Files** | **3,779** | 3,779 | **0 (Exact Match)** | **CONFIRMED** |
| **Total Uncompressed Size** | **547,282,791 bytes** (521.93 MB) | 521.93 MB | **0 (Exact Match)** | **CONFIRMED** |
| **Compiled Papyrus Scripts (`.pex`)** | **1,539 files** | 1,539 | **0 (Exact Match)** | **CONFIRMED** |
| **Source Papyrus Scripts (`.psc`)** | **1,539 files** | 1,539 | **0 (Exact Match)** | **CONFIRMED** |
| **PEX / PSC 1:1 Match Ratio** | **100% (1,539 / 1,539)** | 100% | **0 (Exact Match)** | **CONFIRMED** |
| **3D NetImmerse Meshes (`.nif`)** | **402 files** | 402 | **0 (Exact Match)** | **CONFIRMED** |
| **DirectDraw Surface Textures (`.dds`)** | **290 files** | 290 | **0 (Exact Match)** | **CONFIRMED** |
| **Scaleform Menus (`.swf`)** | **3 files** | 3 | **0 (Exact Match)** | **CONFIRMED** |
| **Audio Streams (`.xwm`)** | **4 files** | 4 | **0 (Exact Match)** | **CONFIRMED** |
| **Quest Sequence Tables (`.seq`)** | **1 file** | 1 | **0 (Exact Match)** | **CONFIRMED** |
| **Plugin Databases (`.esp`)** | **1 file** | 1 | **0 (Exact Match)** | **CONFIRMED** |

---

## 3. Plugin Header & Master Invariants (`conquerskyrim.esp`)

Binary parsing of the `TES4` record confirms the following low-level structures:
- **File Size**: `4,991,035` bytes
- **SHA-256**: `2fe454bf986d2238130ab96a69e1f04e6797e2b2fbb9b0aa82f10ffd15b596e8`
- **Record Signature**: `TES4`
- **Form Version**: `44` (Strictly native 64-bit Skyrim Special Edition; confirmed non-LE)
- **Record Flags**: `0x00000000` (Standard standalone ESP, not ESL or ESM)
- **HEDR Header Version**: `1.70`
- **HEDR Stated Record Count**: `22,690` records
- **HEDR Next Object ID**: `0x00D3D879`
- **Author Tag (CNAM)**: `DEFAULT`
- **Declared Masters (MAST)**: Exactly 5 official DLC masters in strict sequence:
  1. `Skyrim.esm`
  2. `Update.esm`
  3. `Dawnguard.esm`
  4. `HearthFires.esm`
  5. `Dragonborn.esm`

---

## 4. Top-Level GRUP Signatures & Record Distribution

Top-level group scanning confirms **47 distinct GRUP signatures** in the plugin:

| Group Signature | Record Type | Data Size (Bytes) | Verification Role |
| :--- | :--- | :---: | :--- |
| `QUST` | Quest Controllers | 972,437 | 238 Quests (Campaign and state controllers). |
| `WRLD` | Worldspaces | 860,418 | Exterior world edits (siege markers and camps). |
| `DIAL` | Dialogues | 689,499 | Dialogue topics and responses. |
| `PACK` | AI Packages | 472,923 | 757 AI packages (patrols, formations, parades). |
| `NPC_` | Non-Player Characters | 394,120 | 1,126 Actor records (commanders, troops, bards). |
| `MESG` | Message Boxes | 298,776 | 513 UI Message boxes. |
| `CELL` | Cells | 244,403 | Interior palaces and dungeons. |
| `LCTN` | Locations | 162,728 | 168 Location records. |
| `FLST` | Form Lists | 145,269 | 1,213 FormLists (master holding registries). |
| `SCEN` | Scripted Scenes | 116,748 | 75 Scenes (coronation, hearings, surrender). |
| `GLOB` | Global Variables | 93,170 | 1,275 Globals (economy, battle size, war score). |
| `ACTI` | Activators | 92,099 | Interactive war maps and placed banners. |
| `FACT` | Factions | 22,175 | 183 Factions (crime, divisions, ranks). |
| `PERK` | Perks | 20,729 | 71 Perks (martial and arcane unit training). |
| `KYWD` | Keywords | 13,158 | 223 Keywords (CWOwner, Story Manager tags). |
| *(Other 32 GRUPs)*| Secondary Form Types | 129,568 | Spells, items, armors, audio descriptors. |

---

## 5. Master Override Audit (`Skyrim.esm`)

Direct binary scanning of FormIDs having master index `< 5` confirms **exactly 83 overridden vanilla records**:

- `LCTN`: 57 vanilla locations overridden (attaching `CWOwner = 69` and custom holding pointers)
- `WRLD`: 9 worldspace exterior cells modified (adding forward staging markers)
- `DIAL`: 7 dialogue topics injected (summit topics and surrender prompts)
- `PACK`: 3 native guard packages overridden
- `NPC_`: 2 native actors overridden (`0x0001336B` Tullius, `0x0010F9B9` Ulfric)
- `SMQN`: 2 Story Manager nodes overridden (`CWChangeLocationScenes`, `WIChangeLocationNodeA`)
- `QUST`: 1 master quest overridden (`0x00026F45` `CWCampaignObj`)
- `NAVI`: 1 navigation mesh info record modified
- `MSTT`: 1 movable static modified

---

## 6. Differences from Historical Claims

- **Zero Discrepancies**: All measurements made in M05, M06, and M27 were 100% accurate. The historical claims were fully reproduced to the exact byte and FormID.
- **Reproducibility Established**: What previously existed as one-off terminal heredocs is now permanently captured in `scripts/verify_structure.py` and regression-tested via `tests/test_structure.py`.
