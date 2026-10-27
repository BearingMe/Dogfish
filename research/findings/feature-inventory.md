# Comprehensive Feature Inventory: Conquest of Skyrim

- **Milestone**: M03 — Comprehensive Feature Inventory
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9` (March 2023)
- **Status**: CONFIRMED

---

## 1. Feature Classification System

Each feature is evaluated across six observable states (which may overlap):
- **Documented [DOC]**: Described in official author notes, changelogs, or Nexus documentation.
- **Technically Identified [TECH]**: Concrete scripts, FormIDs, or engine hooks identified.
- **Observed [OBS]**: Confirmed through empirical gameplay observations or walkthrough logs.
- **Disabled [DIS]**: Retained in code/data but intentionally deactivated or unreachable.
- **Incomplete [INC]**: Partially implemented; lacks complete game loops, voice lines, or triggers.
- **Unverified [UNV]**: Documented or reported, but internal implementation mechanics remain unverified.

---

## 2. Feature Catalog by Strategic Domain

### 2.1 Faction Creation & Identity
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-FACT-01` | Faction Declaration | Player declares an independent faction after clearing an initial base. | `[DOC, OBS]` | Cleared location | Triggers initial faction initialization quest. |
| `FEAT-FACT-02` | Custom Naming | Player assigns a custom string name to their sovereign faction. | `[DOC, OBS]` | `FEAT-FACT-01` | Input via UI text prompt / virtual keyboard. |
| `FEAT-FACT-03` | Heraldry & Banners | Choice of custom faction banners/emblems placed in conquered territory. | `[DOC, OBS]` | `FEAT-FACT-01` | Swaps 3D banner meshes/textures at claimed camps. |
| `FEAT-FACT-04` | Government Doctrine | Selection of initial faction doctrine (e.g., Feudal, Military, Republic). | `[DOC, OBS, UNV]` | `FEAT-FACT-01` | Affects tax yields and unrest rates; exact multipliers unverified. |

### 2.2 Territory & Ownership
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-TERR-01` | Outpost & Camp Claiming | Clearing and planting banners at bandit camps, redoubts, and caves. | `[DOC, OBS]` | Combat victory | Converts location into a player-controlled resource site. |
| `FEAT-TERR-02` | Fort Captures | Attacking and seizing major military forts across Skyrim's holds. | `[DOC, OBS]` | Army readiness | Cuts enemy hold reinforcement pools and garrison strength. |
| `FEAT-TERR-03` | Territory Hierarchy | Layered territorial model: Outpost &rarr; Fort &rarr; Town &rarr; Hold Capital. | `[DOC, UNV]` | Location types | Internal keyword/alias hierarchy requires static inspection. |
| `FEAT-TERR-04` | Camp Respawn Timers | Added cooldowns preventing defeated enemy factions from instant re-clearing. | `[DOC, OBS]` | v0.3.9 update | Fixes infinite camp respawn exploit present in v0.3.8. |

### 2.3 Settlements & Hold Capitals
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-SETT-01` | Hold Capital Takeover | Full conquest of major walled and open hold capitals. | `[DOC, OBS]` | `FEAT-SIEG-01` | Transitions hold ownership and jarl palace to player faction. |
| `FEAT-SETT-02` | Hold Governor Appointment | Installing an appointed Jarl or military governor to run a conquered hold. | `[DOC, OBS]` | `FEAT-SETT-01` | Replaces vanilla Jarl; dialogue handles local taxation/orders. |
| `FEAT-SETT-03` | Settlement Garrisons | Stationing defensive detachments of soldiers inside towns and cities. | `[DOC, OBS]` | `FEAT-MIL-01` | Suppresses local unrest and defends against enemy counter-sieges. |

### 2.4 Military & Recruitment
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-MIL-01` | Officer Recruitment | Appointing friendly/neutral NPCs as Commander, Blacksmith, Field Generals. | `[DOC, OBS]` | NPC disposition | Friends serve free; neutral NPCs require ~100 gold/day wage. |
| `FEAT-MIL-02` | Troop Recruitment | Enlisting common soldiers (Infantry, Archers, Cavalry/Specialized). | `[DOC, OBS]` | Gold, Food, Arms | Recruited via Commander dialogue or faction war tables. |
| `FEAT-MIL-03` | Army Divisions | Organizing soldiers into up to 10 distinct operational field armies. | `[DOC, OBS]` | Assigned generals | Introduced in v0.2.x; scaled to handle multi-front warfare. |
| `FEAT-MIL-04` | Equipment Standardization | Assigning standardized armor/weapons to troops via the Faction Blacksmith. | `[DOC, OBS]` | Blacksmith, Metals | Supports vanilla tiers (Iron &rarr; Dragonbone) and custom sets. |
| `FEAT-MIL-05` | Troop Training | Upgrading soldier combat ranks, stats, and effectiveness over time. | `[DOC, OBS, UNV]` | Gold, Training time | Exact stat scaling and training script intervals unverified. |

### 2.5 Battles & Tactical Formations
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-BAT-01` | Real-Time Field Engagements | Clashes between player armies and Imperial/Stormcloak field detachments. | `[DOC, OBS]` | World encounters | High script load during simultaneous NPC spawns. |
| `FEAT-BAT-02` | Tactical Formations | Ordering marching armies into structured block formations (e.g., 5x4 blocks). | `[DOC, OBS]` | v0.3.9 update | Uses custom AI packages to lock relative actor offsets. |
| `FEAT-BAT-03` | Victory Parades | Celebratory military parades through conquered hold capitals with drummers. | `[DOC, OBS]` | v0.3.9 update | Custom animations/sound markers; two-column march routes. |
| `FEAT-BAT-04` | Simulated Off-Screen Battles | Resolution of army battles when the player character is not present in cell. | `[DOC, UNV]` | AI scheduler | Exact casualty and victory determination formulas unverified. |

### 2.6 Siege Warfare
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-SIEG-01` | Siege Initiation | Declaring assault on a hold capital from forward staging camps. | `[DOC, OBS]` | War declaration | Spawns siege encampments outside city gates. |
| `FEAT-SIEG-02` | Siege Weaponry | Commissioning battering rams, catapults, and artillery for breaches. | `[DOC, OBS, UNV]` | Wood, Gold, Blacksmith | Visual/scripted destructible barriers at city entrances. |
| `FEAT-SIEG-03` | Breach & Infiltration | Breaching outer gates, fighting through city streets to the Jarl's Palace. | `[DOC, OBS]` | `FEAT-SIEG-01` | Handles civil war barricades and door lock transitions. |
| `FEAT-SIEG-04` | Surrender & Capitulation | Forcing enemy hold commanders to yield without full assault destruction. | `[DOC, OBS, UNV]` | High military leverage | Dialogue option via emissary or second-in-command. |

### 2.7 Economy & Treasury
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-ECON-01` | Faction Treasury | Centralized gold reserve managed through dialogue with the Steward. | `[DOC, OBS]` | Steward appointed | Deposits, withdrawals, and automatic payroll deductions. |
| `FEAT-ECON-02` | Hold Taxation | Adjusting local tax rates across conquered holds to generate revenue. | `[DOC, OBS]` | Captured holds | High tax boosts treasury but directly increases unrest. |
| `FEAT-ECON-03` | Resource Nodes | Extracting raw materials (Wood, Metal, Stone, Food) from claimed sites. | `[DOC, OBS]` | Controlled outposts | E.g., mining iron at Halted Stream Camp, farming in Whiterun hold. |
| `FEAT-ECON-04` | Upkeep & Supply Consumption | Daily consumption of rations and gold wages for standing armies. | `[DOC, OBS, UNV]` | Army size | Failure to supply results in troop desertion and efficiency loss. |

### 2.8 Population & Labor
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-POP-01` | Population Accounting | Abstract representation of civilian numbers in conquered holds and outposts. | `[DOC, UNV]` | Hold records | Drives labor pools and draft capacity; underlying model unverified. |
| `FEAT-POP-02` | Labor Assignment | Directing workers to farming, mining, logging, or infrastructure projects. | `[DOC, OBS]` | Steward orders | Managed through Steward civil dialogue. |
| `FEAT-POP-03` | Military Conscription | Levying citizens from controlled holds into basic militia/recruits. | `[DOC, OBS, UNV]` | Public order | High conscription causes severe civilian resentment. |

### 2.9 Infrastructure & Development
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-INFR-01` | Outpost Construction | Constructing workbenches, watchtowers, palisades, and defensive works. | `[DOC, OBS]` | Wood, Stone, Gold | Placeable statics or scripted constructible objects. |
| `FEAT-INFR-02` | Economic Facilities | Building grain mills, smithies, armories, and storage depots in holds. | `[DOC, OBS, UNV]` | Steward upgrades | Increases local resource processing efficiency. |

### 2.10 Technology & Upgrades
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-TECH-01` | Military Research | Researching weapon tempering, advanced armor forging, and siege tactics. | `[DOC, UNV]` | Blacksmith | Permanent upgrades to army combat stats. |
| `FEAT-TECH-02` | Civil Upgrades | Unlocking agricultural techniques and improved road logistics. | `[DOC, UNV]` | Steward | Incomplete or unverified in v0.3.9; flagged for code audit. |

### 2.11 Government & Administration
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-GOV-01` | The Steward | Chief civil administrator managing treasury, taxes, and domestic policy. | `[DOC, OBS]` | Appointed NPC | Core companion for all non-military governance. |
| `FEAT-GOV-02` | The Commander | Field marshal managing military campaigns, units, tactics, and POWs. | `[DOC, OBS]` | Appointed NPC | Second-in-command entity executing offensive campaigns. |
| `FEAT-GOV-03` | The Blacksmith | Quartermaster managing weapon tiers, armory stocks, and equipment sets. | `[DOC, OBS]` | Appointed NPC | Essential for preventing recruits from spawning unarmed. |

### 2.12 Jarls & Hold Administration
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-JARL-01` | Jarl Replacement | Deposing native Jarls and appointing loyal followers as new rulers. | `[DOC, OBS]` | Capital conquest | Custom dialogue packages assigned to appointed Jarls. |
| `FEAT-JARL-02` | Jarl Loyalty & Fealty | Loyalty tracking for appointed rulers; risk of betrayal or secession. | `[DOC, UNV]` | Governance policies | Documented feature; depth of betrayal mechanics unverified. |

### 2.13 Diplomacy & Foreign Relations
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-DIPL-01` | Formal War Declaration | Dispatching war ultimatums to General Tullius or Ulfric Stormcloak. | `[DOC, OBS]` | Steward dialogue | Transitions factions into permanent hostility. |
| `FEAT-DIPL-02` | Truces & Peace Summits | Negotiating temporary ceasefires or territorial settlements when strained. | `[DOC, OBS, UNV]` | Steward dialogue | Emissary system arranging neutral summits. |
| `FEAT-DIPL-03` | Prisoner of War Ransoms | Ransoming captured officers and soldiers back to their original faction for gold. | `[DOC, OBS]` | Commander dialogue | Overhauled in v0.3.9; prisoners held by Commander. |

### 2.14 Law, Crime & Dungeons
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-LAW-01` | Player Hold Bounties | Autonomous jurisdiction: player factions do not inherit Imperial/Stormcloak crime. | `[DOC, OBS]` | Captured holds | Bounties cleared/reset in conquered territory (patched in v0.3.9). |
| `FEAT-LAW-02` | Prisoner Executions | Formal public execution of captured enemy leaders and defiant Jarls. | `[DOC, OBS]` | Defeated leaders | Scripted execution scenes or dialogue sentencing. |
| `FEAT-LAW-03` | Prison Fort Dungeons | Reassigning captured POWs to specific dungeon cells in controlled forts. | `[DOC, OBS]` | Held forts | Keeps prisoners contained away from field armies. |

### 2.15 Stability & Public Order
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-STAB-01` | Public Order Metric | Local unrest value influenced by taxes, food supply, war duration, and garrisons. | `[DOC, OBS]` | Economy / War | Low order triggers citizen riots and peasant uprisings. |
| `FEAT-STAB-02` | Rebellions & Insurgency | Revolts in poorly garrisoned holds attempting to restore vanilla factions. | `[DOC, OBS]` | Low public order | Spawns rebel militia hostile to player garrison. |

### 2.16 Player Progression
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-PROG-01` | Title Advancement | Rising in rank from Warlord / Rebel &rarr; King/Queen of Hold &rarr; High King. | `[DOC, OBS]` | Conquered territory | Unlocks new dialogue interactions and authority perks. |
| `FEAT-PROG-02` | Royal Court & Guard | Appointing personal housecarls, royal bodyguards, and royal court members. | `[DOC, OBS]` | Capital conquest | Custom AI packages following the player inside palaces. |

### 2.17 Victory Conditions & The Moot
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-VICT-01` | Conquering All 9 Holds | Military elimination of both Imperial and Stormcloak presence across Skyrim. | `[DOC, OBS]` | All hold sieges | Final military milestone of the conquest campaign. |
| `FEAT-VICT-02` | Convening The Moot | Summoning Skyrim's Jarls to formally elect and crown the High King. | `[DOC, OBS, UNV]` | 9 Holds controlled | Scripted summit; subject to known v0.3.9 hold count check bug. |
| `FEAT-VICT-03` | Coronation Ceremony | Official coronation as High King or High Queen of Skyrim. | `[DOC, OBS]` | Passed Moot vote | Culmination of the mod's main quest progression loop. |

### 2.18 Configuration & User Interaction
| Feature ID | Feature Name | Description | Status | Dependencies | Notes / Unknowns |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `FEAT-UI-01` | Immersive Dialogue System | Deep contextual dialogue trees for ordering officers, generals, and troops. | `[DOC, OBS]` | ElevenLabs voice | Primary control surface in v0.3.9; replaces heavy MCM reliance. |
| `FEAT-UI-02` | Mod Configuration Menu (MCM) | SkyUI settings menu for gameplay balance, hotkeys, and diagnostic toggles. | `[DOC, OBS]` | SkyUI, SKSE64 | Global variable sliders and configuration parameters. |

---

## 3. High-Priority Unresolved Features Flagged for Code Investigation

The following documented or reported features exhibit ambiguities that require static Papyrus analysis (M07) or engine integration probing (M06):

1. **`FEAT-BAT-04` (Off-Screen Battle Calculation)**: How does the game calculate battle outcomes when the player is absent? Does it evaluate equipment stats or a simple random dice roll?
2. **`FEAT-VICT-02` (The Moot Bug)**: What is the exact script condition in `v0.3.9` causing The Moot to fail to trigger for some players?
3. **`FEAT-TECH-02` (Civil Research Scope)**: Is civil technology fully implemented in v0.3.9, or did it remain an incomplete placeholder when development halted?
4. **`FEAT-POP-01` (Population Tracking Data Model)**: Is population stored as a single global integer per hold, or tracked per individual NPC alias?
