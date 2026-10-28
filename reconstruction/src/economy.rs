//! Regional economic simulation, facility construction, and resource production.
//!
//! Grounded in contracts FEAT-ECON-01 through FEAT-ECON-04 and FEAT-INFR-01 through FEAT-INFR-02.

use crate::core::{Result, SimulationError};
use crate::state::{HoldingState, ResourceStockpile};

/// Facility type for construction and infrastructure upgrades (FEAT-INFR-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FacilityType {
    Farm,
    Mine,
    LumberMill,
    Docks,
    Barracks,
}

impl FacilityType {
    pub fn name(&self) -> &'static str {
        match self {
            FacilityType::Farm => "Agricultural Farm",
            FacilityType::Mine => "Resource Mine",
            FacilityType::LumberMill => "Lumber Mill",
            FacilityType::Docks => "Trade Docks",
            FacilityType::Barracks => "Garrison Barracks",
        }
    }
}

/// Calculate gross daily resource units produced by a facility.
///
/// Recovered Formula:
/// Gross = 3 * ((Level * Change) + floor(0.3 * DocksChange * DocksLevel) - JobsOpen)
pub fn calculate_gross_production(
    level: u8,
    change: i32,
    docks_level: u8,
    docks_change: i32,
    jobs_open: i32,
) -> i32 {
    let base_production = level as i32 * change;
    let maritime_bonus = ((0.3 * docks_change as f64 * docks_level as f64).floor()) as i32;
    let net_capacity = base_production + maritime_bonus - jobs_open;
    let gross = 3 * net_capacity;
    if gross < 0 { 0 } else { gross }
}

/// Economy manager for managing national treasury and regional infrastructure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EconomyManager;

impl EconomyManager {
    /// Deposit gold into national treasury (FEAT-ECON-01, resolves DEF-TRANS-02).
    pub fn deposit_treasury(treasury: &mut i64, amount: i64) -> Result<()> {
        if amount < 0 {
            return Err(SimulationError::InvalidStateTransition(
                "Deposit amount cannot be negative",
            ));
        }
        *treasury = treasury.checked_add(amount).ok_or(
            SimulationError::InvalidStateTransition("Treasury overflow: deposit exceeds maximum capacity"),
        )?;
        Ok(())
    }

    /// Withdraw gold from national treasury, ensuring non-negative balance (FEAT-ECON-01).
    pub fn withdraw_treasury(treasury: &mut i64, amount: i64) -> Result<()> {
        if amount < 0 {
            return Err(SimulationError::InvalidStateTransition(
                "Withdrawal amount cannot be negative",
            ));
        }
        if *treasury < amount {
            return Err(SimulationError::InsufficientFunds {
                required: amount,
                available: *treasury,
            });
        }
        *treasury = treasury.checked_sub(amount).ok_or(
            SimulationError::InvalidStateTransition("Treasury underflow"),
        )?;
        Ok(())
    }

    /// Calculate facility upgrade costs (Gold and Wood).
    pub fn upgrade_cost(facility: FacilityType, current_level: u8) -> (i64, i32) {
        let next_level = current_level as i64 + 1;
        let gold_cost = match facility {
            FacilityType::Farm => 1500 * next_level,
            FacilityType::Mine => 2500 * next_level,
            FacilityType::LumberMill => 1500 * next_level,
            FacilityType::Docks => 3500 * next_level,
            FacilityType::Barracks => 2000 * next_level,
        };
        let wood_cost = (10 * next_level) as i32;
        (gold_cost, wood_cost)
    }

    /// Upgrade a facility within a holding (FEAT-INFR-01, FEAT-INFR-02).
    ///
    /// Upgrades the facility level (max 4), deducts Gold and Wood, and immediately
    /// expands MaxJobs by the holding's base rate (`AddJobs()`), raising employment capacity.
    pub fn upgrade_facility(
        holding: &mut HoldingState,
        treasury: &mut i64,
        stockpile: &mut ResourceStockpile,
        facility: FacilityType,
    ) -> Result<i32> {
        let (current_level, max_level) = match facility {
            FacilityType::Farm => (holding.facilities.farm, 4),
            FacilityType::Mine => (holding.facilities.mine, 4),
            FacilityType::LumberMill => (holding.facilities.lumber_mill, 4),
            FacilityType::Docks => {
                if holding.docks_change == 0 {
                    return Err(SimulationError::InvalidStateTransition(
                        "This holding has no maritime access for Trade Docks",
                    ));
                }
                (holding.facilities.docks, 4)
            }
            FacilityType::Barracks => (holding.facilities.barracks, 4),
        };

        if current_level >= max_level {
            return Err(SimulationError::InvalidStateTransition(
                "Facility is already at maximum upgrade tier (Level 4)",
            ));
        }

        let (cost_gold, cost_wood) = Self::upgrade_cost(facility, current_level);

        if *treasury < cost_gold {
            return Err(SimulationError::InsufficientFunds {
                required: cost_gold,
                available: *treasury,
            });
        }

        if stockpile.wood < cost_wood {
            return Err(SimulationError::InsufficientResources {
                resource: "Wood",
                required: cost_wood,
                available: stockpile.wood,
            });
        }

        // Apply transaction
        *treasury -= cost_gold;
        stockpile.wood -= cost_wood;

        let jobs_added = match facility {
            FacilityType::Farm => {
                holding.facilities.farm += 1;
                holding.food_change
            }
            FacilityType::Mine => {
                holding.facilities.mine += 1;
                holding.metal_change
            }
            FacilityType::LumberMill => {
                holding.facilities.lumber_mill += 1;
                holding.wood_change
            }
            FacilityType::Docks => {
                holding.facilities.docks += 1;
                holding.docks_change
            }
            FacilityType::Barracks => {
                holding.facilities.barracks += 1;
                5 // Barracks adds 5 guard billets
            }
        };

        // AddJobs() increases holding's employment ceiling
        holding.max_jobs += jobs_added;

        Ok(jobs_added)
    }
}
