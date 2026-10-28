# Static Structure Recovery: Conquest of Skyrim

- **Milestone**: M07 — Static Structure Recovery
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Data Sources**: `conquerskyrim.esp` record database and 1,539 Papyrus scripts (`scripts/source/*.psc`)
- **Status**: CONFIRMED

---

## 1. Architectural System Overview

Static code and record analysis reveals that *Conquest of Skyrim* is structured as a hub-and-spoke state machine orchestrated by five central quest scripts:

```text
                           ┌───────────────────────────────┐
                           │      CS_GovScriptFunctions    │
                           │  (Civil State & Administration)
                           └───────────────┬───────────────┘
                                           │
                   ┌───────────────────────┼───────────────────────┐
                   ▼                       ▼                       ▼
     ┌──────────────────────────┐ ┌──────────────────┐ ┌───────────────────────┐
     │CS_AttackPlanningNewScript│ │CS_BattleTracker  │ │CS_TransferOwnership   │
     │  (Invasion Logistics)    │ │(Ownership Arrays)│ │ (Marker & Holding Swap│
     └─────────────┬────────────┘ └────────┬─────────┘ └───────────┬───────────┘
                   │                       │                       │
                   └───────────────────────┼───────────────────────┘
                                           ▼
                           ┌───────────────────────────────┐
                           │      CS_AttackCityScript      │
                           │  (Siege Lifecycle & Palace)   │
                           └───────────────┬───────────────┘
                                           │
                                           ▼
                           ┌───────────────────────────────┐
                           │      CS_NegotiationScript     │
                           │(War Score & Diplomatic Peace) │
                           └───────────────────────────────┘
```

---

## 2. Core Controller Components & State Ownership

### 2.1 Civil & Government Engine (`CS_GovScriptFunctions` & `CS_GovPositions`)
- **Base Class**: `Quest`
- **Responsibility**: Governs cabinet appointments, throne room interactions, execution mechanics, and capital relocation.
- **Key ReferenceAliases**:
  - `Steward`: The primary domestic and economic administrator.
  - `ASIC`: Assistant / Second-in-Command (military coordinator).
  - `Blacksmith`: Quartermaster managing armory supply.
  - `CourtMage`: Court wizard advisor.
  - `Throne`: Interactive throne activator in conquered hold palaces.
  - `ChoppingBlock`: Public execution block activator for sentencing prisoners.
- **Hold Administration Arrays**:
  - `Stewards[]`, `Housecarls[]`, `JarlsImp[]`, `JarlsSons[]`, `GuardCaptainsImp[]`, `GuardCaptainsSons[]`.
- **Engineering Stocks**:
  - `GlobalVariable Property CatapultsPurchased Auto`
  - `GlobalVariable Property BatteringRamsPurchased Auto`
  - `GlobalVariable Property LaddersPurchased Auto`

### 2.2 Territory Transfer Engine (`CS_TransferOwnershipScript`)
- **Base Class**: `Quest`
- **Responsibility**: Atomic handover of Skyrim locations, holds, forts, and outposts between factions.
- **Master Holding Arrays**:
  - `FormList Property AllHoldings Auto`: Master registry of all conquerable locations in Skyrim.
  - `FormList Property AllHoldingsFort Auto`: Fortifications and military strongholds.
  - `FormList Property PlayerMarkers Auto`: Reference markers toggling player faction banners and furniture.
  - `FormList Property ImpMarkerLists Auto` & `SonsMarkerLists Auto`: Native Imperial and Stormcloak assets disabled upon capture.
- **Jurisdictional Transfer**:
  - `FormList Property CrimeFactions Auto`: Systematically detaches vanilla hold crime factions (`CrimeFactionWhiterun`, etc.) to reset bounties.
  - `FormList Property GovNPCsImp Auto` & `GovNPCsSons Auto`: Disables deposed native Jarls and Stewards.

### 2.3 Battle State Tracking (`CS_BattleTrackerScript`)
- **Base Class**: `Quest`
- **Responsibility**: High-level Boolean state arrays maintaining live ownership coordinates across the game world:
  - `bool[] Property Cities Auto`: 9-element array tracking Skyrim hold capitals.
  - `bool[] Property Forts Auto`: Array tracking military fort ownership.
  - `bool[] Property Camps Auto`: Array tracking bandit outposts and enemy camps.
  - `bool[] Property PlayerCamps Auto`: Tracks claimed player frontier camps.

### 2.4 Invasion Planning & Logistics (`CS_AttackPlanningNewScript`)
- **Base Class**: `Quest` (106 declared properties)
- **Responsibility**: Tactical prerequisite checking and army mobilization:
  - **Adjacency Enforcement**: Verifies territorial connection (`Message Property MessageNoHoldLink Auto`); prevents players from invading distant holds without a contiguous border.
  - **Military Superiority Evaluation**: Checks relative troop strengths (`StartMessageNoSuperiority`).
  - **Operational Rosters**: `InvasionCommanders`, `Commanders`, `SelectedCommanders`, `InvasionTarget`, `HoldLoc`.

### 2.5 Siege Lifecycle Controller (`CS_AttackCityScript`)
- **Base Class**: `Quest Conditional`
- **Responsibility**: Real-time management of city assaults and palace breach sequences:
  - **Simulation Throttling**:
    - `GlobalVariable Property CS_BattleSize Auto`: Regulates maximum concurrent actor spawns to protect the Papyrus VM from crashing.
    - `GlobalVariable Property CS_BattleSpeed Auto`: Tunes AI combat aggression and reinforcement cadence.
  - **Assault Mechanics**:
    - `FormList Property CatapultsList Auto` & `CatapultOpsList Auto`: Scripted artillery batteries.
    - `bool Property BatteringRamEnabled Auto`: Outer gate breaching.
  - **Dramatics & Resolution**:
    - `Scene Property JarlConfrontScene Auto`: Palace standoff with deposed Jarl.
    - `Scene Property JarlSurrenderScene Auto`: Formal capitulation.
    - `Scene Property ToVictoryScene Auto`: Post-siege victory celebration.

### 2.6 Diplomatic War Score Engine (`CS_NegotiationScript`)
- **Base Class**: `Quest`
- **Responsibility**: Bilateral diplomatic negotiation calculations:
  - `GlobalVariable Property WarScore Auto`, `WarScoreImp Auto`, `WarScoreSons Auto`.
  - `GlobalVariable Property TruceImp Auto`, `TruceSons Auto`.
  - `GlobalVariable Property Reparations Auto`.
  - Handles summit positioning via `EvaluateNegotiatingPackages()`.

---

## 3. Papyrus Script Hierarchy & Class Specialization

Quantitative analysis of 1,539 script sources (`.psc`):

| Parent Class | Count | Primary Operational Role |
| :--- | :---: | :--- |
| `extends TopicInfo` | 935 | Contextual dialogue responses (player speaking to commanders, stewards, jarls, bards, and soldiers). |
| `extends Quest` | 254 | Core state machines, economic update loops, and invasion controllers. |
| `extends Package` | 150 | AI package fragments (patrol march routes, two-column parades, formation locking). |
| `extends ReferenceAlias` | 88 | Dynamic actor tracking (Second-in-Command, captured rulers, appointed stewards). |
| `extends ObjectReference` | 47 | Interactive in-world objects (placeable banners, war maps, barricades, levers). |
| `extends Scene` | 33 | Scripted theatrical scenes (Moot voting, executions, coronations). |
| `extends ActiveMagicEffect` | 23 | Passive leadership auras, rally shouts, and stat buffs. |
| `extends Actor` | 6 | Specialized actor override behaviors. |
| `extends Perk` | 2 | Authority and combat specialization perks. |

---

## 4. UI Scaleform Bridge (`uilib_1.psc`)

The user interface bridges Papyrus to Scaleform Flash components via custom engine events:
- **Text Input**:
  - `UILIB_1_textInputOpen()` / `UILIB_1_textInputClose()`
  - Passes user-typed strings directly to Papyrus variables for custom faction and city naming.
- **List Menus**:
  - `UILIB_1_listMenuOpen()` / `UILIB_1_listMenuClose()`
  - Renders scrollable lists for banner heraldry, troop armor sets, and officer rosters.
- **Event Messaging**:
  - Binds via `RegisterForModEvent("UILIB_1_listMenuClose", ...)` to capture player selections asynchronously without freezing the game engine.

---

## 5. Identified Structural Invariants & Limitations

1. **State Centralization**: Live strategic ownership is centralized in `CS_BattleTrackerScript` and `CS_TransferOwnershipScript` rather than being scattered across individual cell scripts.
2. **Adjacency Constraint**: Invasion planning strictly enforces territorial contiguity (`MessageNoHoldLink`).
3. **Throttled Actor Caps**: Battle actor counts are actively constrained by `CS_BattleSize` to mitigate Creation Engine multi-actor physics crashes.
4. **Unknown / Unresolved**: The exact internal calculation loop for off-screen simulated battles (`FEAT-BAT-04`) is not housed in `CS_AttackCityScript` and is investigated in Phase 4.
