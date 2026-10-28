# Combat Simulation Specification: Conquest of Skyrim

- **Milestone**: M15 — Combat Simulation
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_attackcityscript.psc` (Battle lifecycle, retreat execution, spawn loop)
  - `cs_attackcitytroopdeathdefender.psc` (Defender attrition, reinforcement wave triggers)
  - `cs_attackcitytroopdeathattacker.psc` (Attacker attrition, specialist mage accounting)
  - `cs_attackfortcamptroopdeathattacker.psc` (Fort battle percentage HUD tracking)
  - `cs_attackcitytroopdeathtrigger.psc` (`Event OnDying` bridge)
- **Status**: CONFIRMED

---

## 1. Battle Lifecycle Architecture

Combat engagements in *Conquest of Skyrim* transition through a structured state machine designed to simulate large-scale warfare while operating strictly within the Skyrim Creation Engine's multi-actor actor memory limits:

```text
1. Engagement Initialization
   ├── Armies arrive at forward staging markers (CS_CommanderMovingScript)
   └── Proximity trigger or war table dispatch initializes CS_AttackCityScript / CS_AttackFortCampScript
         │
2. Initial Combatant Spawning (CS_BattleSize Cap)
   ├── TroopsRemainingAttackers = AttackersInitial; TroopsRemainingDefenders = DefendersInitial
   └── Spawns actors up to CS_BattleSize.GetValueInt() at AttackMarkers and DefenderMarkers
         │
3. Real-Time Wave Combat & Attrition
   ├── Actor killed -> Event OnDying() fires -> Invokes TroopDeathDefender() / TroopDeathPlayer()
   ├── TroopsRemaining decremented by CS_BattleSpeed.GetValueInt()
   └── Every 20 deaths (20, 40, 60, 80, 100) -> Spawns next reinforcement wave
         │
4. Tactical Formation & Morale Rout (RetreatDefenders)
   ├── Defenders routed or depleted -> Invokes RetreatDefenders()
   ├── Surviving actors assigned to RetreatFaction -> Triggering flee AI packages
   └── Defeated troops marked for surrender -> PrisonerScript.PlayerTakePrisoner()
         │
5. Battle Termination & Victory Resolution
   ├── Attacker Victory -> ToVictoryScene.ForceStart() -> Advances to palace confrontation
   └── Post-Battle Cleanup -> OnCellDetach() releases corpse references; POWs held by ASIC
```

---

## 2. In-Cell Actor Throttling & Simulation Parameters

To prevent the classic Creation Engine physics freeze and actor buffer overflow when dozens of combatants clash, the mod employs two global throttles:

### 2.1 Battle Size Throttling (`CS_BattleSize`)
- **Default Value**: Typically 40 concurrent actors (configurable via MCM).
- **Enforcement Loop**:
  ```papyrus
  while i < CS_BattleSize.GetValueInt() && i < TroopsRemainingDefenders + 10
      ; Spawns next active combatant from reserve roster
  ```
- **Architectural Impact**: An army of 150 soldiers is not dumped into the worldspace simultaneously. Instead, only up to `CS_BattleSize` combatants exist in the active cell at any instant.

### 2.2 Battle Attrition Speed (`CS_BattleSpeed`)
- **Default Value**: 1 (scalable in settings).
- **Function**: When an actor dies, `TroopsRemaining` is decremented by `CS_BattleSpeed.GetValueInt()`.
  ```papyrus
  AttackCityScript.TroopsRemainingDefenders -= AttackCityScript.CS_BattleSpeed.GetValueInt()
  TotalDefenderDeaths += AttackCityScript.CS_BattleSpeed.GetValueInt()
  ```
- **Scale Simulation**: Allows players on low-spec hardware to simulate large 200-soldier battles with fewer physical actor deaths by setting a higher battle speed factor.

---

## 3. Reinforcement Wave Mechanics

Reinforcements do not trickle in continuously; they spawn in coordinated tactical waves anchored to casualty milestones:
1. **Wave Trigger Thresholds**:
   - In `CS_AttackCityTroopDeathDefender.psc`:
     ```papyrus
     If TotalDefenderDeaths == 20 || TotalDefenderDeaths == 40 || TotalDefenderDeaths == 60 || TotalDefenderDeaths == 80 || TotalDefenderDeaths == 100
         ObjectReference SpawnPoint = Spawns2.GetAt(utility.randomint(0, Spawns2.GetSize() - 1)) as ObjectReference
         ; Spawns reinforcement wave at randomized defense rally point
     ```
2. **HUD Percentage Tracking**:
   - Updates the quest objective progress bar dynamically:
     ```papyrus
     NewPercentRemaining = math.floor(100 * (TroopsRemainingAttackers as float / AttackersInitial as float))
     ModObjectiveGlobal(-percentchange, TroopsRemainingAttackersPercent, 11)
     ```

---

## 4. Specialist Unit Impact on Battle Flow

Casualty handlers specifically track specialized unit deaths:
- **Combat Healers (`RestorationMage`)**:
  - `TroopDeathPlayer()` decrements `AttackCityScript.HealersDeployed`.
  - Loss of healers directly reduces passive healing spells cast on frontline foot soldiers.
- **Alteration Mages (`AlterationMage`)**:
  - Decrements defensive ward protection pools for accompanying infantry.
- **Battle Drummers (`CS_ArmyDrummerFaction`)**:
  - Playing drum audio (`BattleDrumsSound`) applies local morale buffs, increasing allied attack speed.

---

## 5. Rout, Surrender & Post-Battle Cleanup

### 5.1 Retreat Execution (`RetreatDefenders()`)
When defender reserves are exhausted:
1. **Fleeing AI Packages**:
   - Surviving defenders and officers are added to `RetreatFaction`:
     ```papyrus
     DefeatedTroop.AddToFaction(RetreatFaction)
     DefeatedTroop.EvaluatePackage()
     ```
   - This immediately clears their attack AI and causes them to sprint toward the nearest exit gate or hold border.
2. **Field Prisoner Processing**:
   - Script marks defeated troops for surrender:
     ```papyrus
     (DefeatedTroop as CS_DefenderDeath).MarkForSurrender()
     PrisonerScript.PlayerTakePrisoner(DefeatedTroop, "Troop", enemyfactionnum)
     ```
   - Equips captives with `PrisonerCuffs` and registers them under the Second-in-Command (`ASIC`).

### 5.2 Corpse Cleanup & Memory Recovery
- Dead combatants are registered in temporary arrays and cleaned when the player transitions cells (`Event OnCellDetach()`), preventing savegame bloat and uncleaned Havok physics corpses.

---

## 6. Identified Inferences & Unknown Formulas

- **CONFIRMED**: Active cell combat lifecycle, `CS_BattleSize` throttling, 20-casualty reinforcement waves, `RetreatFaction` routing, and POW conversion are directly verified in Papyrus source code.
- **UNKNOWN**: Off-screen automated battle resolution formulas (when battles occur in distant holds without the player present) are not processed through `CS_AttackCityScript` and appear to use a simplified comparison of `TotalTroopCount` and commander level; exact mathematical weighting remains unverified.
