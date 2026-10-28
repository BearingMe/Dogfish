# Population & Employment Specification: Conquest of Skyrim

- **Milestone**: M18 — Population and Employment
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_holdingeconomyupdaterscript.psc` (Population growth, conscription, tax yield formulas)
  - `cs_economyvariablefunctions.psc` (Initial population parameters, facility tiers)
- **Status**: CONFIRMED

---

## 1. Population Data Model Architecture

Rather than tracking hundreds of individual civilian actor instances across the entire province, *Conquest of Skyrim* implements an **abstract keyword-driven demographic model**. 

Each location in `AllLocations` stores demographic state directly on the `Location` form via Creation Engine keyword data:
- `Population`: Total civilian population integer.
- `Unemployed`: Civilian labor reserve available for work or military conscription.
- `FoodRequired`: Total daily food sustenance requirement.
- `CS_RecruitablePercentage`: Maximum ratio of unemployed citizens eligible for military service.
- `CS_NumGuards`: Active garrison guards stationed in the holding.

---

## 2. Demographic Growth & Reproduction Mechanics

Civilian growth is evaluated daily during 24h game-time updates in `CS_HoldingEconomyUpdaterScript`:

### 2.1 Growth Trigger Condition
Population expands only if total available employment capacity exceeds the current population:
$$\text{Condition: } \text{Population} < \text{MaxJobs} + 0.25 \times \text{Population}$$
$$\text{Deficit} = \text{MaxJobs} - \lfloor 0.25 \times \text{Population} \rfloor$$
$$\text{PopIncrease} = \text{RandomInt}(\lfloor 0.3 \times \text{Deficit} \rfloor, \, \lfloor 0.5 \times \text{Deficit} \rfloor)$$

- **Labor Invariant**: New population is added directly to `Unemployed`:
  $$\text{Population} \leftarrow \text{Population} + \text{PopIncrease}$$
  $$\text{Unemployed} \leftarrow \text{Unemployed} + \text{PopIncrease}$$

---

## 3. Labor Allocation & Employment Waterfall

When available jobs exist, `UpdateHoldingResources()` pulls from `Unemployed` in a strict four-tier waterfall:

```text
Unemployed Labor Pool
  │
  ├── 1. Agricultural Labor (JobsOpenFood)  [Filled First: Prevents Famine]
  │
  ├── 2. Maritime Docks (CS_CityManagementJobsOpenDocks) [Filled Second: Sustains Trade]
  │
  ├── 3. Forestry & Timber (JobsOpenWood)   [Filled Third: Construction Gear]
  │
  └── 4. Mining & Smelting (JobsOpenMetal)  [Filled Fourth: Weapons & Armor]
```

- **Unfilled Job Penalty**: Every job that remains unfilled due to labor shortages directly subtracts from gross production in the daily yield calculation.

---

## 4. Military Conscription Engine

Military recruits are drafted directly from the local civilian population:

$$\text{RecruitableTroops} = \lceil \text{CS\_RecruitablePercentage} \times \text{Unemployed} \rceil$$

### 4.1 Demographic Drain
When troops are enlisted or auto-recruited into an army division:
$$\text{Population} \leftarrow \text{Population} - \text{RecruitableTroops}$$
$$\text{Unemployed} \leftarrow \text{Unemployed} - \text{RecruitableTroops}$$

- **Strategic Trade-off**:
  - Drafting soldiers permanently depletes the civilian population base.
  - Reduced population lowers daily tax yields and shrinks future labor pools.

---

## 5. Daily Holding Sustenance & Food Requirement

Food consumption scales with both civilian population and stationed military garrisons:

$$\text{FoodRequired} = \text{Population} + \text{CS\_NumGuards}$$

- **Famine Impact**: If `FoodProduced < FoodRequired`, the holding enters a food deficit (`FoodOffset < 0`), which draws from `FoodReserve`. If reserves reach zero, public order plummets and population growth halts.

---

## 6. Exact Daily Tax Yield Formulas by Holding Tier

Daily treasury income scales directly with civilian population (`Pop`) and the local tax rate multiplier (`TaxRate`, integer `0..3`):

$$\text{TreasuryPerDay} = (\text{BaseRate} \times \text{Pop}) + (\text{TaxMultiplier} \times \text{Pop} \times \text{TaxRate})$$

| Holding Classification | Location Index (`holdingnum`) | Base Daily Yield per Citizen | Tax Multiplier per Citizen | Formula |
| :--- | :---: | :---: | :---: | :--- |
| **Major Walled Capitals** | `0 .. 4` (Markarth, Riften, Solitude, Whiterun, Windhelm) | **500 gold** | **250 gold** | $\text{TreasuryPerDay} = 500 \cdot \text{Pop} + 250 \cdot \text{Pop} \cdot \text{TaxRate}$ |
| **Minor Open Capitals** | `5 .. 8` (Dawnstar, Falkreath, Morthal, Winterhold) | **300 gold** | **150 gold** | $\text{TreasuryPerDay} = 300 \cdot \text{Pop} + 150 \cdot \text{Pop} \cdot \text{TaxRate}$ |
| **Dependent Villages / Forts** | `9+` (Riverwood, Rorikstead, Forts, Outposts) | **200 gold** | **100 gold** | $\text{TreasuryPerDay} = 200 \cdot \text{Pop} + 100 \cdot \text{Pop} \cdot \text{TaxRate}$ |

*Treasury Integration*: Daily holding tax deposits credit $50\%$ of `TreasuryPerDay` directly to local holding funds and the remainder to the faction treasury.

---

## 7. Exact Resource Production Equations

Gross material production evaluates facility levels, dock trade bonuses, and labor deficits:

$$\text{FoodProduced} = 3 \times \Big( (\text{FoodChange} \times \text{FarmLevel}) + \lfloor 0.3 \times \text{DocksChange} \times \text{DocksLevel} \rfloor - \text{JobsOpenFood} \Big)$$

$$\text{MetalProduced} = 3 \times \Big( (\text{MetalChange} \times \text{MineLevel}) + \lfloor 0.3 \times \text{DocksChange} \times \text{DocksLevel} \rfloor - \text{JobsOpenMetal} \Big)$$

$$\text{WoodProduced} = 3 \times \Big( (\text{WoodChange} \times \text{LumberMillLevel}) + \lfloor 0.3 \times \text{DocksChange} \times \text{DocksLevel} \rfloor - \text{JobsOpenWood} \Big)$$

- **Surplus National Pooling**:
  - Each holding retains up to 5 units of surplus in its local reserve (`FoodReserve`, `MetalReserve`, `WoodReserve`).
  - Production in excess of 5 units ($\text{Offset} - 5$) is transferred directly into the national faction stockpile.
