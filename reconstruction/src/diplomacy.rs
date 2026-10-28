//! Bilateral diplomacy, truce negotiations, and POW ransoming.
//!
//! Grounded in contracts FEAT-DIPL-01 through FEAT-DIPL-03.

use crate::core::{DeterministicRng, Result, SimulationError};
use crate::types::{DiplomaticStance, FactionId};

/// Bilateral treaty acceptance chance calculator based on recovered cs_negotiationscript.psc.
///
/// Recovered Formula:
/// Base: 25%
/// Relationship: +25% (Friendly) or -25% (Hostile)
/// Ideology: +25% (Compatible) or -25% (Conflicting)
/// Troop Leverage:
///   If player_troops / enemy_troops < 0.60: -25% penalty
///   If player_troops / enemy_troops >= 1.00: +25% bonus
/// Clamped between 5% and 95%.
pub fn calculate_truce_acceptance_chance(
    relationship_score: i32, // -100..100
    compatible_doctrine: bool,
    player_troops: i32,
    enemy_troops: i32,
) -> u32 {
    let mut chance = 25i32;

    if relationship_score > 20 {
        chance += 25;
    } else if relationship_score < -20 {
        chance -= 25;
    }

    if compatible_doctrine {
        chance += 25;
    } else {
        chance -= 25;
    }

    if enemy_troops > 0 {
        let ratio = (player_troops as f64 * 100.0) / enemy_troops as f64;
        if ratio < 60.0 {
            chance -= 25;
        } else if ratio >= 100.0 {
            chance += 25;
        }
    }

    chance.max(5).min(95) as u32
}

/// Bilateral diplomatic relationship between Player and a foreign faction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignFactionRelation {
    pub faction_id: FactionId,
    pub stance: DiplomaticStance,
    pub relationship_score: i32, // -100..100
    pub estimated_troops: i32,
    pub held_pow_count: u32,
}

impl ForeignFactionRelation {
    pub fn new(faction_id: FactionId) -> Self {
        Self {
            faction_id,
            stance: DiplomaticStance::War,
            relationship_score: -50,
            estimated_troops: 150,
            held_pow_count: 0,
        }
    }
}

/// Diplomacy manager tracking foreign relations and negotiations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiplomacyManager {
    pub relations: Vec<ForeignFactionRelation>,
}

impl DiplomacyManager {
    pub fn new() -> Self {
        Self {
            relations: vec![
                ForeignFactionRelation::new(FactionId::Imperial),
                ForeignFactionRelation::new(FactionId::Stormcloak),
            ],
        }
    }

    /// Advance active truce days during daily tick.
    pub fn advance_day(&mut self) {
        for rel in &mut self.relations {
            if let DiplomaticStance::Truce { days_remaining } = &mut rel.stance {
                if *days_remaining > 1 {
                    *days_remaining -= 1;
                } else {
                    rel.stance = DiplomaticStance::Neutral;
                }
            }
        }
    }

    /// Negotiate a bilateral non-aggression truce (FEAT-DIPL-02).
    pub fn negotiate_truce(
        &mut self,
        target_faction: FactionId,
        compatible_doctrine: bool,
        player_troops: i32,
        truce_days: u32,
        rng: &mut DeterministicRng,
    ) -> Result<bool> {
        let rel = self
            .relations
            .iter_mut()
            .find(|r| r.faction_id == target_faction)
            .ok_or(SimulationError::InvalidStateTransition(
                "Foreign faction not recognized for treaty negotiations",
            ))?;

        let chance = calculate_truce_acceptance_chance(
            rel.relationship_score,
            compatible_doctrine,
            player_troops,
            rel.estimated_troops,
        );

        let roll = rng.gen_percent();
        if roll < chance {
            rel.stance = DiplomaticStance::Truce {
                days_remaining: truce_days,
            };
            rel.relationship_score = (rel.relationship_score + 20).min(100);
            Ok(true)
        } else {
            rel.relationship_score = (rel.relationship_score - 10).max(-100);
            Ok(false)
        }
    }

    /// Ransom captured prisoners of war for gold (FEAT-DIPL-03).
    pub fn ransom_pows(
        &mut self,
        target_faction: FactionId,
        treasury: &mut i64,
        gold_per_pow: i64,
    ) -> Result<u32> {
        let rel = self
            .relations
            .iter_mut()
            .find(|r| r.faction_id == target_faction)
            .ok_or(SimulationError::InvalidStateTransition(
                "Target faction not found for POW ransom",
            ))?;

        if gold_per_pow < 0 {
            return Err(SimulationError::InvalidStateTransition(
                "Ransom payment cannot be negative",
            ));
        }

        if rel.held_pow_count == 0 {
            return Ok(0);
        }

        let ransomed = rel.held_pow_count;
        let total_ransom = (ransomed as i64)
            .checked_mul(gold_per_pow)
            .ok_or(SimulationError::InvalidStateTransition("Ransom gold overflow"))?;
        *treasury = treasury
            .checked_add(total_ransom)
            .ok_or(SimulationError::InvalidStateTransition("Treasury overflow on ransom deposit"))?;
        rel.held_pow_count = 0;

        Ok(ransomed)
    }
}

impl Default for DiplomacyManager {
    fn default() -> Self {
        Self::new()
    }
}
