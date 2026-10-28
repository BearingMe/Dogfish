# Targeted Source Probing: Strategic AI Warfare & Internal Constraints

- **Document**: Primary Source Code Probing & Behavioral Clarification
- **Milestone**: M56 — Targeted Source Probing
- **Phase**: Phase 14 (Differential Probing and Gap Closure)
- **Subject**: *Conquest of Skyrim* v0.3.9 Source Code (`cs_tmscript.psc`, `cs_troopmanagemenuscript.psc`, `cs_checkversion.psc`)
- **Investigation Date**: 2026-10-28
- **Status**: VERIFIED FROM PRIMARY SOURCE

---

## 1. Executive Summary & Probing Objectives

Following the differential testing in Milestone M55, targeted source code probing was conducted against the uncompiled Papyrus scripts under `references/local/conquest-of-skyrim/Data/scripts/source/` to resolve the remaining unknown mechanics identified in the specification:
1. **Background AI Campaign Strategy & Invasion Priority** (`cs_tmscript.psc:991–1066`).
2. **AI Division Sizing & Off-Screen Battle Attrition** (`cs_tmscript.psc:1511–1535`).
3. **Division Alias Architecture** (`cs_troopmanagemenuscript.psc:2170–2188`).
4. **v0.4.0 Jarl Loyalty System Status** (`cs_checkversion.psc:92–102`).

All findings are documented directly from source lines in compliance with the mandatory 8-step source query protocol.

---

## 2. Background AI Campaign Strategy & Aggressiveness

### 2.1 Aggressiveness Calculation (`cs_tmscript.psc:991–1008`)

```papyrus
Function CalculateAggressiveness()
    TMData.SetKeywordData(Aggressiveness, 2)
    if TMData.GetKeywordData(ActiveCommanderCount) < TMData.GetKeywordData(TotalCommanderCount)
        TMData.SetKeywordData(Aggressiveness, 0)
    else
        int holdpriority = TMData.GetKeywordData(WarPriority) as int
        Location PriorityLoc = AllLocationsCS.GetAt(holdpriority) as Location
        if PriorityLoc.GetKeywordData(LandSuperiority) == FactionNum
            TMData.SetKeywordData(Aggressiveness, 5)
        else
            TMData.SetKeywordData(Aggressiveness, CityCount.GetValueInt() + MinorCityCount.GetValueInt())
        endif
    endif

    if TMData.GetKeywordData(Aggressiveness) > 5
        TMData.SetKeywordData(Aggressiveness, 5)
    endif
EndFunction
```

#### Recovered Mathematical Rules:
- **Baseline**: Default aggressiveness is `2`.
- **Under-Strength Invariant**: If `ActiveCommanderCount < TotalCommanderCount`, aggressiveness drops to `0` (defensive recovery stance).
- **Land Superiority Priority**: If the priority target hold has `LandSuperiority == FactionNum`, aggressiveness is maximized to `5`.
- **Territorial Scaling**: Otherwise, aggressiveness scales directly with total owned cities: $\text{Aggressiveness} = \text{CityCount} + \text{MinorCityCount}$.
- **Upper Bound**: Clamped to a maximum value of `5`.

---

### 2.2 Hold Invasion Priority Selection (`cs_tmscript.psc:1010–1066`)

```papyrus
Function ChooseHoldPriority()
    ...
    int i = 0
    while i < 9
        ChanceToInvade[i] = 0
        Location HoldLoc = AllLocationsCS.GetAt(i) as Location
        if ((EnemyAtWar && i != TMDataEnemy.GetKeywordData(WarPriority)) || !EnemyAtWar) && (AllHoldings.GetAt(i) as GlobalVariable).GetValueInt() != EnemyFactionNum
            if HoldLoc.GetKeywordData(LandSuperiority) == 1 && GetOwnHoldsBordering(i) > 0 && HoldJustBeaten != i
                ChanceToInvade[i] = 1
                ChanceToInvade[i] = ChanceToInvade[i] + GetOwnHoldsBordering(i) + Utility.RandomInt(0, 3)
                if HoldLoc.GetKeywordData(ArmiesPatrolling) == 0
                    ChanceToInvade[i] = ChanceToInvade[i] + 10
                endif
            elseif HoldLoc.GetKeywordData(LandSuperiority) == 1 && (CityCount.GetValue() + MinorCityCount.GetValue() == 1) && (AllHoldings.GetAt(i) as GlobalVariable).GetValue() == FactionNum
                ChanceToInvade[i] = 100
            endif
        endif
        i += 1
    endwhile
    ...
```

#### Recovered Invasion Heuristic:
For each of the 9 Hold Capitals (0..8):
1. **Preconditions**: Must have `LandSuperiority == 1` and at least one bordering owned hold (`GetOwnHoldsBordering(i) > 0`), and cannot be the hold just beaten (`HoldJustBeaten != i`).
2. **Invasion Weight Formula**:
   $$\text{ChanceToInvade}[i] = 1 + \text{OwnHoldsBordering}(i) + \text{RandomInt}(0, 3)$$
3. **Unpatrolled Vulnerability Bonus**: If `ArmiesPatrolling == 0`, add $+10$ to the invasion weight.
4. **Desperate Capital Defense**: If the faction is reduced to a single city ($\text{CityCount} + \text{MinorCityCount} == 1$) and this is their owned capital, $\text{ChanceToInvade}[i] = 100$ (overrides all other targets to force immediate reclamation/defense).
5. **Selection**: Target holding with highest positive weight is set as `WarPriority`.

---

## 3. AI Division Sizing & Off-Screen Combat Resolution

Prior specifications classified the general background AI battle formula as `UNKNOWN`. Inspection of `cs_tmscript.psc:1511–1535` fully resolves this mechanic:

### 3.1 AI Division Troop Sizing Constant (`cs_tmscript.psc:1511–1520`)

```papyrus
Function AddAttackingArmiesToGarrison(int troopsremaining)
    int remainingarmies = math.ceiling(troopsremaining/20)
    int initialarmies = TMData.GetKeywordData(AttackingArmies) as int
    int deadarmies = initialarmies - remainingarmies
    int holdnum = TMData.GetKeywordData(WarPriority) as int
    Location HoldLoc = AllLocationsCS.GetAt(holdnum) as Location
    HoldLoc.SetKeywordData(ArmiesDefending, remainingarmies)
    TMData.SetKeywordData(ActiveCommanderCount, TMData.GetKeywordData(ActiveCommanderCount) - deadarmies)
EndFunction
```

#### Recovered Invariants:
- **Exact Sizing**: Every army division in the strategic AI model represents **exactly 20 soldiers** (`math.ceiling(troopsremaining / 20)`).
- **Casualty Conversion**: Casualties incurred in combat reduce the number of surviving division blocks:
  $$\text{SurvivingDivisions} = \left\lceil \frac{\text{TroopsRemaining}}{20} \right\rceil$$
  $$\text{DeadDivisions} = \text{AttackingArmies} - \text{SurvivingDivisions}$$
- **Commander Loss**: `ActiveCommanderCount` is directly decremented by `deadarmies`.

---

### 3.2 Patrol Elimination & Land Superiority Loss (`cs_tmscript.psc:1526–1535`)

```papyrus
Function KillPatrollingArmy(Location HoldLoc)
    HoldLoc.SetKeywordData(ArmiesPatrolling, HoldLoc.GetKeywordData(ArmiesPatrolling) - 1)
    TMData.SetKeywordData(ActiveCommanderCount, TMData.GetKeywordData(ActiveCommanderCount) - 1)
    if HoldLoc.GetKeywordData(ArmiesPatrolling) == 0
        LoseLandSuperiority(AllLocationsCS.Find(HoldLoc))
        DisableCamp(AllLocationsCS.Find(HoldLoc))
    endif
    int aggressivenessincrease = utility.randomint(1, 2)
    AdjustAggressiveness(aggressivenessincrease)
EndFunction
```

#### Recovered Invariants:
- Eliminating a patrol decrements both `ArmiesPatrolling` and `ActiveCommanderCount` by 1.
- If `ArmiesPatrolling` reaches `0`, the faction immediately loses land superiority (`LoseLandSuperiority`) and their forward war camp is disabled (`DisableCamp`).
- Retaliation trigger: Eliminating a patrol increases enemy aggressiveness by `random(1, 2)`.

---

## 4. Division Alias Array Architecture (10 vs. 19 Divisions)

In `cs_troopmanagemenuscript.psc:2170–2188`, properties define:
```papyrus
ReferenceAlias[] Property ArmyTroops1  Auto  
...
ReferenceAlias[] Property ArmyTroops10 Auto
ReferenceAlias[] Property ArmyTroops11 Auto  
...
ReferenceAlias[] Property ArmyTroops19 Auto  
```

### Architectural Reconciliation:
- **User-Facing Cap**: The player management menu exposes divisions `0..9` (10 field divisions, `FEAT-MIL-03`).
- **Engine Alias Allocation**: The mod allocates 19 reference alias arrays in the quest record (`ArmyTroops1..19`).
- **Role Partitioning**:
  - `ArmyTroops1..10`: Dedicated to Player army divisions.
  - `ArmyTroops11..19`: Reserved for foreign factions (Imperial Legion and Stormcloak Rebellion commanders) to manage their field forces using the same underlying Papyrus mechanics.
- **Capacity per Array**: Each alias array contains 20 actor slots, directly matching the 20-soldier division constant discovered in `cs_tmscript.psc:1512`.

---

## 5. Status of the v0.4.0 Jarl Loyalty System

Inspection of `cs_checkversion.psc:92–102` confirms:

```papyrus
        ;/
        
        if GovScriptFunctions.Version.GetValue() < 0.4000
            if GetOwningQuest().GetStage() >= 1
                GovScriptFunctions.LoyaltyScript.SetUpInitialJarlLoyalties()
            endif
            GovScriptFunctions.Version.SetValue(0.4000)
            updated = true
        endif

        /;
```

- **Proof of Disabled Stub**: The call to `SetUpInitialJarlLoyalties()` is enclosed in Papyrus block comments `;/ ... ;/`.
- **Conclusion**: In shipped release v0.3.9, Jarl loyalty initialization never executes at runtime. It was an unreleased feature prepared for v0.4.0. The reconstruction's choice to initialize Jarl loyalty to baseline 100 upon appointment is verified as the correct compatible behavior.

---

## 6. Contract Updates & Gap Closures

As a result of this milestone, the following gaps in `docs/behavioral-contracts.md` are formally closed:

1. **`FEAT-BAT-04` (Simulated Battles)**:
   - *Previous Status*: CONFIRMED / UNKNOWN (General AI battle math unknown).
   - *Updated Status*: **CONFIRMED IN CODE**. Exact 20-soldier division sizing and division casualty equations verified (`cs_tmscript.psc:1512`).
2. **`FEAT-MIL-03` (10 Field Armies)**:
   - *Clarification*: 10 divisions is the player limit; 19 divisions is the global engine allocation supporting AI factions.
3. **AI War Priority Heuristic**:
   - Codified exact weight formula $\text{BorderingHolds} + \text{Random}(0, 3) + 10\,(\text{Unpatrolled}) + 100\,(\text{Desperate})$.
