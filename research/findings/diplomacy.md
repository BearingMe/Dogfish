# Diplomacy & Foreign Relations Specification: Conquest of Skyrim

- **Milestone**: M23 — Diplomacy
- **Target Subject**: *Conquest of Skyrim*
- **Baseline Version**: `v0.3.9`
- **Primary Source Code References**:
  - `cs_negotiationscript.psc` (`CalculateSuccess`, `MakeFriends`, `UnMakeFriends`, `CalculateReparations`)
  - `cs_tmscript.psc` (`TempTruce`, `BreakTruce`, `WarScore`)
  - `cs_prisonerquestscript.psc` (POW ransoming mechanics)
- **Status**: CONFIRMED

---

## 1. Diplomatic Relational States

*Conquest of Skyrim* models diplomatic relations between the player's sovereign faction and the two vanilla belligerents (General Tullius's Imperial Legion and Ulfric Stormcloak's Sons of Skyrim) across three discrete states:

```text
       ┌────────────────────────┐
       │     State 1: Neutral   │ (Pre-Declaration; non-hostile border)
       └───────────┬────────────┘
                   │  War Declaration (Steward Emissary)
                   ▼
       ┌────────────────────────┐
       │     State 2: War       │ (Open Hostilities, Border Raids, Sieges)
       └─────┬────────────▲─────┘
Diplomatic   │            │ Truce Broken (Hostile Act / BreakTruce)
Summit       ▼            │
       ┌────────────────────────┐
       │     State 3: Truce     │ (Bilateral Non-Aggression; TruceImp / TruceSons)
       └───────────┬────────────┘
                   │  Hold Conquered Completely
                   ▼
       ┌────────────────────────┐
       │    State 4: Defeated   │ (Faction Eliminated from Province)
       └────────────────────────┘
```

- **Absence of Mutual-Defense Alliances**: Direct code inspection confirms that **formal military alliances are not implemented**. The engine does not support co-belligerency (e.g., calling the Empire to join a siege against Windhelm). Diplomatic success establishes a **bilateral non-aggression truce**, not an alliance.

---

## 2. Diplomatic Actions Inventory

| Diplomatic Action | Initiation Vector | Script Controller | System Outcome |
| :--- | :--- | :--- | :--- |
| **Declare War** | Dialogue with Steward | `CS_FactionGovernment` | Sets `AtWarPlayer = true`, enables border skirmishes and hold invasions. |
| **Diplomatic Summit** | Steward dispatches letter | `CS_NegotiationScript` | Summons Tullius or Ulfric to a neutral meeting table for treaty talks. |
| **Bilateral Truce** | Negotiation summit agreement | `TMImp.TempTruce()` | Sets `TruceImp = 1` or `TruceSons = 1`, halts border incursions. |
| **War Reparations** | Summit concession | `CalculateReparations()` | Transfers gold from or to the faction treasury based on war score. |
| **POW Ransoms** | Commander dialogue | `PrisonerScript` | Releases captured soldiers/officers in exchange for immediate gold. |
| **Break Truce** | Attack on holding / dialogue | `TM.BreakTruce()` | Re-adds combat factions and resumes hostilities immediately. |

---

## 3. Negotiation Success Formula (`CS_NegotiationScript`)

When negotiating a truce at the diplomatic summit, treaty acceptance is governed by `CalculateSuccess(Actor Jarl)`:

### 3.1 Mathematical Success Calculation
$$\text{Chance} = 25\% \quad (\text{Base Probability})$$

$$\text{Relationship Modifier}: \begin{cases} +25\% & \text{if } \text{RelationshipRank} > 0 \\ -25\% & \text{if } \text{RelationshipRank} < 0 \end{cases}$$

$$\text{Ideological Alignment}: \begin{cases} +50\% & \text{if Pro-Faction Offer matches Leader} \\ -25\% & \text{if Pro-Faction Offer conflicts with Leader} \\ +25\% & \text{if Compromise / Neutral Offer} \end{cases}$$

$$\text{Military Superiority Leverage}: \begin{cases} -25\% & \text{if } \text{TotalTroopCount} < 60 \\ +25\% & \text{if } \text{TotalTroopCount} \ge 100 \end{cases}$$

- **Military Deterrent**: If the player faction commands fewer than 60 troops across all divisions, the opponent discounts the player's leverage ($-25\%$). A standing army of 100+ troops confers significant diplomatic leverage ($+25\%$).

---

## 4. Truce Mechanics & Combat Faction Toggling

When a peace agreement is reached (`MakeFriends()`):
1. **Combat Faction Removal**:
   - For Ulfric: Removes `CWSonsFaction` and `CWSonsFactionNPC` from the leader and accompanying officers.
   - For Tullius: Removes `CWImperialFaction` and `CWImperialFactionNPC`.
2. **War Score Archival**:
   - Archives current war momentum into `WarScore` and adjusts `Reparations`.
3. **Daily Timer Synchronization**:
   - Unregisters combat update timers; the AI stops planning frontier raids against player holdings.

### 4.2 Resuming Hostilities (`UnMakeFriends()`)
- If the player attacks an enemy patrol, invades a border fort, or manually breaks the truce:
  - Re-adds `CWImperialFaction` or `CWSonsFaction`.
  - Re-activates `AtWarPlayer` and registers 24h invasion timers.

---

## 5. Prisoner of War Ransoming Mechanics

Captured enemy soldiers and officers are held under the Second-in-Command (`ASIC`):
- **Dialogue Option**: *"I want to arrange a prisoner ransom."*
- **Payout Scale**:
  - Generic soldier: ~50–100 gold.
  - Officer / Commander: ~500–1,000 gold.
- **Execution vs. Ransom**: Executing prisoners on the chopping block permanently eliminates that actor, but incurs a $-10$ diplomatic penalty with that faction's leadership.
