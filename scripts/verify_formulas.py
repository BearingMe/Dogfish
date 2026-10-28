#!/usr/bin/env python3
"""
scripts/verify_formulas.py
Reproducible Mathematical Formula Verification Utility for Conquest of Skyrim v0.3.9
Part of Dogfish Specification Recovery Program (Phase 10: M39)

Usage:
    python -I scripts/verify_formulas.py
"""

import math
import sys

def calc_tax_revenue(pop: int, tax_rate: int, holding_num: int) -> int:
    """
    Verified from cs_holdingeconomyupdaterscript.psc (lines 71-77).
    tax_rate is integer 0..3 (0=None, 1=Low, 2=Normal, 3=High).
    holding_num 0..4 = Major Capitals; 5..8 = Minor Cities; 9+ = Villages/Forts.
    """
    if holding_num < 5:
        return 500 * pop + 250 * pop * tax_rate
    elif 5 <= holding_num < 9:
        return 300 * pop + 150 * pop * tax_rate
    else:
        return 200 * pop + 100 * pop * tax_rate

def calc_max_jobs(farm_level: int, food_change: int,
                  mine_level: int, metal_change: int,
                  mill_level: int, wood_change: int,
                  docks_level: int, docks_change: int) -> dict:
    """
    Verified from cs_holdingeconomyupdaterscript.psc (lines 11-15).
    """
    food_max = farm_level * food_change
    metal_max = mine_level * metal_change
    wood_max = mill_level * wood_change
    docks_max = docks_level * docks_change
    return {
        "food_max": food_max,
        "metal_max": metal_max,
        "wood_max": wood_max,
        "docks_max": docks_max,
        "max_jobs": food_max + metal_max + wood_max + docks_max
    }

def calc_population_growth_bounds(pop: int, max_jobs: int):
    """
    Verified from cs_holdingeconomyupdaterscript.psc (lines 16-20).
    Condition: pop < max_jobs + 0.25 * pop
    Deficit: max_jobs - floor(0.25 * pop)
    Returns (min_increase, max_increase) or (0, 0) if condition not met.
    """
    if pop < max_jobs + 0.25 * pop:
        deficit = max_jobs - math.floor(0.25 * pop)
        min_inc = math.floor(0.3 * deficit)
        max_inc = math.floor(0.5 * deficit)
        return min_inc, max_inc
    return 0, 0

def calc_conscription(unemployed: int, recruitable_percentage: float) -> int:
    """
    Verified from cs_holdingeconomyupdaterscript.psc (lines 60-65).
    Formula: ceil(RecruitablePercentage * Unemployed)
    """
    return math.ceil(recruitable_percentage * unemployed)

def calc_upgrade_cost(target_level: int) -> dict:
    """
    Verified from cs_troopupgradescript.psc (lines 54-55).
    CostGold = 5000 + (target_level - 2) * 10000
    CostMetal = 50 + (target_level - 2) * 100
    (In Papyrus, Level is the starting level, so Level.GetValueInt() - 1 is target_tier - 2)
    """
    level_diff = target_level - 2
    return {
        "gold": 5000 + level_diff * 10000,
        "metal": 50 + level_diff * 100
    }

def calc_gross_resource(level: int, change: int,
                        docks_level: int, docks_change: int,
                        jobs_open: int) -> int:
    """
    Verified from cs_holdingeconomyupdaterscript.psc (lines 78-80).
    Formula: 3 * (change * level + floor(0.3 * docks_change * docks_level) - jobs_open)
    """
    dock_bonus = math.floor(0.3 * docks_change * docks_level)
    return 3 * (change * level + dock_bonus - jobs_open)

def calc_diplomacy_chance(relationship_rank: int, host_offer: int,
                          is_sons: bool, is_imp: bool,
                          troop_count: int, host_explanation: int = 1) -> int:
    """
    Verified from cs_negotiationscript.psc (lines 121-177).
    Base chance = 25
    """
    chance = 25
    if relationship_rank > 0:
        chance += 25
    elif relationship_rank < 0:
        chance -= 25

    if host_offer == 1:
        if is_sons:
            chance += 50
        elif is_imp:
            chance -= 25
        if host_explanation == 1:
            if troop_count < 60:
                chance -= 25
            elif troop_count >= 100:
                chance += 25
    elif host_offer == 2:
        chance += 25
        if host_explanation == 1:
            if troop_count < 60:
                chance -= 25
            elif troop_count >= 100:
                chance += 25
    elif host_offer == 3:
        if is_sons:
            chance -= 25
        elif is_imp:
            chance += 50
        if host_explanation == 1:
            if troop_count < 60:
                chance -= 25
            elif troop_count >= 100:
                chance += 25

    return chance

def calc_bounty_casualties(troops: int, troop_certain: int, victory_roll: float) -> tuple:
    """
    Verified from cs_bountyscript.psc (lines 20-30, 53-60).
    Returns (is_victory, dead_troops)
    """
    chance = troops / float(troop_certain)
    is_victory = chance > victory_roll

    if is_victory and chance < 1.0:
        dead_troops = math.floor((1.0 - (chance - victory_roll)) * troops)
    elif not is_victory and chance < 1.0:
        dead_troops = troops
    else:
        dead_troops = 0

    return is_victory, dead_troops

def main():
    print("=================================================================")
    print(" Dogfish Technical Verification: Mathematical Formula Execution ")
    print("=================================================================")

    # 1. Test Taxation for Whiterun (holdingnum=3, Pop=50, TaxRate=2)
    tax_whiterun = calc_tax_revenue(50, 2, 3)
    print(f"\n[TAXATION] Major City (Whiterun, Pop=50, Tax=2 Normal):")
    print(f"           Formula: 500*50 + 250*50*2 = {tax_whiterun:,} Gold/day (Expected: 50,000)")
    assert tax_whiterun == 50000

    # 2. Test Taxation for Dawnstar (holdingnum=5, Pop=30, TaxRate=1)
    tax_dawnstar = calc_tax_revenue(30, 1, 5)
    print(f"\n[TAXATION] Minor City (Dawnstar, Pop=30, Tax=1 Low):")
    print(f"           Formula: 300*30 + 150*30*1 = {tax_dawnstar:,} Gold/day (Expected: 13,500)")
    assert tax_dawnstar == 13500

    # 3. Test Production for Whiterun Farms (FarmLevel=4, FoodChange=10, Docks=0, OpenJobs=0)
    food_prod = calc_gross_resource(4, 10, 0, 0, 0)
    print(f"\n[PRODUCTION] Whiterun Farms (Level 4, Change 10, No Docks):")
    print(f"             Formula: 3 * (4*10 + 0 - 0) = {food_prod} Food (Expected: 120)")
    assert food_prod == 120

    # 4. Test Production with Docks for Solitude (DocksLevel=4, DocksChange=10)
    # Dock bonus = floor(0.3 * 10 * 4) = 12
    metal_solitude = calc_gross_resource(1, 1, 4, 10, 0)
    print(f"\n[PRODUCTION] Solitude Mine (Level 1, Change 1, Docks 4/10):")
    print(f"             Formula: 3 * (1*1 + floor(0.3*40) - 0) = 3 * (1 + 12) = {metal_solitude} Metal (Expected: 39)")
    assert metal_solitude == 39

    # 5. Test Conscription (Unemployed=20, RecruitablePercentage=0.25)
    conscripted = calc_conscription(20, 0.25)
    print(f"\n[CONSCRIPTION] Unemployed=20, Percentage=0.25:")
    print(f"               Formula: ceil(0.25 * 20) = {conscripted} Recruits (Expected: 5)")
    assert conscripted == 5

    # 6. Test Technology Cost for Tier 5 (from Tier 4, target=5)
    tier5_cost = calc_upgrade_cost(5)
    print(f"\n[TECH COST] Upgrade to Tier 5 (level_diff=3):")
    print(f"            Formula: 5000 + 3*10000 = {tier5_cost['gold']:,} Gold (Expected: 35,000)")
    print(f"            Formula: 50 + 3*100 = {tier5_cost['metal']} Metal (Expected: 350)")
    assert tier5_cost["gold"] == 35000 and tier5_cost["metal"] == 350

    # 7. Test Diplomacy Chance (Tullius, HostOffer=3 [Pro-Imperial], Troops=120, Rel=1)
    dip_chance = calc_diplomacy_chance(relationship_rank=1, host_offer=3, is_sons=False, is_imp=True, troop_count=120)
    print(f"\n[DIPLOMACY] Treaty with Tullius (Rel=1, Offer=3 Pro-Imp, Troops=120):")
    print(f"            Formula: 25(base) + 25(rel) + 50(ideology) + 25(troops) = {dip_chance}% (Expected: 125%)")
    assert dip_chance == 125

    # 8. Test Bounty Casualties (Troops=50, Certain=100 [Chance=0.5], Roll=0.3 -> Victory)
    vic, dead = calc_bounty_casualties(50, 100, 0.3)
    print(f"\n[BOUNTY COMBAT] Troops=50, Certain=100, Roll=0.3:")
    print(f"                Victory: {vic}, Dead: {dead} (Formula: floor((1 - (0.5 - 0.3)) * 50) = floor(0.8 * 50) = 40)")
    assert vic is True and dead == 40

    print("\nALL REPRESENTATIVE TEST CASES PASSED SUCCESSFULLY.")
    return 0

if __name__ == "__main__":
    sys.exit(main())
