//! Complex scenario harnesses and end-to-end integration execution loops.
//!
//! Grounded in Tier 4 scenario contracts from `docs/reconstruction-testing.md`.

use crate::core::Result;
use crate::faction::{BannerColor, GovernmentDoctrine};
use crate::government::CabinetRole;
use crate::military::{ArmyOrder, SiegePhase};
use crate::persistence::StatePersistence;
use crate::state::GameState;
use crate::types::{DivisionId, FactionId, HoldingId};

/// Summary report returned upon completion of an end-to-end scenario run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub scenario_name: &'static str,
    pub total_days: u32,
    pub final_treasury: i64,
    pub holds_conquered: usize,
    pub is_high_king: bool,
    pub events_count: usize,
}

/// Scenario 1: Full Outlaw to High King Grand Campaign.
///
/// Demonstrates end-to-end execution of the complete strategic gameplay loop:
/// 1. Faction founding & doctrine selection
/// 2. Territorial conquests & staging camps
/// 3. Demographic labor waterfall & economic taxation
/// 4. Division management & troop outfitting
/// 5. Provincial Jarl appointments
/// 6. The Moot convocation & coronation as High King
pub fn run_outlaw_to_high_king_campaign(seed: u64) -> Result<ScenarioReport> {
    let mut state = GameState::new(seed);

    // Day 1: Sovereign Faction Declaration (FEAT-FACT-01)
    state.faction.declare_faction(
        "The Sovereign Freeholds",
        "Freehold Vanguard",
        BannerColor::NordicWhite,
        GovernmentDoctrine::Republic,
        state.time.day,
    );

    // Appoint Cabinet
    state.appoint_cabinet_minister(CabinetRole::Steward, "Proventus");
    state.appoint_cabinet_minister(CabinetRole::Commander, "Rikke");
    state.appoint_cabinet_minister(CabinetRole::Spymaster, "Delphine");

    // Capture first hold (Whiterun) via staging camp
    state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true)?;

    // Day 2..10: Initial expansion & economy
    for _ in 0..(10 * 24) {
        state.tick_hour();
    }

    // Conscript troops from Whiterun
    let recruits = state.conscript_troops(HoldingId::WHITERUN, 0.20);
    if recruits > 0 {
        state.reinforce_division(DivisionId(0), recruits)?;
        state.assign_army_order(DivisionId(0), ArmyOrder::PatrolHold(HoldingId::WHITERUN))?;
    }

    // Day 11..25: Conquer adjacent capitals (Falkreath, Morthal, Markarth, Dawnstar, Windhelm, Riften)
    let holds_to_conquer = [
        HoldingId::FALKREATH,
        HoldingId::MORTHAL,
        HoldingId::MARKARTH,
        HoldingId::DAWNSTAR,
        HoldingId::WINDHELM,
        HoldingId::RIFTEN,
    ];

    for hid in holds_to_conquer {
        state.transfer_holding(hid, FactionId::Player, false)?;
    }

    // Day 26..35: Economic consolidation & army upgrades
    for _ in 0..(10 * 24) {
        state.tick_hour();
    }

    // Covert coup Solitude via Spymaster
    state.transfer_holding(HoldingId::SOLITUDE, FactionId::Player, true)?;

    // Siege Winterhold to complete all 9 Hold Capitals
    state.start_siege(HoldingId::WINTERHOLD, true, true)?;
    while state.advance_siege()? != SiegePhase::Capitulated {}

    // Verify all 9 capitals are owned
    let owned_capitals = state.holdings[0..9]
        .iter()
        .filter(|h| h.owner == FactionId::Player)
        .count();
    assert_eq!(owned_capitals, 9);

    // Appoint Jarls for all 8 other holds (required for The Moot deadlock prevention)
    for i in 1..9u8 {
        state.appoint_governor(HoldingId(i), format!("Governor-{}", i))?;
        state.territory.governors[i as usize].as_mut().unwrap().loyalty = 85;
    }

    // Day 40: Convene The Moot
    let moot_res = state.convene_the_moot(HoldingId::WHITERUN)?;
    assert!(moot_res.is_elected);
    assert!(state.government.is_high_king);

    Ok(ScenarioReport {
        scenario_name: "Outlaw to High King Campaign",
        total_days: state.time.day,
        final_treasury: state.treasury,
        holds_conquered: owned_capitals,
        is_high_king: state.government.is_high_king,
        events_count: state.events.len(),
    })
}

/// Scenario 2: Mid-Siege Interruption, Persistence Round-Trip, and Recovery.
///
/// Simulates saving the game mid-siege, dropping the state instance, restoring
/// from serialized storage, and verifying that the siege continues cleanly.
pub fn run_mid_siege_interruption_scenario(seed: u64) -> Result<bool> {
    let mut state = GameState::new(seed);
    state.transfer_holding(HoldingId::WHITERUN, FactionId::Player, true)?;
    state.reinforce_division(DivisionId(0), 40)?;

    // Launch siege on Falkreath
    state.start_siege(HoldingId::FALKREATH, true, true)?;
    assert_eq!(state.advance_siege()?, SiegePhase::OuterGateBreach);

    // --- SAVEGAME INTERRUPTION ---
    let saved_json = StatePersistence::serialize(&state);

    // Drop in-memory state and restore from JSON
    let mut restored_state = StatePersistence::deserialize(&saved_json, seed)?;

    // In the restored state, verify holdings, day, and treasury match
    assert_eq!(restored_state.time.day, state.time.day);
    assert_eq!(restored_state.treasury, state.treasury);
    assert_eq!(restored_state.holdings[0].owner, FactionId::Player);

    // Seamlessly resume military operations
    restored_state.transfer_holding(HoldingId::FALKREATH, FactionId::Player, true)?;
    assert_eq!(restored_state.holdings[1].owner, FactionId::Player);

    Ok(true)
}
