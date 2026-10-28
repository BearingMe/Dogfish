//! Military organization, Task Barrel order dispatching, and combat simulation.
//!
//! Grounded in contracts FEAT-MIL-01 through FEAT-MIL-05, FEAT-BAT-01 through FEAT-BAT-04,
//! and FEAT-SIEG-01 through FEAT-SIEG-04.

use crate::core::{DeterministicRng, Result, SimulationError};
use crate::state::ResourceStockpile;
use crate::types::{DivisionId, HoldingId};

/// Military troop specialization classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TroopClass {
    Infantry,
    ShockWarrior,
    Archer,
    Crossbowman,
    Battlemage,
    Automaton,
}

/// Tactical Task Barrel AI proxy order (FEAT-MIL-03).
///
/// In the original Papyrus architecture, invisible Task Barrel containers in worldspaces
/// update actor package stacks instantaneously without script loops. In the reconstruction,
/// this is represented as an explicit order enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmyOrder {
    FollowPlayer,
    PatrolHold(HoldingId),
    GarrisonFort(HoldingId),
    GarrisonCity(HoldingId),
    CampAt(HoldingId),
    HuntBounties,
    AssaultHolding(HoldingId),
    StandBy,
}

/// Active field division (1 of 10 supported divisions).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Division {
    pub id: DivisionId,
    pub name: String,
    pub commander_name: Option<String>,
    pub troop_class: TroopClass,
    pub troop_count: i32,
    pub power_level: u8, // Levels 1..5
    pub order: ArmyOrder,
}

impl Division {
    pub fn new(id: DivisionId) -> Self {
        Self {
            id,
            name: format!("Division {}", id.0 + 1),
            commander_name: None,
            troop_class: TroopClass::Infantry,
            troop_count: 0,
            power_level: 1,
            order: ArmyOrder::StandBy,
        }
    }

    /// Calculate training upgrade costs based on linear formula:
    /// CostGold = 5000 + (Level - 1) * 10000
    /// CostMetal = 50 + (Level - 1) * 100
    pub fn training_upgrade_cost(current_level: u8) -> (i64, i32) {
        let l = current_level as i64;
        let gold_cost = 5000 + (l - 1) * 10000;
        let metal_cost = (50 + (l - 1) * 100) as i32;
        (gold_cost, metal_cost)
    }

    /// Upgrade division power level (FEAT-MIL-05).
    pub fn upgrade_power(
        &mut self,
        treasury: &mut i64,
        stockpile: &mut ResourceStockpile,
    ) -> Result<()> {
        if self.power_level >= 5 {
            return Err(SimulationError::InvalidStateTransition(
                "Division is already at maximum training tier (Level 5)",
            ));
        }

        let (gold_cost, metal_cost) = Self::training_upgrade_cost(self.power_level);

        if *treasury < gold_cost {
            return Err(SimulationError::InsufficientFunds {
                required: gold_cost,
                available: *treasury,
            });
        }

        if stockpile.metal < metal_cost {
            return Err(SimulationError::InsufficientResources {
                resource: "Metal",
                required: metal_cost,
                available: stockpile.metal,
            });
        }

        *treasury -= gold_cost;
        stockpile.metal -= metal_cost;
        self.power_level += 1;

        Ok(())
    }
}

/// Result of an off-screen detachment combat simulation (cs_bountyscript.psc).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatResult {
    pub is_victory: bool,
    pub starting_troops: i32,
    pub surviving_troops: i32,
    pub casualties: i32,
    pub gold_reward: i64,
}

/// Simulate off-screen detachment combat based on recovered cs_bountyscript.psc math (FEAT-BAT-04).
///
/// Formula:
/// Chance = min(100, (Troops * 100) / EnemyDifficulty)
/// Roll = random(0..100)
/// If Victory (Roll < Chance):
///   Dead = floor((1.0 - (Chance - Roll)/100.0) * Troops)
/// If Defeat:
///   All dispatched troops perish.
pub fn simulate_bounty_combat(
    troops: i32,
    enemy_difficulty: i32,
    reward_gold: i64,
    rng: &mut DeterministicRng,
) -> CombatResult {
    if troops <= 0 {
        return CombatResult {
            is_victory: false,
            starting_troops: 0,
            surviving_troops: 0,
            casualties: 0,
            gold_reward: 0,
        };
    }

    let chance = ((troops as f64 * 100.0) / enemy_difficulty as f64).min(100.0) as u32;
    let roll = rng.gen_percent();

    if roll < chance {
        // Victory: partial casualties based on margin of victory
        let margin_pct = (chance - roll) as f64 / 100.0;
        let dead_ratio = (1.0 - margin_pct).clamp(0.0, 1.0);
        let casualties = (dead_ratio * troops as f64).floor() as i32;
        let surviving = (troops - casualties).max(1);

        CombatResult {
            is_victory: true,
            starting_troops: troops,
            surviving_troops: surviving,
            casualties: troops - surviving,
            gold_reward: reward_gold,
        }
    } else {
        // Defeat: entire detachment wiped out
        CombatResult {
            is_victory: false,
            starting_troops: troops,
            surviving_troops: 0,
            casualties: troops,
            gold_reward: 0,
        }
    }
}

/// Tactical phases of a city siege assault (FEAT-SIEG-01 through FEAT-SIEG-04).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiegePhase {
    /// 1. Staging Camp established outside target walls.
    StagingCamp,
    /// 2. Outer gate breach with catapults, rams, or pincer assault.
    OuterGateBreach,
    /// 3. Street warfare inside city walls with active battle throttling.
    UrbanStreetCombat,
    /// 4. Palace breached; Jarl confronted.
    PalaceConfrontation,
    /// 5. Jarl capitulates; victory parade and territory transfer.
    Capitulated,
}

/// Active siege operational state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiegeOperation {
    pub target_holding: HoldingId,
    pub phase: SiegePhase,
    pub pincer_assault: bool,
    pub artillery_enabled: bool,
    pub battering_ram: bool,
    pub attacker_casualties: i32,
    pub defender_casualties: i32,
    pub reinforcement_wave: u8,
    pub battle_concurrency_cap: i32, // CS_BattleSize (default 40)
}

impl SiegeOperation {
    pub fn new(target: HoldingId, pincer: bool, artillery: bool) -> Self {
        Self {
            target_holding: target,
            phase: SiegePhase::StagingCamp,
            pincer_assault: pincer,
            artillery_enabled: artillery,
            battering_ram: artillery,
            attacker_casualties: 0,
            defender_casualties: 0,
            reinforcement_wave: 0,
            battle_concurrency_cap: 40,
        }
    }

    /// Register enemy casualty and check for reinforcement wave trigger.
    ///
    /// Original Mod Behavior: Reinforcements spawn in waves at 20, 40, 60, 80, 100 defender deaths.
    pub fn record_defender_casualty(&mut self) -> bool {
        self.defender_casualties += 1;
        let trigger_wave = match self.defender_casualties {
            20 => Some(1),
            40 => Some(2),
            60 => Some(3),
            80 => Some(4),
            100 => Some(5),
            _ => None,
        };

        if let Some(wave) = trigger_wave {
            self.reinforcement_wave = wave;
            true
        } else {
            false
        }
    }

    /// Calculate casualty mitigation factor from siege artillery (DEF-FIDEL-03).
    pub fn attacker_casualty_factor(&self) -> f64 {
        if self.artillery_enabled {
            0.5 // 50% casualty reduction during gate breach operations
        } else {
            1.0
        }
    }

    /// Calculate defender casualty multiplier from pincer assault flanking (DEF-FIDEL-03).
    pub fn defender_casualty_factor(&self) -> f64 {
        if self.pincer_assault {
            1.25 // 25% increased defender casualties from two-pronged assault
        } else {
            1.0
        }
    }

    /// Advance siege to next tactical phase.
    pub fn advance_phase(&mut self) -> Result<SiegePhase> {
        let next = match self.phase {
            SiegePhase::StagingCamp => SiegePhase::OuterGateBreach,
            SiegePhase::OuterGateBreach => SiegePhase::UrbanStreetCombat,
            SiegePhase::UrbanStreetCombat => SiegePhase::PalaceConfrontation,
            SiegePhase::PalaceConfrontation => SiegePhase::Capitulated,
            SiegePhase::Capitulated => {
                return Err(SimulationError::InvalidStateTransition(
                    "Siege has already concluded with Capitulation",
                ))
            }
        };
        self.phase = next;
        Ok(next)
    }
}

/// Military manager tracking all 10 divisions and active sieges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilitaryManager {
    pub divisions: Vec<Division>,
    pub active_siege: Option<SiegeOperation>,
}

impl MilitaryManager {
    pub fn new() -> Self {
        let mut divisions = Vec::with_capacity(DivisionId::MAX_DIVISIONS);
        for i in 0..DivisionId::MAX_DIVISIONS {
            divisions.push(Division::new(DivisionId(i as u8)));
        }

        Self {
            divisions,
            active_siege: None,
        }
    }

    /// Assign an order to a division (FEAT-MIL-03).
    pub fn assign_order(&mut self, div_id: DivisionId, order: ArmyOrder) -> Result<()> {
        if !div_id.is_valid() {
            return Err(SimulationError::InvalidDivisionId(div_id.0));
        }
        self.divisions[div_id.0 as usize].order = order;
        Ok(())
    }

    /// Assign recruited troops to a division.
    pub fn reinforce_division(&mut self, div_id: DivisionId, count: i32) -> Result<()> {
        if !div_id.is_valid() {
            return Err(SimulationError::InvalidDivisionId(div_id.0));
        }
        if count <= 0 {
            return Err(SimulationError::InvalidStateTransition(
                "Reinforcement troop count must be positive",
            ));
        }
        self.divisions[div_id.0 as usize].troop_count += count;
        Ok(())
    }

    /// Appoint commander to division (FEAT-MIL-01).
    pub fn appoint_commander(
        &mut self,
        div_id: DivisionId,
        commander: impl Into<String>,
    ) -> Result<()> {
        if !div_id.is_valid() {
            return Err(SimulationError::InvalidDivisionId(div_id.0));
        }
        self.divisions[div_id.0 as usize].commander_name = Some(commander.into());
        Ok(())
    }

    /// Total active soldiers across all 10 divisions.
    pub fn total_soldiers(&self) -> i32 {
        self.divisions.iter().map(|d| d.troop_count).sum()
    }
}

impl Default for MilitaryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard soldier capacity per army division in the strategic AI model (cs_tmscript.psc:1512).
pub const DIVISION_SOLDIER_CAPACITY: i32 = 20;

/// Calculate surviving vs dead division blocks from surviving troop counts (cs_tmscript.psc:1511-1520).
///
/// Recovered Formula:
/// RemainingArmies = ceil(TroopsRemaining / 20)
/// DeadArmies = InitialArmies - RemainingArmies
pub fn calculate_surviving_divisions(troops_remaining: i32, initial_armies: i32) -> (i32, i32) {
    if troops_remaining <= 0 {
        return (0, initial_armies);
    }
    let remaining = ((troops_remaining as f64) / DIVISION_SOLDIER_CAPACITY as f64).ceil() as i32;
    let dead = (initial_armies - remaining).max(0);
    (remaining, dead)
}

/// Calculate strategic campaign aggressiveness for background AI factions (cs_tmscript.psc:991-1008).
///
/// Recovered Rules:
/// - If active commanders < total commanders: drops to 0 (defensive recovery).
/// - If priority hold has land superiority: maximized to 5.
/// - Otherwise: CityCount + MinorCityCount, clamped to max 5.
pub fn calculate_ai_aggressiveness(
    active_commanders: i32,
    total_commanders: i32,
    has_land_superiority: bool,
    city_count: i32,
    minor_city_count: i32,
) -> u8 {
    if active_commanders < total_commanders {
        0
    } else if has_land_superiority {
        5
    } else {
        let total_cities = city_count + minor_city_count;
        total_cities.clamp(0, 5) as u8
    }
}

/// Calculate hold invasion priority weight for AI campaign targeting (cs_tmscript.psc:1030-1045).
///
/// Recovered Formula:
/// - If faction reduced to 1 city and this is their capital: 100 (desperate defense).
/// - If not superior or no bordering holds: 0.
/// - Base = 1 + OwnHoldsBordering + RandomInt(0, 3).
/// - If ArmiesPatrolling == 0: +10 vulnerability bonus.
pub fn calculate_hold_invasion_priority(
    has_land_superiority: bool,
    own_bordering_holds: i32,
    armies_patrolling: i32,
    total_cities_owned: i32,
    is_own_capital: bool,
    rng: &mut DeterministicRng,
) -> u32 {
    if has_land_superiority && total_cities_owned == 1 && is_own_capital {
        return 100;
    }
    if !has_land_superiority || own_bordering_holds <= 0 {
        return 0;
    }

    let mut chance = 1 + own_bordering_holds + rng.gen_range_i32(0, 3);
    if armies_patrolling == 0 {
        chance += 10;
    }

    chance.max(0) as u32
}
