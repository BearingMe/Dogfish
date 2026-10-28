//! Territorial hierarchy, spatial adjacency, and atomic ownership transitions.
//!
//! Grounded in contracts FEAT-TERR-01 through FEAT-TERR-04 and FEAT-SETT-01 through FEAT-SETT-03.
//! Addresses the non-transactional split-brain vulnerability identified in M40.

use crate::core::{Result, SimulationError};
use crate::types::{FactionId, HoldingId};

/// Spatial adjacency matrix for the 9 Hold Capitals of Skyrim.
///
/// Ensures military invasions require geographic contiguity or established staging camps.
pub fn are_capitals_adjacent(a: HoldingId, b: HoldingId) -> bool {
    if !a.is_capital() || !b.is_capital() {
        return false;
    }
    if a == b {
        return false;
    }

    // Direct geographic border connections between Skyrim's 9 holds:
    // 0: Whiterun borders 1 (Falkreath), 3 (Windhelm), 4 (Dawnstar), 5 (Riften), 6 (Morthal), 7 (Markarth)
    // 1: Falkreath borders 0 (Whiterun), 5 (Riften), 7 (Markarth)
    // 2: Solitude borders 6 (Morthal)
    // 3: Windhelm borders 0 (Whiterun), 4 (Dawnstar), 5 (Riften), 8 (Winterhold)
    // 4: Dawnstar borders 0 (Whiterun), 3 (Windhelm), 6 (Morthal), 8 (Winterhold)
    // 5: Riften borders 0 (Whiterun), 1 (Falkreath), 3 (Windhelm)
    // 6: Morthal borders 0 (Whiterun), 2 (Solitude), 4 (Dawnstar)
    // 7: Markarth borders 0 (Whiterun), 1 (Falkreath)
    // 8: Winterhold borders 3 (Windhelm), 4 (Dawnstar)
    match (a.0, b.0) {
        (0, 1) | (1, 0) => true,
        (0, 3) | (3, 0) => true,
        (0, 4) | (4, 0) => true,
        (0, 5) | (5, 0) => true,
        (0, 6) | (6, 0) => true,
        (0, 7) | (7, 0) => true,
        (1, 5) | (5, 1) => true,
        (1, 7) | (7, 1) => true,
        (2, 6) | (6, 2) => true,
        (3, 4) | (4, 3) => true,
        (3, 5) | (5, 3) => true,
        (3, 8) | (8, 3) => true,
        (4, 6) | (6, 4) => true,
        (4, 8) | (8, 4) => true,
        _ => false,
    }
}

/// Appointed provincial governor / Jarl for a conquered hold capital.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointedGovernor {
    pub name: String,
    pub holding_id: HoldingId,
    pub loyalty: i32,
    pub appointment_day: u32,
}

/// Territorial management manager handling atomic transitions and garrisons.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerritoryManager {
    /// Active appointed governors keyed by HoldingId.
    pub governors: Vec<Option<AppointedGovernor>>,
    /// Active camp cooldown timers (HoldingId index -> days remaining).
    pub camp_cooldowns: [u32; HoldingId::COUNT],
    /// Local crime gold per holding (reset to 0 upon atomic capture).
    pub crime_gold: [u32; HoldingId::COUNT],
}

impl TerritoryManager {
    pub fn new() -> Self {
        let mut governors = Vec::with_capacity(HoldingId::COUNT);
        for _ in 0..HoldingId::COUNT {
            governors.push(None);
        }

        Self {
            governors,
            camp_cooldowns: [0; HoldingId::COUNT],
            crime_gold: [0; HoldingId::COUNT],
        }
    }

    /// Appoint a governor (Jarl) to an owned capital (FEAT-SETT-02).
    pub fn appoint_governor(
        &mut self,
        holding_id: HoldingId,
        governor_name: impl Into<String>,
        day: u32,
    ) -> Result<()> {
        if !holding_id.is_capital() {
            return Err(SimulationError::InvalidStateTransition(
                "Governors can only be appointed to Hold Capitals (0..8)",
            ));
        }

        self.governors[holding_id.0 as usize] = Some(AppointedGovernor {
            name: governor_name.into(),
            holding_id,
            loyalty: 100,
            appointment_day: day,
        });

        Ok(())
    }

    /// Check if a capital has an appointed living governor (required for The Moot FEAT-VICT-02).
    pub fn has_appointed_governor(&self, holding_id: HoldingId) -> bool {
        if (holding_id.0 as usize) < self.governors.len() {
            self.governors[holding_id.0 as usize].is_some()
        } else {
            false
        }
    }

    /// Set wilderness camp into a 24-hour respawn recovery cooldown (FEAT-TERR-04).
    pub fn trigger_camp_cooldown(&mut self, holding_id: HoldingId, days: u32) {
        if (holding_id.0 as usize) < self.camp_cooldowns.len() {
            self.camp_cooldowns[holding_id.0 as usize] = days;
        }
    }

    /// Advance cooldown timers during daily tick.
    pub fn advance_day(&mut self) {
        for cd in &mut self.camp_cooldowns {
            if *cd > 0 {
                *cd -= 1;
            }
        }
    }

    /// Execute an ATOMIC territory transfer (FEAT-SETT-01, FEAT-TERR-02).
    ///
    /// Unlike the original mod which executes non-transactional sequential script calls
    /// risking split-brain desync (setting CWOwner before AllHoldings), this updates
    /// ownership, resets crime gold, and validates adjacency within a single atomic step.
    pub fn transfer_ownership(
        &mut self,
        holdings: &mut [crate::state::HoldingState],
        target_id: HoldingId,
        new_owner: FactionId,
        has_staging_camp: bool,
    ) -> Result<()> {
        if !target_id.is_valid() {
            return Err(SimulationError::InvalidHoldingId(target_id.0));
        }

        let idx = target_id.0 as usize;
        let old_owner = holdings[idx].owner;
        if old_owner == new_owner {
            return Err(SimulationError::InvalidStateTransition(
                "Target holding is already owned by this faction",
            ));
        }

        // Spatial adjacency check: If taking a capital, must share a border with an already-owned
        // capital OR have an established wilderness staging camp (FEAT-SIEG-01).
        if target_id.is_capital() && !has_staging_camp {
            let mut has_adjacent_base = false;
            for other_id in 0..9u8 {
                let other_hid = HoldingId(other_id);
                if holdings[other_id as usize].owner == new_owner
                    && are_capitals_adjacent(other_hid, target_id)
                {
                    has_adjacent_base = true;
                    break;
                }
            }

            // If new owner has at least one capital already, adjacency is required
            let total_owned_capitals = holdings[0..9]
                .iter()
                .filter(|h| h.owner == new_owner)
                .count();

            if total_owned_capitals > 0 && !has_adjacent_base {
                return Err(SimulationError::HoldingNotAdjacent {
                    from: 255,
                    to: target_id.0,
                });
            }
        }

        // --- ATOMIC TRANSACTION ---
        // 1. Update holding owner in domain state
        holdings[idx].owner = new_owner;

        // 2. Clear local crime bounty (FEAT-LAW-01: Autonomous Jurisdiction)
        self.crime_gold[idx] = 0;

        // 3. Reset provincial governor alias (requires new appointment)
        self.governors[idx] = None;

        // 4. Reset holding loyalty base to 60 for conquered populace
        holdings[idx].loyalty = 60;

        Ok(())
    }
}

impl Default for TerritoryManager {
    fn default() -> Self {
        Self::new()
    }
}
