# Evidence Traceability Audit: Conquest of Skyrim

- **Document**: Research Corpus Evidence Audit
- **Milestone**: M34 — Evidence Traceability Audit
- **Phase**: Phase 9 (Evidence Integrity and Provenance)
- **Target Scope**: All 32 Prior Milestones (M01 through M32)
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & CLASSIFIED

---

## 1. Audit Scope & Methodology

This audit evaluates the evidence traceability of all 32 prior milestones. Each milestone's deliverables, technical claims, cited sources, and confidence labels were audited against three criteria:
1. **Primary Source Verification**: Does the cited file, record ID, or line exist in the physical package?
2. **Evidence Quality Distinction**: Is the claim supported by direct primary inspection (code/record), secondary documentation (Nexus/Reddit), or runtime observation?
3. **Classification Integrity**: Were `CONFIRMED` and `OBSERVED` labels applied legitimately according to `docs/execution-discipline.md`, or was there confidence inflation?

---

## 2. Global Evidence Integrity Findings

### 2.1 The "Observed" Playtesting Fallacy
- **Finding**: Several milestones (M03, M11, M15, M16, M22, M29) labeled features as `OBSERVED`.
- **Audit Defect**: The directory `research/playtesting/` contains **zero playtest reports** (only `.gitkeep`).
- **Correction**: Gameplay claims drawn from third-party YouTube videos, Reddit reviews, or author notes are **`REPORTED`**, not project-`OBSERVED`. A claim can only be classified as `OBSERVED` when a documented test is recorded in `research/playtesting/` with reproduction steps and evidence.

### 2.2 Documentation Phase Confidence Inflation (M01–M04)
- **Finding**: Deliverables M01 through M04 carried top-level status headers of `CONFIRMED`.
- **Audit Defect**: M01–M04 were completed during Phase 1 prior to the physical presence of the local mod files, relying exclusively on Nexus Mods descriptions, Reddit threads, and community discussions.
- **Correction**: External web and community claims must be classified as **`REPORTED`**. They become `CONFIRMED` only when verified by primary binary/script inspection in Phase 10.

### 2.3 Script Citation Authenticity: 100% Traceable
- **Finding**: Across all deliverables, **33 unique Papyrus scripts** were cited.
- **Audit Verification**: All 33 cited scripts exist physically in `references/local/conquest-of-skyrim/Data/scripts/source/` with matching names and properties. Zero script names, functions, or properties were fabricated.

---

## 3. Milestone-by-Milestone Traceability Matrix (M01–M32)

| Milestone | Claim / Subject | Cited Source Reference | Primary Evidence Quality | Historical Label | Audited Status | Required Revalidation / Action |
| :--- | :--- | :--- | :--- | :---: | :---: | :--- |
| **M01** | Project Identity & History | Nexus #65628, Reddit | Secondary Web Sources | `CONFIRMED` | **`REPORTED`** | Verify version string against ESP header in M37. |
| **M02** | Documentation Inventory | Nexus, Bethesda.net, Reddit | Secondary Web Sources | `CONFIRMED` | **`REPORTED`** | Index is valid, but claims within docs remain reported. |
| **M03** | 51 Feature Inventory | Community guides, changelogs | Secondary Web Sources | `CONFIRMED` | **`REPORTED`** | Audit individual feature implementations in M38–M40. |
| **M04** | Known Issues & Conflicts | Nexus forums, Reddit crash reports | Secondary User Reports | `CONFIRMED` | **`REPORTED`** | Revalidate crash mechanisms in M40 & M42. |
| **M05** | Package Layout & Masters | `conquerskyrim.esp` TES4 | Primary Direct Inspection | `CONFIRMED` | **`CONFIRMED`** | Validated. 5 DLC masters, Form Version 44 verified. |
| **M06** | Engine Integration Groups | `conquerskyrim.esp` GRUPs | Primary Binary Inspection | `CONFIRMED` | **`CONFIRMED`** | Validated. 47 GRUPs, 238 QUST, 1,275 GLOB verified. |
| **M07** | Controller Architecture | 1,539 `.psc` files | Primary Code Parsing | `CONFIRMED` | **`CONFIRMED`** | Validated. 5 core controller quests verified. |
| **M08** | Runtime Lifecycle | `cs_checkversion.psc`, `cs_readstartbook.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Version migration steps verified in code. |
| **M09** | Faction Creation | `qf_cs_factiongovernment_*.psc`, `cs_selectname.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Container proxy naming verified in code. |
| **M10** | Territory State Machine | `cs_transferownershipscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. CWOwner 69 and globals 1/11/12 verified. |
| **M11** | The Moot & Coronation | `cs_meetingscript.psc`, `cs_coronationscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Code logic verified; in-game execution requires M41 playtest. |
| **M12** | Player Interactions | `cs_manageholding.psc`, `cs_mapmanagemenuscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Dialogue and war table scripts verified. |
| **M13** | Military Roster & Demographics | `cs_armytroopdeath.psc`, `cs_choosetroopracesscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. 10 divisions, class counts verified in code. |
| **M14** | Orders & Task Barrels | `cs_patrolholdscript.psc`, `cs_commandermovingscript.psc`| Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. TaskBarrel proxy architecture verified. |
| **M15** | Combat Simulation | `cs_attackcityscript.psc`, `cs_bountyscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | In-cell wave logic verified; off-screen math verified in bounty script. |
| **M16** | Siege Warfare | `cs_attackcitysiegescript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Staging markers, catapults, rams verified. |
| **M17** | Economic Production | `cs_holdingeconomyupdaterscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Mathematical equations verified in code. |
| **M18** | Population & Taxes | `cs_holdingeconomyupdaterscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Conscription and tiered tax math verified. |
| **M19** | Infrastructure Upgrades | `cs_upgradecityscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. 5 facility types, 4 tiers, AddJobs verified. |
| **M20** | Technology Perks | `cs_troopupgradescript.psc`, 71 PERKs | Primary Script & ESP | `CONFIRMED` | **`CONFIRMED`** | Validated. Linear cost equations and perks verified. |
| **M21** | Government & Spymaster | `cs_govpositions.psc`, `cs_intelreportscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Cabinet aliases, spy missions verified in code. |
| **M22** | Jarls & Stability | `cs_recruitjarl.psc`, `cs_loyaltycheckscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. Loyalty vs JarlLoyalty keywords verified. |
| **M23** | Diplomacy & Truces | `cs_negotiationscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. CalculateSuccess() formula verified in code. |
| **M24** | Law & Off-Screen Combat | `cs_courtscript.psc`, `cs_bountyscript.psc` | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. 7 crime factions, bounty casualty math verified. |
| **M25** | System Dependencies | Cross-script synthesis | Technical Inference | `CONFIRMED` | **`INFERRED`** | Cross-system feedback models are deductive synthesis. |
| **M26** | Persistence & Reload | `cs_checkversion.psc`, load scripts | Primary Script Analysis | `CONFIRMED` | **`CONFIRMED`** | Validated. OnPlayerLoadGame hooks verified in code. |
| **M27** | Vanilla Compatibility | Binary ESP override scan | Primary Binary Inspection | `CONFIRMED` | **`CONFIRMED`** | Validated. 83 overridden FormIDs verified in ESP. |
| **M28** | Failure Modes | Script inspection + crash logs | Hybrid Code & Reports | `CONFIRMED` | **`MIXED`** | Code causes CONFIRMED; runtime crash repros are REPORTED. |
| **M29** | Behavioral Specification | Master synthesis (M01–M28) | Consolidated Synthesis | `CONFIRMED` | **`MIXED`** | Static mechanics CONFIRMED; runtime playtest claims REPORTED. |
| **M30** | Recovered System Model | Conceptual architecture model | Descriptive Modeling | `CONFIRMED` | **`INFERRED`** | Architectural model is valid descriptive synthesis. |
| **M31** | Completeness Audit | Corpus audit | Internal Corpus Check | `CONFIRMED` | **`CONFIRMED`** | Validated. 51 features and unreleased stubs verified. |
| **M32** | Final Specification | Consolidated package | Master Package | `CONFIRMED` | **`MIXED`** | Master package inherits underlying claim classifications. |

---

## 4. Specific Unsupported Assertions Flagged for Revalidation

1. **"94% of Recovered Specifications are Confirmed"**:
   - *Defect*: Uncalibrated metric without an empirical denominator or distinct weighting between code presence and functional runtime verification.
   - *Action*: Replaced in M36 with rigorous classification based on source evidence type.
2. **"Observed in Gameplay"**:
   - *Defect*: Applied in M03, M11, M15, M16, and M29 without corresponding test runs in `research/playtesting/`.
   - *Action*: Reclassified to `REPORTED` (from community playthroughs) pending execution of the M41 playtesting program.
3. **M01–M04 "CONFIRMED" Status**:
   - *Defect*: Initial web reconnaissance findings were marked `CONFIRMED` before inspecting the local mod files.
   - *Action*: Reclassified to `REPORTED` documentation knowledge; verified against physical package in Phase 10.
