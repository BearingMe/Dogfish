//! Dogfish: Independent Behavioral Reconstruction of Conquest of Skyrim
//!
//! Grounded in the behavioral contracts recovered in `docs/behavioral-contracts.md`
//! and tested under the validation strategy in `docs/reconstruction-testing.md`.

pub mod core;
pub mod economy;
pub mod faction;
pub mod military;
pub mod population;
pub mod state;
pub mod territory;
pub mod types;

pub use core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
pub use economy::{calculate_gross_production, EconomyManager, FacilityType};
pub use faction::{BannerColor, FactionProfile, GovernmentDoctrine};
pub use military::{simulate_bounty_combat, ArmyOrder, CombatResult, Division, MilitaryManager, SiegeOperation, SiegePhase, TroopClass};
pub use population::{allocate_holding_labor, calculate_population_growth, conscript_troops, LaborAllocationResult};
pub use state::{FacilityLevels, GameState, HoldingState, ResourceStockpile};
pub use territory::{AppointedGovernor, TerritoryManager};
pub use types::{DiplomaticStance, DivisionId, FactionId, HoldingId, HoldingTier, ResourceType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_time_progression() {
        let mut time = SimulationTime::new();
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 0);

        for _ in 0..23 {
            assert!(!time.advance_hour());
        }
        assert_eq!(time.day, 1);
        assert_eq!(time.hour, 23);

        // 24th hour triggers new day
        assert!(time.advance_hour());
        assert_eq!(time.day, 2);
        assert_eq!(time.hour, 0);
    }

    #[test]
    fn test_deterministic_rng_reproducibility() {
        let mut rng1 = DeterministicRng::new(123456789);
        let mut rng2 = DeterministicRng::new(123456789);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
            assert_eq!(rng1.gen_range_i32(10, 50), rng2.gen_range_i32(10, 50));
            assert_eq!(rng1.gen_percent(), rng2.gen_percent());
        }
    }

    #[test]
    fn test_holding_identifiers_and_tiers() {
        assert_eq!(HoldingId::COUNT, 15);
        for i in 0..15 {
            let hid = HoldingId(i as u8);
            assert!(hid.is_valid());
            if i <= 8 {
                assert!(hid.is_capital());
                assert!(!hid.is_village());
            } else {
                assert!(!hid.is_capital());
                assert!(hid.is_village());
            }
        }

        assert_eq!(HoldingTier::for_holding(HoldingId::WHITERUN), HoldingTier::CapitalMajor);
        assert_eq!(HoldingTier::for_holding(HoldingId::SOLITUDE), HoldingTier::CapitalMajor);
        assert_eq!(HoldingTier::for_holding(HoldingId::FALKREATH), HoldingTier::CapitalMinor);
        assert_eq!(HoldingTier::for_holding(HoldingId::WINTERHOLD), HoldingTier::CapitalMinor);
        assert_eq!(HoldingTier::for_holding(HoldingId::RIVERWOOD), HoldingTier::Village);
    }

    #[test]
    fn test_hand_calculated_taxation_vectors() {
        let mut whiterun = HoldingState::new(HoldingId::WHITERUN, FactionId::Player);
        whiterun.population = 50;
        whiterun.tax_rate = 2;
        assert_eq!(whiterun.calculate_tax(), 50000);

        let mut falkreath = HoldingState::new(HoldingId::FALKREATH, FactionId::Player);
        falkreath.population = 30;
        falkreath.tax_rate = 1;
        assert_eq!(falkreath.calculate_tax(), 13500);

        let mut riverwood = HoldingState::new(HoldingId::RIVERWOOD, FactionId::Player);
        riverwood.population = 15;
        riverwood.tax_rate = 1;
        assert_eq!(riverwood.calculate_tax(), 4500);
    }

    #[test]
    fn test_game_state_invariants_and_deterministic_loop() {
        let mut state = GameState::new(42);
        assert!(state.check_invariants().is_ok());
        assert_eq!(state.holdings.len(), 15);
        assert_eq!(state.treasury, 10000);

        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true).is_ok());
        state.holdings[1].tax_rate = 1;

        for _ in 0..24 {
            state.tick_hour();
        }

        assert_eq!(state.time.day, 2);
        assert!(state.treasury > 10000);
        assert!(state.check_invariants().is_ok());
    }

    #[test]
    fn test_deterministic_simulation_replay() {
        let mut sim1 = GameState::new(9999);
        let mut sim2 = GameState::new(9999);

        for _ in 0..168 {
            sim1.tick_hour();
            sim2.tick_hour();
        }

        assert_eq!(sim1.time, sim2.time);
        assert_eq!(sim1.treasury, sim2.treasury);
        assert_eq!(sim1.stockpile, sim2.stockpile);
        assert_eq!(sim1.holdings, sim2.holdings);
        assert_eq!(sim1.events.len(), sim2.events.len());
        assert_eq!(sim1.events, sim2.events);
    }

    // --- MILESTONE M50 TESTS ---

    #[test]
    fn test_faction_declaration_and_doctrine() {
        let mut state = GameState::new(101);
        assert!(!state.faction.is_declared);

        state.faction.declare_faction(
            "The Free Holds",
            "Sons of the North",
            BannerColor::Crimson,
            GovernmentDoctrine::Republic,
            state.time.day,
        );

        assert!(state.faction.is_declared);
        assert_eq!(state.faction.name, "The Free Holds");
        assert_eq!(state.faction.army_name, "Sons of the North");
        assert_eq!(state.faction.banner_color, BannerColor::Crimson);
        assert_eq!(state.faction.doctrine.tax_modifier_pct(), 20);
        assert_eq!(state.faction.doctrine.conscription_modifier_pct(), -15);
    }

    #[test]
    fn test_spatial_adjacency_matrix() {
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::FALKREATH));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::WINDHELM));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::DAWNSTAR));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::RIFTEN));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::MORTHAL));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::MARKARTH));

        assert!(territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::MORTHAL));
        assert!(!territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::WHITERUN));
        assert!(!territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::RIFTEN));

        assert!(territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::WINDHELM));
        assert!(territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::DAWNSTAR));
        assert!(!territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::FALKREATH));
    }

    #[test]
    fn test_atomic_holding_transfer_and_split_brain_protection() {
        let mut state = GameState::new(202);
        assert_eq!(state.holdings[0].owner, FactionId::Imperial);

        assert!(state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).is_ok());

        assert_eq!(state.holdings[0].owner, FactionId::Player);
        assert_eq!(state.territory.crime_gold[0], 0);
        assert_eq!(state.holdings[0].loyalty, 60);
        assert!(!state.territory.has_appointed_governor(HoldingId::WHITERUN));

        assert!(state.appoint_governor(HoldingId::WHITERUN, "Vignar Gray-Mane").is_ok());
        assert!(state.territory.has_appointed_governor(HoldingId::WHITERUN));

        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, false).is_ok());
        assert_eq!(state.holdings[1].owner, FactionId::Player);
    }

    #[test]
    fn test_invalid_non_adjacent_invasion_blocked() {
        let mut state = GameState::new(303);
        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true).is_ok());

        let result = state.transfer_holding(HoldingId::WINTERHOLD, FactionId::Player, false);
        assert!(matches!(result, Err(SimulationError::HoldingNotAdjacent { .. })));
        assert_eq!(state.holdings[8].owner, FactionId::Stormcloak);

        let camp_result = state.transfer_holding(HoldingId::WINTERHOLD, FactionId::Player, true);
        assert!(camp_result.is_ok());
        assert_eq!(state.holdings[8].owner, FactionId::Player);
    }

    #[test]
    fn test_wilderness_camp_respawn_cooldown() {
        let mut state = GameState::new(404);
        state.territory.trigger_camp_cooldown(HoldingId::RIVERWOOD, 2);
        assert_eq!(state.territory.camp_cooldowns[9], 2);

        for _ in 0..24 {
            state.tick_hour();
        }
        assert_eq!(state.territory.camp_cooldowns[9], 1);

        for _ in 0..24 {
            state.tick_hour();
        }
        assert_eq!(state.territory.camp_cooldowns[9], 0);
    }

    // --- MILESTONE M51 TESTS ---

    #[test]
    fn test_gross_resource_production_math() {
        assert_eq!(calculate_gross_production(4, 10, 0, 0, 0), 120);
        assert_eq!(calculate_gross_production(4, 5, 4, 10, 0), 96);
        assert_eq!(calculate_gross_production(4, 5, 4, 10, 10), 66);
    }

    #[test]
    fn test_labor_allocation_waterfall_priority() {
        let mut holding = HoldingState::new(HoldingId::WHITERUN, FactionId::Player);
        holding.population = 45;

        let labor = allocate_holding_labor(&holding);
        assert_eq!(labor.food_labor, 40);
        assert_eq!(labor.food_jobs_open, 0);
        assert_eq!(labor.docks_labor, 0);
        assert_eq!(labor.wood_labor, 5);
        assert_eq!(labor.wood_jobs_open, 5);
        assert_eq!(labor.metal_labor, 0);
        assert_eq!(labor.metal_jobs_open, 10);
        assert_eq!(labor.unemployed, 0);
    }

    #[test]
    fn test_conscription_math() {
        let mut holding = HoldingState::new(HoldingId::WHITERUN, FactionId::Player);
        holding.population = 100;
        holding.unemployed = 20;

        let drafted = conscript_troops(&mut holding, 0.15);
        assert_eq!(drafted, 3);
        assert_eq!(holding.unemployed, 17);
        assert_eq!(holding.population, 97);

        holding.unemployed = 11;
        let drafted2 = conscript_troops(&mut holding, 0.10);
        assert_eq!(drafted2, 2);
        assert_eq!(holding.unemployed, 9);
        assert_eq!(holding.population, 95);
    }

    #[test]
    fn test_facility_upgrade_transaction() {
        let mut state = GameState::new(505);
        let max_err = state.upgrade_facility(HoldingId::WHITERUN, FacilityType::Farm);
        assert!(max_err.is_err());

        let initial_gold = state.treasury;
        let initial_wood = state.stockpile.wood;
        let initial_jobs = state.holdings[1].max_jobs;

        let res = state.upgrade_facility(HoldingId::FALKREATH, FacilityType::Mine);
        assert!(res.is_ok());
        assert_eq!(state.holdings[1].facilities.mine, 3);
        assert_eq!(state.holdings[1].max_jobs, initial_jobs + state.holdings[1].metal_change);
        assert!(state.treasury < initial_gold);
        assert!(state.stockpile.wood < initial_wood);
    }

    #[test]
    fn test_famine_loyalty_cascade() {
        let mut state = GameState::new(606);
        assert!(state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).is_ok());

        state.holdings[0].facilities.farm = 0;
        state.stockpile.food = -100;
        let initial_loyalty = state.holdings[0].loyalty;

        for _ in 0..72 {
            state.tick_hour();
        }

        assert_eq!(state.holdings[0].loyalty, initial_loyalty - 30);
        assert_eq!(state.holdings[0].loyalty, 30);
    }

    // --- MILESTONE M52 TESTS ---

    #[test]
    fn test_division_ceiling_and_orders() {
        let mut state = GameState::new(707);
        assert_eq!(state.military.divisions.len(), 10);

        // Assign orders to division 0
        let res = state.assign_army_order(DivisionId(0), ArmyOrder::PatrolHold(HoldingId::WHITERUN));
        assert!(res.is_ok());
        assert_eq!(state.military.divisions[0].order, ArmyOrder::PatrolHold(HoldingId::WHITERUN));

        // Invalid division ID 10 is rejected
        let invalid_res = state.assign_army_order(DivisionId(10), ArmyOrder::FollowPlayer);
        assert!(matches!(invalid_res, Err(SimulationError::InvalidDivisionId(10))));
    }

    #[test]
    fn test_division_training_costs_and_upgrade() {
        // Recovered formula: CostGold = 5000 + 10000(L-1), CostMetal = 50 + 100(L-1)
        assert_eq!(Division::training_upgrade_cost(1), (5000, 50));
        assert_eq!(Division::training_upgrade_cost(2), (15000, 150));
        assert_eq!(Division::training_upgrade_cost(3), (25000, 250));
        assert_eq!(Division::training_upgrade_cost(4), (35000, 350));

        let mut state = GameState::new(808);
        state.treasury = 50000;
        state.stockpile.metal = 500;

        assert_eq!(state.military.divisions[0].power_level, 1);
        assert!(state.upgrade_division_power(DivisionId(0)).is_ok());
        assert_eq!(state.military.divisions[0].power_level, 2);
        assert_eq!(state.treasury, 50000 - 5000);
        assert_eq!(state.stockpile.metal, 500 - 50);
    }

    #[test]
    fn test_bounty_combat_simulation_math() {
        let mut rng = DeterministicRng::new(42);

        // 50 troops vs 50 difficulty -> 100% chance -> always victory with casualties
        let res = simulate_bounty_combat(50, 50, 1000, &mut rng);
        assert!(res.is_victory);
        assert!(res.surviving_troops > 0);
        assert!(res.casualties < 50);
        assert_eq!(res.gold_reward, 1000);

        // 0 troops -> defeat
        let res_zero = simulate_bounty_combat(0, 50, 1000, &mut rng);
        assert!(!res_zero.is_victory);
    }

    #[test]
    fn test_siege_operation_phases_and_reinforcement_waves() {
        let mut state = GameState::new(909);
        assert!(state.start_siege(HoldingId::WINDHELM, true, true).is_ok());

        let siege = state.military.active_siege.as_mut().unwrap();
        assert_eq!(siege.phase, SiegePhase::StagingCamp);

        // Record casualties and test reinforcement wave triggers (at 20, 40, 60, 80, 100)
        for _ in 0..19 {
            assert!(!siege.record_defender_casualty());
        }
        assert!(siege.record_defender_casualty()); // 20th casualty triggers Wave 1
        assert_eq!(siege.reinforcement_wave, 1);

        for _ in 0..19 {
            assert!(!siege.record_defender_casualty());
        }
        assert!(siege.record_defender_casualty()); // 40th casualty triggers Wave 2
        assert_eq!(siege.reinforcement_wave, 2);

        // Advance phases
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::OuterGateBreach);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::UrbanStreetCombat);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::PalaceConfrontation);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::Capitulated);

        // Capitulation transfers holding
        assert_eq!(state.holdings[3].owner, FactionId::Player);
    }
}
