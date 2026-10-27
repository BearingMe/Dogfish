# Local Reference: Conquest of Skyrim v0.3.9

This directory houses the manually supplied local reference files for **Conquest of Skyrim**, created by galvatron304.

The mod files are distributed via Nexus Mods and are not tracked in Git. Users must download and extract the files manually following the instructions below.

---

## 1. Source Information

- **Mod Name**: Conquest of Skyrim
- **Author**: galvatron304
- **Version**: 0.3.9
- **Release Period**: March 2023
- **File Name**: `Conquest of Skyrim - WIP Version`
- **Official Download URL**: [https://www.nexusmods.com/skyrimspecialedition/mods/65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628)
- **Format**: 7z archive (~547 MB uncompressed)

---

## 2. User Acquisition Guide

Follow these steps on your local machine to supply the reference files:

### Step 1: Download Manually
1. Open the official Nexus Mods page: [https://www.nexusmods.com/skyrimspecialedition/mods/65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628)
2. Navigate to the **Files** section.
3. Locate **Conquest of Skyrim - WIP Version** (Version **0.3.9**, released March 2023).
4. Download the archive manually. *(Do not use third-party mirrors or unofficial sources.)*

### Step 2: Extract the Archive
1. Open the downloaded `.7z` archive using 7-Zip, WinRAR, or another compatible tool.
2. Inside the archive, locate the top-level directory named `Conquest of Skyrim/`.
3. Verify the archive contains:
   - `interface/`
   - `meshes/`
   - `music/`
   - `scripts/`
   - `SEQ/`
   - `sound/`
   - `textures/`
   - `conquerskyrim.esp`

### Step 3: Place Files in `Data/`
1. In this directory (`references/local/conquest-of-skyrim/`), create a subfolder named `Data/` (if it does not already exist).
2. Copy the **contents** of `Conquest of Skyrim/` directly into `Data/`. Do not copy the enclosing folder itself.

#### Correct Directory Layout:
```text
references/local/conquest-of-skyrim/
├── README.md
├── .gitignore
└── Data/
    ├── interface/
    ├── meshes/
    ├── music/
    ├── scripts/
    ├── SEQ/
    ├── sound/
    ├── textures/
    └── conquerskyrim.esp
```

#### Incorrect Layout (Avoid Nested Folder):
```text
Data/
└── Conquest of Skyrim/          <-- WRONG: Do not nest the folder inside Data/
    ├── scripts/
    └── conquerskyrim.esp
```

*Rules: Keep the directory structure intact. Do not rename files. Do not modify scripts, plugins, or assets.*

### Step 4: Verify Installation
Verify that `references/local/conquest-of-skyrim/Data/conquerskyrim.esp` exists and that the subdirectories are present.

---

## 3. Troubleshooting

- **Missing `Data/`**: Create the directory `Data/` manually and extract the files inside it.
- **Incorrect Nesting**: If files are located at `Data/Conquest of Skyrim/...`, move the contents up one level so `conquerskyrim.esp` sits directly inside `Data/`.
- **Wrong Archive**: Ensure you downloaded version 0.3.9 (March 2023) from the main files section, not an older release or optional patch.
- **Incomplete Extraction**: If asset folders or `scripts/` are missing, re-extract the archive ensuring no extraction errors occurred.
- **Missing Plugin**: If `conquerskyrim.esp` is missing, verify the archive extraction completed fully.

---

## 4. Agent Detection Procedure

Before initiating any research task or milestone that depends on the original mod files, AI agents must perform this read-only inspection:

### Verification Protocol
1. Check whether `references/local/conquest-of-skyrim/Data/` exists.
2. Check whether `Data/conquerskyrim.esp` exists and is non-empty.
3. Check whether `Data/scripts/` exists and contains `.pex` script files.
4. Check whether expected asset directories (`meshes/`, `textures/`, `SEQ/`, etc.) exist.
5. Verify that files are readable.
6. Verify via `git status` that all files in `Data/` are ignored by Git.

### Status Classification

- **`AVAILABLE`**:
  - The `Data/` directory and required files (`conquerskyrim.esp`, `scripts/`, asset directories) are present, non-empty, readable, and properly ignored by Git.
  - *Action*: Proceed with read-only investigation. Never modify original files.
- **`INCOMPLETE`**:
  - Some expected folders or files are missing (e.g., `scripts/` is missing).
  - *Action*: Report the exact missing items to the user and reference the setup guide above.
- **`MISSING`**:
  - The `Data/` directory does not exist or is empty.
  - *Action*: Direct the user to the official Nexus link and this README guide. Continue only independent reconnaissance tasks that do not require local mod files.
- **`INVALID`**:
  - Files are present but nested incorrectly (e.g., `Data/Conquest of Skyrim/conquerskyrim.esp`).
  - *Action*: Explain the structural discrepancy to the user with the expected layout. Never automatically move, rename, or delete the user's files.

*Note: Agents must not infer version 0.3.9 merely because `conquerskyrim.esp` exists. Treat version as user-declared until confirmed via metadata inspection.*

---

## 5. Reference Isolation Rules

- `Data/` is strictly read-only and immutable.
- Never modify original mod files, scripts, or plugin records.
- Never output generated logs, dumps, or temporary scripts into `Data/`.
- Never commit mod files or binaries to Git.
