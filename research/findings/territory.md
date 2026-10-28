# Territory & Ownership Specification: Conquest of Skyrim

- **Milestone**: M10 — Territory and Ownership
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_transferownershipscript.psc` (`TransferOwnershipCity`, `TransferOwnershipFort`, `TransferOwnershipVillage`)
  - `cs_battletrackerscript.psc` (`Cities[]`, `Forts[]`, `Camps[]`, `PlayerCamps[]`)
  - `cs_attackplanningnewscript.psc` (`MessageNoHoldLink`, `InvasionTarget`, `HoldLoc`)
  - `cs_govscriptfunctions.psc` (`SetUpFactionOwners()`, `AllLocations`, `AllHoldings`)
- **Status**: CONFIRMED

---

## 1. Territorial Hierarchy & Holding Types

*Conquest of Skyrim* divides the province of Skyrim into a structured, four-tier territorial hierarchy:

```text
Province of Skyrim
  │
  ├── 1. Hold Capitals (9 Major / Minor Capitals)
  │     ├── Major Walled Cities (citynum 0–4): Markarth, Riften, Solitude, Whiterun, Windhelm
  │     └── Minor Open Capitals (citynum 5–8): Dawnstar, Falkreath, Morthal, Winterhold
  │
  ├── 2. Strategic Fortifications (AllHoldingsFort)
  │     └── Major military bastions (e.g., Fort Greymoor, Fort Dunstad, Fort Amol)
  │
  ├── 3. Dependent Villages & Settlements (AllLocations)
  │     └── Riverwood, Rorikstead, Dragon Bridge, Ivarstead, Shor's Stone, Karthwasten
  │
  └── 4. Frontier Camps & Outposts (PlayerCamps / Camps)
        └── Claimed bandit camps, mines, redoubts, and cave entrances
```

---

## 2. Technical Ownership State Encoding

The mod tracks territorial control using two synchronized data layers: global variables for script state, and native Creation Engine keywords for world actor interaction.

### 2.1 State Integer Matrix

| Controlling Faction | Global Variable Value (`AllHoldings`, `AllHoldingsFort`) | Creation Engine `CWOwner` Keyword | Engine Representation |
| :--- | :---: | :---: | :--- |
| **Player Faction** | **`1`** | **`69`** | Custom independent sovereignty (`PlayerArmyFaction`). |
| **Imperial Legion** | **`11`** | **`1`** | Native Civil War Imperial allegiance (`CWImperialFaction`). |
| **Stormcloaks** | **`12`** | **`2`** | Native Civil War Stormcloak allegiance (`CWSonsFaction`). |

*Mathematical Relationship*: When a holding transitions to an NPC faction, `CWOwner` is derived mathematically as `CWOwner = newfaction - 10` (e.g., `11 - 10 = 1`, `12 - 10 = 2`). For player control, `CWOwner` is assigned the fixed signature integer `69`.

---

## 3. Hold Capital Indexing (`citynum`)

The 9 hold capitals of Skyrim are indexed by `citynum` integer IDs:

| `citynum` | Hold Capital | Hold Name | Classification | Governing Palace Cell |
| :---: | :--- | :--- | :--- | :--- |
| `0` | **Markarth** | The Reach | Major Walled City (`< 5`) | Understone Keep |
| `1` | **Riften** | The Rift | Major Walled City (`< 5`) | Mistveil Keep |
| `2` | **Solitude** | Haafingar | Major Walled City (`< 5`) | Castle Dour / Blue Palace |
| `3` | **Whiterun** | Whiterun Hold | Major Walled City (`< 5`) | Dragonsreach |
| `4` | **Windhelm** | Eastmarch | Major Walled City (`< 5`) | Palace of the Kings |
| `5` | **Dawnstar** | The Pale | Minor Open City (`>= 5`) | The White Hall |
| `6` | **Falkreath** | Falkreath Hold | Minor Open City (`>= 5`) | Jarl's Longhouse |
| `7` | **Morthal** | Hjaalmarch | Minor Open City (`>= 5`) | Highmoon Hall |
| `8` | **Winterhold** | Winterhold | Minor Open City (`>= 5`) | Jarl's Longhouse |

---

## 4. Ownership Transition Workflows (`CS_TransferOwnershipScript`)

Territorial transitions execute atomically through three dedicated functions:

### 4.1 Capital City Conquest (`TransferOwnershipCity`)
When the player captures a city (`newfaction == 69`):
1. **Crime Jurisdiction Reset**:
   ```papyrus
   (CrimeFactions.GetAt(citynum) as Faction).SetCrimeGold(0)
   (CrimeFactions.GetAt(citynum) as Faction).SetCrimeGoldViolent(0)
   ```
   Clears all accrued local bounties to prevent newly spawned faction guards from attacking the player.
2. **Keyword & Marker Flipping**:
   - `CityLoc.SetKeywordData(CWOwner, 69)`
   - Disables opposing guard markers (`ImpMarkerLists[citynum]` or `SonsMarkerLists[citynum]`).
   - Decrements enemy city count (`CityCountImp.Mod(-1)` or `MinorCityCountImp.Mod(-1)`).
3. **Deposing Rulers**:
   - Unless `keepjarl == true`, clears `JarlAliases[citynum]`.
   - Deposed native rulers (`GovNPCsImp` / `GovNPCsSons`) are assigned to `CitizenFaction`, revoking their royal authority.
4. **Enemy Capital Relocation**:
   - If the conquered city served as the enemy's seat of government, invokes `TMImperial.ChooseRandomCapital()` to force the AI to select a surviving hold capital.
5. **State Finalization**:
   - `(AllHoldings.GetAt(citynum) as GlobalVariable).SetValue(1)`
   - Refreshes strategic invasion objectives across all factions.

### 4.2 Fort Capture (`TransferOwnershipFort`)
1. **Marker Synchronization**:
   - Player capture (`newfaction == 1`): Disables enemy fort markers, enables `FortMarkersPlayer[fortnum]`, and enables `MapMarkersForts[fortnum]`.
   - Enemy reconquest (`newfaction != 1`): Disables player markers, enables enemy garrison, and executes `PrisonerScript.FreeAllFortPrisoners(fortnum)` to liberate detained POWs.
2. **Holding Value**:
   - Sets `AllHoldingsFort[fortnum]` global to `1` (Player), `11` (Imperial), or `12` (Stormcloak).

### 4.3 Village & Settlement Transfer (`TransferOwnershipVillage`)
1. Enables/disables corresponding entries in `PlayerMarkers`, `ImpMarkerLists`, and `SonsMarkerLists`.
2. Updates `AllHoldings[villagenum]` global.
3. Sets `VillageLoc.SetKeywordData(CWOwner, 69)`.

---

## 5. Strategic Preconditions & System Dependencies

### 5.1 Territorial Contiguity Invariant
Invasion planning (`CS_AttackPlanningNewScript`) strictly enforces contiguous borders:
- If a player attempts to launch an invasion against a hold that does not share a border with an already-controlled holding, the script halts and displays `Message Property MessageNoHoldLink Auto`.
- Prevents players from leapfrogging across Skyrim without establishing logistics lines.

### 5.2 Economic & Supply Consequences
Each captured holding links directly to the economic engine:
- **Food Production**: Holdings carry dedicated daily food globals (`CS_ResourceFoodPerDayMinorCityMorthal`, `CS_ResourceFoodPerDayVillageRiverwood`, etc.).
- **Taxation**: Daily 24h game-time updates calculate tax revenue based on the holding count.
- **Outpost Workers**: Controlled mines (e.g., Halted Stream Camp) permit hiring miners to generate raw iron/silver ingots directly into the faction armory.

---

## 6. Enemy Reconquest & Insurgency Mechanics

Enemy AI territory managers (`TMImp` and `TMSons`) actively launch counter-offensives:
1. **Target Selection**: Enemy factions identify vulnerable border holdings and fortresses.
2. **Reconquest Execution**: If the player's garrison is overwhelmed or absent during an off-screen attack, `TransferOwnershipFort` or `TransferOwnershipCity` is invoked with `newfaction = 11` or `12`.
3. **Liberation of POWs**: Enemy fort retakes automatically execute `FreeAllFortPrisoners(fortnum)`, converting captives back into active enemy combatants.
