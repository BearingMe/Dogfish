# Military Organization & Recruitment Specification: Conquest of Skyrim

- **Milestone**: M13 — Army and Recruitment Systems
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_armytroopdeath.psc` (Troop types, death events, reinforcement accounting)
  - `cs_troopmanagemenuscript.psc` (Division management, transit distance calculation)
  - `cs_choosetroopracesscript.psc` (Racial and gender recruitment demographics)
  - `cs_commanderoutfitscript.psc` (Armory container-proxy outfit standardization)
  - `cs_troopequipmentupdaterscript.psc` (Equipment lists, magic tiers, weapon assignments)
- **Status**: CONFIRMED

---

## 1. Military Structure & Division Organization

*Conquest of Skyrim* supports a standing army divided into up to **10 distinct operational field divisions**:

```text
Faction Military Command
  │
  ├── High Commander / Second-in-Command (ASIC)
  │     └── Oversees global military readiness, POWs, and invasion planning
  │
  └── 10 Field Army Divisions (Commanders[0] through Commanders[9])
        ├── Division Commander (Assigned Unique or Generic NPC)
        ├── Standardized Division Equipment Form (Custom or Tiered Armor)
        ├── Unit Composition (Infantry, Ranged, Mages, Automatons, Beasts)
        ├── Dynamic Food & Supply Upkeep (TroopCountFood)
        └── Real-Time Transit State (Distance calculations between hold markers)
```

- **Division Scaling**: Introduced in v0.2.x and refined in v0.3.x; state is processed through five pairs of multi-threaded script updaters (`Updater1` through `Updater10`).
- **Capacity Caps**:
  - `GlobalVariable Property MaxArmies Auto`: Fixed engine limit of **10 active field armies**.
  - `GlobalVariable Property TotalArmies Auto`: Tracks current active armies.

---

## 2. Commander Appointments & Officer Hierarchy

### 2.1 Eligibility & Wage Mechanics
- **Recruitment Vector**: The player approaches NPCs across Skyrim and offers them command roles via contextual dialogue.
- **Disposition Scaling**:
  - **Allies / Friends (`RelationshipRank >= 1`)**: Serve voluntarily without wages (`0 gold/day`).
  - **Neutral NPCs**: Require an ongoing mercenary wage of **100 gold/day**, deducted from the faction treasury during daily updates.
- **Follower Framework Invariant**:
  - Assigning an NPC as a division commander binds custom AI packages (`FollowScoutCommander`, `CS_ArmyDrummerFaction`).
  - *Engine Constraint*: Managing faction commanders through third-party follower frameworks (AFT, NFF, EFF) corrupts their division package stack and stalls army movement.

### 2.2 Vampire Command Option
- The mod contains dedicated vampire military subroutines (`bool Property VampireCommander Auto`, `bool Property VampireArmy Auto`, `cs_govvampirescript.psc`).
- If the player character or commander is a vampire, the division can be configured into an undead host, unlocking nocturnal combat bonuses and altering recruitment requirements.

---

## 3. Troop Categories & Unit Roster

Inspection of `cs_armytroopdeath.psc` reveals a diverse, multi-class unit classification engine:

### 3.1 Standard Mortal Formations
1. **Infantry / Foot Soldiers (`CountSoldier`)**: Core frontline melee troops with sword/shield or two-handed weapons.
2. **Heavy Shock Warriors (`CountWarrior`)**: High-health shock infantry deployed for frontline breach assaults.
3. **Archers (`CountArcher`)**: Conventional light ranged units equipped with hunting/composite bows.
4. **Crossbowmen (`CountCrossbow`)**: Heavy armor-piercing ranged units (utilizing Dawnguard DLC crossbow mechanics).

### 3.2 Arcane Divisions (Battlemages)
Magic units scale according to dedicated class globals (`DestructionMageLevel`, `RestorationMageLevel`, `ConjurationMageLevel`, `AlterationMageLevel`):
- **Pyromancers (`CountMageDestructionFire`)**: Offense-oriented fire spellcasters.
- **Cryomancers (`CountMageDestructionFrost`)**: Area-denial stamina drain casters.
- **Electromancers (`CountMageDestructionShock`)**: Magicka drain burst casters.
- **Combat Healers (`CountMageRestoration`)**: Support casters casting heal-other spells on wounded soldiers.
- **Conjurers & Warders (`CountMageConjuration`, `CountMageAlteration`)**: Defensive shield casters and summoners.

### 3.3 Dwemer Mechanical Units (Engineering Research)
Conquering Dwemer ruins or unlocking technology allows commissioning automaton units:
- **Dwarven Spiders (`CountDwarvenSpider`)**: Agile harassment units.
- **Dwarven Spheres (`CountDwarvenSphere`)**: Armored mobile skirmishers.
- **Dwarven Ballistas (`CountDwarvenBallista`)**: Heavy mobile artillery.
- **Dwarven Centurions (`CountDwarvenCenturion`)**: Massive mechanical siege tanks.

### 3.4 War Beasts & Auxiliary Creatures
- Frontier units captured or trained: `CountCreatureWolf`, `CountCreatureBear`, `CountCreatureSabreCat`, `CountCreatureTroll`, `CountCreatureGiant`, and `CountCreatureMammoth`.

---

## 4. Recruitment, Demographics & Demobilization

### 4.1 Demographic Filtering (`CS_ChooseTroopRacesScript.psc`)
The player can explicitly configure the ethnic and demographic composition of their military:
- **Playable Race Toggles**:
  - `AllowedRacesGlobals`: Independent Boolean globals for Nord, Imperial, Redguard, Breton, Dunmer, Altmer, Bosmer, Orc, Khajiit, and Argonian.
- **Gender Balance**:
  - `AllowedGendersGlobals`: Toggles male-only, female-only, or mixed-gender recruitment.
  - Dynamically updates `TroopsGenderedList` upon recruitment.

### 4.2 Recruitment Costs & Resource Upkeep
- **Initial Bounty**: Enlisting soldiers requires upfront gold from the treasury.
- **Daily Food Upkeep (`TroopCountFood`)**:
  - Every standing soldier consumes a set ratio of food daily.
  - If food yield across `CS_AllHoldings` enters a deficit, military effectiveness degrades, triggering desertions and local garrison revolts.
- **Automated Recruitment Option**:
  - `bool Property AutoRecruitment Auto`: When enabled via the Commander, divisions automatically replenish combat losses using available treasury gold and armory reserves.

---

## 5. Equipment Standardization & The Blacksmith (`CS_CommanderOutfitScript`)

To prevent the common Creation Engine "naked soldier" bug, equipment management uses a container-proxy pattern:

```text
Player chooses "Standardize Equipment" in Commander / Blacksmith menu
       │
       ▼
Opens Container Proxy Inventory (OutfitActors.OpenInventory(true))
       │
       ▼
Player deposits desired armor set (Helmets, Cuirasses, Gauntlets, Boots, Weapons)
       │
       ▼
Script generates FormList CustomOutfit and sets CommanderOutfitFaction rank
       │
       ▼
Troop spawn scripts evaluate HasAllItems() on cell attach and apply gear
```

1. **Vanilla Tier Progression**:
   - Player can select standard vanilla armor tiers: Iron &rarr; Steel &rarr; Dwarven &rarr; Elven &rarr; Glass &rarr; Ebony &rarr; Dragonbone.
   - Requires the Faction Blacksmith to possess matching raw ingots in the faction stockpile.
2. **Custom Equipment Support**:
   - The container proxy allows assigning mod-added armor and weapons to armies without modifying base Skyrim forms.
3. **Enchantment & Perk Scaling**:
   - Upgrading military perks (`CS_WarriorUpgradePerk*`, `CS_ArcherUpgradePerk*`) applies passive damage and armor rating buffs across all deployed division actors.
