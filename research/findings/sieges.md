# Siege Warfare Specification: Conquest of Skyrim

- **Milestone**: M16 — Siege Warfare
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_attackcitysiegescript.psc` (Siege setup, staging camps, catapults, pincer attacks)
  - `cs_attackplanningnewscript.psc` (Precondition validation, siege equipment purchases)
  - `cs_attackcityscript.psc` (Breach execution, palace standoff, capitulation)
  - `cs_transferownershipscript.psc` (`TransferOwnershipCity`)
- **Status**: CONFIRMED

---

## 1. Siege Warfare Lifecycle Architecture

Sieges represent the most complex operational military sequences in *Conquest of Skyrim*, bridging strategic map planning with multi-cell tactical assaults:

```text
1. Campaign Authorization (CS_AttackPlanningNewScript)
   ├── Validate territorial contiguity (MessageNoHoldLink check)
   ├── Evaluate military superiority ratio (MessageNoSuperiority check)
   └── Requisition siege apparatus: BatteringRams, Catapults, Ladders
         │
2. Forward Encampment & Staging (CS_AttackCitySiegeScript)
   ├── Deploy staging camps: CampMarker1 and/or CampMarker2 outside city walls
   ├── Add markers to player world map (CampMarker.AddToMap(true))
   └── Converge participating division commanders to forward staging zones
         │
3. Outer Gate Assault & Artillery Bombardment
   ├── Single vs. Dual-Front Attack (DoubleAttack pincer configuration)
   ├── Enable siege artillery (CatapultsList & CatapultOpsList)
   └── Breach outer wooden barricades and gate barriers (BatteringRamEnabled)
         │
4. Urban Street Combat (CS_AttackCityScript)
   ├── Wave combat throttled by CS_BattleSize (Melee, Ranged, Healers, Buffers)
   └── Push surviving defenders through city streets toward the Jarl's Keep
         │
5. Palace Breach & Jarl Confrontation
   ├── Interior cell breach into Jarl's palace (Dragonsreach, Blue Palace, etc.)
   └── Scripted resolution: JarlConfrontScene -> JarlSurrenderScene
         │
6. Holding Transfer & Sovereign Occupation
   ├── TransferOwnershipCity(citynum, 69) -> Resets crime gold to 0
   ├── Depose native ruler to CitizenFaction -> Appoint loyal Governor
   └── Trigger victory parade with drummers (drums of chaos.xwm)
```

---

## 2. Siege Preparation & Engineering Equipment

Siege gear is purchased through the Second-in-Command or Faction Blacksmith during the planning phase (`CS_AttackPlanningNewScript`):

| Siege Asset | Controlling Global Variable | Technical Implementation | Operational Role |
| :--- | :--- | :--- | :--- |
| **Battering Rams** | `BatteringRamsPurchased` | `bool Property BatteringRamEnabled` | Enables assault squads to batter down fortified exterior city gates. |
| **Catapults / Artillery** | `CatapultsPurchased` | `FormList CatapultsList` / `CatapultOpsList` | Spawns functional artillery batteries outside walls firing explosive munitions at battlements. |
| **Siege Ladders** | `LaddersPurchased` | `int Property NumLadders` | Facilitates wall escalade in fortified holds. |

---

## 3. Staging & Attack Vector Selection

### 3.1 Forward Encampments
- Rather than spawning armies directly at city gates, the mod spawns forward military camps in the wilderness outside the hold capital:
  - `CampMarker1` (Primary Approach)
  - `CampMarker2` (Secondary Approach)
- Markers are dynamically enabled and added to the player's map:
  ```papyrus
  CampMarker1.GetRef().enable()
  CampMarker1.GetRef().AddToMap(true)
  ```

### 3.2 Dual-Front Pincer Assaults (`DoubleAttack`)
- The invasion planner supports flanking strategies:
  - **Single Vector (`AttackEntrance == 1` or `2`)**: Armies concentrate at one gate.
  - **Pincer Attack (`DoubleAttack == true`)**:
    ```papyrus
    int NumCommandersHalf = math.floor(0.5 * (NumCommanders as float))
    ; Divides participating division commanders equally between CommanderMarkers1 and CommanderMarkers2
    ```
  - Forces defenders to split their garrison between multiple gates.

---

## 4. Tactical Squad Composition in Urban Combat

During the breach, the siege engine evaluates tactical roles across spawned combatant FormLists:
- **Melee Vanguard (`CS_ArmyMeleeFaction`)**: Heavy warriors and foot soldiers leading the street charge.
- **Ranged Suppressors (`CS_ArmyRangedFaction`)**: Archers and crossbowmen targeting defenders on ramparts.
- **Field Healers (`CS_ArmyHealerFaction`)**: Restoration mages casting targeted healing on frontline allies.
- **Tactical Buffers (`CS_ArmyBufferFaction`)**: Alteration mages casting defensive stoneflesh wards and courage buffs.
- **War Drummers (`CS_ArmyDrummerFaction`)**: Equipped with `WarDrums`, playing audio streams to boost friendly movement and attack speed.

---

## 5. Palace Breach, Capitulation & Hold Transfer

### 5.1 Palace Standoff (`JarlConfrontScene`)
When street defenders are eliminated or forced into retreat:
1. The assault transitions into the interior seat of government (e.g., Palace of the Kings, Understone Keep).
2. The remaining Jarl's personal guard makes a final stand around the throne.
3. Once guards fall, `JarlConfrontScene` initiates, giving the player dialogue options:
   - Accept unconditional surrender (`JarlSurrenderScene`).
   - Imprison the deposed Jarl in fort dungeons.
   - Order public execution on the chopping block.

### 5.2 Atomic Holding Transfer
The siege concludes with `TransferOwnershipCity(citynum, 69)`:
- Sets `AllHoldings[citynum]` global to `1`.
- Clears hold crime bounties to zero.
- Replaces city banners with custom faction heraldry.
- Unseats native Jarl to `CitizenFaction` and unlocks governor appointment dialogue.

---

## 6. Failure Modes, Interruptions & Vanilla Clashes

1. **Vanilla Civil War Navmesh Conflicts**:
   - Exterior city overhauls (e.g., *JK's Skyrim*, *The Great Cities*) alter wall structures and navmeshes. Catapults or battering ram squads can become trapped in pathing loops, causing siege scripts to hang.
2. **Aborting an Active Siege (`CancellingAttack`)**:
   - If the player calls off an attack via dialogue, `CS_AttackCitySiegeScript` executes clean-up routines: disables staging camps, returns division commanders to their prior patrol task barrels, and restores vanilla guard markers.
3. **Essential NPC Traps**:
   - If a resident NPC in the palace is flagged essential by an active vanilla quest (e.g., Blood on the Ice in Windhelm), they cannot be defeated or routed, stalling `JarlConfrontScene`.
