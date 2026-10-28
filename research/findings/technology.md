# Technology & Research Specification: Conquest of Skyrim

- **Milestone**: M20 — Technology and Research
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_troopupgradescript.psc` (`SetUp`, `UpgradeTroop`, cost formulas)
  - `cs_troopequipmentupdaterscript.psc` (Magic level scaling, LeveledSpells)
  - `conquerskyrim.esp` (71 `PERK` records, `FLST` upgrade formlists)
- **Status**: CONFIRMED

---

## 1. Technology & Research Architecture

Research and technological development in *Conquest of Skyrim* are structured as military, arcane, and engineering advancement trees managed via the Faction Blacksmith and Commander.

Rather than an abstract tech web with timers, technologies function as **progressive institutional upgrades** paid for with Treasury Gold and physical Metal reserves, immediately conferring permanent passive perks (`PERK` records) and stat scaling across all deployed division actors.

---

## 2. Technology Inventory by Discipline

Analysis of the 71 `PERK` records in `conquerskyrim.esp` identifies four major advancement branches:

### 2.1 Martial Discipline Trees (10 Tiers per Class)
Each martial class advances from Level 1 to Level 10, granting cumulative damage bonuses, health scaling, and higher armor rating multipliers:
- **Infantry Regulars**: `CS_SoldierUpgradePerk01` through `CS_SoldierUpgradePerk10`
- **Heavy Shock Warriors**: `CS_WarriorUpgradePerk01` through `CS_WarriorUpgradePerk10`
- **Skirmish Archers**: `CS_ArcherUpgradePerk01` through `CS_ArcherUpgradePerk10`
- **Dawnguard Crossbowmen**: `CS_CrossbowmanUpgradePerk01` through `CS_CrossbowmanUpgradePerk10`
- **Municipal & Fort Guards**: `CS_GuardUpgradePerk01` through `CS_GuardUpgradePerk10`

### 2.2 Arcane Research Trees (Magic Disciplines)
Magic unit capabilities scale through school rank globals (`DestructionMageLevel`, `RestorationMageLevel`, `ConjurationMageLevel`, `AlterationMageLevel`):
- **Destruction Specialization (Levels 1–5)**:
  - Pyromancy: `CS_DestructionMageSpellsFire1` through `CS_DestructionMageSpellsFire5`
  - Cryomancy: `CS_DestructionMageSpellsFrost1` through `CS_DestructionMageSpellsFrost5`
  - Electromancy: `CS_DestructionMageSpellsShock1` through `CS_DestructionMageSpellsShock5`
  - Unlocks higher-tier LeveledSpells (Flames &rarr; Fireball &rarr; Incinerate).
- **Arcane Master Perk**: `CS_DestructionMagePowerPerk` (broad magicka cost reduction and spell magnitude multiplier).

### 2.3 Dwemer Automaton Engineering
Conquering Dwemer strongholds or commissioning engineering research unlocks mechanical combat units within `CS_ArmyTroopDeath`:
- Tier 1: **Dwarven Spiders** (`CountDwarvenSpider`)
- Tier 2: **Dwarven Spheres** (`CountDwarvenSphere`)
- Tier 3: **Dwarven Ballistas** (`CountDwarvenBallista`)
- Tier 4: **Dwarven Centurions** (`CountDwarvenCenturion`)

### 2.4 Dragon Mastery
- **Special Apex Perk**: `CS_SpeakToTamedDragonPerk` (EditorID: `CS_SpeakToTamedDragonPerk`).
- Unlocked through late-game progression, enabling dialogue interaction and tactical flight commands with tamed dragons via `cs__qf_cs_dragonjoinfaction_053d5e5e.psc`.

---

## 3. Exact Research Cost Formulas (`CS_TroopUpgradeScript`)

The cost to upgrade any martial unit tier scales linearly with the target level:

$$\text{CostGold} = 5,000 + (\text{Level} - 1) \times 10,000$$

$$\text{CostMetal} = 50 + (\text{Level} - 1) \times 100$$

### 3.1 Upgrade Cost Progression Table

| Target Tier | Gold Required (`CostGold`) | Metal Ingots Required (`CostMetal`) | Unlocked Armor Categories |
| :---: | :---: | :---: | :--- |
| **Tier 1 &rarr; 2** | `5,000 Gold` | `50 Metal` | Steel / Leather |
| **Tier 2 &rarr; 3** | `15,000 Gold` | `150 Metal` | Dwarven / Elven |
| **Tier 3 &rarr; 4** | `25,000 Gold` | `250 Metal` | Orcish / Scaled |
| **Tier 4 &rarr; 5** | `35,000 Gold` | `350 Metal` | Plate / Glass |
| **Tier 5 &rarr; 6** | `45,000 Gold` | `450 Metal` | Ebony |
| **Tier 6 &rarr; 7** | `55,000 Gold` | `550 Metal` | Dragonplate / Dragonscale |
| **Tier 7 &rarr; 8** | `65,000 Gold` | `650 Metal` | Daedric / Master Forged |
| **Tier 8 &rarr; 9** | `75,000 Gold` | `750 Metal` | Legendary Military Arts |
| **Tier 9 &rarr; 10** | `85,000 Gold` | `850 Metal` | Maximum Combat Mastery |

*Economic Link*: Because upgrading all 5 martial classes to Tier 10 requires hundreds of thousands of gold and thousands of metal ingots, research directly drives territorial conquest of mining outposts (e.g., Markarth, Halted Stream Camp, Shor's Stone).

---

## 4. Upgrade Execution & Stat Application (`UpgradeTroop`)

```text
Player initiates research in Blacksmith dialogue
       │
       ▼
Script verifies: Treasury >= CostGold && Metal >= CostMetal
       │
       ▼
Deducts Resources: Treasury.Mod(-CostGold) & Metal.Mod(-CostMetal)
       │
       ▼
Plays Feedback Audio: CoinSound & MetalSound
       │
       ▼
Level Increment: Levels.GetAt(TroopType).Mod(1)
       │
       ▼
Applies Global Multiplier: ManageTroops.TroopPowerLevels[TroopType] += 1
       │
       ▼
Unlocks Advanced Armor Forms in UnlocksHeavy[] and UnlocksLight[]
```

- **Troop Power Level Scaling**:
  - `ManageTroops.TroopPowerLevels[TroopType]` scales the damage output and base attributes of all current and future soldiers spawned in that class.
- **Armor Form Unlocks**:
  - Unlocks higher armor sets in `UnlocksHeavy[LevelNext]` and `UnlocksLight[LevelNext]`, allowing the Blacksmith to auto-equip higher quality cuirasses and weapons.

---

## 5. Incomplete Features & Architectural Observations

1. **Civil Technology Tree Omission**:
   - Unlike martial and arcane upgrades, civil technologies (e.g., crop rotation, road taxation) do not possess dedicated `PERK` records. Instead, civil advancements were implemented as physical holding upgrades (`FarmLevel`, `DocksLevel` in M19).
2. **Instant Unlock Model**:
   - In v0.3.9, technologies unlock immediately upon paying gold and metal; there is no multi-day waiting timer. The author drafted a research time delay system in early design notes, but implemented it as an instant resource deduction in the final release.
