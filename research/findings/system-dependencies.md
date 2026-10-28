# Cross-System Dependency Recovery: Conquest of Skyrim

- **Milestone**: M25 — Cross-System Dependency Recovery
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Data Sources**: Synthesized across M05–M24 deliverables, Papyrus script property bindings, and `conquerskyrim.esp` shared records
- **Status**: CONFIRMED

---

## 1. Cross-System Interaction Matrix

*Conquest of Skyrim* is an interconnected 4X strategic ecosystem where no individual subsystem operates in isolation. The core mechanics form a continuous feedback loop:

```text
                  ┌──────────────────────────────┐
                  │    1. Territory & Holdings   │
                  └──────────────┬───────────────┘
                                 │ Generates Tax & Resources
                                 ▼
                  ┌──────────────────────────────┐
                  │    2. Economy & Treasury     │
                  └──────────────┬───────────────┘
                                 │ Sustains Wages & Equips
                                 ▼
                  ┌──────────────────────────────┐
                  │    3. Military & Armies      │
                  └──────────────┬───────────────┘
                                 │ Executes Sieges & Breaches
                                 ▼
                  ┌──────────────────────────────┐
                  │    4. Conquest & Expansion   │
                  └──────────────┬───────────────┘
                                 │ Installs Rulers & Courts
                                 ▼
                  ┌──────────────────────────────┐
                  │    5. Government & Stability │
                  └──────────────┬───────────────┘
                                 │ Convenes The Moot
                                 ▼
                  ┌──────────────────────────────┐
                  │    6. High King Coronation   │
                  └──────────────────────────────┘
```

---

## 2. Core Inter-Subsystem Relationships

### 2.1 Territory $\longleftrightarrow$ Economy
- **Inputs & Yields**: Territorial control directly determines daily tax and resource production.
- **Formulas**: Holding classification dictates baseline tax revenue ($\text{TreasuryPerDay} = \text{Base} \cdot \text{Pop} + \text{Multiplier} \cdot \text{Pop} \cdot \text{TaxRate}$).
- **Node Integration**: Claiming specific mines (e.g., Halted Stream Camp) and agricultural settlements (e.g., Rorikstead) injects raw metal and food directly into the faction stockpile.

### 2.2 Economy $\longleftrightarrow$ Military
- **Troop Sustenance**: Standing divisions consume daily rations ($\text{TroopCountFood}$) and mercenary officer wages ($100 \text{ gold/day}$ for neutral commanders).
- **Armory Requisitions**: Enlisting recruits requires liquid treasury gold; standardizing equipment requires metal ingots in faction reserves.
- **Insolvency Cascade**: Treasury exhaustion halts recruitment, degrades army morale, and triggers division desertions.

### 2.3 Population $\longleftrightarrow$ Recruitment
- **Civilian Labor Reserve**: Recruits are conscripted directly from the holding's `Unemployed` pool:
  $$\text{RecruitableTroops} = \lceil \text{CS\_RecruitablePercentage} \times \text{Unemployed} \rceil$$
- **Economic Drain**: Enlisting soldiers permanently depletes civilian `Population`, reducing daily tax revenue and shrinking future labor pools.

### 2.4 Military $\longleftrightarrow$ Conquest
- **Siege Operations**: Capturing walled hold capitals requires military superiority ($\text{MessageNoSuperiority}$ check) and engineering assets (Battering Rams, Catapults).
- **Garrisoning**: Securing conquered territory demands stationing military detachments (`CS_NumGuards`) to suppress local rebellions.

### 2.5 Conquest $\longleftrightarrow$ Government
- **Cabinet Expansion**: Conquering hold capitals unlocks appointment of local Jarls, Stewards, Housecarls, and Guard Captains.
- **Native Rulers**: Conquered native Jarls are unseated into `CitizenFaction` or transferred to fort dungeons (`JarlPrisonersImp` / `JarlPrisonersSons`).

### 2.6 Government $\longleftrightarrow$ Diplomacy
- **Diplomatic Authority**: The Steward dispatches formal war declarations and peace treaty emissaries.
- **War Score Coupling**: Bilateral war momentum (`WarScoreImp` / `WarScoreSons`) dictates the success probability of peace summits and war reparation payments.

### 2.7 Loyalty $\longleftrightarrow$ Stability
- **Public Sentiment**: Citizen `Loyalty` is mediated by food surpluses, garrison presence, royal court sessions, and tax severity.
- **Harshness Multiplier**: Draconian court sentences inflame unrest, whereas royal feasts and judicial pardons restore loyalty.

### 2.8 Stability $\longleftrightarrow$ Rebellion
- **Insurrection Thresholds**:
  - `Loyalty < 40`: Triggers civilian palace demonstrations (`ProtestQuest`).
  - `Loyalty < 20`: Spawns armed civil uprisings (`RebelDefendQuestStarted`), attempting to liberate the city and restore vanilla factions.

### 2.9 Research $\longleftrightarrow$ Military
- **Stat & Tier Unlocks**: Investing Treasury Gold and Metal reserves into technology unlocks 10 tiers of martial perks (`CS_WarriorUpgradePerk*`, `CS_ArcherUpgradePerk*`), scaling `TroopPowerLevels` across all deployed armies.

### 2.10 Progression $\longleftrightarrow$ Political Authority
- **The Moot Prerequisite**: Securing all 9 hold capitals and maintaining high ruler loyalty ($\text{JarlLoyalty} \ge 60$) is required to win election at The Moot and achieve coronation as High King/Queen.

---

## 3. Shared State Architecture & Data Synchronization

| Shared State Asset | Type | Controlling Systems | Invariant Requirement |
| :--- | :--- | :--- | :--- |
| **`AllHoldings`** | `FormList` of Globals | Territory, Economy, Diplomacy, Quests | Synchronized across `1` (Player), `11` (Imperial), `12` (Stormcloak). |
| **`CS_FactionTreasury`** | `GlobalVariable` | Economy, Military, Government, Tech | Must remain $\ge 0$; negative balances halt army payrolls. |
| **`CWOwner`** | `Keyword` | Territory, Factions, Civil War | Set to `69` for Player, `1` for Imperial, `2` for Stormcloak. |
| **`Cities[]`** | `bool[9]` array | `CS_BattleTrackerScript`, Sieges, The Moot | Must reflect true holding ownership; all 9 true required for The Moot. |
| **`WarScore`** | `GlobalVariable` | Combat, Diplomacy, Territory | Shifts dynamically based on territory conquered and battles won. |
| **`IsHighKing`** | `GlobalVariable` | Progression, Coronation, Dialogue | Set to `1` upon completion of coronation ceremony. |

---

## 4. Daily Update Execution Ordering (24h Tick)

When `RegisterForUpdateGameTime(24.0)` fires, the engine executes daily maintenance in strict order:

```text
Step 1: Production Evaluation
   └── Iterates AllHoldings -> Computes FoodProduced, MetalProduced, WoodProduced, TreasuryPerDay.
         │
Step 2: Domestic Consumption
   └── Deducts civilian food requirements (FoodRequired = Population + CS_NumGuards).
         │
Step 3: Military Upkeep
   └── Deducts army rations (TroopCountFood) and mercenary commander wages (100g/day).
         │
Step 4: Reserve Pooling
   └── Retains up to 5 units in local holding reserves; transfers excess to national stockpile.
         │
Step 5: Stability & Loyalty Update
   └── Evaluates food deficits (NoFood), missing garrisons (NoGarrison), and taxes -> Adjusts Loyalty.
         │
Step 6: AI Offensives & Recovery
   └── TMImp / TMSons evaluate border targets and advance camp respawn cooldown timers.
```

---

## 5. Circular Feedback Loops & Economic Death Spirals

Systemic analysis reveals two dangerous circular failure cascades:

1. **The Famine Death Spiral**:
   $$\text{Food Deficit} \longrightarrow \text{Public Loyalty Drops} \longrightarrow \text{Protests / Riots} \longrightarrow \text{Garrison Casualties} \longrightarrow \text{Workers Flee} \longrightarrow \text{Food Collapses Further}$$
2. **The Taxation / Mutiny Spiral**:
   $$\text{Extortionate Taxes} \longrightarrow \text{Civilian Revolt} \longrightarrow \text{Hold Secedes} \longrightarrow \text{Tax Base Shrinks} \longrightarrow \text{Treasury Bankrupt} \longrightarrow \text{Unpaid Troops Desert}$$
