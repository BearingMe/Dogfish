//! Dogfish: Independent Behavioral Reconstruction of Conquest of Skyrim
//!
//! Grounded in the behavioral contracts recovered in `docs/behavioral-contracts.md`
//! and tested under the validation strategy in `docs/reconstruction-testing.md`.

pub mod core;
pub mod state;
pub mod types;

pub use core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
pub use state::{FacilityLevels, GameState, HoldingState, ResourceStockpile};
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

        // Give player Falkreath
        state.holdings[1].owner = FactionId::Player;
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
}
