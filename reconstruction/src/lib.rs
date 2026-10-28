//! Dogfish: Independent Behavioral Reconstruction of Conquest of Skyrim
//!
//! Grounded in the behavioral contracts recovered in `docs/behavioral-contracts.md`
//! and tested under the validation strategy in `docs/reconstruction-testing.md`.

pub mod core;
pub mod faction;
pub mod state;
pub mod territory;
pub mod types;

pub use core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
pub use faction::{BannerColor, FactionProfile, GovernmentDoctrine};
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
        // Test Vector A: Major Walled Capital (Pop 50, TaxRate 2)
        // Expected: 500 * 50 + 250 * 50 * 2 = 25,000 + 25,000 = 50,000 gold
        let mut whiterun = HoldingState::new(HoldingId::WHITERUN, FactionId::Player);
        whiterun.population = 50;
        whiterun.tax_rate = 2;
        assert_eq!(whiterun.calculate_tax(), 50000);

        // Test Vector B: Minor Open Capital (Pop 30, TaxRate 1)
        // Expected: 300 * 30 + 150 * 30 * 1 = 9,000 + 4,500 = 13,500 gold
        let mut falkreath = HoldingState::new(HoldingId::FALKREATH, FactionId::Player);
        falkreath.population = 30;
        falkreath.tax_rate = 1;
        assert_eq!(falkreath.calculate_tax(), 13500);

        // Test Vector C: Dependent Village (Pop 15, TaxRate 1)
        // Expected: 200 * 15 + 100 * 15 * 1 = 3,000 + 1,500 = 4,500 gold
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

        // Transfer Falkreath to Player directly
        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true).is_ok());
        state.holdings[1].tax_rate = 1; // 13,500 tax
        let initial_food = state.stockpile.food;
        let food_needed = state.holdings[1].food_required(); // 30 pop + 10 guards = 40

        // Advance 24 hours (1 day)
        for _ in 0..24 {
            state.tick_hour();
        }

        assert_eq!(state.time.day, 2);
        assert_eq!(state.treasury, 10000 + 13500);
        assert_eq!(state.stockpile.food, initial_food - food_needed);
        assert!(state.check_invariants().is_ok());
    }

    #[test]
    fn test_deterministic_simulation_replay() {
        let mut sim1 = GameState::new(9999);
        let mut sim2 = GameState::new(9999);

        // Run 7 full simulation days (168 hourly ticks)
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
        // Whiterun (0) shares borders with Falkreath (1), Windhelm (3), Dawnstar (4), Riften (5), Morthal (6), Markarth (7)
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::FALKREATH));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::WINDHELM));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::DAWNSTAR));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::RIFTEN));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::MORTHAL));
        assert!(territory::are_capitals_adjacent(HoldingId::WHITERUN, HoldingId::MARKARTH));

        // Solitude (2) only directly borders Morthal (6)
        assert!(territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::MORTHAL));
        assert!(!territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::WHITERUN));
        assert!(!territory::are_capitals_adjacent(HoldingId::SOLITUDE, HoldingId::RIFTEN));

        // Winterhold (8) only borders Windhelm (3) and Dawnstar (4)
        assert!(territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::WINDHELM));
        assert!(territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::DAWNSTAR));
        assert!(!territory::are_capitals_adjacent(HoldingId::WINTERHOLD, HoldingId::FALKREATH));
    }

    #[test]
    fn test_atomic_holding_transfer_and_split_brain_protection() {
        let mut state = GameState::new(202);
        assert_eq!(state.holdings[0].owner, FactionId::Imperial);

        // Player captures first hold (Whiterun) via staging camp
        assert!(state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).is_ok());

        // Assert atomic updates:
        assert_eq!(state.holdings[0].owner, FactionId::Player);
        assert_eq!(state.territory.crime_gold[0], 0);
        assert_eq!(state.holdings[0].loyalty, 60);
        assert!(!state.territory.has_appointed_governor(HoldingId::WHITERUN));

        // Appoint governor
        assert!(state.appoint_governor(HoldingId::WHITERUN, "Vignar Gray-Mane").is_ok());
        assert!(state.territory.has_appointed_governor(HoldingId::WHITERUN));

        // Adjacent hold Falkreath (1) can now be conquered without staging camp because it borders Whiterun (0)
        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, false).is_ok());
        assert_eq!(state.holdings[1].owner, FactionId::Player);
    }

    #[test]
    fn test_invalid_non_adjacent_invasion_blocked() {
        let mut state = GameState::new(303);
        // Give player Falkreath (1)
        assert!(state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true).is_ok());

        // Player attempts to invade Winterhold (8) without staging camp - MUST BE BLOCKED
        // Falkreath does not border Winterhold!
        let result = state.transfer_holding(HoldingId::WINTERHOLD, FactionId::Player, false);
        assert!(matches!(result, Err(SimulationError::HoldingNotAdjacent { .. })));
        assert_eq!(state.holdings[8].owner, FactionId::Stormcloak); // Preserved intact

        // But with a staging camp established, deep assault is permitted (FEAT-SIEG-01)
        let camp_result = state.transfer_holding(HoldingId::WINTERHOLD, FactionId::Player, true);
        assert!(camp_result.is_ok());
        assert_eq!(state.holdings[8].owner, FactionId::Player);
    }

    #[test]
    fn test_wilderness_camp_respawn_cooldown() {
        let mut state = GameState::new(404);
        state.territory.trigger_camp_cooldown(HoldingId::RIVERWOOD, 2);
        assert_eq!(state.territory.camp_cooldowns[9], 2);

        // Advance 1 day (24 hours)
        for _ in 0..24 {
            state.tick_hour();
        }
        assert_eq!(state.territory.camp_cooldowns[9], 1);

        // Advance another day
        for _ in 0..24 {
            state.tick_hour();
        }
        assert_eq!(state.territory.camp_cooldowns[9], 0);
    }
}
