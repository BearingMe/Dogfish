//! Adversarial Regression Test Suite: Known Weaknesses and Failure Scenarios
//!
//! Milestone M65: Specifically designed to expose known defects in the current implementation.
//! These tests establish the baseline for Phase 17 and Phase 18 repairs.

use dogfish_reconstruction::{
    ArmyOrder, DivisionId, FacilityType, FactionId, GameState, HoldingId, JudicialSentence,
    LegalOffense, SiegePhase, StatePersistence,
};

/// Case 1: Save/load round-trip with an active siege operation.
///
/// SUSPECTED DEFECT: StatePersistence::serialize omits military.active_siege.
/// EXPECTED: A restored game state retains the active siege operation, its phase, and casualties.
/// ACTUAL: military.active_siege is None upon deserialization.
#[test]
fn test_case_01_save_load_with_active_siege() {
    let mut state = GameState::new(101);
    state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).unwrap();

    // Start siege on Falkreath and advance to UrbanStreetCombat
    state.start_siege(HoldingId::FALKREATH, true, true).unwrap();
    state.advance_siege().unwrap(); // OuterGateBreach
    state.advance_siege().unwrap(); // UrbanStreetCombat

    let saved_json = StatePersistence::serialize(&state);
    let restored = StatePersistence::deserialize(&saved_json, 101).unwrap();

    // ADVERSARIAL ASSERTION: Active siege must survive serialization
    assert!(
        restored.military.active_siege.is_some(),
        "DEFECT CONFIRMED: active_siege is lost across serialization!"
    );
    let restored_siege = restored.military.active_siege.as_ref().unwrap();
    assert_eq!(restored_siege.target_holding, HoldingId::FALKREATH);
    assert_eq!(restored_siege.phase, SiegePhase::UrbanStreetCombat);
}

/// Case 2: Save/load with modified armies and appointed governors.
///
/// SUSPECTED DEFECT: StatePersistence::serialize omits military.divisions and territory.governors.
/// EXPECTED: Restored game state retains division soldier counts, orders, and appointed Jarls.
/// ACTUAL: Divisions and governors are reset to initial defaults upon deserialization.
#[test]
fn test_case_02_save_load_with_modified_armies_and_governors() {
    let mut state = GameState::new(202);
    state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true).unwrap();

    // 1. Reinforce division 0 with 50 soldiers and assign Patrol order
    state.reinforce_division(DivisionId(0), 50).unwrap();
    state.assign_army_order(DivisionId(0), ArmyOrder::PatrolHold(HoldingId::WHITERUN)).unwrap();

    // 2. Appoint governor to Whiterun
    state.appoint_governor(HoldingId::WHITERUN, "Vignar Gray-Mane").unwrap();

    let saved_json = StatePersistence::serialize(&state);
    let restored = StatePersistence::deserialize(&saved_json, 202).unwrap();

    // ADVERSARIAL ASSERTION 1: Division state must survive serialization
    assert_eq!(
        restored.military.divisions[0].troop_count, 50,
        "DEFECT CONFIRMED: Division troop_count is lost across serialization!"
    );
    assert_eq!(
        restored.military.divisions[0].order,
        ArmyOrder::PatrolHold(HoldingId::WHITERUN),
        "DEFECT CONFIRMED: Division order is lost across serialization!"
    );

    // ADVERSARIAL ASSERTION 2: Appointed governor must survive serialization
    assert!(
        restored.territory.has_appointed_governor(HoldingId::WHITERUN),
        "DEFECT CONFIRMED: Appointed governor is lost across serialization!"
    );
}

/// Case 3: Faction names containing quotes and backslashes.
///
/// SUSPECTED DEFECT: StatePersistence::serialize formats string literals directly into JSON
/// without escaping quotes (") and backslashes (\), producing invalid JSON that breaks the parser.
/// EXPECTED: Faction names with quotes and backslashes serialize and deserialize cleanly.
/// ACTUAL: JsonParser fails with syntax error or corrupted string.
#[test]
fn test_case_03_faction_names_with_quotes_and_backslashes() {
    let mut state = GameState::new(303);
    let raw_name = r#"The "Sovereign" \ King's / Realm"#;
    state.faction.name = raw_name.to_string();

    let saved_json = StatePersistence::serialize(&state);
    let restored_res = StatePersistence::deserialize(&saved_json, 303);

    assert!(
        restored_res.is_ok(),
        "DEFECT CONFIRMED: JSON serializer produces invalid JSON when string contains quotes!"
    );
    let restored = restored_res.unwrap();
    assert_eq!(restored.faction.name, raw_name);
}

/// Case 4: Facility upgrades in enemy-owned holdings.
///
/// SUSPECTED DEFECT: GameState::upgrade_facility does not check if holding.owner == Player.
/// EXPECTED: Player attempting to upgrade a facility in an enemy capital is rejected with an error.
/// ACTUAL: Function succeeds, deducting player gold and wood to upgrade an enemy city.
#[test]
fn test_case_04_facility_upgrade_in_enemy_holding() {
    let mut state = GameState::new(404);
    // Solitude (2) is owned by Imperial
    assert_eq!(state.holdings[2].owner, FactionId::Imperial);

    // Player attempts to upgrade Imperial mine in Solitude
    let res = state.upgrade_facility(HoldingId::SOLITUDE, FacilityType::Mine);

    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Player can upgrade facilities in enemy-owned holdings!"
    );
}

/// Case 5: Governor appointments in enemy-owned holdings.
///
/// SUSPECTED DEFECT: GameState::appoint_governor does not verify holding ownership.
/// EXPECTED: Player appointing a Jarl to an unowned/enemy capital is rejected.
/// ACTUAL: Function succeeds, appointing a player governor to an enemy-held city.
#[test]
fn test_case_05_governor_appointment_in_enemy_holding() {
    let mut state = GameState::new(505);
    // Windhelm (3) is owned by Stormcloak
    assert_eq!(state.holdings[3].owner, FactionId::Stormcloak);

    // Player attempts to appoint a Jarl to Windhelm
    let res = state.appoint_governor(HoldingId::WINDHELM, "Brunwulf Free-Winter");

    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Player can appoint governors in enemy-owned holdings!"
    );
}

/// Case 6: Repeated siege initiation without concluding previous siege.
///
/// SUSPECTED DEFECT: GameState::start_siege overwrites active_siege silently.
/// EXPECTED: Starting a new siege while a siege is already active is rejected or requires abandonment.
/// ACTUAL: Silently overwrites previous siege operation state.
#[test]
fn test_case_06_repeated_siege_initiation() {
    let mut state = GameState::new(606);
    state.start_siege(HoldingId::WINDHELM, true, true).unwrap();

    // Attempt to start a second concurrent siege on Markarth
    let res = state.start_siege(HoldingId::MARKARTH, false, false);

    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Starting a second siege silently overwrites existing active siege!"
    );
}

/// Case 7: Siege progression without a participating army.
///
/// SUSPECTED DEFECT: GameState::advance_siege progresses all 5 phases without checking army presence.
/// EXPECTED: Advancing siege phases requires an assigned field division with living troops.
/// ACTUAL: Player can advance all phases to Capitulation with 0 troops in 0 seconds.
#[test]
fn test_case_07_siege_progression_without_participating_army() {
    let mut state = GameState::new(707);
    assert_eq!(state.military.total_soldiers(), 0);

    state.start_siege(HoldingId::WINDHELM, true, true).unwrap();

    // Advancing siege with 0 total soldiers in player faction should be rejected
    let res = state.advance_siege();

    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Sieges can be advanced to victory without any participating troops!"
    );
}

/// Case 8: Negative court fines and negative ransom payments.
///
/// SUSPECTED DEFECT: judge_court_case and ransom_pows accept negative values that corrupt treasury.
/// EXPECTED: Negative fines or ransoms are rejected with an error.
/// ACTUAL: Treasury is decremented or negative values applied without validation.
#[test]
fn test_case_08_negative_court_fines_and_ransom() {
    let mut state = GameState::new(808);

    // Negative fine
    let res = state.judge_court_case(LegalOffense::Theft, JudicialSentence::Fine(-5000));
    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Negative court fines are accepted and corrupt treasury!"
    );
}

/// Case 9: Extreme and overflowing resource balances.
///
/// SUSPECTED DEFECT: Treasury additions near i64::MAX cause integer overflow panic.
/// EXPECTED: Operations handle overflow gracefully or clamp to maximum safe bounds.
#[test]
fn test_case_09_resource_overflow_protection() {
    let mut state = GameState::new(909);
    state.treasury = i64::MAX - 100;

    // Deposit that would overflow i64
    let res = state.deposit_treasury(500);
    assert!(
        res.is_err(),
        "DEFECT CONFIRMED: Treasury deposit overflows i64 without checked addition!"
    );
}

/// Case 10: Invalid faction or holding transitions.
///
/// EXPECTED: Transferring holding to its existing owner returns an error.
#[test]
fn test_case_10_invalid_holding_transitions() {
    let mut state = GameState::new(1010);
    // Whiterun starts Imperial
    assert_eq!(state.holdings[0].owner, FactionId::Imperial);

    // Transferring Whiterun to Imperial again should fail
    let res = state.transfer_holding(HoldingId::WHITERUN, FactionId::Imperial, true);
    assert!(res.is_err());
}

/// Case 11: Repeated reconstruction and save idempotency.
///
/// EXPECTED: Serializing and deserializing state across multiple cycles preserves exact values.
#[test]
fn test_case_11_savegame_idempotency_cycles() {
    let state = GameState::new(1111);
    let json_1 = StatePersistence::serialize(&state);
    let state_2 = StatePersistence::deserialize(&json_1, 1111).unwrap();
    let json_2 = StatePersistence::serialize(&state_2);

    assert_eq!(json_1, json_2, "Serialization is not idempotent across cycles!");
}

/// Case 12: Interrupted operations and multi-step transaction safety.
///
/// EXPECTED: Facility upgrade fails atomically if resources are insufficient, leaving prior state intact.
#[test]
fn test_case_12_interrupted_operation_resource_conservation() {
    let mut state = GameState::new(1212);
    state.stockpile.wood = 5; // Insufficient wood for upgrade (requires 30)
    let initial_gold = state.treasury;

    let res = state.upgrade_facility(HoldingId::FALKREATH, FacilityType::Mine);
    assert!(res.is_err());

    // Treasury must not be partially deducted if wood was insufficient
    assert_eq!(state.treasury, initial_gold);
}
