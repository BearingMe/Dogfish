# Buildings & Infrastructure Specification: Conquest of Skyrim

- **Milestone**: M19 — Buildings and Infrastructure
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_upgradecityscript.psc` (`UpgradeCity`, `UpgradeVillage`, `PayForUpgrade`, `AddJobs`)
  - `cs_upgrademenufunctions.psc` (`MenuStart`, `MenuStartVillage`, `SetGlobals`)
  - `cs_economyvariablefunctions.psc` (Facility keywords: `FarmLevel`, `MineLevel`, `LumberMillLevel`, `BarracksLevel`, `DocksLevel`)
- **Status**: CONFIRMED

---

## 1. Infrastructure Catalog & Facility Categories

*Conquest of Skyrim* models regional development through five upgradable facility types across the 9 hold capitals (`citynum 0..8`) and 6 major dependent villages (`villagenum 9..14`):

| Facility Type | Keyword Identifier | Max Tier | Resource Produced | Strategic Role |
| :--- | :--- | :---: | :--- | :--- |
| **Agricultural Farms** | `FarmLevel` | **4** | Food | Sustains civilian population (`FoodRequired`) and army upkeep (`TroopCountFood`). |
| **Lumber Mills** | `LumberMillLevel` | **4** | Wood / Timber | Provides raw lumber required for constructing siege engines and further infrastructure. |
| **Mines & Smelters** | `MineLevel` | **4** | Metal / Ingots | Supplies raw ore to the Faction Blacksmith for equipping soldiers with advanced gear. |
| **Maritime Docks** | `DocksLevel` / `CS_CityLevelDocks` | **4** | Trade Revenue / Gold | Boosts commercial trade income and provides a +30% yield bonus across all resources. |
| **Military Barracks** | `BarracksLevel` / `CS_CityLevelBarracks` | **4** | Garrison Capacity | Increases maximum stationed garrison guards (`CS_NumGuards`) to suppress unrest. |

---

## 2. Construction Lifecycle & Financial Mechanics

Infrastructure construction is initiated through dialogue with the Steward or local Governor (`CS_UpgradeCityScript`):

```text
Player selects "Upgrade Holding" in Steward Menu (CS_UpgradeMenuFunctions)
       │
       ▼
Evaluates UpgradeCost (Gold) and UpgradeCostWood (Timber Reserves)
       │
       ▼
Deducts Materials: PayForUpgrade()
  ├── Treasury.SetValue(Treasury - UpgradeCost)
  └── Wood.SetValue(Wood - UpgradeCostWood)
       │
       ▼
Level Increment: CityLoc.SetKeywordData(Level, CurrentLevel + 1)
       │
       ▼
Immediate Job Creation: AddJobs(CityLoc, jobnum)
  ├── Farm Upgrade: JobsOpenFood += FoodChange
  ├── Lumber Upgrade: JobsOpenWood += WoodChange
  └── Mine Upgrade: JobsOpenMetal += MetalChange
       │
       ▼
Daily Simulation Loop absorbs Unemployed into new jobs and scales production
```

### 2.1 Dual-Resource Construction Costs
Upgrades demand both liquid capital and raw timber:
- **Treasury Gold (`UpgradeCost`)**: Pays contracted labor and architect wages.
- **Wood Reserves (`UpgradeCostWood`)**: Expends lumber stockpiles gathered by logging camps.
- *Sound Feedback*: Plays native `CoinSound` and `WoodSound` upon transaction completion.

---

## 3. Direct Employment & Capacity Scaling (`AddJobs`)

Upgrading a facility immediately expands employment capacity in the holding based on that holding's intrinsic base yield rating (`Change`):

```papyrus
Function AddJobs(Location Holding, int jobnum)
    if jobnum == 0
        Holding.SetKeywordData(JobsOpenFood, Holding.GetKeywordData(JobsOpenFood) + Holding.GetKeywordData(FoodChange))
    elseif jobnum == 1
        Holding.SetKeywordData(JobsOpenWood, Holding.GetKeywordData(JobsOpenWood) + Holding.GetKeywordData(WoodChange))
    elseif jobnum == 2
        Holding.SetKeywordData(JobsOpenMetal, Holding.GetKeywordData(JobsOpenMetal) + Holding.GetKeywordData(MetalChange))
    endif
EndFunction
```

- **Growth Driver**: By increasing open jobs, the holding's `MaxJobs` ceiling rises. In the next 24h economic update (`CS_HoldingEconomyUpdaterScript`), this job surplus satisfies the growth condition ($\text{Population} < \text{MaxJobs} + 0.25 \cdot \text{Population}$), attracting civilian migrants.

---

## 4. Settlement Development Scope & Geography

### 4.1 Supported Major Hold Capitals (`citynum 0..8`)
All 9 hold capitals support 4-tier facility construction:
- Markarth (`0`), Riften (`1`), Solitude (`2`), Whiterun (`3`), Windhelm (`4`), Dawnstar (`5`), Falkreath (`6`), Morthal (`7`), Winterhold (`8`).

### 4.2 Supported Dependent Villages (`villagenum 9..14`)
Six major villages can be upgraded via `MenuStartVillage()`:
1. **Riverwood** (`villagenum 9`): Primary timber and mill hub.
2. **Rorikstead** (`villagenum 10`): Agricultural farming center.
3. **Dragon Bridge** (`villagenum 11`): Strategic bridge outpost and lumber camp.
4. **Ivarstead** (`villagenum 12`): Pilgrimage village with forestry potential.
5. **Shor's Stone** (`villagenum 13`): Iron mining outpost.
6. **Karthwasten** (`villagenum 14`): Silver and metal mining settlement.

---

## 5. Constraints & Architectural Incomplete Features

1. **Terrain Restrictions (Coastal Docks)**:
   - Maritime docks (`CS_CityLevelDocks`) are geographically restricted to coastal or major river ports (Solitude, Windhelm, Riften, Dawnstar). Landlocked holds (Whiterun, Markarth, Falkreath) return null keywords for dock expansions.
2. **Hard Tier Limit**:
   - Facilities cannot exceed Level 4. Attempting upgrades past Level 4 is prevented by conditional menu dialogue checks (`AllowUpgrades.SetValue(0)`).
3. **No Dynamic 3D Building Models**:
   - Upgrading a farm or mine increases abstract mathematical keyword data (`FarmLevel`, `JobsOpenFood`) rather than spawning new physical 3D buildings in the Skyrim worldspace, avoiding severe cell navmesh conflicts.
