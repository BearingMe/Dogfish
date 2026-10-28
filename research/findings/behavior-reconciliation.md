# Static & Runtime Evidence Reconciliation: Conquest of Skyrim

- **Document**: Evidence Reconciliation & Behavioral Taxonomy
- **Milestone**: M42 — Static and Runtime Evidence Reconciliation
- **Phase**: Phase 11 (Empirical Validation and Final Recovery Audit)
- **Target Subject**: *Conquest of Skyrim* v0.3.9
- **Investigation Date**: 2026-10-28
- **Status**: RECONCILED (Runtime-dependent playtest execution explicitly marked PENDING / BLOCKED)

---

## 1. Five-Tier Behavioral Taxonomy

To eliminate the discrepancy between static code architecture and actual in-game execution, observed features are categorized across five distinct states:
1. **Implemented Behavior**: Present as functional compiled bytecode and plugin records.
2. **Reachable Behavior**: Can be triggered, navigated to, and executed in unmodded gameplay.
3. **Observed Behavior (Community)**: Reported in playthrough logs, bug reports, and videos.
4. **Intended Behavior**: Documented in author design notes or changelogs.
5. **Broken Behavior**: Code exists but fails to complete or triggers unhandled engine locks.

---

## 2. Deep Reconciliation of Major Subsystem Discrepancies

### 2.1 Crime Gold Zeroing vs. Lingering Guard Hostility
- **Code Claim (M10, M24)**: `TransferOwnershipCity()` executes `SetCrimeGold(0)` and `SetCrimeGoldViolent(0)` on `CrimeFactions[citynum]`.
- **Observed Bug (M04, M28)**: Players reported that guards continue attacking the player character after city conquest.
- **Engine Reconciliation**:
  - In Skyrim's engine, zeroing a faction's crime gold updates future crime detection, but does **not** abort currently active combat target lists on spawned actors.
  - If a city guard was already alerted or in combat (`IsInCombat() == 1`), they continue attacking until `StopCombat()` is explicitly called or the cell detaches.
  - **Verdict**: **IMPLEMENTED BUT ENGINE-CONSTRAINED**. The script correctly zeroes the bounty, but lacks an actor-level `StopCombatAlarmOnActor()` call on active cell guards.

### 2.2 The Moot Convocation: Intended vs. Broken Prerequisite
- **Intended Behavior**: After conquering all 9 hold capitals, the player convenes The Moot to achieve coronation.
- **Code Architecture**: `CS_MeetingScript.GetJarls()` iterates `while i < 9`, requiring `JarlAliases[i].GetActorRef() != None` and membership in `GovFaction`.
- **Observed Failure**: Players frequently conquer all holds yet The Moot dialogue never appears.
- **Reconciliation**:
  - If a player leaves native Jarls unseated without appointing a replacement follower via `RecruitJarl()`, `JarlAliases[i]` evaluates to `None`.
  - The script aborts the convocation loop without user feedback.
  - **Verdict**: **REACHABLE WITH STRICT HIDDEN PREREQUISITE**. The feature is implemented, but brittle; it requires 8 appointed living followers plus the player's home capital.

### 2.3 Mid-Siege Save / Reload Desynchronization
- **Intended Behavior**: Players should be able to save and resume during large city assaults.
- **Code Architecture**: `CS_AttackCityScript` manages `TroopsRemainingDefenders` and `TotalDefenderDeaths`, but contains zero `OnPlayerLoadGame` handlers.
- **Observed Failure**: Reloading mid-siege locks gates, causes battering rams to disappear, and halts wave spawns.
- **Engine Reconciliation**:
  - Destruction stages on Havok movable statics (wooden barricades, portcullises) reset to default states on cell reload if not linked to persistent enable-parents.
  - Papyrus wave counters retain their decremented numbers, but physical gate triggers reset to closed, creating a permanent soft-lock.
  - **Verdict**: **BROKEN BEHAVIOR**. Mid-siege saving is fundamentally unsafe under v0.3.9.

### 2.4 Army Limits: Global Variable vs. Thread Updaters
- **Intended Behavior**: Up to 10 armies commanded by player allies.
- **Code Architecture**: `MaxArmies` global variable defaults to `10`. `CS_TroopManageMenuScript` allocates 5 pairs of multi-threaded script updaters (`Updater1` through `Updater10`).
- **Reconciliation**:
  - Fully implemented and reachable. The script thread architecture accurately supports all 10 divisions simultaneously.
  - **Verdict**: **IMPLEMENTED AND FULLY REACHABLE**.

### 2.5 Civil Technology vs. Martial Perks
- **Intended Behavior (Community Expectation)**: Separate civil research trees for farming and mining.
- **Code Architecture**: 71 `PERK` records in `conquerskyrim.esp` are strictly martial/arcane. Civil development is handled via location keywords (`FarmLevel`, `MineLevel`).
- **Reconciliation**:
  - Civil research was never implemented as perks; it exists solely as physical holding upgrades (`cs_upgradecityscript.psc`).
  - **Verdict**: **COMMUNITY MISCONCEPTION RESOLVED**.

### 2.6 The v0.4.0 Jarl Loyalty System
- **Intended Behavior**: Dynamic cabinet betrayals and secession mutinies.
- **Code Architecture**: Commented out in `cs_checkversion.psc` (lines 95–103).
- **Reconciliation**:
  - Unreleased developmental stub. Left inactive in the v0.3.9 public release.
  - **Verdict**: **DISABLED / INCOMPLETE STUB**.

---

## 3. Reconciliation Matrix Across All 18 Domains

| Domain | Implemented in Code? | Reachable in Unmodded Game? | Verified at Runtime? | Broken / Hazard Notes |
| :--- | :---: | :---: | :---: | :--- |
| **Faction Founding** | **Yes** | **Yes** | Pending M41 | Safe on clean save; crashes if Civil War active. |
| **Custom Naming** | **Yes** | **Yes** | Pending M41 | Requires UIlib v1 Scaleform menu files. |
| **Banner Swapping** | **Yes** | **Yes** | Pending M41 | Scale 1.53; background placement lag on large arrays. |
| **Outpost Claiming** | **Yes** | **Yes** | Pending M41 | Functional via `BannerSpell`. |
| **Fort Captures** | **Yes** | **Yes** | Pending M41 | Fort Amol proximity trigger bug patched by #119992. |
| **Capital Conquest** | **Yes** | **Yes** | Pending M41 | Non-transactional; split-brain risk if interrupted. |
| **Garrisons** | **Yes** | **Yes** | Pending M41 | Outfit lag causes temporary naked guards. |
| **Army Recruitment** | **Yes** | **Yes** | Pending M41 | Drafts from civilian `Unemployed` pool. |
| **Division Command** | **Yes** | **Yes** | Pending M41 | Capped at 10 armies; broken by external follower mods. |
| **Battle Throttling** | **Yes** | **Yes** | Pending M41 | `CS_BattleSize` clamps concurrent actors. |
| **5x4 Formations** | **Yes** | **Yes** | Pending M41 | Locked relative AI offsets; drummer audio. |
| **Siege Artillery** | **Yes** | **Yes** | Pending M41 | Catapult batteries and battering rams functional. |
| **Daily Economy** | **Yes** | **Yes** | Pending M41 | 24h game-time updates; strict labor waterfall. |
| **Tiered Taxation** | **Yes** | **Yes** | Pending M41 | 500/250, 300/150, 200/100 formulas verified. |
| **4-Tier Facilities** | **Yes** | **Yes** | Pending M41 | Max Level 4; AddJobs() employment expansion. |
| **71 Perk Techs** | **Yes** | **Yes** | Pending M41 | Linear cost scaling; instant unlock upon payment. |
| **7-Member Cabinet** | **Yes** | **Yes** | Pending M41 | Spymaster coups functional; v0.4.0 loyalty stub disabled. |
| **Throne Hearings** | **Yes** | **Yes** | Pending M41 | 7 crime categories; chopping block executions. |
| **Jarl Appointments** | **Yes** | **Yes** | Pending M41 | Dual loyalty keywords; protests and armed revolts. |
| **Bilateral Truces** | **Yes** | **Yes** | Pending M41 | Non-aggression truces verified; alliances un-implemented. |
| **The Moot** | **Yes** | **Yes (Conditional)** | Pending M41 | Deadlocks if any hold lacks appointed living Jarl. |
| **Coronation** | **Yes** | **Yes** | Pending M41 | Kneeling animations, fealty oaths, crown delivery. |

---

## 4. Status of Empirical Playtesting Program

- In accordance with `docs/execution-discipline.md`, **no simulated gameplay observations are manufactured**.
- The 12 controlled experiments specified in `research/playtesting/test-plan.md` (`EXP-FACT-01` through `EXP-VICT-01`) represent the definitive empirical verification agenda.
- **Formal Status**: The runtime-dependent empirical execution portion of this reconciliation remains **`PENDING HUMAN EXECUTION (BLOCKED)`**, while all static and code-reconciled findings are **`PASS`**.
