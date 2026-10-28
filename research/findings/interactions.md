# Player Interaction & Configuration Specification: Conquest of Skyrim

- **Milestone**: M12 — Player Interaction and Configuration
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_manageholding.psc` (Interactive banner activator menu)
  - `cs_mapmanagemenuscript.psc` (Tactical war map interface)
  - `cs_selectname.psc` (Scaleform `UIlib_1` text entry)
  - `cs_troopmanagemenuscript.psc` (Troop division, armor, and upgrade menus)
  - `conquerskyrim.esp` (513 `MESG` records and 935 `TopicInfo` dialogue responses)
- **Status**: CONFIRMED

---

## 1. Interaction Surface Architecture

*Conquest of Skyrim* employs a multi-tiered player interaction system designed to minimize immersion-breaking menus while providing full strategic command:

```text
Player Interaction Channels
  │
  ├── 1. Contextual Dialogue Interface (Primary Control Surface)
  │     ├── Steward: Civil administration, taxes, treasury, diplomatic letters
  │     ├── Commander: Recruitment, marching orders, formations, invasion planning, POWs
  │     ├── Blacksmith: Armory supply, equipment sets, troop weapon tiers
  │     └── Appointed Jarls: Hold governance, local garrison orders, fealty
  │
  ├── 2. Tactical War Table Maps (CS_MapManageMenuScript)
  │     ├── Contiguous invasion targeting (HoldLoc, CityLoc)
  │     ├── Patrol & garrison assignment (PatrolCommanders, GarrisonCommanders)
  │     └── Precondition feedback (HoldsNotLinked, MessageNoSuperiority)
  │
  ├── 3. In-World Object Activators (CS_ManageHolding)
  │     ├── Claimed Faction Banners: Outpost upgrade & resource management menus
  │     ├── Interactive Thrones: Holding court, judging civil petitions
  │     └── Public Chopping Blocks: Sentencing prisoners of war & traitors
  │
  └── 4. Scaleform GUI & Configuration Bridges
        ├── UIlib v1: Virtual keyboard text input (faction/city naming)
        └── SkyUI MCM & SettingsMenu Item: Global engine parameter toggles
```

---

## 2. In-World Activator & Contextual Menu Interactions

### 2.1 The Claimed Banner Activator (`CS_ManageHolding.psc`)
When the player plants a banner via `BannerSpell` in a cleared camp, mine, or redoubt, an interactive `CS_ManageHolding` script attaches to the banner:
- **`Event OnActivate()` Hook**:
  - Converts vanilla location to Conquest holding: `NewHolding = ConvertLocToCS(self.GetCurrentLocation())`.
  - Opens `ManageHoldingMenu(Holding, SelectedBanner)`.
- **Holding Menu Options**:
  - *Check Resource Output*: Displays daily metal, stone, or food yield (`CS_ManageHoldingMine`, `CS_ManageHoldingHideout`).
  - *Hire Workers/Miners*: Deducts treasury gold to assign laborers, boosting daily resource extraction.
  - *Station Local Guards*: Recruits a local militia garrison to deter enemy counter-raids (`CS_RecruitGuardsVillage`).

### 2.2 The Tactical War Table (`CS_MapManageMenuScript.psc`)
Interacting with the strategic war map opens a map-driven command interface:
- **Command Dispatch**:
  - Assigns commanders to active division roles: `PatrolCommanders[]`, `GarrisonCommanders[]`, `IncomingCommanders[]`.
- **Validation Messages & Error Feedback**:
  - `Message Property HoldsNotLinked Auto`: Rejects invasion orders if the target hold capital does not border a player-controlled holding.
  - `Message Property MessageNoSuperiority Auto`: Warns the player if their army's troop strength is critically inferior to the defender's garrison.
- **Roster & Prisoner Telemetry**:
  - Reports `TotalArmies`, `MaxArmies` (capped at 10), and `PrisonerCount`.

---

## 3. Dialogue Command Surface (935 `TopicInfo` Scripts)

In v0.3.x, the vast majority of operational commands were relocated from abstract menus into fully voiced NPC dialogue trees:

### 3.1 Civil Dialogue with the Steward
| Dialogue Branch | Input / Cost | Result / System Effect |
| :--- | :--- | :--- |
| **"I want to manage the treasury"** | Gold quantity | Deposits or withdraws gold into `CS_FactionTreasury` global. |
| **"Let us adjust hold taxes"** | Selection (Low / Normal / High / Extortion) | Updates hold tax globals; high taxes generate gold but increase `Unrest` and reduce `Loyalty`. |
| **"Send an emissary to [Faction]"** | Target faction | Dispatches diplomatic letters to General Tullius or Ulfric Stormcloak (`CS_NegotiationScript`). |
| **"Relocate our capital"** | Conquered capital | Relocates player court and throne markers to the new hold capital. |

### 3.2 Military Dialogue with the Commander (Second-in-Command)
| Dialogue Branch | Input / Cost | Result / System Effect |
| :--- | :--- | :--- |
| **"Plan an invasion"** | Target location | Opens `CS_AttackPlanningNewScript` invasion planning interface. |
| **"Assume battle formation"** | Formation type (5x4 Block / Line / Charge) | Locks troops into tactical coordinate offsets (`cs_attackcitysiegedrum_*.psc`). |
| **"What should we do with the prisoners?"** | Ransom / Imprison / Execute | Ransoms POWs for gold, transfers them to fort dungeons, or initiates execution scenes. |
| **"Sound the victory parade"** | Conquered city | Initiates celebratory parade with custom drum music (`conquest_of_skyrim__parade_drums.xwm`). |

### 3.3 Quartermaster Dialogue with the Blacksmith
| Dialogue Branch | Input / Cost | Result / System Effect |
| :--- | :--- | :--- |
| **"Standardize soldier equipment"** | Armor Tier selection | Sets default outfit forms for all recruits across the army. |
| **"Supply weapon stocks"** | Raw metals (Iron, Steel, Ebony ingots) | Crafts standardized weapons in the faction armory to prevent unarmed spawns. |

---

## 4. Scaleform GUI Bridges (`UIlib v1`)

Where Skyrim's native dialogue trees are insufficient, the mod invokes embedded Flash menus via `UIlib_1`:
1. **Dynamic Text Input (`UILIB_1_textInputOpen`)**:
   - Opens an on-screen text box allowing the player to type custom strings (e.g., Faction Name, Army Prefix).
   - Binds to `FactionNameChestObject.SetName()` and `ChestObject.SetName()`.
2. **Scrollable List Selection (`UILIB_1_listMenuOpen`)**:
   - Renders scrollable lists for selecting custom banner heraldry from 60+ Incaendo banner designs.

---

## 5. Configuration Surfaces (MCM & Fallbacks)

### 5.1 SkyUI Mod Configuration Menu (MCM)
- **`CS_BattleSize`**: Slider regulating maximum concurrent combatant actors in worldspaces (default: 40; adjustable to reduce engine crashes on lower-end systems).
- **`CS_BattleSpeed`**: Slider tuning AI reinforcement spawn pacing.
- **Economic Yield Multipliers**: Sliders adjusting daily gold taxation and resource harvest rates.
- **Diagnostic Toggles**: Enables logging of troop death events and holding ownership changes.

### 5.2 Controller & Console Fallback (`SettingsMenu` Inventory Item)
- Players on Xbox or PC players using gamepads without SKSE/SkyUI receive the `SettingsMenu` inventory misc item.
- Activating the item in inventory opens native message box menus providing essential configuration toggles without requiring SkyUI MCM.

---

## 6. Usability & Accessibility Limitations

1. **Dialogue Tree Traversal Depth**:
   - Managing dozens of outposts and 10 separate armies through dialogue requires navigating deep multi-tier conversation trees, leading to dialogue fatigue.
2. **Scaleform Input on Gamepads**:
   - The `UIlib` text input menu requires keyboard character input, which can be awkward on Steam Deck or Xbox controller setups.
3. **Delayed Visual Feedback**:
   - Large banner replacements (`CS_ChangeBanners.ReplaceBanners()`) iterate through dozens of references in the background, displaying the notice *"Replacing Banners. This may take awhile..."* while Skyrim's Papyrus VM completes object placement.
