# Issues & Compatibility Inventory: Conquest of Skyrim

- **Milestone**: M04 — Issues and Compatibility Inventory
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9` (March 2023)
- **Status**: CONFIRMED

---

## 1. Overview & Classification Strategy

Due to the complex scope of *Conquest of Skyrim*—injecting full 4X strategic systems into the Creation Engine's native quest and AI frameworks—the mod exhibits notable technical failure modes and compatibility constraints.

This document systematically catalogues reported defects, crashes to desktop (CTDs), progression blockers, and mod incompatibilities. In accordance with project discipline, **symptoms are strictly distinguished from root causes**, and unsupported speculation is excluded.

---

## 2. Crash to Desktop (CTD) Triggers

| Issue ID | Symptom / Description | Trigger Conditions | Suspected Mechanism / Engine Context | Affected Versions | Evidence / Source |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `ISSUE-CTD-01` | Siege Cell Load CTD | Instant CTD when approaching or loading into an outdoor hold capital during an active siege. | High Papyrus script thread congestion combined with simultaneous spawning of 40+ combatant actors and siege physics objects. | All versions, including v0.3.9 | Nexus #65628 user reports; Reddit crash logs |
| `ISSUE-CTD-02` | Navmesh Assault Crash | Crash during urban street fighting when armies march through exterior gates into interior city worldspaces. | Actor pathfinding engine freeze caused by modified or obstructed navmeshes (especially prevalent when city overhaul mods are active). | v0.1.0 – v0.3.9 | Reddit r/skyrimmods; Crash Logger analysis |
| `ISSUE-CTD-03` | FaceGen Mesh Crash | Crash to desktop when opening recruitment menus or encountering freshly generated generic faction troops. | Corrupt or missing FaceGen mesh/texture records for dynamically spawned custom soldier variants. | v0.1.x – v0.2.x (partially mitigated in v0.3.x) | Patreon development bug notes; Nexus changelogs |
| `ISSUE-CTD-04` | Memory Allocation Spike | Crash during prolonged multi-hold warfare sessions after multiple cell transitions. | Papyrus VM script heap exhaustion and actor buffer overflow under vanilla memory allocation defaults. | All versions | Mitigated by *SSE Engine Fixes* memory patches |

---

## 3. Broken Quests & Progression Blockers

| Issue ID | Symptom / Description | Reproduction Conditions | Known Workarounds / Patches | Affected Versions | Evidence / Source |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `ISSUE-QST-01` | The Moot Fails to Trigger | Conquering all 9 holds does not trigger the endgame Moot or allow player coronation as High King/Queen. | Conquering Skyrim hold capitals in specific non-linear sequences causes an internal hold-count mismatch in the master quest script. | Fixed by community patch [*Conquest of Skyrim Fixes* (Nexus #119992)](https://www.nexusmods.com/skyrimspecialedition/mods/119992). Manual console stage set (`setstage`). | v0.3.9 | Nexus #119992; Reddit user bug reports |
| `ISSUE-QST-02` | Fort Amol Capture Failure | Invading Fort Amol (Eastmarch) fails to initiate the battle quest, or defeating defenders fails to flip fort ownership. | Approaching Fort Amol exterior cell fails to trigger the proximity script event or complete the capture objective. | Fixed by [*Conquest of Skyrim Fixes*](https://www.nexusmods.com/skyrimspecialedition/mods/119992). Console quest stage advancement. | v0.3.9 | Nexus #119992 changelog; Reddit thread |
| `ISSUE-QST-03` | Vanilla Civil War Aliasing Lock | Vanilla hold Jarls become invulnerable, palace doors remain permanently locked, or dialogue fails to advance. | Starting the *Conquest of Skyrim* faction questline while vanilla Civil War quests (`CW01`, `CW02`, or `Season Unending`) are active in the journal. | **Author Warning**: Do not install/activate faction features while vanilla Civil War is active. Must finish vanilla Civil War or avoid starting it entirely. | All versions | Author description; Nexus pinned FAQ |
| `ISSUE-QST-04` | Palace Takeover Desync | Castle Dour (Solitude) or Palace of the Kings (Windhelm) fails to transition to player faction after final city breach. | Interior palace cell retains native Civil War battle state aliases or fails to disable enemy faction leaders. | Re-entering cell after 24 in-game hours, or resetting cell via console `pcb`. | v0.3.6 – v0.3.9 | Community walkthrough logs |

---

## 4. Gameplay Inconsistencies & AI Anomalies

| Issue ID | Symptom / Description | Reproduction Conditions | Workaround / Mitigation | Affected Versions | Evidence / Source |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `ISSUE-GAME-01` | "Naked Troops" Glitch | Newly recruited soldiers or city guards spawn or stand without armor/clothes equipped. | Faction armory script fails to evaluate and apply default outfit forms upon actor cell attach or recruitment initialization. | Target NPC and execute console command `resetinventory` or `disable` followed by `enable`. Ensure Blacksmith has sufficient metal/armor stockpile. | v0.2.x – v0.3.9 | Reddit r/skyrimmods; Nexus forums |
| `ISSUE-GAME-02` | Lingering Hold Bounties | Conquered holds retain original Imperial/Stormcloak crime bounties; guards attack player upon entry. | Hold crime faction faction ownership script failed to clear vanilla crime faction IDs upon territorial transfer. | Partially addressed in v0.3.9. Workaround: pay off bounty before invasion or use console `player.paycrimegold 0 0 <FactionID>`. | Pre-v0.3.9 (occasional edge case in v0.3.9) | Nexus v0.3.9 changelog; Reddit |
| `ISSUE-GAME-03` | Officer Package Stalling | Recruited Steward or Commander becomes unmovable, stops responding to dialogue, or fails to navigate to base. | Assigning NPCs that possess complex vanilla alias packages or conflicting package stacks. | Cast Courage/Calm or use `recycleactor`. Avoid recruiting NPCs tied to ongoing vanilla quest scenes. | All versions | Reddit troubleshooting guides |
| `ISSUE-GAME-04` | Prisoner Trailing Bug | Captured Prisoners of War continuously trail behind the player character across the worldspace. | Occurred in pre-v0.3.9 builds where POWs used a basic follower package. | **Resolved in v0.3.9**: POWs are now reassigned to the Second-in-Command upon capture. | v0.1.x – v0.3.8 (Fixed in v0.3.9) | v0.3.9 Changelog |

---

## 5. Compatibility Matrix

### 5.1 Severely Incompatible (Game-Breaking Conflicts)
The following mods modify the same underlying Civil War quest records, hold ownership variables, or garrison scripts and must not be used concurrently:
- **Open Civil War (OCW)**: Complete script collision. Both mods attempt to control battle cell spawns and hold ownership.
- **Civil War Overhaul (CWO / Redux)**: Fatal quest alias conflicts.
- **The Second Great War**: Substantial overlap with hold conquest and faction territory systems.
- **Mods modifying vanilla `CWScript` or Civil War quest stages directly**.

### 5.2 High Risk (Follower Framework Conflicts)
- **Frameworks**: *Nether's Follower Framework (NFF)*, *Amazing Follower Tweaks (AFT)*, *Extensible Follower Framework (EFF)*.
- **Conflict Nature**: Importing NPCs who are assigned as Faction Officers (Steward, Commander, Blacksmith) into third-party follower frameworks overrides their custom *Conquest of Skyrim* AI packages, breaking army movement, war planning, and hold administration.
- **Mitigation**: Do not manage *Conquest of Skyrim* commanders using external follower frameworks.

### 5.3 Medium Risk (City & Navmesh Overhauls)
- **Overhauls**: *JK's Skyrim*, *The Great Cities*, *Dawn of Skyrim*, *Cities of the North*.
- **Conflict Nature**: Structural additions (walls, new market stalls, gates) disrupt custom army marching navmeshes and siege battering ram pathing, causing AI soldiers to run into barriers or crash during siege battles.
- **Mitigation**: Use compatibility navmesh patches where available, or avoid heavy exterior city overhauls.

---

## 6. Recommended Stability & Diagnostic Architecture

For researchers and playtesters reproducing *Conquest of Skyrim* mechanics, the community has established the following required stability stack:

1. **SSE Engine Fixes**: Mandatory. Fixes the Papyrus FormID bug, script heap limits, and max studio lights.
2. **Crash Logger SSE AE**: Mandatory diagnostic tool. Essential for capturing register values and full Papyrus callstacks during CTDs (avoid simplified loggers like Trainwreck which strip stack details).
3. **Scrambled Bugs & Display Tweaks**: Resolves engine-level physics and actor update bottlenecks.
4. **Conquest of Skyrim Fixes (Nexus #119992)**: Recommended community patch resolving the v0.3.9 Fort Amol trigger and Moot calculation defects.
