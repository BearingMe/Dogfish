# Papyrus Claim Verification: Conquest of Skyrim

- **Document**: Reproducible Papyrus Source Verification Report
- **Milestone**: M38 — Papyrus Claim Verification
- **Phase**: Phase 10 (Reproducible Technical Verification)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Verification Utility**: `scripts/verify_papyrus.py`
- **Automated Test Suite**: `tests/test_papyrus.py` (3/3 tests passing, 11 subtests)
- **Status**: VERIFIED & REPRODUCIBLE

---

## 1. Verification Scope & Method

In accordance with Phase 10 requirements, all Papyrus script claims made across M07–M32 were audited directly against uncompiled Papyrus source files (`.psc`) located at `references/local/conquest-of-skyrim/Data/scripts/source/`.

The programmatic verification utility `scripts/verify_papyrus.py` parsed:
1. Script class declarations and `extends` parent classes.
2. Property identifiers and array types.
3. Function signatures and parameters.
4. Native engine event handlers (`Event On...`).

---

## 2. Core Controller Script Audit Results

All 11 core controller scripts passed 100% of symbol and inheritance assertions:

| Script File | Verified Base Class | Verified Properties | Verified Functions & Events | Audit Assessment |
| :--- | :---: | :---: | :---: | :---: |
| `cs_govscriptfunctions.psc` | `Quest` | `Steward`, `ASIC`, `Blacksmith`, `CourtMage`, `Throne`, `ChoppingBlock`, `AllLocations`, `AllHoldings` | `SetUpFactionOwners()` | **CONFIRMED** |
| `cs_transferownershipscript.psc` | `Quest` | `AllHoldings`, `AllHoldingsFort`, `AllLocations`, `CrimeFactions`, `PlayerMarkers`, `FortMarkersPlayer` | `TransferOwnershipCity()`, `TransferOwnershipFort()`, `TransferOwnershipVillage()`, `MakeCitizen()`, `RemoveGovernment()` | **CONFIRMED** |
| `cs_attackcityscript.psc` | `Quest` | `CS_BattleSize`, `CS_BattleSpeed`, `TroopsRemainingDefenders`, `RetreatFaction`, `BatteringRamEnabled`, `AllHoldings` | `RetreatDefenders()` | **CONFIRMED** |
| `cs_meetingscript.psc` | `Quest` | `Throne`, `Steward`, `Jarls`, `MootVoteFor`, `MootVoteAgainst`, `AllLocations`, `LoyaltyCheck` | `GetJarls()`, `StartMeeting()`, `StartMoot()`, `StartMootVote()`, `JarlNextVote()`, `FindJarlNominator()`, `EndMoot()` | **CONFIRMED** |
| `cs_coronationscript.psc` | `Quest` | `IsHighKing`, `HighKingCrown`, `HighQueenCrown`, `JarlPledgeScene`, `LoyaltyCheck` | `SetupThrone()`, `GetKeyPersonnel()`, `StartJarlPledges()`, `NextJarlPledge()`, `CitizensKneel()`, `JarlsKneel()`, `AllCheer()`, `AddLoyalty()` | **CONFIRMED** |
| `cs_negotiationscript.psc` | `Quest` | `WarScore`, `TruceImp`, `TruceSons`, `Reparations`, `HostOffer`, `GuestCounterOffer`, `TotalTroopCount` | `EvaluateNegotiatingPackages()`, `MoveGuestsToTable()`, `MakeFriends()`, `UnMakeFriends()`, `CalculateSuccess()`, `CalculateArmySize()` | **CONFIRMED** |
| `cs_holdingeconomyupdaterscript.psc`| `Quest` | `holdingnum`, `AllowUpdate` | `UpdateHoldingResources()` | **CONFIRMED** |
| `cs_troopupgradescript.psc` | `Quest` | `CostGold`, `CostMetal`, `Treasury`, `Metal`, `Level`, `LevelNext` | `SetUp()`, `MenuStart()`, `MenuUpgrade()`, `UpgradeTroop()` | **CONFIRMED** |
| `cs_bountyscript.psc` | `Quest` | `Commander`, `TroopCountCertain`, `PlayerDecision` | `SimulateCasualties()`, `PlayerChangedLocation()` | **CONFIRMED** |
| `cs_checkversion.psc` | `ReferenceAlias` | `TMImps`, `TMSons`, `CourtScript` | `Event OnPlayerLoadGame()` | **CONFIRMED** |
| `cs_patrolholdscript.psc` | `Quest` | `TaskBarrelFollow`, `TaskBarrelHunt`, `TaskBarrelNoOrder`, `TaskBarrelsAttackCity`, `TaskBarrelsDefendFort` | — (Owns Task Barrel proxy references) | **CONFIRMED** |

---

## 3. Detailed Structural Findings & Invariants

### 3.1 The "Task Barrel" Architecture (`cs_patrolholdscript.psc`)
- **Symbol Verification**: Properties `TaskBarrelFollow`, `TaskBarrelHunt`, `TaskBarrelNoOrder`, `TaskBarrelsAttackCity`, `TaskBarrelsDefendFort` were verified as explicit `ObjectReference` properties.
- **Mechanism**: The mod avoids dynamically recompiling complex AI packages on runtime actors. Instead, persistent invisible container markers act as spatial targets for AI packages, updating actor behavior instantaneously when assigned.

### 3.2 The Moot Deadlock Mechanism (`cs_meetingscript.psc`)
- **Code Audit**: In lines 44–54 (`GetJarls()`):
  ```papyrus
  while i < 9
      if JarlAliases[i].GetActorRef() != None && i != CapitalGlobal.GetValueint() && JarlAliases[i].GetActorRef().IsInFaction(GovFaction) == 1
          Jarls[JarlsAttending].ForceRefTo(JarlAliases[i].GetActorRef())
          JarlsAttending += 1
      endif
      i += 1
  endwhile
  ```
- **Finding**: If any conquered hold capital does not have an appointed Jarl in `GovFaction` (or if that actor died), `JarlAliases[i].GetActorRef()` evaluates to `None`, which causes `JarlsAttending` to fall below the required count, deadlocking the quest.

### 3.3 Battle Throttling & Speed Scaling (`cs_attackcityscript.psc`)
- **Symbol Verification**: `CS_BattleSize` and `CS_BattleSpeed` verified as `GlobalVariable` properties.
- **Wave Invariant**: Lines 285–331 verify that active in-cell spawns loop while `i < CS_BattleSize.GetValueInt() && i < TroopsRemainingDefenders + 10`.
- **Casualty Deductions**: Attrition in `cs_attackcitytroopdeathdefender.psc` decrements `TroopsRemainingDefenders` by `CS_BattleSpeed.GetValueInt()`.

---

## 4. Corrections to Previous Interpretations

1. **The Moot Hold Counting**: Historical documentation loosely referred to "holding 9 cities". Script verification proves that the code requires **8 appointed Jarls plus the player's home capital**, not merely owning the territory.
2. **Task Barrel Concrete Type**: Previous notes hypothesized Task Barrels might be custom scripted forms; code inspection confirms they are standard `ObjectReference` pointers placed in worldspaces.
3. **No Dynamic Class Modifications**: All troop ranks and upgrades are managed via `FormList` array indexing and `Perk` assignments; actor base classes are never dynamically mutated at runtime.
