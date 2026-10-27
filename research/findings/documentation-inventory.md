# Documentation Inventory: Conquest of Skyrim

- **Milestone**: M02 — Documentation Recovery
- **Target Subject**: *Conquest of Skyrim*
- **Primary Investigated Version**: `v0.3.9` (March 2023)
- **Status**: CONFIRMED

---

## 1. Overview & Strategy

Unlike standard commercial software or large open-source projects with dedicated wikis, *Conquest of Skyrim* documentation is distributed across author-published mod descriptions, changelogs, Patreon announcements, Reddit development threads, community walkthrough guides, and video breakdowns.

This document establishes a structured, navigable inventory of all significant documentation sources, cataloging their origin, version applicability, credibility tier, and key technical insights.

---

## 2. Primary Sources (Author-Published)

Primary sources originate directly from the creator, `galvatron304` (`galvatron530`).

### 2.1 Nexus Mods Portal
- **Source**: [Nexus Mods #65628 — Conquest of Skyrim](https://www.nexusmods.com/skyrimspecialedition/mods/65628)
- **Author**: `galvatron304`
- **Applicable Versions**: v0.1.0 through v0.3.9
- **Content Coverage**:
  - Main mod description: overview of faction creation, NPC recruitment, army management, equipment customization, and siege warfare.
  - Hard requirements (SKSE64, SkyUI).
  - Pinned author comments and known early warnings.
  - Changelogs tab (`v0.1.0` to `v0.3.9`).

### 2.2 Bethesda.net Creations (Xbox Port)
- **Source**: [Bethesda.net Creations Listing](https://creations.bethesda.net/en/skyrim/details/13071/_Conquest_of_Skyrim)
- **Author**: `galvatron530`
- **Applicable Versions**: Console build adaptations (2022–2023)
- **Content Coverage**:
  - Console-specific limitations (removal of SKSE-dependent features).
  - Controller-oriented input adaptations and fallback menu setups.

### 2.3 Reddit Developer Threads (`r/skyrimmods`)
- **Source**: [Reddit r/skyrimmods Initial Release (2022-03-28)](https://www.reddit.com/r/skyrimmods/comments/tqf9tx/mod_release_conquest_of_skyrim_wip/)
- **Author**: `u/galvatron530`
- **Content Coverage**:
  - Original design manifesto: motivation for creating an independent third civil war faction.
  - Initial system architecture: bandit camp claiming, steward recruitment, army assembly.
- **Source**: [Reddit r/skyrimmods Open-Source Announcement (2024)](https://www.reddit.com/r/skyrimmods/comments/1f6k0w9/conquest_of_skyrim_is_now_opensource/)
- **Content Coverage**:
  - Official announcement of author stepping down.
  - Release of source materials, scripts, and assets to the community under an open-source license.
  - Transfer of active maintenance to the official Discord community.

### 2.4 Patreon Developer Archives
- **Source**: Patreon (`galvatron530`)
- **Author**: `galvatron530`
- **Applicable Versions**: Historical development posts (2022–2024)
- **Content Coverage**:
  - Development previews: SKVA Synth 2 voice generation, ElevenLabs AI dialogue remaster, formation mechanics, and siege AI pathfinding.
  - Final farewell post: *"Thank You - Final"*, releasing all underlying code.

---

## 3. Secondary Sources (Community Documentation & Guides)

Secondary sources provide empirical observations, troubleshooting protocols, and community-verified mechanics.

### 3.1 Community Strategy Guides & Overviews
- **Source**: [Reddit: Overview and Suggestions for Conquest of Skyrim](https://www.reddit.com/r/skyrimmods/comments/1azl35b/overview_and_suggestions_for_conquest_of_skyrim_a/)
  - *Content*: Comprehensive analysis of the core civil and military loops.
  - *Key Insights*: Details the roles of the Steward (economy, diplomacy, resource gathering) vs. Commander (recruitment, army formations, invasion planning, POWs). Recommends establishing an economic reserve of 100k+ gold before declaring open war.
- **Source**: [Reddit: A Roleplayer's Mini-Review & Guide](https://www.reddit.com/r/skyrimmods/comments/12dunoz/conquest_of_skyrim_a_roleplayers_minireview/)
  - *Content*: Detailed playthrough analysis of v0.3.9 features, ElevenLabs voice quality, and faction progression pacing.
- **Source**: [The Sky Forge: Conquest of Skyrim Walkthrough Discussion](https://theskyforge.ning.com/groups/tes-character-building/forum/the-besieger-conquest-of-skyrim)
  - *Content*: Step-by-step roleplaying and faction conquest strategies.

### 3.2 Video Walkthroughs and Tactical Tutorials
- **Source**: [Conquest of Skyrim Full Guide Series (YouTube)](https://www.youtube.com/watch?v=XOdUEeQkzUo)
  - *Part 1: Starting Your Faction*: Banner placement, naming, government selection, claiming initial camps.
  - *Part 2: Outposts and Resource Management*: Assigning miners/hunters, stockpile maintenance, food distribution.
  - *Part 3: War Declaration and City Sieges*: Marching armies, meeting at forward staging camps, breaching city gates, Hold capital capture.

### 3.3 Community Bugfix Documentation
- **Source**: [Nexus Mods #94506 / #119992 — Conquest of Skyrim Fixes](https://www.nexusmods.com/skyrimspecialedition/mods/94506)
  - *Content*: Technical documentation detailing unpatched edge-case bugs in v0.3.9, including:
    - Hold count calculation bugs during The Moot.
    - Specific fort siege quest triggers failing to advance (e.g., Fort Amol).
    - Inconsistent bounty assignment in captured holds.

---

## 4. Conflicting Documentation & Discrepancies

Research identified several key discrepancies across primary author notes and community guides:

| Topic | Primary / Author Documentation | Community Documentation | Technical Synthesis |
| :--- | :--- | :--- | :--- |
| **Civil War Interaction** | Author notes suggest playing alongside normal gameplay, but warn against breaking vanilla quests. | Experienced community guides strongly mandate either **completing the vanilla Civil War first** or **never touching it**. | **Conflict Confirmed**. The mod hijacks location ownership, hold keywords, and garrison AI packages. Starting a faction while the vanilla Civil War quest is active frequently results in broken script stages. |
| **Interface / Interaction** | Early documentation mentions MCM menu settings for configuration. | Later gameplay documentation (v0.3.x) emphasizes that almost all operational management is conducted via **in-game dialogue** with the Steward and Commander. | **Version Evolution**. Early v0.1/v0.2 relied more heavily on configuration menus, whereas v0.3.x shifted primary operational controls into immersive dialogue trees. |
| **Save Upgrading** | Author explicitly stated that updating between major versions (0.1 &rarr; 0.2 &rarr; 0.3) required a **clean save or new game**. | Occasional forum users claimed successful in-place upgrading using console resets. | **Author Rule Prevails**. Major architectural rewrites altered persistent Papyrus quest scripts; in-place upgrading results in orphaned script instances and desynchronized state. |
| **Endgame Moot Trigger** | Advertised as triggering after conquering Skyrim's holds. | Community bugfix notes identify an off-by-one error or hold detection bug in v0.3.9 that can prevent The Moot quest from initiating. | **Known Defect**. Documented in community bugfix patches (#94506). |

---

## 5. Missing & Incomplete Documentation

The following technical areas lack formal documentation in public sources and require independent technical investigation or playtest verification:

1. **Exact Mathematical Formulas**:
   - Precise rate formulas for resource extraction (wood, metal, stone, food per worker per in-game day).
   - Taxation formula balancing hold revenue against citizen unrest and rebellion probability.
   - Combat casualty resolution algorithms for off-screen / simulated battles.
2. **Internal Quest & FormID Architecture**:
   - Internal quest FormIDs, stage indices, and script event aliases.
   - Exact Papyrus script hierarchy and global variables used for state synchronization.
3. **Formal Specification Wiki**:
   - No central developer wiki or API reference was ever published; all technical knowledge must be reconstructed from scattered posts, changelogs, and decompiled script sources.

---

## 6. Documentation Inventory Index

| ID | Title / Source | Author / Origin | Type | URL | Applicable Versions |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **DOC-01** | Conquest of Skyrim Mod Portal | `galvatron304` | Primary / Official | [Nexus #65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628) | v0.1.0 – v0.3.9 |
| **DOC-02** | Bethesda.net Xbox Page | `galvatron530` | Primary / Official | [Bethesda.net](https://creations.bethesda.net/en/skyrim/details/13071/_Conquest_of_Skyrim) | Console builds |
| **DOC-03** | Initial Release Announcement | `u/galvatron530` | Primary / Community | [Reddit r/skyrimmods](https://www.reddit.com/r/skyrimmods/comments/tqf9tx/mod_release_conquest_of_skyrim_wip/) | v0.1.0 |
| **DOC-04** | Open-Source Transition Notice | `u/galvatron530` / Patreon | Primary / Community | [Reddit r/skyrimmods](https://www.reddit.com/r/skyrimmods/comments/1f6k0w9/conquest_of_skyrim_is_now_opensource/) | v0.3.9+ |
| **DOC-05** | Overview & Strategic Suggestions | Community (`r/skyrimmods`) | Secondary / Guide | [Reddit Guide](https://www.reddit.com/r/skyrimmods/comments/1azl35b/overview_and_suggestions_for_conquest_of_skyrim_a/) | v0.3.9 |
| **DOC-06** | Roleplayer's Review & Feature Audit | Community (`r/skyrimmods`) | Secondary / Guide | [Reddit Review](https://www.reddit.com/r/skyrimmods/comments/12dunoz/conquest_of_skyrim_a_roleplayers_minireview/) | v0.3.9 |
| **DOC-07** | Video Guide Series: Faction & War | Community (YouTube) | Secondary / Video | [YouTube Guide](https://www.youtube.com/watch?v=XOdUEeQkzUo) | v0.3.9 |
| **DOC-08** | Community Bugfix Documentation | Community (Nexus Mods) | Secondary / Technical | [Nexus #94506](https://www.nexusmods.com/skyrimspecialedition/mods/94506) | v0.3.9 |
| **DOC-09** | Legendary Edition Backport | Community (Nexus Mods) | Secondary / Port | [Nexus #117786](https://www.nexusmods.com/skyrim/mods/117786) | LE Backport |
