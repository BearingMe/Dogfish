# Government & Officials Specification: Conquest of Skyrim

- **Milestone**: M21 — Government and Officials
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_govpositions.psc` (Cabinet and governor reference arrays)
  - `cs_govscriptfunctions.psc` (Administrative hierarchy, furniture bindings)
  - `cs_courtscript.psc` (Court hearings, crime categorization, chopping block executions)
  - `cs_intelreportscript.psc` (Spymaster intelligence network, line of succession, coups)
  - `qf_cs_factiongovernment_050095ca.psc` (Master government quest fragments)
- **Status**: CONFIRMED

---

## 1. Sovereign Government Hierarchy

The sovereign government in *Conquest of Skyrim* operates as a centralized royal court presiding over semi-autonomous hold governors:

```text
High Monarch (Player Character)
  │
  ├── Royal Cabinet (Central Administration)
  │     ├── The Steward (Civil Administration, Treasury, Taxation, Foreign Letters)
  │     ├── The Second-in-Command / ASIC (War Marshal, Armies, POWs, Invasions)
  │     ├── The Spymaster (Covert Networks, Loyalty Infiltration, Coup Operations)
  │     ├── The Faction Blacksmith (Quartermaster, Armory Stocks, Equipment Tiers)
  │     ├── The Court Mage (Arcane Advisor, Magical Research, Ward Scaling)
  │     ├── The Court Bard (Royal Ceremonies, Court Music, Morale Buffs)
  │     └── The High Executioner (Judicial Capital Sentences, Chopping Block)
  │
  └── Provincial Administration (Hold Rulers)
        ├── Appointed Jarls (Jarls[0..8] governing the 9 hold capitals)
        ├── Local Stewards (Stewards[0..8] managing hold-level production)
        ├── Hold Housecarls (Housecarls[0..8] assigned as palace guardians)
        └── Guard Captains (GuardCaptainsImp / GuardCaptainsSons replacements)
```

---

## 2. Cabinet Roles & Operational Responsibilities

### 2.1 The Steward (Chief Domestic Minister)
- **Recruitment**: Any friendly/neutral NPC. Friendly NPCs serve free; neutral NPCs receive 100 gold/day.
- **Authority**:
  - Sets regional tax rates (Low &rarr; Extortionate).
  - Deposits and withdraws gold from `CS_FactionTreasury`.
  - Dispatches diplomatic emissaries and declarations of war.
  - Relocates the royal seat to any conquered hold capital.

### 2.2 Second-in-Command (`ASIC` / General)
- **Authority**:
  - Plans and launches hold invasions (`CS_AttackPlanningNewScript`).
  - Commands tactical formations (5x4 blocks, line formations).
  - Holds custody of captured Prisoners of War (POWs).
  - Sounds victory parades through conquered hold capitals.

### 2.3 The Spymaster & Intelligence Network (`CS_IntelReportScript`)
A sophisticated espionage subsystem operated through the intelligence officer:
- **Spy Network Mechanics**:
  - `SpyNetworkGlobal`, `SpyNetworkFundingGlobal`, `SpyNetworkAccuracyGlobal`.
  - Funded via treasury allocations (`TreasuryPerDay`).
- **Covert Missions**:
  - *Infiltration (`IntelReportSetUpNetwork`)*: Establishes active informant cells in hostile hold capitals.
  - *Loyalty Sabotage (`IntelReportMissionInfluenceMessage`)*: Weakens citizen loyalty and provokes unrest in target enemy holds.
  - *Line of Succession Tracking (`LoSTitleTexts`)*: Uncovers who will inherit the hold if the current Jarl is assassinated or deposed.
  - *Sponsored Coup d'État (`StoryCoup` & `CoupQuest`)*: Incites internal palace uprisings to flip hold ownership without mounting a military siege.

### 2.4 The Faction Blacksmith (Quartermaster)
- **Authority**:
  - Evaluates raw metal ingots in faction stockpiles.
  - Enforces army-wide equipment standardization via container-proxy inventories (`CS_CommanderOutfitScript`).
  - Upgrades martial technology perks (`CS_TroopUpgradeScript`).

### 2.5 The Court Mage & Court Bard
- **Court Mage**: Unlocks arcane unit training (Fire, Frost, Shock battlemages) and passive ward protections.
- **Court Bard**: Stationed in the palace; performs drum and flute accompaniments during formal court sessions (`CS_CourtBardDrum`, `CS_CourtBardFlute`).

---

## 3. Holding Court & Judicial Administration (`CS_CourtScript`)

From the palace throne (`Thrones[CapitalGlobal]`), the player convenes formal court hearings to rule on civilian petitions and criminal cases:

### 3.1 Judicial Jurisdictions & Crime Categories
`CS_CourtScript` categorizes offenses into seven distinct crime factions:
1. `CrimeTheft`: Property theft and larceny.
2. `CrimeAssault`: Physical violence and battery.
3. `CrimeMurder`: Homicide.
4. `CrimeSpeakingOut`: Sedition, defamation of the monarch, and political dissent.
5. `CrimeWorship`: Religious heresy or illicit worship (e.g., Talos prohibition or Daedric worship bans).
6. `CrimeTreason`: Aiding Imperial or Stormcloak agents.
7. `CrimeWeaponPossession`: Carrying contraband or unauthorized arms inside city gates.

### 3.2 Court Sentencing & Capital Punishment
- **Pardon / Fine**: Accused pays restitution gold into the faction treasury.
- **Imprisonment**: Prisoner transferred to fort dungeons (`PrisonScript`).
- **Capital Punishment (`ExecutionQuest`)**:
  - Accused is escorted to the palace chopping block (`ChoppingBlock`).
  - Plays scripted execution scene with royal guard assembly and decapitator animation.

---

## 4. Official Removal & Replacement Workflows

- **Dismissal**:
  - Officials can be dismissed through dialogue, returning them to their native faction routines.
- **Governor Replacement**:
  - Appointed Jarls can be unseated by visiting their palace throne or reassigning their reference alias in `CS_RecruitJarl.JarlAliases[citynum]`.
- **Incomplete / Unreleased Features**:
  - In `cs_checkversion.psc`, stub code for `LoyaltyScript.SetUpInitialJarlLoyalties()` (v0.4.0) indicates the author intended to implement dynamic cabinet mutinies and jarl betrayals, which remained unreleased in v0.3.9.
