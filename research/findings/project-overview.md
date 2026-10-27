# Project Overview: Conquest of Skyrim

- **Milestone**: M01 — Project Identity & History
- **Target Subject**: *Conquest of Skyrim*
- **Primary Investigated Version**: `v0.3.9` (Released March 23, 2023)
- **Status**: CONFIRMED

---

## 1. Project Identity

- **Mod Title**: *Conquest of Skyrim* (also referred to as *Conquest of Skyrim - WIP Version*)
- **Primary Author**: `galvatron304` (Nexus Mods username) / `galvatron530` (Reddit, Bethesda.net, Patreon)
- **Distribution Platform**: Nexus Mods (Skyrim Special Edition)
- **Nexus Mods Identifier**: `65628`
- **Official Nexus URL**: [https://www.nexusmods.com/skyrimspecialedition/mods/65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628)
- **Console / Bethesda.net Page**: [Bethesda.net Creations - Conquest of Skyrim (Xbox)](https://creations.bethesda.net/en/skyrim/details/13071/_Conquest_of_Skyrim)
- **Core Genre / Category**: Grand Strategy Overhaul / Faction & Civil War Management / Gameplay & Quest Mechanics

---

## 2. Historical Development Objectives

The original *Conquest of Skyrim* was created to solve a long-standing limitation in Skyrim: the player's forced binary choice between the Imperial Legion and the Stormcloaks in the Civil War.

### Core Design Goals:
1. **Total Sovereignty**: Allow the player character to build an entirely independent faction from scratch.
2. **Grand Strategy Integration**: Integrate strategic 4X gameplay elements (territorial conquest, troop recruitment, garrisoning, hold management, and diplomacy) directly into the Creation Engine runtime without switching to external menus or separate engines.
3. **Economic Simulation**: Implement regional production, taxation, food/resource management, and infrastructure construction.
4. **Political Progression**: Establish hold governance (appointing Jarls, Stewards, Commanders), convening The Moot, and achieving coronation as High King or Queen of Skyrim.
5. **Dynamic Warfare**: Support real-time field battles, tactical troop formations, and city/fort siege assaults led by player-commanded armies.

---

## 3. Release Chronology & Version History

Development progressed through three major architectural eras between March 2022 and March 2023:

```text
2022-03-28: v0.1.0 Initial Alpha Release
    │
    ├── 2022-04-07: Xbox Console Release (Bethesda.net)
    │
2022-07 / 2022-08: v0.2.x Architecture Era
    │   (Clean save required; SKVA Synth voice acting, army expansion)
    │
2022-11 / 2023-01: v0.3.0 - v0.3.8 Rapid Stabilization
    │   (Save compatibility break from v0.1/v0.2; siege & pathing fixes)
    │
2023-03-23: v0.3.9 Final Official Release
    │   (ElevenLabs AI voice remaster, parade system, tactical formations, POW rework)
    │
2024-03 / 2024-09: Author Retirement & Open-Source Transition
    │   (Source code released on Patreon; community Discord continuations)
    └── 2024-03: Skyrim LE Backport by community
```

### 3.1 Version 0.1 Series (Initial Alpha – March/April 2022)
- **Release Date**: March 28, 2022 (announced on `r/skyrimmods`).
- **Milestone Scope**:
  - Foundational faction declaration framework.
  - Basic NPC recruitment (Steward, Blacksmith, Second-in-Command).
  - Rudimentary territory conquest (clearing bandit camps and claiming forts with custom faction banners).
  - Basic siege triggers for hold capitals.

### 3.2 Version 0.2 Series (Mid 2022)
- **Key Enhancements**:
  - Introduction of voiced dialogue files synthesized with SKVA Synth 2 (leveraging *Fuz Ro D-oh*).
  - Expansion of military system allowing up to 10 distinct player-commanded armies.
  - Overhaul of territory transfer logic to reduce Civil War quest conflicts.
- **Compatibility Break**: Updating from v0.1.x to v0.2.x required a clean save or faction reset due to significant Papyrus script restructuring.

### 3.3 Version 0.3 Series (Late 2022 – Early 2023)
- **v0.3.0 – v0.3.5**:
  - Major architectural rewrite of economic and governance loops.
  - Upgrading from v0.1/v0.2 to v0.3 required a completely new game save.
- **v0.3.6 – v0.3.8 (December 2022 – January 2023)**:
  - Iterative bugfixes resolving crashes during major city sieges, army marching navigation/navmesh failures, and hold flipping logic.
- **v0.3.9 (March 23, 2023 – Baseline Version for Research)**:
  - **Voice Overhaul**: All custom dialogue regenerated via ElevenLabs AI for natural, high-fidelity speech.
  - **Parade & Formation System**: Added formal victory parades with drummers and two-column marching; introduced basic tactical troop formations (5x4 blocks).
  - **POW System Rework**: Prisoners of War automatically transfer to the Second-in-Command rather than following the player entity.
  - **Camp Cooldowns**: Added respawn cooldowns to defeated faction camps to prevent immediate re-clearing loops.

---

## 4. Supported Environments & Runtime Requirements

### 4.1 Skyrim Editions
- **Skyrim Special Edition (SE)**: Primary development target (runtime `1.5.97`). Fully compatible.
- **Skyrim Anniversary Edition (AE)**: Compatible with appropriate SKSE64 build (runtime `1.6.640` / `1.6.1170`). Requires matching SKSE versions.
- **Skyrim VR**: Not officially supported; reports of partial functionality with community workarounds.
- **Skyrim Legendary Edition (LE - 32-bit)**: Not originally supported by galvatron. A separate community backport was published in March 2024 ([Nexus Mods LE #117786](https://www.nexusmods.com/skyrim/mods/117786)).
- **Xbox (Bethesda.net)**: Ported and maintained by `galvatron530` with stripped SKSE-dependent features.

### 4.2 Hard Engine Dependencies
- **Skyrim Script Extender (SKSE64)**: Mandatory for Papyrus utility functions, input handling, and runtime memory hooks.
- **SkyUI**: Required for the Mod Configuration Menu (MCM) interface and HUD elements.

---

## 5. Development Status & Succession

- **Current Official Status**: **Inactive / On Hiatus**.
- **Final Official Version**: `v0.3.9` (March 23, 2023).
- **Open-Source Transition**: In 2024, the author published a final Patreon announcement (*"Thank You - Final"*) stepping down from active Skyrim modding and releasing all underlying Papyrus source code, assets, and project files under an open community license.
- **Community Continuations & Ecosystem**:
  - **Official Discord**: The community Discord server (linked from the Nexus page) serves as the primary coordination hub for bug tracking, unofficial patches, and collaborative updates.
  - **Community Bugfix Patches**: Unofficial patches such as *Conquest of Skyrim Fixes* (Nexus Mods #94506, #119992) resolve remaining issues in v0.3.9 (e.g., Moot hold calculation edge cases and specific fort siege triggers).

---

## 6. Official Communication & Distribution Channels

| Channel | Platform | Identifier / Link |
| :--- | :--- | :--- |
| **Primary Nexus Page** | Nexus Mods (PC SE) | [Conquest of Skyrim #65628](https://www.nexusmods.com/skyrimspecialedition/mods/65628) |
| **Console Distribution** | Bethesda.net (Xbox) | [Conquest of Skyrim by galvatron530](https://creations.bethesda.net/en/skyrim/details/13071/_Conquest_of_Skyrim) |
| **LE Backport** | Nexus Mods (PC LE) | [Conquest of Skyrim LE #117786](https://www.nexusmods.com/skyrim/mods/117786) |
| **Community Hub** | Discord | Official Conquest of Skyrim Community Discord |
| **Creator Patreon** | Patreon | `galvatron530` (archived development posts & source release) |
| **Public Announcements** | Reddit | `r/skyrimmods` posts by `u/galvatron530` |

---

## 7. Evidence Citations & Source Provenance

- **E01-01**: [Nexus Mods #65628 Mod Page & Description](https://www.nexusmods.com/skyrimspecialedition/mods/65628) (Target version 0.3.9, author galvatron304).
- **E01-02**: [Reddit r/skyrimmods Initial Release Announcement (2022-03-28)](https://www.reddit.com/r/skyrimmods/comments/tqf9tx/mod_release_conquest_of_skyrim_wip/).
- **E01-03**: [Reddit r/skyrimmods Open-Source Transition Announcement (2024)](https://www.reddit.com/r/skyrimmods/comments/1f6k0w9/conquest_of_skyrim_is_now_opensource/).
- **E01-04**: [Bethesda.net Creations Xbox Listing](https://creations.bethesda.net/en/skyrim/details/13071/_Conquest_of_Skyrim).
- **E01-05**: Community Release Archive & Changelogs (Nexus Mods v0.1.0 – v0.3.9).
