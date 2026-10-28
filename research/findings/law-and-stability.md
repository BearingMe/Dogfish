# Law, Crime & Stability Specification: Conquest of Skyrim

- **Milestone**: M24 — Law, Crime, and Stability
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_courtscript.psc` (Court hearings, 7 crime factions, chopping block executions)
  - `cs_prisonscript.psc` (Municipal dungeons per hold, arrest scenes)
  - `cs_bountyscript.psc` (Law enforcement dispatch, simulated casualty formulas)
  - `cs_loyaltycheckscript.psc` (`Harshness`, `Loyalty`, `ProtestQuest`, `RebelDefendQuestStarted`)
- **Status**: CONFIRMED

---

## 1. Legal & Sovereign Jurisdiction Architecture

When a hold is conquered by the player's faction, native Imperial or Stormcloak law enforcement is abolished and replaced by an autonomous legal framework:

```text
Law Enforcement Lifecycle
  │
  ├── 1. Offense Detection & Guard Arrest (CS_PrisonScript)
  │     ├── Faction guards witness or detect crime across 7 legal categories
  │     └── ArrestScene forces suspect into custody (ArrestGuard / PrisonerGuard)
  │
  ├── 2. Confinement to Municipal Dungeons (MoveToPrisonMarkers)
  │     └── Incarcerated in hold jail aliases: PrisonersWhiterun, PrisonersSolitude, etc.
  │
  ├── 3. Judicial Hearing & Court Sentencing (CS_CourtScript)
  │     ├── Player convenes court from palace throne (ThroneSpecific)
  │     └── Case presented by Steward -> Evaluates evidence and crime severity
  │
  ├── 4. Judicial Punishments
  │     ├── Restitution / Fine -> Gold deposited directly into CS_FactionTreasury
  │     ├── Extended Incarceration -> PrisonScript fort dungeon sentence
  │     └── Capital Punishment -> ExecutionQuest scripted execution on ChoppingBlock
  │
  └── 5. Stability & Insurgency Repercussions (CS_LoyaltyCheckScript)
        ├── Excessively harsh sentences increase Unrest and reduce Hold Loyalty
        └── Low Loyalty (< 20) triggers armed secession revolt (RebelDefendQuestStarted)
```

---

## 2. Seven Sovereign Crime Categories (`CS_CourtScript`)

The mod implements seven distinct legal offenses categorized under dedicated crime factions:

| Crime Faction | Offense Type | Standard Severity | Available Penalties |
| :--- | :--- | :---: | :--- |
| `CrimeTheft` | Larceny, burglary, pickpocketing | Minor | Restitution fine, short dungeon sentence. |
| `CrimeAssault` | Physical violence against citizens | Moderate | Fine, public flogging / jail time. |
| `CrimeMurder` | Unlawful killing of citizens/guards | Severe | Extended dungeon sentence, execution on chopping block. |
| `CrimeSpeakingOut` | Sedition, speaking against the monarch | Political | Banishment, prison detention, or execution for treason. |
| `CrimeWorship` | Illicit religious practices | Ideological | Depends on government doctrine (banning Talos or Daedra). |
| `CrimeTreason` | Espionage, aiding Imperial/Stormcloak forces | Capital | Imprisonment in fort dungeons or public execution. |
| `CrimeWeaponPossession` | Unauthorized open carry within city walls | Municipal | Confiscation of arms, monetary fine. |

---

## 3. Municipal Jail Architecture (`CS_PrisonScript`)

Each of the 9 hold capitals maintains independent prisoner reference arrays:
- `PrisonersMarkarth[]`, `PrisonersRiften[]`, `PrisonersSolitude[]`, `PrisonersWhiterun[]`, `PrisonersWindhelm[]`, `PrisonersDawnstar[]`, `PrisonersFalkreath[]`, `PrisonersMorthal[]`, `PrisonersWinterhold[]`.
- **Special High-Value Arrays**:
  - `JarlPrisonersImp[]` and `JarlPrisonersSons[]`: Houses deposed native Jarls arrested during city captures.
- **Dungeon Cell Placement (`MoveToPrisonMarkers`)**:
  - Iterates through the holding's prisoner array.
  - Teleports inmates to internal cell markers:
    ```papyrus
    PrisonerAliases[i].GetActorRef().MoveToPackageLocation()
    ```

---

## 4. Off-Screen Law Enforcement & Bounty Hunting (`CS_BountyScript`)

When bandit outlaws or deserters threaten a hold, the player can dispatch a military division commander or hire bounty hunters.

### 4.1 Off-Screen Combat & Casualty Simulation Formula
Direct source inspection of `CS_BountyScript.psc` reveals the exact mathematical formula used by the engine to simulate off-screen detachment engagements:

$$\text{ChanceOfVictory} = \frac{\text{TroopCounts}}{\text{TroopCountCertain}}$$

$$\text{VictoryRoll} = \text{RandomFloat}(0, \, 1)$$

- **If Attacker Wins ($\text{ChanceOfVictory} > \text{VictoryRoll}$)**:
  $$\text{DeadTroops} = \lfloor (1 - (\text{ChanceOfVictory} - \text{VictoryRoll})) \times \text{TroopCounts} \rfloor$$
  - The detachment succeeds, eliminating the outlaws with proportional casualties.
- **If Attacker Defeated ($\text{ChanceOfVictory} \le \text{VictoryRoll}$)**:
  $$\text{DeadTroops} = \text{TroopCounts}$$
  - The entire dispatched detachment is wiped out; the commander returns wounded (`ReturnCommander()`) and the quest objective fails.

---

## 5. Public Order, Stability & Insurgency Mechanics

Hold stability is mediated through the relationship between `Loyalty`, `Harshness`, and `Unrest`:
1. **Harshness Multiplier (`Keyword Harshness`)**:
   - Passing draconian sentences (frequent executions, high fines) increases the monarch's fear rating but inflames citizen unrest.
2. **Rebellion Cascade**:
   - **Loyalty < 40**: Triggers `ProtestQuest`. Mobs gather outside palaces waving torches and demanding tax relief.
   - **Loyalty < 20**: Triggers `RebelDefendQuestStarted`. Armed insurgents attempt to seize the city gates and re-establish vanilla allegiance.

---

## 6. Interaction with Vanilla Skyrim Crime

- In vanilla Skyrim, crime is partitioned across distinct hold factions (`CrimeFactionWhiterun`, `CrimeFactionHaafingar`, etc.).
- When a hold is captured in `CS_TransferOwnershipScript`, the mod zeroes out vanilla crime gold:
  ```papyrus
  (CrimeFactions.GetAt(citynum) as Faction).SetCrimeGold(0)
  (CrimeFactions.GetAt(citynum) as Faction).SetCrimeGoldViolent(0)
  ```
- *Known Defect*: If a player installs the mod with an active vanilla bounty exceeding 1,000 gold, engine script timing can occasionally fail to zero the violent crime flag, causing newly spawned faction guards to attack the player character.
