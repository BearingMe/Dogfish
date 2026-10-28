//! Dogfish: Independent Behavioral Reconstruction of Conquest of Skyrim
//!
//! Grounded in the behavioral contracts recovered in `docs/behavioral-contracts.md`
//! and tested under the validation strategy in `docs/reconstruction-testing.md`.

pub mod core;
pub mod diplomacy;
pub mod economy;
pub mod faction;
pub mod government;
pub mod military;
pub mod persistence;
pub mod population;
pub mod scenario;
pub mod state;
pub mod territory;
pub mod types;

pub use core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
pub use diplomacy::{calculate_truce_acceptance_chance, DiplomacyManager, ForeignFactionRelation};
pub use economy::{calculate_gross_production, EconomyManager, FacilityType};
pub use faction::{BannerColor, FactionProfile, GovernmentDoctrine};
pub use government::{CabinetMinister, CabinetRole, GovernmentManager, JudicialSentence, LegalOffense, MootOutcome};
pub use military::{
    calculate_ai_aggressiveness, calculate_hold_invasion_priority, calculate_surviving_divisions,
    simulate_bounty_combat, ArmyOrder, CombatResult, Division, MilitaryManager, SiegeOperation,
    SiegePhase, TroopClass, DIVISION_SOLDIER_CAPACITY,
};
pub use persistence::{JsonParser, SimpleJson, StatePersistence};
pub use population::{allocate_holding_labor, calculate_population_growth, conscript_troops, LaborAllocationResult};
pub use scenario::{run_mid_siege_interruption_scenario, run_outlaw_to_high_king_campaign, ScenarioReport};
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

        let res = state.assign_army_order(DivisionId(0), ArmyOrder::PatrolHold(HoldingId::WHITERUN));
        assert!(res.is_ok());
        assert_eq!(state.military.divisions[0].order, ArmyOrder::PatrolHold(HoldingId::WHITERUN));

        let invalid_res = state.assign_army_order(DivisionId(10), ArmyOrder::FollowPlayer);
        assert!(matches!(invalid_res, Err(SimulationError::InvalidDivisionId(10))));
    }

    #[test]
    fn test_division_training_costs_and_upgrade() {
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

        let res = simulate_bounty_combat(50, 50, 1000, &mut rng);
        assert!(res.is_victory);
        assert!(res.surviving_troops > 0);
        assert!(res.casualties < 50);
        assert_eq!(res.gold_reward, 1000);

        let res_zero = simulate_bounty_combat(0, 50, 1000, &mut rng);
        assert!(!res_zero.is_victory);
    }

    #[test]
    fn test_siege_operation_phases_and_reinforcement_waves() {
        let mut state = GameState::new(909);
        assert!(state.start_siege(HoldingId::WINDHELM, true, true).is_ok());

        let siege = state.military.active_siege.as_mut().unwrap();
        assert_eq!(siege.phase, SiegePhase::StagingCamp);

        for _ in 0..19 {
            assert!(!siege.record_defender_casualty());
        }
        assert!(siege.record_defender_casualty());
        assert_eq!(siege.reinforcement_wave, 1);

        for _ in 0..19 {
            assert!(!siege.record_defender_casualty());
        }
        assert!(siege.record_defender_casualty());
        assert_eq!(siege.reinforcement_wave, 2);

        assert_eq!(state.advance_siege().unwrap(), SiegePhase::OuterGateBreach);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::UrbanStreetCombat);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::PalaceConfrontation);
        assert_eq!(state.advance_siege().unwrap(), SiegePhase::Capitulated);

        assert_eq!(state.holdings[3].owner, FactionId::Player);
    }

    // --- MILESTONE M53 TESTS ---

    #[test]
    fn test_cabinet_appointment_and_covert_coup() {
        let mut state = GameState::new(1001);
        state.treasury = 50000;

        let no_spymaster = state.attempt_covert_coup(HoldingId::MARKARTH);
        assert!(no_spymaster.is_err());

        state.appoint_cabinet_minister(CabinetRole::Spymaster, "Malborn");
        assert!(state.government.has_minister(CabinetRole::Spymaster));

        let initial_gold = state.treasury;
        let coup_res = state.attempt_covert_coup(HoldingId::MARKARTH);
        assert!(coup_res.is_ok());
        assert_eq!(state.treasury, initial_gold - 10000);
    }

    #[test]
    fn test_judicial_court_hearings() {
        let mut state = GameState::new(1102);
        let initial_treasury = state.treasury;

        let collected = state.judge_court_case(LegalOffense::Theft, JudicialSentence::Fine(500)).unwrap();
        assert_eq!(collected, 500);
        assert_eq!(state.treasury, initial_treasury + 500);

        assert_eq!(state.government.prisoners_in_dungeon, 0);
        assert!(state.judge_court_case(LegalOffense::Assault, JudicialSentence::DungeonImprisonment { days: 30 }).is_ok());
        assert_eq!(state.government.prisoners_in_dungeon, 1);
    }

    #[test]
    fn test_truce_negotiation_probability_math() {
        assert_eq!(calculate_truce_acceptance_chance(50, true, 150, 100), 95);
        assert_eq!(calculate_truce_acceptance_chance(-50, false, 50, 150), 5);
        assert_eq!(calculate_truce_acceptance_chance(0, true, 100, 100), 75);
    }

    #[test]
    fn test_the_moot_deadlock_prevention_and_election() {
        let mut state = GameState::new(1203);

        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true).is_ok());
        let early_moot = state.convene_the_moot(HoldingId::FALKREATH);
        assert!(early_moot.is_err());

        for i in 0..9u8 {
            state.holdings[i as usize].owner = FactionId::Player;
        }

        let unappointed_moot = state.convene_the_moot(HoldingId::WHITERUN);
        assert!(unappointed_moot.is_err());
        assert!(matches!(
            unappointed_moot,
            Err(SimulationError::InvalidStateTransition(msg)) if msg.contains("Moot Deadlock Prevention")
        ));

        for i in 1..9u8 {
            assert!(state.appoint_governor(HoldingId(i), format!("Jarl-{}", i)).is_ok());
            state.territory.governors[i as usize].as_mut().unwrap().loyalty = 80;
        }

        let moot_res = state.convene_the_moot(HoldingId::WHITERUN);
        assert!(moot_res.is_ok());
        let outcome = moot_res.unwrap();
        assert_eq!(outcome.total_jarls_voting, 8);
        assert_eq!(outcome.votes_in_favor, 8);
        assert_eq!(outcome.votes_against, 0);
        assert!(outcome.is_elected);
        assert!(state.government.is_high_king);
        assert_eq!(state.government.coronation_day, Some(state.time.day));
    }

    // --- MILESTONE M54 TESTS ---

    #[test]
    fn test_state_serialization_round_trip() {
        let mut state = GameState::new(1304);
        state.treasury = 45000;
        state.stockpile.food = 350;
        state.faction.declare_faction(
            "Kingdom of the North",
            "Northern Watch",
            BannerColor::RoyalBlue,
            GovernmentDoctrine::Feudalism,
            state.time.day,
        );
        state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).unwrap();
        state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, false).unwrap();

        // Serialize state to JSON
        let json_str = StatePersistence::serialize(&state);
        assert!(json_str.contains("\"version\": 1"));
        assert!(json_str.contains("\"treasury\": 45000"));

        // Deserialize state from JSON
        let restored = StatePersistence::deserialize(&json_str, 1304).unwrap();

        // Assert equivalence
        assert_eq!(restored.time.day, state.time.day);
        assert_eq!(restored.time.hour, state.time.hour);
        assert_eq!(restored.treasury, state.treasury);
        assert_eq!(restored.stockpile.food, state.stockpile.food);
        assert_eq!(restored.faction.name, state.faction.name);
        assert_eq!(restored.faction.is_declared, state.faction.is_declared);
        assert_eq!(restored.holdings[0].owner, FactionId::Player);
        assert_eq!(restored.holdings[1].owner, FactionId::Player);
    }

    #[test]
    fn test_invalid_savegame_rejection() {
        // Corrupted negative treasury
        let corrupted_treasury_json = "{\"version\": 1, \"day\": 5, \"hour\": 12, \"treasury\": -5000}";
        let res1 = StatePersistence::deserialize(corrupted_treasury_json, 42);
        assert!(matches!(res1, Err(SimulationError::InsufficientFunds { .. })));

        // Missing fields
        let missing_fields_json = "{\"version\": 1}";
        let res2 = StatePersistence::deserialize(missing_fields_json, 42);
        assert!(res2.is_err());
    }

    #[test]
    fn test_scenario_outlaw_to_high_king_full_campaign() {
        let report = run_outlaw_to_high_king_campaign(777).unwrap();
        assert_eq!(report.scenario_name, "Outlaw to High King Campaign");
        assert_eq!(report.holds_conquered, 9);
        assert!(report.is_high_king);
        assert!(report.total_days > 20);
        assert!(report.events_count > 0);
    }

    #[test]
    fn test_scenario_mid_siege_interruption_and_recovery() {
        let result = run_mid_siege_interruption_scenario(888);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    // --- MILESTONE M57 REGRESSION TESTS ---

    #[test]
    fn test_ai_division_sizing_and_casualty_conversion() {
        assert_eq!(DIVISION_SOLDIER_CAPACITY, 20);

        // 3 initial armies (60 max soldiers)
        // 45 soldiers survive -> ceil(45/20) = 3 surviving, 0 dead
        assert_eq!(calculate_surviving_divisions(45, 3), (3, 0));

        // 21 soldiers survive -> ceil(21/20) = 2 surviving, 1 dead
        assert_eq!(calculate_surviving_divisions(21, 3), (2, 1));

        // 20 soldiers survive -> ceil(20/20) = 1 surviving, 2 dead
        assert_eq!(calculate_surviving_divisions(20, 3), (1, 2));

        // 1 soldier survives -> ceil(1/20) = 1 surviving, 2 dead
        assert_eq!(calculate_surviving_divisions(1, 3), (1, 2));

        // 0 soldiers survive -> 0 surviving, 3 dead
        assert_eq!(calculate_surviving_divisions(0, 3), (0, 3));
    }

    #[test]
    fn test_ai_aggressiveness_calculation_rules() {
        // Rule 1: Under-strength active commanders drops to 0
        assert_eq!(calculate_ai_aggressiveness(8, 10, true, 3, 2), 0);

        // Rule 2: Land superiority in priority target maximizes to 5
        assert_eq!(calculate_ai_aggressiveness(10, 10, true, 1, 0), 5);

        // Rule 3: No superiority scales with owned city counts
        assert_eq!(calculate_ai_aggressiveness(10, 10, false, 2, 1), 3);
        assert_eq!(calculate_ai_aggressiveness(10, 10, false, 4, 3), 5); // Clamped to 5 max
    }

    #[test]
    fn test_ai_hold_invasion_priority_math() {
        let mut rng = DeterministicRng::new(555);

        // No land superiority -> 0
        assert_eq!(calculate_hold_invasion_priority(false, 3, 2, 5, false, &mut rng), 0);

        // No bordering holds -> 0
        assert_eq!(calculate_hold_invasion_priority(true, 0, 2, 5, false, &mut rng), 0);

        // Superiority + 2 bordering holds, patrolled (armies_patrolling = 2) -> 1 + 2 + [0..3] = 3..6
        let weight_patrolled = calculate_hold_invasion_priority(true, 2, 2, 5, false, &mut rng);
        assert!(weight_patrolled >= 3 && weight_patrolled <= 6);

        // Superiority + 2 bordering holds, UNPATROLLED -> +10 bonus -> 13..16
        let weight_unpatrolled = calculate_hold_invasion_priority(true, 2, 0, 5, false, &mut rng);
        assert!(weight_unpatrolled >= 13 && weight_unpatrolled <= 16);

        // Desperate capital defense (down to 1 city and this is own capital) -> 100
        assert_eq!(calculate_hold_invasion_priority(true, 0, 0, 1, true, &mut rng), 100);
    }

    // --- MILESTONE M67 COMPLETE PERSISTENCE TEST ---

    #[test]
    fn test_complete_state_persistence_equivalence() {
        let mut state = GameState::new(777);
        state.treasury = 65000;
        state.stockpile.food = 500;
        state.stockpile.metal = 300;
        state.stockpile.wood = 250;
        state.stockpile.trade_docks = 50;

        // Faction
        state.faction.declare_faction(
            "The Great North",
            "Winter Legion",
            BannerColor::EmeraldGreen,
            GovernmentDoctrine::Feudalism,
            state.time.day,
        );

        // Holdings & Governors
        state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).unwrap();
        state.appoint_governor(HoldingId::WHITERUN, "Vignar Gray-Mane").unwrap();

        // Divisions
        state.reinforce_division(DivisionId(0), 40).unwrap();
        state.assign_army_order(DivisionId(0), ArmyOrder::PatrolHold(HoldingId::WHITERUN)).unwrap();
        state.military.divisions[0].commander_name = Some("General Sinmir".to_string());

        // Active Siege
        state.start_siege(HoldingId::FALKREATH, true, true).unwrap();
        state.advance_siege().unwrap(); // OuterGateBreach
        state.advance_siege().unwrap(); // UrbanStreetCombat

        // Cabinet & Diplomacy
        state.appoint_cabinet_minister(CabinetRole::Steward, "Proventus");
        state.diplomacy.relations[0].stance = DiplomaticStance::Truce { days_remaining: 18 };

        // Cooldowns & Crime
        state.territory.camp_cooldowns[9] = 2;
        state.territory.crime_gold[3] = 250;

        // Round-trip serialization
        let json_str = StatePersistence::serialize(&state);
        let restored = StatePersistence::deserialize(&json_str, 777).unwrap();

        // Comprehensive Assertions across all 7 previously omitted subsystems:
        // 1. Active Siege
        assert!(restored.military.active_siege.is_some());
        let siege = restored.military.active_siege.as_ref().unwrap();
        assert_eq!(siege.target_holding, HoldingId::FALKREATH);
        assert_eq!(siege.phase, SiegePhase::UrbanStreetCombat);
        assert!(siege.pincer_assault);
        assert!(siege.artillery_enabled);

        // 2. Divisions
        assert_eq!(restored.military.divisions[0].troop_count, 40);
        assert_eq!(restored.military.divisions[0].order, ArmyOrder::PatrolHold(HoldingId::WHITERUN));
        assert_eq!(restored.military.divisions[0].commander_name, Some("General Sinmir".to_string()));

        // 3. Governors
        assert!(restored.territory.has_appointed_governor(HoldingId::WHITERUN));
        assert_eq!(restored.territory.governors[0].as_ref().unwrap().name, "Vignar Gray-Mane");

        // 4. Cabinet
        assert!(restored.government.has_minister(CabinetRole::Steward));

        // 5. Diplomacy
        assert_eq!(restored.diplomacy.relations[0].stance, DiplomaticStance::Truce { days_remaining: 18 });

        // 6. Cooldowns & Crime
        assert_eq!(restored.territory.camp_cooldowns[9], 2);
        assert_eq!(restored.territory.crime_gold[3], 250);

        // 7. PRNG State Continuation
        assert_eq!(restored.rng.state(), state.rng.state());
    }

    // --- MILESTONE M68 SERIALIZATION SAFETY TESTS ---

    #[test]
    fn test_json_unicode_and_escape_sequences() {
        let json_input = r#"{"name": "Jarl Thórsteinn of Hjaalmarch", "escapes": "\"Line1\\Line2\n\tTab\""}"#;
        let parsed = JsonParser::parse(json_input).unwrap();

        assert_eq!(parsed.get("name").unwrap().as_str().unwrap(), "Jarl Thórsteinn of Hjaalmarch");
        assert_eq!(parsed.get("escapes").unwrap().as_str().unwrap(), "\"Line1\\Line2\n\tTab\"");
    }

    #[test]
    fn test_json_trailing_garbage_rejection() {
        let json_with_garbage = r#"{"version": 1, "day": 1} trailing garbage"#;
        assert!(JsonParser::parse(json_with_garbage).is_err());
    }

    #[test]
    fn test_json_unsupported_version_rejection() {
        let unsupported_json = r#"{"version": 99, "day": 1, "hour": 0, "treasury": 1000}"#;
        let res = StatePersistence::deserialize(unsupported_json, 42);
        assert!(matches!(res, Err(SimulationError::InvalidStateTransition(msg)) if msg.contains("Unsupported savegame version")));
    }

    #[test]
    fn test_json_malformed_syntax_no_panic() {
        // Must reject all malformed input cleanly without panicking
        let malformed_inputs = [
            "",
            "   ",
            "{",
            "}",
            "[",
            "]",
            "{\"unclosed\": ",
            "{\"key\": }",
            "{unquoted: 1}",
            "{\"trailing_comma\": 1, }",
            "{\"key\": \"unclosed string}",
            "[1, 2, ]",
            "null extra",
            "123.456.789",
        ];

        for input in malformed_inputs {
            let res = JsonParser::parse(input);
            assert!(res.is_err(), "Input should be rejected: {}", input);
        }
    }
}
