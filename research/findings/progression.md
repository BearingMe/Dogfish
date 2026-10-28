# Progression & Victory Specification: Conquest of Skyrim

- **Milestone**: M11 — Progression and Victory
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_meetingscript.psc` (The Moot convocation, nomination, and voting tally)
  - `cs_coronationscript.psc` (Ceremonial coronation, kneeling animations, crown award)
  - `cs_recruitjarl.psc` (`JarlAliases[]`, `GovFaction`)
  - `cs_checkversion.psc` (`CapitalGlobal`, `TMImp.Defeated`, `TMSons.Defeated`)
  - `cs__sf_cs_meetingmootvote_05408eb4.psc` (Moot voting scene fragment)
- **Status**: CONFIRMED

---

## 1. Main Progression Loop Architecture

The overarching campaign progression advances through six sequential tiers from outlaw rebellion to high monarchy:

```text
Tier 1: Outlaw Faction Founding
   ├── Read starter book -> MenuStart() -> Stage 0
   └── Claim initial frontier bandit camp/mine with BannerSpell
         │
Tier 2: Outpost Network & Economic Foundation
   ├── Appoint executive cabinet: Steward, Commander (ASIC), Blacksmith
   └── Seize independent bandit camps & mines to establish raw resource flows
         │
Tier 3: Territorial Invasions & Hold Captures
   ├── Launch contiguous hold border campaigns (CS_AttackPlanningNewScript)
   └── Siege and capture minor capitals (Dawnstar, Falkreath, Morthal, Winterhold)
         │
Tier 4: Capital City Sieges & Province Domination
   ├── Battering rams & catapult assaults on major walled capitals
   └── Conquer Markarth, Riften, Solitude, Whiterun, Windhelm
         │
Tier 5: Convening The Moot (CS_MeetingScript)
   ├── All 9 hold capitals secured; Imperial & Stormcloak armies defeated
   └── Jarl nomination scene -> Hold loyalty roll-call voting
         │
Tier 6: Royal Coronation (CS_CoronationScript)
   ├── High King / High Queen Crown awarded (IsHighKing = 1)
   └── Attending Jarls fealty pledges, citizen kneeling, and royal victory parade
```

---

## 2. Endgame Mechanics: The Moot (`CS_MeetingScript`)

The Moot represents the constitutional and diplomatic climax of the mod's narrative progression.

### 2.1 Preconditions & Convocation
- **Holding Prerequisite**: All 9 Skyrim holds must be cleared of hostile Imperial and Stormcloak control.
- **Ruler Registry (`GetJarls()`)**:
  - Iterates `while i < 9`:
    ```papyrus
    if JarlAliases[i].GetActorRef() != None && i != CapitalGlobal.GetValueint() && JarlAliases[i].GetActorRef().IsInFaction(GovFaction) == 1
        Jarls[JarlsAttending].ForceRefTo(JarlAliases[i].GetActorRef())
        JarlsAttending += 1
    endif
    ```
  - Appointed Jarls travel to the player's designated capital palace (`Thrones[CapitalGlobal]`).

### 2.2 Nomination Logic (`FindJarlNominator()`)
The meeting dynamically determines who nominates the player character:
1. **Friendly Jarl Nomination**:
   - The script scans all hold locations for `Location.GetKeywordData(Loyalty) >= 60`.
   - If an appointed Jarl possesses 60+ loyalty, that Jarl formally nominates the player, triggering `MootNominatedScene.ForceStart()`.
2. **Self-Nomination Fallback**:
   - If no Jarl meets the 60 loyalty threshold (`nominatorholdingnum == -1`), a random attending Jarl is forced into `JarlNominator`, and the player must make a contentious self-nomination speech (`MootSelfNominationScene.ForceStart()`).

### 2.3 Roll-Call Voting Engine (`StartMootVote()` & `JarlNextVote()`)
- **Tally Counters**: `MootVoteFor` and `MootVoteAgainst` initialized to 0.
- **Vote Resolution per Jarl**:
  - Each attending Jarl steps forward (`JarlSpeaking.ForceRefTo(Jarls[VotingJarls])`).
  - Evaluates local hold loyalty:
    ```papyrus
    MootVoteHoldingLoyalty = MootVoteHolding.GetKeywordData(Loyalty) as int
    ```
  - High loyalty Jarls vote **FOR**; resentful or low-loyalty Jarls vote **AGAINST**.
- **Player Capital Vote**: The player's home capital automatically adds +1 to `MootVoteFor`.

### 2.4 Branching Outcomes (`EndMoot()`)
- **Victory Condition (`MootVoteFor >= MootVoteAgainst`)**:
  - Unlocks `MootVictoryScene.ForceStart()`.
  - Disloyal or unassigned holds suffer a severe diplomatic penalty:
    ```papyrus
    LoyaltyCheck.AdjustLoyalty(HoldingLocation, -80)
    ```
  - Launches `CoronationQuest` and schedules `CoronationDay`.
- **Defeat / Failure Condition (`MootVoteFor < MootVoteAgainst`)**:
  - Unlocks `MootFailureScene.ForceStart()`.
  - The Moot rejects the player's claim; high king status is denied, requiring the player to replace dissident Jarls or raise hold loyalties before convening again.

---

## 3. Coronation Ceremony (`CS_CoronationScript`)

When `GameDaysPassed >= CoronationDay`, the coronation sequence initiates at Stage 10:

### 3.1 Ceremonial Staging & Animations
- **Personnel Positioning (`MoveToMarkers()`)**:
  - Cabinet members (`Steward`, `ASIC`, `Blacksmith`, `CourtMage`) take flanking positions beside the throne.
  - Attending Jarls assemble at `JarlMarkers[]`.
  - Civilians and townspeople populate `CitizenMarkers[]`.
- **Choreographed Animation Events**:
  - `JarlsKneel()` / `CitizensKneel()`: Sends `IdleKneelingEnter` animation events to all attending actors.
  - `AllCheer()`: Triggers randomized applause animations (`IdleApplaud2` through `IdleApplaud5`).
  - `AllLookAtPlayer()`: Forces all attending NPCs to focus head-tracking on `Game.GetPlayer()`.

### 3.2 Jarl Fealty Pledges (`StartJarlPledges()`)
- Iterates sequentially through each attending Jarl (`JarlTribute.ForceRefTo(...)`).
- Each Jarl approaches the throne and delivers an oath of fealty in `JarlPledgeScene`.
- Grants a permanent +30 loyalty bonus to that Jarl's holding:
  ```papyrus
  LoyaltyCheck.AdjustLoyalty(HoldingLocation, 30)
  ```

### 3.3 Royal Rewards & State Finalization (Stage 21)
- **Crown Delivery**:
  - Equips player with `HighKingCrown` (male) or `HighQueenCrown` (female) armor records.
- **Global Flag Finalization**:
  - Sets `IsHighKing.SetValue(1)`.
  - Enables sovereign High Monarch dialogue perks across all holds in Skyrim.

---

## 4. Known Progression Failures & Root Cause Analysis

### 4.1 The Moot Convocation Deadlock (`ISSUE-QST-01`)
- **Symptom**: Player conquers all 9 holds, but the dialogue option to call The Moot never appears with the Steward.
- **Root Cause Verified in Script**:
  - `GetJarls()` requires that `(GovQuest as CS_RecruitJarl).JarlAliases[i].GetActorRef() != None` for all holds other than `CapitalGlobal`.
  - If the player conquered a hold but failed to appoint a custom Jarl, or if an appointed Jarl was killed, `JarlAliases[i]` is null, aborting the count.
  - Furthermore, v0.3.9 contains an off-by-one verification check where Winterhold or Falkreath territory tags fail to register in `Cities[]`.
- **Workaround / Fix**:
  - Ensure every hold has an appointed Jarl in `GovFaction`.
  - Apply *Conquest of Skyrim Fixes* (Nexus #119992).
  - Console override: `setstage <MeetingQuestID> 10`.

### 4.2 Fort Amol Invasion Trigger Freeze (`ISSUE-QST-02`)
- **Symptom**: Approaching Fort Amol does not start the battle quest.
- **Root Cause**: Proximity trigger box exterior alias fails to trigger `OnTriggerEnter()` if Eastmarch border variables were initialized out of order.
- **Fix**: Patched directly in community bugfix patch #119992.
