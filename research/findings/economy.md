# Resource & Treasury Systems: Conquest of Skyrim

- **Milestone**: M17 — Resource and Treasury Systems
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_holdingeconomyupdaterscript.psc` (Production equations, labor priority, population growth)
  - `cs_economyvariablefunctions.psc` (Initial hold keywords, base yields, facility tiers)
  - `cs_economymenufunctions.psc` (Treasury withdrawals, deposits, steward menus)
- **Status**: CONFIRMED

---

## 1. Resource Categories & Economic Framework

*Conquest of Skyrim* features a simulated regional economy driven by five primary economic resources:

| Resource | Primary Source | Storage Container | Primary Usage |
| :--- | :--- | :--- | :--- |
| **Gold (Treasury)** | Hold taxes, trade docks, POW ransoms, dungeon clearing | `GlobalVariable CS_FactionTreasury` | Troop wages, cabinet salaries, infrastructure upgrades, army recruitment. |
| **Food** | Farms, hunting camps, agricultural villages | Global holding food pools (`CS_ResourceFoodPerDay*`) | Daily civilian sustenance (`FoodRequired`) and army upkeep (`TroopCountFood`). |
| **Metal / Ore** | Mines, smelters, claimed caves | Faction Armory Stockpile | Blacksmith weapon/armor crafting, siege equipment maintenance. |
| **Wood / Timber** | Lumber mills, logging outposts | Faction Infrastructure Stockpile | Constructing barricades, catapults, rams, ladders, outpost upgrades. |
| **Trade / Commerce** | Maritime docks (Solitude, Windhelm, Riften, Dawnstar) | Direct treasury gold yields | Multiplies hold commercial revenue and provides luxury goods. |

---

## 2. Mathematical Production Formulas (`CS_HoldingEconomyUpdaterScript`)

Resource generation is calculated per holding during daily 24h game-time updates:

### 2.1 Maximum Job Capacity Formulas
Every facility has a level (`1..4`) and a base multiplier (`Change`):
$$\text{FoodMax} = \text{FarmLevel} \times \text{FoodChange}$$
$$\text{MetalMax} = \text{MineLevel} \times \text{MetalChange}$$
$$\text{WoodMax} = \text{LumberMillLevel} \times \text{WoodChange}$$
$$\text{DocksMax} = \text{CS\_CityLevelDocks} \times \text{CS\_DocksChange}$$
$$\text{MaxJobs} = \text{FoodMax} + \text{MetalMax} + \text{WoodMax} + \text{DocksMax}$$

### 2.2 Population Growth Equation
Civilian growth occurs dynamically if available jobs exceed current population:
$$\text{Condition: } \text{Population} < \text{MaxJobs} + 0.25 \times \text{Population}$$
$$\text{Deficit} = \text{MaxJobs} - \lfloor 0.25 \times \text{Population} \rfloor$$
$$\text{PopIncrease} = \text{RandomInt}(\lfloor 0.3 \times \text{Deficit} \rfloor, \, \lfloor 0.5 \times \text{Deficit} \rfloor)$$
- Upon growth:
  - $\text{Population} \leftarrow \text{Population} + \text{PopIncrease}$
  - $\text{Unemployed} \leftarrow \text{Unemployed} + \text{PopIncrease}$

### 2.3 Strict Labor Allocation Priority
When assigning unemployed workers to open jobs, `UpdateHoldingResources()` executes a strict waterfall:
1. **Priority 1: Food Jobs (`JobsOpenFood`)** — Filled first to prevent starvation and unrest.
2. **Priority 2: Maritime Docks (`CS_CityManagementJobsOpenDocks`)** — Filled second to sustain trade income.
3. **Priority 3: Timber (`JobsOpenWood`)** — Filled third for construction materials.
4. **Priority 4: Mining (`JobsOpenMetal`)** — Filled fourth for armory supply.

---

## 3. Hold Economic Profiles (`CS_EconomyVariableFunctions`)

Skyrim's major hold capitals have distinct starting economic endowments:

| Hold Capital | Farm / Food | Mine / Metal | Mill / Wood | Docks / Trade | Starting Role |
| :--- | :---: | :---: | :---: | :---: | :--- |
| **Whiterun** | **Level 4 (Yield 10)** | Level 0 (Yield 2) | Level 1 (Yield 3) | — | **Agricultural Breadbasket**: Primary food supplier for Central Skyrim. |
| **Markarth** | Level 1 (Yield 3) | **Level 4 (Yield 7)** | Level 2 (Yield 2) | — | **Mining Capital**: Primary metal/silver producer. |
| **Solitude** | Level 1 (Yield 3) | Level 0 (Yield 1) | Level 1 (Yield 2) | **Level 4 (Yield 10)** | **Commercial Hub**: Massive maritime trade revenue. |
| **Riften** | Level 3 (Yield 7) | Level 2 (Yield 2) | Level 3 (Yield 3) | Level 2 (Yield 3) | **Diversified Economy**: Balanced food, timber, and canal trade. |
| **Windhelm** | Level 2 (Yield 7) | Level 2 (Yield 2) | Level 1 (Yield 1) | Level 3 (Yield 7) | **Northern Trade Port**: Secondary food and seaport hub. |

- **Baseline Metrics for Major Cities**:
  - `Population = 50`
  - `FoodRequired = 50`
  - `BarracksLevel = 2`

---

## 4. Treasury & Taxation Mechanics (`CS_EconomyMenuFunctions`)

### 4.1 Faction Treasury
- Centralized gold reserve accessed through dialogue with the Steward:
  - Deposit / Withdrawal Tiers: `100`, `1,000`, `10,000`, `100,000`, `1,000,000` gold.
  - Automatically receives daily tax deposits and deducts army wages.

### 4.2 Taxation Tiers & Unrest Consequences
Hold tax rates are set per hold or globally via the Steward:
- **Low Taxes**: Generates minimal revenue; boosts hold `Loyalty` and reduces `Unrest`.
- **Normal Taxes**: Standard balanced revenue; neutral impact on stability.
- **High Taxes**: Generates substantial gold; steadily increases `Unrest` and reduces public order.
- **Extortionate Taxes**: Maximum immediate gold extraction; triggers rapid citizen rebellions and peasant riots.

---

## 5. Resource Shortages & Failure Cascades

1. **Food Deficits (Famine)**:
   - If total food production across all holdings falls below $\sum \text{FoodRequired} + \text{TroopCountFood}$, a famine event triggers.
   - *Consequences*: Negative population growth, civilian unrest spikes, and standing army desertions.
2. **Treasury Insolvency (Bankrupt Military)**:
   - If gold in `CS_FactionTreasury` reaches zero while active armies require daily wages, division commanders enter an unpaid state, reducing combat effectiveness and increasing mutiny probability.
