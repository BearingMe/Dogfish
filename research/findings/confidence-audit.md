# Confidence Reclassification: Conquest of Skyrim

- **Document**: Corpus Confidence Reclassification & Denominator Audit
- **Milestone**: M36 — Confidence Reclassification
- **Phase**: Phase 9 (Evidence Integrity and Provenance)
- **Target Scope**: Entire Research Corpus (M01 through M35)
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & RECLASSIFIED

---

## 1. Executive Summary & Calibration Mandate

Milestone M31 historically claimed:
> *"Across the 32 milestones and 28 completed investigations: CONFIRMED Findings: 94% of recovered specifications; OBSERVED Findings: 4%; INFERRED / UNKNOWN Findings: 2%."*

Milestones M34 and M35 identified that this metric suffered from **confidence inflation**:
1. **Lack of an Explicit Denominator**: The percentage lacked a transparent mathematical denominator distinguishing between static code presence and dynamic runtime verification.
2. **Conflation of Code and Gameplay**: Static Papyrus bytecode existence was conflated with verified in-game execution.
3. **The Playtesting Gap**: Claims labeled `OBSERVED` were drawn from third-party community videos and player reviews, despite `research/playtesting/` containing zero local playtest reports.

This document establishes a **dual-axis classification framework** that separates **Static Verification (Code/Record)** from **Runtime Verification (Gameplay Observation)**, eliminating ambiguity.

---

## 2. Dual-Axis Evidence Classification Framework

To represent technical knowledge truthfully, every feature and claim is evaluated along two orthogonal dimensions:

```text
                                 [Static Code Dimension]
                     UNVERIFIED                           VERIFIED
                         │                                    │
 [Runtime Dimension]     ▼                                    ▼
 UNTESTED ────────► [REPORTED]                           [CONFIRMED IN CODE]
                         │                                    │
                         ▼                                    ▼
 TESTED IN-GAME ──► [OBSERVED ANOMALY]                   [EMPIRICALLY CONFIRMED]
```

### 2.1 The Six Calibrated Evidence States
- **`CONFIRMED (STATIC)`**: Primary physical evidence directly verified in `conquerskyrim.esp` records or `Data/scripts/source/*.psc` Papyrus source code.
- **`OBSERVED (RUNTIME)`**: Demonstrated through a controlled, documented playtest experiment recorded in `research/playtesting/`. Currently **0 features** meet this threshold.
- **`INFERRED`**: A logical deduction or cross-system synthesis supported by evidence but not directly expressed as a single script variable.
- **`REPORTED`**: Documented in author release notes, Nexus descriptions, or community walkthroughs, but unverified at runtime or lacking full primary script coverage.
- **`INCOMPLETE / STUB`**: Documented or referenced in design notes, but verified as disabled, commented-out, or missing linking response scenes in v0.3.9.
- **`UNKNOWN`**: Mathematical formulas or engine heuristics that cannot be reconstructed from available files.

---

## 3. Mathematical Feature Inventory Audit ($N = 55$)

A systematic re-audit of all 55 feature entries in `research/findings/feature-inventory.md` establishes the following mathematically rigorous breakdown:

| Classification Category | Feature Count | Percentage ($N = 55$) | Category Criteria & Scope |
| :--- | :---: | :---: | :--- |
| **`CONFIRMED (STATIC)`** | **52** | **94.5%** | Discrete Papyrus scripts, FormIDs, and property bindings physically verified in package. |
| **`OBSERVED (RUNTIME)`** | **0** | **0.0%** | Zero features verified via local playtests (awaiting Phase 11 M41 playtesting program). |
| **`REPORTED (RUNTIME)`** | **52** | **94.5%** | In-game behavior reported by community walkthroughs; requires Phase 11 reconciliation. |
| **`INCOMPLETE / STUB`** | **2** | **3.6%** | `FEAT-FACT-04` (Doctrine multipliers unverified) and `FEAT-TECH-02` (Civil tech tree stub). |
| **`UNKNOWN CALCULATION`**| **1** | **1.8%** | `FEAT-BAT-04` (Off-screen AI vs. AI battle formula; detachment math known, general AI unknown). |
| **`TOTAL AUDITED`** | **55** | **100.0%** | Complete feature inventory. |

*Resolution of the "94%" Metric*: The historical "94% confirmed" metric accurately represented static code presence ($52 / 55 = 94.54\%$), but was misleadingly presented as full operational certainty. At runtime, 100% of gameplay behaviors remain to be empirically confirmed.

---

## 4. Reclassification of Disputed Claims

### 4.1 Phase 1 Reconnaissance (M01–M04)
- **Historical Label**: `CONFIRMED`
- **Reclassified Label**: **`REPORTED`**
- **Rationale**: Phase 1 was conducted before local mod files were present. Information derived from Nexus descriptions, Reddit announcements, and Patreon posts is high-credibility reported documentation, but cannot be classified as technically confirmed until validated against primary binaries in Phase 10.

### 4.2 Playtesting Claims (M11, M15, M16, M22, M29)
- **Historical Label**: `OBSERVED`
- **Reclassified Label**: **`REPORTED (COMMUNITY RUNTIME)`**
- **Rationale**: `research/playtesting/` contains zero reports. All gameplay accounts derive from third-party video playthroughs and player reviews. Reclassified pending the M41 playtest program.

### 4.3 Diplomatic Alliances (M23)
- **Historical Label**: Sometimes ambiguously referred to as "alliances" in player guides.
- **Audited Label**: **`CONTRADICTED / UNIMPLEMENTED`**
- **Rationale**: `CS_NegotiationScript` implements `TempTruce()` and `BreakTruce()`. The mod strictly implements bilateral non-aggression truces; co-belligerent military alliances do not exist.

### 4.4 The Moot Deadlock (M11, M28)
- **Historical Label**: Often described as an "unexplained random bug".
- **Audited Label**: **`CONFIRMED DEFECT IN CODE`**
- **Rationale**: `CS_MeetingScript.GetJarls()` requires non-null `JarlAliases[i]` for all 9 holds. Failing to recruit replacement Jarls deadlocks the quest in Papyrus bytecode.

---

## 5. Revalidation Priorities for Phases 10 and 11

The audit establishes three prioritized technical revalidation targets:

### Priority 1: Primary Structural & Binary Verification (Phase 10: M37–M40)
- **Goal**: Transition M01–M04 reported claims and M05–M28 static script analyses into reproducible, script-backed verifications.
- **Actions**:
  1. Write and commit reproducible inspection utilities in `scripts/` (e.g., `inspect_esp.py`, `audit_papyrus.py`).
  2. Implement automated regression assertions in `tests/`.
  3. Formally verify all 83 overridden vanilla FormIDs and record group counts.
  4. Verify exact mathematical formulas in Papyrus source code (taxation, production, conscription, bounty combat).

### Priority 2: Controlled Empirical Playtesting (Phase 11: M41–M42)
- **Goal**: Convert `REPORTED (COMMUNITY RUNTIME)` claims into genuine `OBSERVED` findings.
- **Actions**:
  1. Author `research/playtesting/test-plan.md` (M41) with structured, reproducible test cases.
  2. Provide clear test execution protocols for the human researcher.
  3. Reconcile runtime observations with static code expectations in M42.

### Priority 3: Adversarial Review & Final Integrity Gate (Phase 11: M43–M44)
- **Goal**: Deliver a final specification package (`docs/specification-integrity-report.md`) that explicitly delineates confirmed code, observed gameplay, reported claims, and known unknowns.
