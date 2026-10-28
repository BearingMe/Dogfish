#!/usr/bin/env python3
"""
tests/test_formulas.py
Automated Numerical & Formula Assertions for Conquest of Skyrim v0.3.9
Part of Dogfish Specification Recovery Program (Phase 10: M39)

Usage:
    python -I tests/test_formulas.py
"""

import sys
import unittest
from pathlib import Path

# Add project root to sys.path
root_dir = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(root_dir))

from scripts.verify_formulas import (
    calc_tax_revenue,
    calc_max_jobs,
    calc_population_growth_bounds,
    calc_conscription,
    calc_upgrade_cost,
    calc_gross_resource,
    calc_diplomacy_chance,
    calc_bounty_casualties
)

class TestFormulas(unittest.TestCase):
    def test_tiered_taxation(self):
        """Assert tiered taxation equations across all 3 holding categories."""
        # Major Walled City (0..4) with Pop 50, Tax 0, 1, 2, 3
        self.assertEqual(calc_tax_revenue(pop=50, tax_rate=0, holding_num=0), 25000)
        self.assertEqual(calc_tax_revenue(pop=50, tax_rate=1, holding_num=1), 37500)
        self.assertEqual(calc_tax_revenue(pop=50, tax_rate=2, holding_num=2), 50000)
        self.assertEqual(calc_tax_revenue(pop=50, tax_rate=3, holding_num=3), 62500)

        # Minor Open City (5..8) with Pop 40, Tax 1
        # Formula: 300 * 40 + 150 * 40 * 1 = 12000 + 6000 = 18000
        self.assertEqual(calc_tax_revenue(pop=40, tax_rate=1, holding_num=6), 18000)

        # Dependent Village (9+) with Pop 20, Tax 2
        # Formula: 200 * 20 + 100 * 20 * 2 = 4000 + 4000 = 8000
        self.assertEqual(calc_tax_revenue(pop=20, tax_rate=2, holding_num=10), 8000)

    def test_max_jobs_and_growth(self):
        """Assert facility capacity and dynamic population growth bounds."""
        jobs = calc_max_jobs(
            farm_level=2, food_change=5,
            mine_level=1, metal_change=4,
            mill_level=1, wood_change=3,
            docks_level=0, docks_change=0
        )
        self.assertEqual(jobs["food_max"], 10)
        self.assertEqual(jobs["metal_max"], 4)
        self.assertEqual(jobs["wood_max"], 3)
        self.assertEqual(jobs["docks_max"], 0)
        self.assertEqual(jobs["max_jobs"], 17)

        # Test growth with pop 10, max_jobs 17
        # 10 < 17 + 0.25*10 = 19.5 (True)
        # deficit = 17 - floor(2.5) = 17 - 2 = 15
        # min = floor(0.3 * 15) = 4, max = floor(0.5 * 15) = 7
        min_inc, max_inc = calc_population_growth_bounds(pop=10, max_jobs=17)
        self.assertEqual(min_inc, 4)
        self.assertEqual(max_inc, 7)

    def test_conscription_ceiling(self):
        """Assert conscription formula evaluates ceiling of percentage * unemployed."""
        self.assertEqual(calc_conscription(unemployed=15, recruitable_percentage=0.20), 3)
        self.assertEqual(calc_conscription(unemployed=11, recruitable_percentage=0.10), 2)
        self.assertEqual(calc_conscription(unemployed=0, recruitable_percentage=0.50), 0)

    def test_technology_costs(self):
        """Assert linear progression of technology upgrade costs across all tiers."""
        expected = {
            2: (5000, 50),
            3: (15000, 150),
            4: (25000, 250),
            5: (35000, 350),
            6: (45000, 450),
            7: (55000, 550),
            8: (65000, 650),
            9: (75000, 750),
            10: (85000, 850)
        }
        for tier, (gold, metal) in expected.items():
            costs = calc_upgrade_cost(tier)
            self.assertEqual(costs["gold"], gold, f"Tier {tier} gold mismatch")
            self.assertEqual(costs["metal"], metal, f"Tier {tier} metal mismatch")

    def test_gross_production(self):
        """Assert gross resource output with and without docks trade multiplier."""
        # Baseline farm level 3, change 8, no docks, open jobs 2
        # 3 * (3*8 + 0 - 2) = 3 * (24 - 2) = 66
        self.assertEqual(calc_gross_resource(3, 8, 0, 0, 2), 66)

        # With Level 3 docks and Change 10 -> dock_bonus = floor(0.3 * 30) = 9
        # 3 * (3*8 + 9 - 2) = 3 * (24 + 9 - 2) = 3 * 31 = 93
        self.assertEqual(calc_gross_resource(3, 8, 3, 10, 2), 93)

    def test_diplomacy_probabilities(self):
        """Assert treaty acceptance probability formula under varied leverage."""
        # Unfavorable: Negative relationship (-25), Conflict offer (-25), Low troops < 60 (-25)
        # Base: 25 - 25 - 25 - 25 = -50%
        self.assertEqual(
            calc_diplomacy_chance(relationship_rank=-1, host_offer=1, is_sons=False, is_imp=True, troop_count=40),
            -50
        )

        # Favorable: Neutral relationship (0), Neutral offer (+25), High troops >= 100 (+25)
        # Base: 25 + 0 + 25 + 25 = 75%
        self.assertEqual(
            calc_diplomacy_chance(relationship_rank=0, host_offer=2, is_sons=True, is_imp=False, troop_count=100),
            75
        )

    def test_bounty_casualty_simulation(self):
        """Assert off-screen casualty calculation in victory and defeat scenarios."""
        # Victory scenario: Chance 0.8, Roll 0.4 -> diff = 0.4 -> dead = floor((1 - 0.4)*80) = floor(0.6*80) = 48
        is_vic, dead = calc_bounty_casualties(troops=80, troop_certain=100, victory_roll=0.4)
        self.assertTrue(is_vic)
        self.assertEqual(dead, 48)

        # Defeat scenario: Chance 0.4, Roll 0.6 -> Defeat -> Total wipeout of dispatched troops
        is_vic, dead = calc_bounty_casualties(troops=40, troop_certain=100, victory_roll=0.6)
        self.assertFalse(is_vic)
        self.assertEqual(dead, 40)

if __name__ == "__main__":
    unittest.main()
