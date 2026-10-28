# Faction Creation Specification: Conquest of Skyrim

- **Milestone**: M09 — Faction Creation
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `qf_cs_factiongovernment_050095ca.psc` (`GovQuest` Stage 0 / 10 / 11)
  - `cs_readstartbook.psc` (`OnRead()` hook)
  - `cs_startfactionmenu.psc` (`MenuStart()`)
  - `cs_selectname.psc` (`ChangeName()`, `UpdateName()`, `UpdateGuardNames()`)
  - `cs_changebanners.psc` (`ReplaceBanners()`, `ReplaceCampColors()`, `ReplaceCityBanners()`)
  - `cs_govscriptfunctions.psc` (`SetUpFactionOwners()`)
- **Status**: CONFIRMED

---

## 1. Creation Workflow & Lifecycle Reconstructed

The complete technical initialization sequence from zero state to active sovereign faction:

```text
1. Prerequisites Met
   └── Player clears an eligible bandit camp, mine, or redoubt.
   
2. Initiation Trigger (CS_ReadStartBook.psc)
   └── Player activates/reads starter book -> invokes CS_StartFactionMenu.MenuStart().
   
3. Confirmation Prompt (CS_StartFactionMenu.psc)
   └── Message prompt displays -> Option 0 sets GovQuest to Stage 0.
   
4. Faction Identity Configuration (CS_SelectName.psc)
   ├── FactionStarted.SetValue(1)
   ├── Player receives BannerSpell (powers/lesser powers) and SettingsMenu item.
   ├── UILIB_1.ShowTextInput("Enter Faction Name") -> Sets FactionNameChestObject name.
   └── UILIB_1.ShowTextInput("Enter Army Prefix") -> Sets ChestObject name.
   
5. Customization Chamber Teleportation
   ├── PreviousLoc marker placed at player's worldspace coordinates.
   └── Player teleported to BannerRoomTeleport interior cell.
   
6. Visual & Map Overhaul
   ├── Banner selection triggers CS_ChangeBanners.ReplaceBanners().
   └── Tactical war maps updated via CS_ChangeWarMaps.ReplaceMaps().
   
7. Strategic Worldspace Registration (CS_GovScriptFunctions.psc)
   ├── SetUpFactionOwners() iterates AllLocations FormList.
   ├── Assigns holding states (11 = Imperial, 12 = Stormcloak).
   ├── Launches opposing faction AI controllers: TMImp.Start() & TMSons.Start().
   └── Player teleported back to PreviousLoc to plant the first territory banner.
   
8. Executive Cabinet Recruitment
   └── Player recruits local NPCs as Steward, Second-in-Command (ASIC), and Blacksmith.
```

---

## 2. Identity Customization Engine

### 2.1 Dynamic Name Propagation (`CS_SelectName.psc`)
Because Skyrim's engine does not allow arbitrary runtime renaming of native ActorBase records, *Conquest of Skyrim* employs a container-proxy name injection pattern:
- **Faction Name**:
  - Captures input string via `ShowTextInput("Enter Faction Name", "")`.
  - Applies string to base object: `FactionNameChestObject.GetBaseObject().SetName(NewFactionName)`.
  - Quests and dialogue tokens reference this object's name dynamically (`<Alias=FactionName>`).
- **Army Prefix & Soldier Renaming**:
  - Captures prefix (e.g., `Ironborn`, `Dread`, `Nordic`).
  - Sets `ChestObject.GetBaseObject().SetName(NewArmyName)`.
  - `UpdateGuardNames()` iterates through `GuardFormLists[]`, dynamically applying `NameChanger.ForceRefTo()` to update in-world guard names.

### 2.2 Heraldry & Banner Replacement (`CS_ChangeBanners.psc`)
Customization occurs inside `BannerRoomTeleport`:
- **Faction Color Index**: Stored in `GlobalVariable Property FactionColor Auto`.
- **Mesh Replacement**:
  - `ReplaceBanners()` iterates through `PlacedBanners` FormList.
  - Spawns `NewBannerAlias` using `PlaceAtMe(..., abForcePersist=true)`.
  - Deletes the old vanilla marker and records the new reference in `PlacedBanners`.
- **Camp Palette Synchronization**:
  - `ReplaceCampColors()` iterates through `CampColorMarkers`, enabling markers matching `FactionColor.GetValueInt()` and disabling conflicting colors.
- **Hold Capital & Fort Banners**:
  - Scale factor for city banners is fixed at `1.53` (`NewBanner.SetScale(1.53)`).

---

## 3. Initial Strategic State & Resource Allocations

| System Attribute | Initial State | Implementation Detail |
| :--- | :--- | :--- |
| **Faction Treasury** | `0 Gold` | The mod grants zero free gold; player must physically deposit funds via Steward dialogue (`CS_FactionTreasury`). |
| **Material Stockpiles** | `0 Wood, 0 Metal, 0 Stone` | Raw resources must be gathered by assigning laborers at claimed resource nodes. |
| **Food Allocation** | Local node production only | Evaluated on daily 24h game-time ticks. |
| **Starting Territory** | Initial Cleared Camp | Single claimed outpost designated by the first planted banner. |
| **Diplomatic Status** | Neutral / Non-Declared | Imperials and Stormcloaks do not attack the player's outpost until formal war declaration or border raid. |
| **Holding Registers** | Imperial (`11`) / Stormcloak (`12`) | `SetUpFactionOwners()` queries vanilla `Holding.GetKeywordData(CWOwner)` across `AllLocations` FormList. |

---

## 4. Executive Cabinet Appointment Mechanics

Once founded, the faction requires three essential personnel:

### 4.1 The Steward (Civil & Economic Governor)
- **Recruitment**: Approaching a friendly or neutral NPC outside combat.
- **Compensation**:
  - Friendly NPCs (RelationshipRank &ge; 1, followers, quest allies): `0 gold` (work for friendship).
  - Neutral NPCs: `100 gold/day` wage deducted automatically from the treasury.
- **Responsibilities**: Treasury deposits/withdrawals, hold tax rates, outpost worker assignments, foreign diplomatic emissaries.

### 4.2 Second-in-Command (`ASIC` / Military Marshal)
- **Recruitment**: High-disposition combatant NPC.
- **Responsibilities**: Manages army division rosters, issues tactical marching orders, opens the invasion planning map (`CS_AttackPlanningNewScript`), and detains captured Prisoners of War (POWs).

### 4.3 The Faction Blacksmith (Quartermaster)
- **Recruitment**: Blacksmith or craftsman NPC.
- **Responsibilities**: Equipping recruits. Converts raw ingots in the stockpile into standardized weapon and armor sets. Prevents the "naked troop" bug.

---

## 5. Failure Behaviors & Critical Edge Cases

1. **Active Civil War Collision**:
   - If the player initializes `GovQuest` while the vanilla Civil War is actively running, `SetUpFactionOwners()` queries corrupted `CWOwner` keywords, causing hold ownership globals to desynchronize.
2. **Missing UIlib Scaleform Dependency**:
   - If `interface/uilib/` Flash files are missing or blocked by UI overhauls, `ShowTextInput()` fails to return, hanging the `ChangeName()` script thread and preventing teleportation out of the setup room.
3. **Faction Reset Protection**:
   - `FactionStarted` global acts as a single-entry gate. Once set to `1`, calling `MenuStart()` again is blocked to prevent accidental state wipes.
