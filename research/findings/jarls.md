# Jarls & Hold Administration Specification: Conquest of Skyrim

- **Milestone**: M22 — Jarls and Hold Administration
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_recruitjarl.psc` (`RecruitJarl`, `FireJarl`, `SetUpJarlInitials`, `JarlAliases[]`)
  - `cs_loyaltycheckscript.psc` (`JarlLoyalty`, `Loyalty`, `AdjustLoyalty`, `ProtestQuest`)
  - `cs_govpositions.psc` (`JarlsImp[]`, `JarlsSons[]`, `HousecarlsImp[]`, `StewardsImp[]`)
- **Status**: CONFIRMED

---

## 1. Jarlship Lifecycle Architecture

Hold administration in *Conquest of Skyrim* relies on delegating regional governance to appointed rulers across the 9 holds of Skyrim (`holdnum 0..8`):

```text
1. Capital Conquest & Deposing Native Ruler
   └── Native Jarl unseated -> Added to CitizenFaction -> JarlAliases[holdnum] cleared
         │
2. Jarl Appointment (CS_RecruitJarl.psc)
   ├── Player approaches friendly NPC -> Offers Jarlship of holdnum
   ├── Citizen added to GovFaction -> Binds to JarlAliases[holdnum]
   └── JarlStartManageEconomy(Citizen) -> Triggers CS_StoryJarlChooseGov
         │
3. Hold Administration & Court Staffing
   ├── Appoint local Hold Steward (StewardAliases[holdnum])
   ├── Appoint local Hold Housecarl (HousecarlAliases[holdnum])
   └── Station municipal garrison guards (CS_NumGuards)
         │
4. Dynamic Loyalty & Stability Monitoring (CS_LoyaltyCheckScript.psc)
   ├── Evaluates Citizen Loyalty and JarlLoyalty
   ├── Modifiers applied: Food supply, Garrisons, Court sessions, Tax rates
   └── High Loyalty (>= 60) -> Unlocks nomination in The Moot (M11)
         │
5. Insurrection or Dismissal
   ├── Severe Disloyalty (< 30) -> Triggers ProtestQuest (Civilian demonstrations)
   ├── Armed Insurgency -> RebelDefendQuest (Secession revolt)
   └── Dismissal -> FireJarl() clears alias and revokes GovFaction
```

---

## 2. Jarl Appointment & Dismissal Mechanics (`CS_RecruitJarl`)

### 2.1 The Appointment Workflow (`RecruitJarl`)
- **Eligibility**: Any recruited NPC or follower with high disposition toward the player character.
- **State Changes**:
  ```papyrus
  JarlAliases[holdnum].ForceRefTo(Citizen)
  JarlsRecruited += 1
  Citizen.AddToFaction(GovFaction)
  EconomyUpdater.JarlStartManageEconomy(Citizen)
  (JarlAppointed.GetAt(holdnum) as GlobalVariable).SetValue(1)
  CS_StoryJarlChooseGov.SendStoryEvent(City, Citizen, aiValue1 = holdnum)
  ```
- **Story Manager Integration**: `CS_StoryJarlChooseGov` dispatches an engine event, instructing the appointed NPC to travel to the hold palace, don regal attire, and assume the throne AI package.

### 2.2 Dismissal & Deposition (`FireJarl`)
- Rulers can be dismissed at any time through dialogue:
  ```papyrus
  EconomyUpdater.JarlStopManageEconomy(Citizen)
  JarlsRecruited -= 1
  Citizen.RemoveFromFaction(GovFaction)
  JarlAliases[i].Clear()
  ```
- Returning the NPC to civilian status halts hold-level economic management until a new ruler is appointed.

---

## 3. Dual-Layer Loyalty Mechanics (`CS_LoyaltyCheckScript`)

The mod tracks political stability through two distinct keyword data channels attached to each holding `Location`:

### 3.1 Citizen Public Order (`Keyword Loyalty`)
Reflects general civilian approval across the hold:
- **Baseline Starting Value**: 70 in aligned holds (e.g., Imperial holds when Imperial, Stormcloak holds when Stormcloak); 30 in occupied hostile holds.
- **Negative Modifiers**:
  - `NoFood`: Severe penalty if local food consumption exceeds production.
  - `NoGarrison`: Penalty if zero garrison soldiers are stationed in the capital.
  - `NoCourt`: Penalty if the player monarch neglects holding royal court in the capital palace.
  - *High Taxation*: Compounding daily unrest penalty.
- **Positive Modifiers**:
  - `Parties` / Royal Festivals: Steward dialogue option to fund public feasts.
  - `ArmiesDefending`: Victory against enemy border incursions.
  - Successful judicial hearings resolving civilian petitions.

### 3.2 Ruler Loyalty (`Keyword JarlLoyalty`)
Tracks the appointed Jarl's personal allegiance to the High Monarch:
- Drives roll-call voting behavior during **The Moot** (`CS_MeetingScript`).
- If `JarlLoyalty >= 60`, the Jarl will formally nominate the player for High King/Queen.

---

## 4. Insurrection, Protests & Secession Revolts

When public `Loyalty` deteriorates, stability collapses in two progressive stages:

1. **Stage 1: Public Demonstrations (`ProtestQuest`)**:
   - Triggered when `Loyalty < 40`.
   - Disgruntled townspeople assemble outside the Jarl's Palace carrying torches and protest signs, chanting anti-tax or anti-famine slogans.
   - *Resolution*: Player can address the crowd, lower hold taxes, distribute food, or order guards to disperse the mob.
2. **Stage 2: Armed Secession Revolt (`RebelDefendQuestStarted`)**:
   - Triggered when `Loyalty < 20`.
   - An armed civilian and militia uprising spawns inside the city, hostile to the player's garrison.
   - If the player's garrison is wiped out, the hold breaks away, resetting `AllHoldings` to neutral or restoring the original vanilla faction.

---

## 5. Interaction with Native Vanilla Jarls

When capturing a hold capital in `CS_TransferOwnershipScript`:
- Native rulers (Balgruuf, Elisif, Ulfric, Igmund, Siddgeir, Laila, Skald, Idgrod, Korir) are unseated:
  - If `keepjarl == false`, they are added to `CitizenFaction` and stripped of their ruling dialogue packages.
  - They remain resident in the city or are transferred to fort dungeons if arrested as prisoners of war.
  - *Warning*: Slaying a native Jarl before their associated vanilla quests complete can break non-Civil War questlines (e.g., Thane quests).
