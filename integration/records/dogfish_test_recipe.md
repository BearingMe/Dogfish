# Dogfish Minimal Test Plugin Record Recipe (`dogfish_test.esp`)

- **Milestone**: M78 — Papyrus Native Bridge and Test Plugin
- **Phase**: Phase 19 (Native Skyrim Integration Repair)
- **Plugin Name**: `dogfish_test.esp`
- **Format**: Form Version 44, ESL-flagged ESP (`ESP-FE`, load slot `0xFE`)
- **Master Dependency**: `Skyrim.esm`

---

## 1. Master Record Inventory

The minimal vertical slice requires exactly four records in `dogfish_test.esp`:

| Record Type | FormID | Editor ID | Description |
| :--- | :---: | :--- | :--- |
| **`TES4`** (Header) | `0x00000000` | Header | Form Version 44, Flag `0x0200` (ESL), Master: `Skyrim.esm`. |
| **`CELL`** (Cell) | `0xFE000800` | `DogfishTestCell` | Interior test cell cloned from `qasmoke` with interior ambient lighting. |
| **`ACTI`** (Activator)| `0xFE000801` | `DogfishClaimBannerActi` | Interactive banner using `clutter/banners/bannergeneric.nif` with script `DogfishClaimBannerScript`. |
| **`REFR`** (Reference)| `0xFE000802` | `DogfishClaimBannerREFR` | Placed reference of `DogfishClaimBannerActi` in `DogfishTestCell` at `(0.0, 0.0, 0.0)`. |
| **`QUST`** (Quest) | `0xFE000803` | `DogfishAnchorQuest` | Start Game Enabled quest initializing script hooks and event sinks. |

---

## 2. Step-by-Step Creation Kit / xEdit Recipe

### Method A: xEdit / SSEEdit (Recommended)
1. Launch `SSEEdit.exe`. Load only `Skyrim.esm`.
2. Right-click &rarr; **Create New Plugin (`dogfish_test.esp`)**.
3. In File Header (`TES4`):
   - Edit Record Flags: Check **`ESL`**.
4. In `CELL` category:
   - Create new cell `DogfishTestCell`.
   - Set Lighting Template to `DefaultLightingTemplate`.
5. In `ACTI` category:
   - Create new Activator `DogfishClaimBannerActi`.
   - Model: `Meshes\Clutter\Banners\BannerImperial01.nif`.
   - Add Script: `VMAD` subrecord pointing to `DogfishClaimBannerScript`, setting property `LocationID = 0`.
6. In `REFR` category under `DogfishTestCell`:
   - Place reference `DogfishClaimBannerActi` at coordinates $X=0, Y=0, Z=0$.
7. In `QUST` category:
   - Create new Quest `DogfishAnchorQuest`.
   - Flags: Check **Start Game Enabled**. Priority: 50.
8. Save and exit. The resulting `dogfish_test.esp` consumes slot `0xFE` and weighs less than 4 KB.

### Method B: Creation Kit 2.0
1. Open Creation Kit 2.0. Set active plugin to `dogfish_test.esp` with `Skyrim.esm` as master.
2. In Cell View, duplicate `qasmoke`, rename to `DogfishTestCell`.
3. In Object Window &rarr; WorldObjects &rarr; Activator, create `DogfishClaimBannerActi`.
4. Attach `DogfishClaimBannerScript.psc` and compile.
5. Drag into `DogfishTestCell` render window.
6. Create `DogfishAnchorQuest`, check Start Game Enabled.
7. Save plugin.
