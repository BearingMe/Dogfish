# Numerical Mechanics & Formula Verification: Conquest of Skyrim

- **Document**: Reproducible Mathematical Formula Verification Report
- **Milestone**: M39 — Numerical Mechanics and Formula Verification
- **Phase**: Phase 10 (Reproducible Technical Verification)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Verification Utility**: `scripts/verify_formulas.py`
- **Automated Test Suite**: `tests/test_formulas.py` (7/7 test suites passing)
- **Status**: VERIFIED & REPRODUCIBLE

---

## 1. Verification Scope & Method

In accordance with Phase 10 standards, all mathematical claims and simulation formulas recovered across M13–M24 were audited directly against uncompiled Papyrus source code (`.psc`). 

The utility `scripts/verify_formulas.py` and unit test suite `tests/test_formulas.py` independently execute and assert each equation across representative and edge-case values.

---

## 2. Reconstructed & Verified Mathematical Formulations

### 2.1 Tiered Regional Taxation (`CS_HoldingEconomyUpdaterScript.psc` lines 71–77)
$$\text{TreasuryPerDay} = (\text{Base} \times \text{Pop}) + (\text{Multiplier} \times \text{Pop} \times \text{TaxRate})$$

- **Major Walled Capitals (`citynum 0..4`: Markarth, Riften, Solitude, Whiterun, Windhelm)**:
  $$\text{TreasuryPerDay} = 500 \cdot \text{Pop} + 250 \cdot \text{Pop} \cdot \text{TaxRate}$$
  - *Test Case*: $\text{Pop} = 50, \, \text{TaxRate} = 2 \implies 500(50) + 250(50)(2) = 50,000 \text{ gold/day}$.
- **Minor Open Capitals (`citynum 5..8`: Dawnstar, Falkreath, Morthal, Winterhold)**:
  $$\text{TreasuryPerDay} = 300 \cdot \text{Pop} + 150 \cdot \text{Pop} \cdot \text{TaxRate}$$
  - *Test Case*: $\text{Pop} = 30, \, \text{TaxRate} = 1 \implies 300(30) + 150(30)(1) = 13,500 \text{ gold/day}$.
- **Dependent Villages & Forts (`holdingnum 9+`: Riverwood, Rorikstead, Forts)**:
  $$\text{TreasuryPerDay} = 200 \cdot \text{Pop} + 100 \cdot \text{Pop} \cdot \text{TaxRate}$$
  - *Test Case*: $\text{Pop} = 20, \, \text{TaxRate} = 2 \implies 200(20) + 100(20)(2) = 8,000 \text{ gold/day}$.
- **Audit Assessment**: **CONFIRMED IN CODE**. Exact integer arithmetic matches Papyrus lines 71–77.

---

### 2.2 Facility Employment Ceilings (`CS_HoldingEconomyUpdaterScript.psc` lines 11–15)
$$\text{FoodMax} = \text{FarmLevel} \times \text{FoodChange}$$
$$\text{MetalMax} = \text{MineLevel} \times \text{MetalChange}$$
$$\text{WoodMax} = \text{LumberMillLevel} \times \text{WoodChange}$$
$$\text{DocksMax} = \text{CS\_CityLevelDocks} \times \text{CS\_DocksChange}$$
$$\text{MaxJobs} = \text{FoodMax} + \text{MetalMax} + \text{WoodMax} + \text{DocksMax}$$

- *Test Case*: Farm 2 (Change 5), Mine 1 (Change 4), Mill 1 (Change 3), Docks 0:
  $$\text{MaxJobs} = 10 + 4 + 3 + 0 = 17 \text{ jobs}.$$
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

### 2.3 Dynamic Population Growth Equation (`CS_HoldingEconomyUpdaterScript.psc` lines 16–20)
$$\text{Growth Condition: } \text{Population} < \text{MaxJobs} + 0.25 \times \text{Population}$$
$$\text{Deficit} = \text{MaxJobs} - \lfloor 0.25 \times \text{Population} \rfloor$$
$$\text{PopIncrease} = \text{RandomInt}\Big(\lfloor 0.3 \times \text{Deficit} \rfloor, \, \lfloor 0.5 \times \text{Deficit} \rfloor\Big)$$

- *Test Case*: $\text{Pop} = 10, \, \text{MaxJobs} = 17 \implies 10 < 17 + 2.5 = 19.5 \text{ (Condition Met)}$.
  $$\text{Deficit} = 17 - \lfloor 2.5 \rfloor = 17 - 2 = 15$$
  $$\text{PopIncrease Range} = [\lfloor 0.3(15) \rfloor, \, \lfloor 0.5(15) \rfloor] = [4, \, 7] \text{ citizens.}$$
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

### 2.4 Military Conscription Draft (`CS_HoldingEconomyUpdaterScript.psc` lines 60–65)
$$\text{RecruitableTroops} = \lceil \text{CS\_RecruitablePercentage} \times \text{Unemployed} \rceil$$

- *Test Case*: $\text{Unemployed} = 11, \, \text{RecruitablePercentage} = 0.10 \implies \lceil 1.1 \rceil = 2 \text{ recruits}$.
- **Audit Assessment**: **CONFIRMED IN CODE**. Uses `math.ceiling()`.

---

### 2.5 Gross Material Production with Docks Multipliers (lines 78–80)
$$\text{GrossProduced} = 3 \times \Big( (\text{Level} \times \text{Change}) + \lfloor 0.3 \times \text{DocksChange} \times \text{DocksLevel} \rfloor - \text{JobsOpen} \Big)$$

- *Test Case*: Farm Level 3 (Change 8), Docks Level 3 (Change 10), JobsOpen 2:
  $$\text{DockBonus} = \lfloor 0.3 \times 30 \rfloor = 9$$
  $$\text{FoodProduced} = 3 \times (24 + 9 - 2) = 3 \times 31 = 93 \text{ Food.}$$
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

### 2.6 Technology Research Costs (`CS_TroopUpgradeScript.psc` lines 54–55)
$$\text{CostGold} = 5,000 + (\text{Level} - 1) \times 10,000$$
$$\text{CostMetal} = 50 + (\text{Level} - 1) \times 100$$

- *Progression*:
  - Tier 1 &rarr; 2: $5,000 \text{ gold}, \, 50 \text{ metal}$.
  - Tier 4 &rarr; 5: $35,000 \text{ gold}, \, 350 \text{ metal}$.
  - Tier 9 &rarr; 10: $85,000 \text{ gold}, \, 850 \text{ metal}$.
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

### 2.7 Diplomatic Treaty Probability (`CS_NegotiationScript.psc` lines 121–177)
$$\text{SuccessChance} = 25\% \pm 25\% (\text{Relationship}) \pm 25\%/50\% (\text{Ideology}) \pm 25\% (\text{Troop Leverage})$$
- *Troop Count Thresholds*:
  - $< 60 \text{ troops} \implies -25\%$
  - $\ge 100 \text{ troops} \implies +25\%$
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

### 2.8 Off-Screen Detachment Casualties (`CS_BountyScript.psc` lines 22–24, 53–60)
$$\text{ChanceOfVictory} = \frac{\text{TroopCounts}}{\text{TroopCountCertain}}, \quad \text{VictoryRoll} = \text{RandomFloat}(0, \, 1)$$

$$\text{If Victory: } \text{DeadTroops} = \lfloor (1 - (\text{Chance} - \text{Roll})) \times \text{TroopCounts} \rfloor$$

$$\text{If Defeat: } \text{DeadTroops} = \text{TroopCounts} \quad (\text{Total Wipeout})$$

- *Test Case*: $\text{Troops} = 80, \, \text{Certain} = 100, \, \text{Roll} = 0.4 \implies \text{Chance} = 0.8 > 0.4 \text{ (Victory)}$.
  $$\text{DeadTroops} = \lfloor (1 - (0.8 - 0.4)) \times 80 \rfloor = \lfloor 0.6 \times 80 \rfloor = 48 \text{ troops lost.}$$
- **Audit Assessment**: **CONFIRMED IN CODE**.

---

## 3. Unresolved & Unrecoverable Formulas

- **General Off-Screen Army Battle Resolution (`FEAT-BAT-04`)**:
  - The exact formula governing clashes between an Imperial field division and a Stormcloak field division without the player present remains **UNKNOWN**.
  - While `CS_BountyScript` establishes the detachment math above, `cs_tmscript.psc` handles off-screen hold attacks via abstracted keyword comparisons.
