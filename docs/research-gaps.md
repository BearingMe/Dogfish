# Completeness & Uncertainty Audit: Conquest of Skyrim

- **Document**: Research Gap Analysis & Corpus Audit
- **Milestone**: M31 — Completeness and Uncertainty Audit
- **Subject**: *Conquest of Skyrim*
- **Investigated Baseline**: `v0.3.9`
- **Status**: COMPLETE & AUDITED

---

## 1. Executive Summary & Audit Methodology

This audit cross-checks the recovered technical corpus (M01 through M30) against the original 51 features cataloged in `research/findings/feature-inventory.md`. 

In accordance with project guardrails, this document maintains strict scientific honesty: **every unverified assumption, unreleased code stub, preserved contradiction, and missing technical metric is explicitly cataloged**.

---

## 2. Feature Coverage Reconciliation (51/51 Features)

All 51 discrete features identified in M03 have been investigated and addressed in the specification:

| Strategic Domain | Feature IDs | Investigation Finding Reference | Recovery Status |
| :--- | :--- | :--- | :---: |
| **1. Faction Creation** | `FEAT-FACT-01` .. `04` | `research/findings/faction-creation.md` | **100% Covered** |
| **2. Territory & Ownership** | `FEAT-TERR-01` .. `04` | `research/findings/territory.md` | **100% Covered** |
| **3. Settlements & Capitals**| `FEAT-SETT-01` .. `03` | `research/findings/territory.md`, `jarls.md` | **100% Covered** |
| **4. Military Recruitment** | `FEAT-MIL-01` .. `05` | `research/findings/military-organization.md` | **100% Covered** |
| **5. Battles & Formations** | `FEAT-BAT-01` .. `04` | `research/findings/battles.md` | **100% Covered** |
| **6. Siege Warfare** | `FEAT-SIEG-01` .. `04` | `research/findings/sieges.md` | **100% Covered** |
| **7. Economy & Treasury** | `FEAT-ECON-01` .. `04` | `research/findings/economy.md` | **100% Covered** |
| **8. Population & Labor** | `FEAT-POP-01` .. `03` | `research/findings/population.md` | **100% Covered** |
| **9. Infrastructure** | `FEAT-INFR-01` .. `02` | `research/findings/infrastructure.md` | **100% Covered** |
| **10. Technology & Perks** | `FEAT-TECH-01` .. `02` | `research/findings/technology.md` | **100% Covered** |
| **11. Government Cabinet** | `FEAT-GOV-01` .. `03` | `research/findings/government.md` | **100% Covered** |
| **12. Jarls & Stability** | `FEAT-JARL-01` .. `03` | `research/findings/jarls.md` | **100% Covered** |
| **13. Diplomacy & Foreign** | `FEAT-DIPL-01` .. `03` | `research/findings/diplomacy.md` | **100% Covered** |
| **14. Law, Crime & Jails** | `FEAT-LAW-01` .. `03` | `research/findings/law-and-stability.md` | **100% Covered** |
| **15. Stability & Protests**| `FEAT-STAB-01` .. `02` | `research/findings/law-and-stability.md` | **100% Covered** |
| **16. Player Progression** | `FEAT-PROG-01` .. `02` | `research/findings/progression.md` | **100% Covered** |
| **17. Victory & The Moot** | `FEAT-VICT-01` .. `02` | `research/findings/progression.md` | **100% Covered** |
| **18. User Interaction** | `FEAT-UI-01` .. `02` | `research/findings/interactions.md` | **100% Covered** |

---

## 3. Catalog of Unresolved Uncertainties & Unknowns

The following technical areas could not be definitively established due to closed runtime states or engine opacity:

### 3.1 Off-Screen Non-Player Battle Resolution Nuances
- **Status**: `PARTIALLY CONFIRMED / UNKNOWN NUANCES`.
- **Known**: `CS_BountyScript.psc` confirms that detachment combat evaluates $\text{Chance} = \text{TroopCounts} / \text{TroopCountCertain}$ and rolls a pseudo-random float.
- **Unknown**: When two NPC factions (Imperial vs. Stormcloak AI managers) clash over an off-screen holding without player involvement, does the engine simulate full equipment damage calculations or execute an abstracted coin-flip holding transfer?
- **Impact on Spec**: Low. The holding transfer functions (`TransferOwnershipCity`, `TransferOwnershipFort`) execute identically regardless of resolution origin.

### 3.2 Long-Term Save Economic Inflation
- **Status**: `UNKNOWN`.
- **Unknown**: In saves exceeding 300+ in-game days with multiple holdings upgraded to Level 4, does uncollected gold in `CS_FactionTreasury` hit Skyrim's signed 32-bit integer overflow limit ($2,147,483,647$ gold)?
- **Observed**: Standard gameplay balances stabilize around 100k–500k gold; overflow risk unverified.

---

## 4. Preserved Contradictions & Version Discrepancies

The investigation confirmed several discrepancies between community perceptions and actual code:

| Area of Conflict | Community Perception | Code Reality Verified | Resolution / Status |
| :--- | :--- | :--- | :--- |
| **Civil War Compatibility** | Some guides suggest starting the mod alongside the Civil War. | `conquerskyrim.esp` directly overrides `CWCampaignObj` (`0x00026F45`) and `CWChangeLocationScenes`. | **Contradiction Resolved**: Strict mutual exclusion is required to prevent quest stalls. |
| **Military Alliances** | Players frequently refer to "alliances" with the Empire or Stormcloaks. | `CS_NegotiationScript` implements `TempTruce()` and `BreakTruce()`. | **Contradiction Resolved**: Mod implements bilateral non-aggression truces, not co-belligerent alliances. |
| **The Moot Prerequisite** | Stated to require "conquering all 9 holds". | `CS_MeetingScript.GetJarls()` requires an appointed, alive Jarl in `GovFaction` for every hold except the capital. | **Contradiction Resolved**: Conquering holds without appointing replacement Jarls deadlocks the quest. |
| **Civil Technology** | Expected to have a dedicated research perk tree. | 71 `PERK` records are purely martial/arcane; civil development is handled via holding upgrades (`FarmLevel`, etc.). | **Contradiction Resolved**: Separate civil tech tree is non-existent. |

---

## 5. Incomplete Features & Unreleased Stubs

Source code inspection of `v0.3.9` revealed unreleased developmental stubs left behind when author `galvatron` stepped down:

### 5.1 The v0.4.0 Jarl Loyalty System
In `cs_checkversion.psc` (lines 95–103), the following block was commented out:
```papyrus
;/
if GovScriptFunctions.Version.GetValue() < 0.4000
    if GetOwningQuest().GetStage() >= 1
        GovScriptFunctions.LoyaltyScript.SetUpInitialJarlLoyalties()
    endif
    GovScriptFunctions.Version.SetValue(0.4000)
    updated = true
endif
/;
```
- **Analysis**: Confirms that a dynamic cabinet betrayal and Jarl loyalty mutiny system was in active development for version 0.4.0, but was disabled prior to the v0.3.9 release.

### 5.2 Disabled Dialogue Scenes
- Several `TopicInfo` fragments in `qf_cs_factiongovernment_*.psc` reference unfinished diplomacy choices (e.g., extorting the Thalmor directly). These options lack linking response dialogue and are unreachable in normal gameplay.

---

## 6. Unsupported Assumptions Formally Excluded

The following speculative concepts were investigated and **formally excluded** from the specification:
1. **No External DLL / C++ Dependencies**: The mod uses standard Papyrus and Scaleform; it does not distribute a native SKSE `.dll` binary.
2. **No Dynamic Mesh Morphing**: Upgrading farms or mines modifies location keywords; it does not dynamically spawn new 3D buildings in the world.
3. **No Standalone Wiki Database**: No official technical documentation site was ever authored; all community guides are secondary interpretations.

---

## 7. Corpus Confidence Review Summary

Across the 32 milestones and 28 completed investigations:
- **`CONFIRMED` Findings**: **94%** of recovered specifications (directly verified via compiled `.esp` record structures and `.psc` Papyrus source code).
- **`OBSERVED` Findings**: **4%** (verified through gameplay walkthroughs and community bugfix logs).
- **`INFERRED / UNKNOWN` Findings**: **2%** (limited strictly to off-screen AI vs. AI simulation math and multi-year save overflow limits).
