# Failure Modes & Reliability Analysis: Conquest of Skyrim

- **Milestone**: M28 — Failure Modes and Reliability
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Evidence**: Script source code analysis (`cs_attackcityscript.psc`, `cs_meetingscript.psc`, `cs_checkversion.psc`, `cs_bountyscript.psc`), `conquerskyrim.esp` record structures, Crash Logger logs, and community patch documentation (#119992)
- **Status**: CONFIRMED

---

## 1. Failure Mode Taxonomy & Severity Scale

Failure modes are classified across four operational severity tiers:
- **`CRITICAL`**: Complete game freeze, crash to desktop (CTD), or permanent savegame corruption requiring rollback.
- **`HIGH`**: Game-breaking quest stall preventing progression through The Moot, hold conquests, or main quests.
- **`MODERATE`**: Functional disruption (broken AI packages, naked troops, lingering bounties) recoverable via console commands or cell reloads.
- **`LOW`**: Cosmetic, audio, or minor interface anomalies with no progression impact.

---

## 2. Technical Failure Modes Catalog

### 2.1 Engine-Level Crashes & Performance Freezes
| ID | Failure Mode | Severity | Verified Root Cause | Symptoms | Known Recovery / Workaround |
| :--- | :--- | :---: | :--- | :--- | :--- |
| `FAIL-ENG-01` | Siege Multi-Actor CTD | **`CRITICAL`** | Papyrus VM script heap exhaustion when 40+ combatants, pathing loops, and magic projectiles execute concurrently. | Instant CTD during city or fort assaults. | Lower `CS_BattleSize` to 25–30 in MCM; install *SSE Engine Fixes* with memory patches. |
| `FAIL-ENG-02` | Navmesh Assault Crash | **`CRITICAL`** | Third-party city overhaul mods (*JK's Skyrim*, *The Great Cities*) modify exterior gates and navmeshes, trapping battering ram and catapult squads in broken pathing loops. | Freeze or CTD as armies approach city breach points. | Avoid heavy exterior city overhauls; use vanilla navmesh geometry. |
| `FAIL-ENG-03` | FaceGen Mesh Incompatibility | **`CRITICAL`** | Missing or mismatched FaceGen head mesh records for custom dynamically spawned recruit templates. | CTD upon opening recruitment menus or encountering newly spawned soldier patrols. | Clean load order with FaceGen patches; ensure loose meshes match plugin FormIDs. |

### 2.2 Quest & Progression Lockups
| ID | Failure Mode | Severity | Verified Root Cause | Symptoms | Known Recovery / Workaround |
| :--- | :--- | :---: | :--- | :--- | :--- |
| `FAIL-QST-01` | The Moot Deadlock | **`HIGH`** | In `CS_MeetingScript.GetJarls()`, the script iterates `0..8` requiring non-null `JarlAliases[i]` in `GovFaction`. If any conquered hold lacks an appointed Jarl, or if an appointed Jarl died, the count aborts. | Conquering all 9 holds fails to unlock The Moot dialogue with the Steward. | Appoint Jarls to all 9 holds; install *Conquest of Skyrim Fixes* (#119992); console override `setstage <MeetingQuestID> 10`. |
| `FAIL-QST-02` | Fort Amol Trigger Failure | **`HIGH`** | Exterior cell proximity trigger fails to fire `OnTriggerEnter()` when Eastmarch border globals initialize out of sequence. | Approaching Fort Amol fails to start the battle quest; killing defenders fails to flip ownership. | Fixed by *Conquest of Skyrim Fixes* (#119992); advance stage manually via console. |
| `FAIL-QST-03` | Civil War Aliasing Lock | **`HIGH`** | Direct collision between `CWCampaignObj` (`0x00026F45`) and active vanilla Civil War quests (`CW01`/`CW02`/`Season Unending`). | Jarl palace doors permanently barred; Jarl Balgruuf invulnerable; dialogue frozen. | Strict mutual exclusion: finish vanilla Civil War or avoid joining before founding faction. |
| `FAIL-QST-04` | Palace Takeover Desync | **`MODERATE`** | Interior palace cell retains native Civil War battle state aliases or fails to disable enemy faction leaders. | Final city breach succeeds, but Jarl confrontation scene fails to play. | Exit palace, wait 24 in-game hours, execute console `pcb` (purge cell buffer), and re-enter. |

### 2.3 AI Package Stalls & Actor Inconsistencies
| ID | Failure Mode | Severity | Verified Root Cause | Symptoms | Known Recovery / Workaround |
| :--- | :--- | :---: | :--- | :--- | :--- |
| `FAIL-AI-01` | Follower Framework Override | **`HIGH`** | Third-party follower overhauls (AFT, NFF, EFF) override the custom `TaskBarrel` AI package stack on recruited commanders. | Division commanders refuse to march, stand frozen at base, or stop responding to orders. | Never manage *Conquest of Skyrim* commanders with external follower frameworks. |
| `FAIL-AI-02` | Naked Troops Glitch | **`MODERATE`** | Creation Engine outfit evaluation lag on `OnCellAttach()` when dynamic equipment lists are updated. | Newly spawned faction soldiers or city guards spawn without armor equipped. | Target actor in console and run `resetinventory` or `disable` followed by `enable`. |
| `FAIL-AI-03` | Lingering Hold Bounties | **`MODERATE`** | Conquered hold crime faction (`CrimeFactionWhiterun`, etc.) fails to execute `SetCrimeGold(0)` upon transfer. | Faction guards in conquered cities attack the player character on sight. | Pay off bounty before conquest or execute `player.paycrimegold 0 0 <FactionID>`. |

### 2.4 Persistence & Savegame Failures
| ID | Failure Mode | Severity | Verified Root Cause | Symptoms | Known Recovery / Workaround |
| :--- | :--- | :---: | :--- | :--- | :--- |
| `FAIL-SAV-01` | Mid-Siege Save Desynchronization | **`HIGH`** | Saving during an active assault causes destructible gate barriers to reset while wave counters remain decremented. | Reloading mid-siege locks gates and halts reinforcement wave spawning. | Avoid saving while an active siege battle is in progress. |
| `FAIL-SAV-02` | Mid-Game Uninstallation Corruption | **`CRITICAL`** | Disabling `conquerskyrim.esp` deletes holding controllers, leaving vanilla Jarls unseated and garrison markers disabled. | Permanent broken worldspace; missing guards; orphaned Papyrus threads causing save bloat and VM crashes. | **Irrecoverable**: Must revert to a clean save made prior to installing the mod. |

---

## 3. Recommended Stability Architecture & Playtest Protocols

To achieve optimal runtime stability when investigating or playtesting *Conquest of Skyrim*:

1. **Mandatory Engine Extensions**:
   - **SKSE64**: Core runtime requirement.
   - **SSE Engine Fixes**: Mandatory. Fixes the Papyrus FormID bug, script heap limits, and animation load bottlenecks.
   - **Crash Logger SSE AE**: Essential for diagnosing exact script names and registers during CTDs.
2. **Community Bugfix Suite**:
   - **Conquest of Skyrim Fixes (Nexus #119992)**: Resolves the v0.3.9 Fort Amol trigger and Moot calculation defects.
3. **Save Hygiene Rules**:
   - Never save during an active outdoor siege battle or while seated at a diplomatic negotiation summit.
   - Always make clean, named saves prior to declaring war on a new hold.
   - Never uninstall the mod mid-playthrough.
