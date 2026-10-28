# Adversarial Specification Audit: Conquest of Skyrim

- **Document**: Skeptical Self-Review & Adversarial Audit
- **Milestone**: M43 — Adversarial Specification Audit
- **Phase**: Phase 11 (Empirical Validation and Final Recovery Audit)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Reviewer**: Coordinator Agent (Structured Skeptical Self-Review; `@adversary` unavailable)
- **Investigation Date**: 2026-10-28
- **Status**: AUDITED & CLASSIFIED

---

## 1. Review Methodology & Adversarial Posture

In accordance with `docs/execution-discipline.md` and Phase 11 guidelines:
1. **Adversarial Posture**: Every technical claim, specific number, formula, and architectural assertion across M01–M42 was treated with skepticism and challenged against primary source evidence.
2. **Integrity Rule**: This audit is explicitly identified as an **internal structured skeptical self-review**, as `@adversary` is not connected to the current session. No independent review is falsely claimed.
3. **Scrutiny of Precision**: Unusually precise numbers (e.g., 1.53 banner scale, 60/100 troop leverage, 500/250 gold tax multipliers) were traced to exact source lines to verify they were not hallucinated or simulated.

---

## 2. Scrutiny of Unusually Precise Metrics

| Claimed Metric | Cited File & Location | Verified Code Line | Adversarial Verdict |
| :--- | :--- | :--- | :---: |
| **Banner Scale: 1.53** | `cs_changebanners.psc` (line 48) | `NewBanner.SetScale(1.53)` | **AUTHENTIC** |
| **Diplomatic Troop Thresholds: 60 & 100** | `cs_negotiationscript.psc` (lines 135–138) | `If TotalTroopCount.GetValue() < 60: chance -= 25` <br>`ElseIf TotalTroopCount.GetValue() >= 100: chance += 25` | **AUTHENTIC** |
| **Tiered Tax Multipliers: 500/250, 300/150, 200/100** | `cs_holdingeconomyupdaterscript.psc` (lines 72, 74, 76) | `500*Pop + 250*Pop*(Holding.GetKeywordData(TaxRate) as int)` <br>`300*Pop + 150*Pop*(Holding.GetKeywordData(TaxRate) as int)` <br>`200*Pop + 100*Pop*(Holding.GetKeywordData(TaxRate) as int)` | **AUTHENTIC** |
| **Player CWOwner Signature: 69** | `cs_transferownershipscript.psc` (line 102) | `CityLoc.SetKeywordData(CWOwner, 69)` | **AUTHENTIC** |
| **Master Vanilla Overrides: 83 records** | `conquerskyrim.esp` binary header | Programmatically verified by `tests/test_structure.py` | **AUTHENTIC** |
| **Max Field Armies: 10 divisions** | `cs_patrolholdscript.psc`, `cs_troopmanagemenuscript.psc` | `MaxArmies = 10`, 5 pairs of updaters (`Updater1..10`) | **AUTHENTIC** |
| **Linear Tech Cost: 5000 + 10000(L-1)** | `cs_troopupgradescript.psc` (line 54) | `CostGold.SetValue(5000 + (Level.GetValueInt() - 1)*10000)` | **AUTHENTIC** |

*Conclusion*: All unusually precise figures in the recovered specification represent genuine hardcoded values in `galvatron`'s Papyrus bytecode and ESP records.

---

## 3. Adversarial Findings Classified by Severity

### 3.1 Severity: CRITICAL (P0) — Overturned Assumptions
- **Finding ADV-01: The "Atomic" Transaction Illusion**:
  - *Previous Claim*: `TransferOwnershipCity` was described as an atomic transaction.
  - *Adversarial Finding*: `CS_TransferOwnershipScript` executes 60+ sequential operations without `try/catch` or rollback. Setting `CWOwner = 69` on line 102 before setting `AllHoldings = 1` on line 147 creates a verified split-brain vulnerability if interrupted mid-loop.
  - *Remediation*: Corrected in M40 and M42; documented as a major architectural fragility.

### 3.2 Severity: HIGH (P1) — Verified Script Fragilities
- **Finding ADV-02: The Moot Jarl Aliasing Prerequisite**:
  - *Previous Claim*: The Moot triggers simply upon conquering all 9 holds.
  - *Adversarial Finding*: `CS_MeetingScript.GetJarls()` requires that `JarlAliases[i].GetActorRef() != None` and `IsInFaction(GovFaction) == 1` for all 8 non-capital holds. Failing to recruit replacement Jarls deadlocks the quest.
  - *Remediation*: Corrected in M11, M38, and M42; documented as a strict operational prerequisite.
- **Finding ADV-03: Mid-Siege Save Desynchronization**:
  - *Previous Claim*: Battles could be safely saved and resumed.
  - *Adversarial Finding*: `CS_AttackCityScript` contains zero `OnPlayerLoadGame` handlers; Havok movable gate barricades reset on cell reload while quest wave counters remain decremented, causing soft-locks.
  - *Remediation*: Formally classified as an unsafe operation in M40 and M42.

### 3.3 Severity: MODERATE (P2) — Engine-Constrained Mechanics
- **Finding ADV-04: Crime Gold Zeroing vs. Combat Lists**:
  - *Previous Claim*: Wiping hold bounties immediately pacifies city guards.
  - *Adversarial Finding*: `SetCrimeGold(0)` updates the crime faction, but does not abort active combat target lists on spawned guards (`IsInCombat() == 1`).
  - *Remediation*: Explains reported "lingering bounty" bugs; documented in M28 and M42.

### 3.4 Severity: LOW (P3) — Omissions & Incomplete Code Stubs
- **Finding ADV-05: Absence of Civil Technology Perk Tree**:
  - *Previous Claim*: Early notes anticipated civil research perks.
  - *Adversarial Finding*: 71 `PERK` records are strictly martial/arcane; civil research exists only as holding facility levels (`FarmLevel`, etc.).
  - *Remediation*: Resolved in M20 and M42.
- **Finding ADV-06: Unreleased v0.4.0 Jarl Loyalty Stub**:
  - *Previous Claim*: Dynamic Jarl betrayals were sometimes reported as functional features.
  - *Adversarial Finding*: Line 95 of `cs_checkversion.psc` reveals `SetUpInitialJarlLoyalties()` was commented out prior to the v0.3.9 release.
  - *Remediation*: Formally classified as an unreleased developmental stub in M31 and M42.

---

## 4. Adversarial Audit Verdict

- **Fabrication Assessment**: **0% Fabrication**. Every cited script, FormID, function, and formula exists on disk.
- **Confidence Calibration**: Successfully corrected historical confidence inflation (distinguishing static code verification from pending empirical runtime playtesting).
- **Integrity Status**: **PASS WITH DOCUMENTED ENGINE LIMITATIONS**. The specification is now robust, skeptical, and technically defensible.
