# Vanilla Skyrim Compatibility: Conquest of Skyrim

- **Milestone**: M27 — Vanilla Skyrim Compatibility
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Data Sources**: Direct FormID override analysis of `conquerskyrim.esp` against `Skyrim.esm`, `cs_negotiationscript.psc`, and community compatibility logs
- **Status**: CONFIRMED

---

## 1. Verified Master FormID Overrides

Direct binary inspection of `conquerskyrim.esp` confirms that the mod overrides **83 native vanilla records** from `Skyrim.esm`:

| Record Signature | Count | Key Overridden FormIDs & EditorIDs | Technical Impact |
| :--- | :---: | :--- | :--- |
| `QUST` (Quests) | 1 | `0x00026F45` (`CWCampaignObj`) | Intercepts and rewrites vanilla Civil War campaign objectives. |
| `SMQN` (Story Manager) | 2 | `0x000D5176` (`CWChangeLocationScenes`), `0x000350F6` (`WIChangeLocationNodeA`) | Hijacks location-change event triggers for hold capital conquests. |
| `NPC_` (Actors) | 2 | `0x0001336B` (General Tullius), `0x0010F9B9` (Ulfric Stormcloak) | Injects negotiation dialogue and combat package overrides. |
| `DIAL` (Dialogue) | 7 | `0x00074785` through `0x0007478A`, `0x00075C9B` | Injects peace summit topics and formal war responses. |
| `LCTN` (Locations) | 57 | Major hold capitals, forts, military redoubts, and settlement locations | Replaces native keyword data with `CWOwner = 69` and custom holding pointers. |
| `WRLD` (Worldspaces) | 9 | Tamriel exterior (`0x0000003C`) and walled city worldspaces | Adds forward staging camps (`CampMarker1/2`), war maps, and siege barricades. |
| `PACK` (AI Packages) | 3 | Native palace guard and patrol packages | Overridden to allow faction soldiers to replace vanilla city guards. |
| `NAVI` / `MSTT` | 2 | Native navigation mesh navigation info and movable statics | Adjusts gate barricade destructible state. |

---

## 2. Interaction with the Vanilla Civil War Questline

### 2.1 The Core Conflict: `CWCampaignObj` Hijacking
- **Verified Conflict**: The mod directly overrides `CWCampaignObj` (FormID `0x00026F45`), which acts as the master tracker for Skyrim's vanilla Civil War.
- **Consequence**:
  - If a player initializes *Conquest of Skyrim* while active in `CW01` (*Joining the Legion* / *Joining the Stormcloaks*), `CW02` (*The Jagged Crown*), or `CWSiege` (*Battle for Whiterun*), the quest scripts enter a mutual lock.
  - Palace doors to Dragonsreach or the Palace of the Kings become permanently barred, and dialogue scenes fail to advance.
- **Rule of Operation**: **Strict Mutual Exclusion**. The player must either:
  1. Complete the entire vanilla Civil War questline first before founding a sovereign faction; OR
  2. Avoid joining either vanilla faction entirely, leaving the Civil War unstarted.

---

## 3. Interaction with the Skyrim Main Questline

| Main Quest Stage | Affected Vanilla Systems | Conflict Nature & Severity | Mitigation / Playtest Protocol |
| :--- | :--- | :--- | :--- |
| **Dragon Rising (`MQ104`)** | Jarl Balgruuf, Dragonsreach, Western Watchtower | **High / Breaking**: Conquering Whiterun unseats Balgruuf to `CitizenFaction`, preventing the Western Watchtower dragon battle from triggering and halting main quest progression. | Complete *Dragon Rising* and establish the Dragonborn identity before declaring war on Whiterun. |
| **A Blade in the Dark (`MQ106`)** | Delphine, Kynesgrove | **Low**: Minor village combat if Kynesgrove is contested; generally safe. | None required. |
| **Diplomatic Immunity (`MQ201`)** | Thalmor Embassy (Haafingar) | **Low**: Independent worldspace cell; unaffected by hold conquest. | Safe. |
| **Season Unending (`MQ302`)** | High Hrothgar Peace Council | **CRITICAL / GAME-BREAKING**: `MQ302` dynamically swaps hold ownership between Tullius and Ulfric. If the player has already conquered holds, `MQ302` quest aliases fail to fill, permanently locking the council scene. | **Mandatory**: Do not capture any hold capitals prior to resolving `MQ302`, or defeat Alduin before launching territorial conquest. |

---

## 4. Interaction with Major Guilds & Factions

### 4.1 The Thieves Guild (Riften)
- **Status**: `COMPATIBLE WITH LIMITATIONS`.
- **Mechanism**: The mod preserves the Ratway interior cells untouched. However, conquering Riften unseats Jarl Laila Law-Giver. If Maven Black-Briar was appointed vanilla Jarl, she is relegated to `CitizenFaction`. Thieves Guild radiant quests continue to function normally.

### 4.2 The Dark Brotherhood (Falkreath / Dawnstar)
- **Status**: `COMPATIBLE`.
- **Mechanism**: Sanctuary cells are unaffected. Conquering Falkreath or Dawnstar does not interfere with sanctuary door scripts. *Bound Until Death* (Vittoria Vici assassination in Solitude) functions normally provided Solitude is not under active military siege at the time.

### 4.3 The Companions (Whiterun) & College of Winterhold
- **Status**: `COMPATIBLE`.
- **Mechanism**: Jorrvaskr and the College of Winterhold reside in autonomous sub-cells. Conquering Whiterun or Winterhold replaces exterior guards but leaves guild leaders intact.

---

## 5. Third-Party Mod Ecosystem Compatibility Matrix

### 5.1 Hard Incompatibilities (Verified Game-Breaking)
- **Open Civil War (OCW)**: Fatal collision on `CWCampaignObj` and holding keywords.
- **Civil War Overhaul (CWO / Redux)**: Complete duplicate quest script conflict.
- **The Second Great War**: Conflicts over hold capital conquest flags and garrison ownership.

### 5.2 High-Risk Frameworks (Verified AI Collisions)
- **Follower Management Mods (AFT, NFF, EFF)**:
  - *Verified Conflict*: Importing faction officers (Steward, Commander, Blacksmith) into third-party follower frameworks overrides the custom `TaskBarrel` AI package stack, permanently freezing army movement orders (`ISSUE-GAME-03`).

### 5.3 Medium-Risk Layout Mods (Suspected Navmesh Collisions)
- **City Overhauls (*JK's Skyrim*, *The Great Cities*, *Dawn of Skyrim*)**:
  - *Mechanism*: Alterations to city gates and exterior perimeters obstruct custom battering ram approach navmeshes and catapult munition trajectories, causing siege crashes (`ISSUE-CTD-02`).
  - *Recommendation*: Use vanilla city exterior layouts or dedicated navmesh patches.
