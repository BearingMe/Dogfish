//! Demographic simulation: labor waterfall, conscription, and population growth.
//!
//! Grounded in contracts FEAT-POP-01 through FEAT-POP-03 and FEAT-STAB-01 through FEAT-STAB-02.

use crate::core::DeterministicRng;
use crate::state::HoldingState;

/// Result of executing the 4-stage labor allocation waterfall.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaborAllocationResult {
    pub food_labor: i32,
    pub docks_labor: i32,
    pub wood_labor: i32,
    pub metal_labor: i32,
    pub unemployed: i32,
    pub food_jobs_open: i32,
    pub docks_jobs_open: i32,
    pub wood_jobs_open: i32,
    pub metal_jobs_open: i32,
}

/// Execute the strict 4-stage demographic labor allocation waterfall (FEAT-POP-02).
///
/// Priority order:
/// 1. Food (Farms): `FarmLevel * FoodChange` jobs
/// 2. Docks (Maritime): `DocksLevel * DocksChange` jobs
/// 3. Wood (Lumber Mills): `LumberMillLevel * WoodChange` jobs
/// 4. Metal (Mines): `MineLevel * MetalChange` jobs
///
/// Any remaining citizens become Unemployed.
pub fn allocate_holding_labor(holding: &HoldingState) -> LaborAllocationResult {
    let mut available_labor = holding.population;

    let food_capacity = holding.facilities.farm as i32 * holding.food_change;
    let docks_capacity = holding.facilities.docks as i32 * holding.docks_change;
    let wood_capacity = holding.facilities.lumber_mill as i32 * holding.wood_change;
    let metal_capacity = holding.facilities.mine as i32 * holding.metal_change;

    // 1. Food allocation
    let food_labor = available_labor.min(food_capacity);
    available_labor -= food_labor;
    let food_jobs_open = food_capacity - food_labor;

    // 2. Docks allocation
    let docks_labor = available_labor.min(docks_capacity);
    available_labor -= docks_labor;
    let docks_jobs_open = docks_capacity - docks_labor;

    // 3. Wood allocation
    let wood_labor = available_labor.min(wood_capacity);
    available_labor -= wood_labor;
    let wood_jobs_open = wood_capacity - wood_labor;

    // 4. Metal allocation
    let metal_labor = available_labor.min(metal_capacity);
    available_labor -= metal_labor;
    let metal_jobs_open = metal_capacity - metal_labor;

    LaborAllocationResult {
        food_labor,
        docks_labor,
        wood_labor,
        metal_labor,
        unemployed: available_labor,
        food_jobs_open,
        docks_jobs_open,
        wood_jobs_open,
        metal_jobs_open,
    }
}

/// Calculate dynamic civilian population growth for a holding based on employment ceiling.
///
/// Recovered Formula:
/// Deficit = MaxJobs - floor(0.25 * Population)
/// If Deficit > 0: Increase = random(floor(0.3 * Deficit), floor(0.5 * Deficit))
pub fn calculate_population_growth(
    population: i32,
    max_jobs: i32,
    rng: &mut DeterministicRng,
) -> i32 {
    let baseline = population / 4; // floor(0.25 * Pop)
    let deficit = max_jobs - baseline;

    if deficit <= 0 {
        return 0;
    }

    let min_increase = ((0.3 * deficit as f64).floor()) as i32;
    let max_increase = ((0.5 * deficit as f64).floor()) as i32;

    if max_increase <= 0 {
        return 0;
    }

    rng.gen_range_i32(min_increase, max_increase)
}

/// Execute conscription draft from unemployed labor reserve (FEAT-POP-03).
///
/// Recovered Formula:
/// RecruitableTroops = ceil(RecruitablePercentage * Unemployed)
/// Deducted from both Population and Unemployed.
pub fn conscript_troops(
    holding: &mut HoldingState,
    recruitable_percentage: f64,
) -> i32 {
    if holding.unemployed <= 0 || recruitable_percentage <= 0.0 {
        return 0;
    }

    let recruits = (recruitable_percentage * holding.unemployed as f64).ceil() as i32;
    let actual_drafted = recruits.min(holding.unemployed);

    holding.unemployed -= actual_drafted;
    holding.population -= actual_drafted;

    actual_drafted
}
