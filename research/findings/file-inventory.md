# Distribution & Dependency Analysis: Conquest of Skyrim

- **Milestone**: M05 — Distribution and Dependency Analysis
- **Target Subject**: *Conquest of Skyrim*
- **Investigated Package**: `v0.3.9` ("Conquest of Skyrim - WIP Version", March 23, 2023 release)
- **Status**: CONFIRMED

---

## 1. Package Structure & Physical Inventory

*Conquest of Skyrim* v0.3.9 is distributed as an uncompressed loose-file mod archive (no `.bsa` container).

### 1.1 Summary Metrics
- **Total Files**: 3,779 files
- **Total Uncompressed Size**: 547,282,791 bytes (521.93 MB)
- **Plugin Container**: Single master-dependent plugin (`conquerskyrim.esp`, 4,991,035 bytes)
- **Localization**: Embedded in plugin (no standalone `.strings`, `.dlstrings`, or `.ilstrings` files)

### 1.2 Component Breakdown

```text
references/local/conquest-of-skyrim/Data/
├── conquerskyrim.esp        # Main plugin (4.99 MB, Form Version 44)
├── interface/               # UIlib Flash menu assets (3 files)
│   └── uilib/
│       ├── buttonArt.swf
│       ├── UILIB_1_ListMenu.swf
│       └── UILIB_1_TextInputMenu.swf
├── meshes/                  # 3D models and collision data (402 files)
│   ├── actors/              # FaceGen geometry (167 files)
│   └── conquest of skyrim/  # Banners, siege objects, war tables, drums (235 files)
├── music/                   # Custom parade music (1 file)
│   └── conquest of skyrim/
│       └── parade/drums of chaos.xwm
├── scripts/                 # Papyrus scripts (3,078 files total)
│   ├── *.pex                # Compiled bytecode scripts (1,539 files)
│   └── *.psc                # Source script files (1,539 files)
├── SEQ/                     # Start-game quest dialogue sequence table (1 file)
│   └── conquerskyrim.seq    # 552 bytes (ensures initial quest dialogue loads)
├── sound/                   # Custom SFX and battle drum audio (3 files)
│   └── fx/conquest of skyrim/drums/
│       ├── conquest_of_skyrim__parade_drums.xwm
│       ├── conquest_of_skyrim_battle_drums.xwm
│       └── conquest_of_skyrim_war_drum.xwm
└── textures/                # Diffuse, normal, and tintmask maps (290 files)
    ├── actors/              # NPC FaceGen tintmasks (167 files)
    ├── conquest of skyrim/  # Custom heraldry, flags, and UI textures (64 files)
    └── incaendo/            # Incaendo banner resource templates (59 files)
```

---

## 2. File Format Analysis

| Extension | Count | Format Description | Engine Usage |
| :--- | :---: | :--- | :--- |
| `.esp` | 1 | Bethesda Plugin Record Database | Defines quests, factions, dialogues, world edits, statics, and AI packages. |
| `.pex` | 1,539 | Compiled Papyrus Bytecode | Runtime logic executed by the Skyrim Papyrus Virtual Machine. |
| `.psc` | 1,539 | Papyrus Source Code Script | Uncompiled human-readable Papyrus source code supplied in the release. |
| `.nif` | 402 | NetImmerse / Gamebryo 3D Mesh | Static meshes (war tables, banners), siege apparatus, and actor FaceGen. |
| `.dds` | 290 | DirectDraw Surface Texture | Texture maps (BC1-BC7 compression) for banners, objects, and NPC faces. |
| `.xwm` | 4 | Bethesda Native Audio File | Compressed xWMA audio streams for battle drums and victory parades. |
| `.swf` | 3 | Shockwave Flash Movie | Scaleform GFx UI menus for text input and dynamic list selection. |
| `.seq` | 1 | Skyrim Engine Quest Table | Pre-cached voice/dialogue pointer table for start-game enabled quests. |

*Note: All formats are native to the Skyrim Special Edition engine ecosystem; no proprietary or unknown file containers are present.*

---

## 3. Plugin Architecture (`conquerskyrim.esp`)

Header inspection of `conquerskyrim.esp` via direct binary analysis:
- **Record Signature**: `TES4`
- **Header Data Size**: 209 bytes
- **Record Flags**: `0x00000000` (Standard `.esp` file; unflagged as ESL or ESM)
- **Form Version**: `44` (Strictly native Skyrim Special Edition 64-bit form version)
- **HEDR Version**: `1.70`
- **Total Stated Records**: `22,690` records
- **Next Object ID**: `0x00D3D879`
- **Author Field (CNAM)**: `DEFAULT`
- **Description Field (SNAM)**: `""` (Empty string)

### 3.1 Master File Dependencies (Load Order Prerequisites)
The `MAST` subrecords declare five strict master dependencies:
1. `Skyrim.esm` (Base Game)
2. `Update.esm` (Official Update)
3. `Dawnguard.esm` (DLC 1)
4. `HearthFires.esm` (DLC 2)
5. `Dragonborn.esm` (DLC 3)

*Finding: Conquest of Skyrim v0.3.9 requires the complete Special Edition DLC suite. It does not declare third-party `.esp` or `.esm` master dependencies in its header.*

---

## 4. Script Ecosystem & Naming Taxonomy

The 1,539 compiled scripts (`.pex`) and matching source files (`.psc`) follow a strict prefix naming convention:

| Prefix | Count | Description / Role |
| :--- | :---: | :--- |
| `cs_` / `cs__` | 1,359 | **Core Conquest Systems**: Faction management, hold patrols, war declarations, economy, and sieges. |
| `pf_` | 132 | **Package Fragments**: AI behavior scripts attached to patrol, parade, and guard AI packages. |
| `qf_` | 32 | **Quest Fragments**: Quest stage scripts handling startup, objectives, triggers, and completion. |
| `sf_` | 13 | **Scene Fragments**: Scripted dialogue scenes (e.g., coronation, court sessions, moot voting). |
| `prkf_` | 1 | **Perk Fragment**: Script attached to an authority or progression perk. |
| `uilib_` | 1 | **UI Library Interface**: Script binding Papyrus to the bundled Flash UI menus. |
| *(Other)* | 1 | Core auxiliary utility. |

---

## 5. Third-Party Dependencies & Integrated Libraries

### 5.1 Hard Runtime Requirements
- **SKSE64 (Skyrim Script Extender 64-bit)**:
  - *Requirement*: Mandatory.
  - *Technical Role*: Used for extended string operations, virtual keyboard text input, UI invocation calls, keypress listeners, and native array management.
  - *Reference*: Verified via `references/sources/skse64`.
- **SkyUI SE**:
  - *Requirement*: Mandatory.
  - *Technical Role*: Provides Mod Configuration Menu (MCM) registration and inventory menu hooks.
  - *Reference*: Verified via `references/sources/skyui-community`.

### 5.2 Embedded Libraries (Bundled In-Package)
- **UIlib (Version 1)**:
  - *Integrated Assets*: `interface/uilib/buttonArt.swf`, `UILIB_1_ListMenu.swf`, `UILIB_1_TextInputMenu.swf`, `scripts/uilib_1.pex`.
  - *Role*: Enables the player to type a custom faction name and choose heraldry from dynamic menu lists without opening external configuration programs.

### 5.3 Integrated Art Assets
- **Incaendo Banners**:
  - *Integrated Assets*: 59 texture files under `textures/incaendo/`.
  - *Role*: Standard heraldic patterns, sigils, and banner textures licensed from community banner creator Incaendo.

---

## 6. Installation & Environmental Assumptions

1. **Clean Save / Load Order Placement**:
   - Must load after all vanilla masters and SkyUI.
   - Must be installed on a save where the vanilla Civil War has either completed or has not progressed past the initial quest.
2. **Memory & Engine Constraints**:
   - Because the package distributes 3,000+ scripts and creates dozens of active quest aliases, it assumes the presence of **SSE Engine Fixes** to prevent FormID heap crashes.
3. **Voice Playback Engine**:
   - Voice files were remastered in v0.3.9 via ElevenLabs AI and SKVA Synth; uses `.xwm` audio format.
