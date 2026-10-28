//! Global simulation state container and deterministic execution loop.

use crate::core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
use crate::economy::{calculate_gross_production, EconomyManager, FacilityType};
use crate::faction::FactionProfile;
use crate::population::{allocate_holding_labor, calculate_population_growth, conscript_troops};
use crate::territory::TerritoryManager;
use crate::types::{FactionId, HoldingId, HoldingTier};

/// National resource stockpiles pooled across holdings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceStockpile {
    pub food: i32,
    pub metal: i32,
    pub wood: i32,
    pub trade_docks: i32,
}

impl ResourceStockpile {
    pub fn new() -> Self {
        Self {
            food: 100,
            metal: 50,
            wood: 50,
            trade_docks: 0,
        }
    }
}

impl Default for ResourceStockpile {
    fn default() -> Self {
        Self::new()
    }
}

/// Facility upgrade levels within a holding (Levels 1..4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacilityLevels {
    pub farm: u8,
    pub mine: u8,
    pub lumber_mill: u8,
    pub docks: u8,
    pub barracks: u8,
}

impl FacilityLevels {
    pub fn new() -> Self {
        Self {
            farm: 1,
            mine: 1,
            lumber_mill: 1,
            docks: 0,
            barracks: 1,
        }
    }
}

impl Default for FacilityLevels {
    fn default() -> Self {
        Self::new()
    }
}

/// State of an individual territorial holding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoldingState {
    pub id: HoldingId,
    pub tier: HoldingTier,
    pub owner: FactionId,
    pub population: i32,
    pub unemployed: i32,
    pub max_jobs: i32,
    pub tax_rate: i32,
    pub loyalty: i32,
    pub facilities: FacilityLevels,
    pub guards: i32,
    pub food_change: i32,
    pub metal_change: i32,
    pub wood_change: i32,
    pub docks_change: i32,
}

impl HoldingState {
    pub fn new(id: HoldingId, owner: FactionId) -> Self {
        let tier = HoldingTier::for_holding(id);
        let (pop, food_chg, metal_chg, wood_chg, docks_chg) = match id.0 {
            0 => (50, 10, 5, 5, 0),  // Whiterun: Agricultural breadbasket
            1 => (30, 6, 4, 8, 0),   // Falkreath: Forestry center
            2 => (50, 5, 4, 5, 10),  // Solitude: Maritime trade capital
            3 => (50, 6, 6, 6, 8),   // Windhelm: Port capital
            4 => (30, 3, 8, 4, 6),   // Dawnstar: Mining & port
            5 => (50, 8, 4, 8, 4),   // Riften: Fishery & forestry
            6 => (30, 4, 5, 7, 0),   // Morthal: Marsh timber
            7 => (50, 4, 10, 4, 0),  // Markarth: Deep mining capital
            8 => (20, 2, 4, 3, 0),   // Winterhold: Sparse northern outpost
            _ => (15, 4, 3, 4, 0),   // Villages
        };

        let facilities = FacilityLevels {
            farm: if tier == HoldingTier::CapitalMajor { 4 } else { 2 },
            mine: if id == HoldingId::MARKARTH { 4 } else { 2 },
            lumber_mill: if id == HoldingId::FALKREATH { 4 } else { 2 },
            docks: if id == HoldingId::SOLITUDE || id == HoldingId::WINDHELM { 4 } else { 0 },
            barracks: 2,
        };

        let max_jobs = (facilities.farm as i32 * food_chg)
            + (facilities.mine as i32 * metal_chg)
            + (facilities.lumber_mill as i32 * wood_chg);

        Self {
            id,
            tier,
            owner,
            population: pop,
            unemployed: 0,
            max_jobs,
            tax_rate: 1,
            loyalty: 100,
            facilities,
            guards: 10,
            food_change: food_chg,
            metal_change: metal_chg,
            wood_change: wood_chg,
            docks_change: docks_chg,
        }
    }

    /// Calculate daily taxation revenue from this holding based on recovered formulas:
    /// Major: `500 * Pop + 250 * Pop * TaxRate`
    /// Minor: `300 * Pop + 150 * Pop * TaxRate`
    /// Village: `200 * Pop + 100 * Pop * TaxRate`
    pub fn calculate_tax(&self) -> i64 {
        let base = self.tier.base_tax();
        let rate_mult = self.tier.rate_multiplier();
        (base * self.population as i64) + (rate_mult * self.population as i64 * self.tax_rate as i64)
    }

    /// Compute food requirements: Population + active guards.
    pub fn food_required(&self) -> i32 {
        self.population + self.guards
    }
}

/// The root deterministic game simulation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub time: SimulationTime,
    pub rng: DeterministicRng,
    pub faction: FactionProfile,
    pub territory: TerritoryManager,
    pub treasury: i64,
    pub stockpile: ResourceStockpile,
    pub holdings: Vec<HoldingState>,
    pub events: Vec<SimulationEvent>,
}

impl GameState {
    /// Initialize a fresh simulation world with a deterministic seed.
    pub fn new(seed: u64) -> Self {
        let mut holdings = Vec::with_capacity(HoldingId::COUNT);
        for id in 0..(HoldingId::COUNT as u8) {
            let hid = HoldingId(id);
            let owner = match id {
                0 | 1 | 2 | 6 => FactionId::Imperial,
                3 | 4 | 5 | 8 => FactionId::Stormcloak,
                7 => FactionId::Imperial,
                _ => FactionId::Neutral,
            };
            holdings.push(HoldingState::new(hid, owner));
        }

        let mut events = Vec::new();
        events.push(SimulationEvent::SimulationStarted { day: 1 });

        Self {
            time: SimulationTime::new(),
            rng: DeterministicRng::new(seed),
            faction: FactionProfile::new_player(),
            territory: TerritoryManager::new(),
            treasury: 10000,
            stockpile: ResourceStockpile::new(),
            holdings,
            events,
        }
    }

    /// Advance simulation by one hour. If day boundary crossed, executes daily tick.
    pub fn tick_hour(&mut self) -> bool {
        let new_day = self.time.advance_hour();
        if new_day {
            self.tick_day();
        }
        new_day
    }

    /// Execute the complete 24-hour daily economic, demographic, and upkeep simulation cycle.
    pub fn tick_day(&mut self) {
        self.events.push(SimulationEvent::DayAdvanced { day: self.time.day });
        self.territory.advance_day();

        let mut daily_tax_revenue = 0i64;
        let mut total_food_consumed = 0i32;
        let mut total_food_produced = 0i32;
        let mut total_metal_produced = 0i32;
        let mut total_wood_produced = 0i32;
        let mut total_docks_produced = 0i32;

        for holding in &mut self.holdings {
            if holding.owner == FactionId::Player {
                // 1. Taxation revenue
                daily_tax_revenue += holding.calculate_tax();

                // 2. Demographic labor allocation waterfall (Food -> Docks -> Wood -> Metal)
                let labor = allocate_holding_labor(holding);
                holding.unemployed = labor.unemployed;

                // 3. Resource production calculation
                let food_yield = calculate_gross_production(
                    holding.facilities.farm,
                    holding.food_change,
                    holding.facilities.docks,
                    holding.docks_change,
                    labor.food_jobs_open,
                );
                let metal_yield = calculate_gross_production(
                    holding.facilities.mine,
                    holding.metal_change,
                    0,
                    0,
                    labor.metal_jobs_open,
                );
                let wood_yield = calculate_gross_production(
                    holding.facilities.lumber_mill,
                    holding.wood_change,
                    0,
                    0,
                    labor.wood_jobs_open,
                );
                let docks_yield = calculate_gross_production(
                    holding.facilities.docks,
                    holding.docks_change,
                    0,
                    0,
                    labor.docks_jobs_open,
                );

                total_food_produced += food_yield;
                total_metal_produced += metal_yield;
                total_wood_produced += wood_yield;
                total_docks_produced += docks_yield;

                // 4. Food consumption
                total_food_consumed += holding.food_required();

                // 5. Population growth when Deficit > 0
                let pop_increase = calculate_population_growth(
                    holding.population,
                    holding.max_jobs,
                    &mut self.rng,
                );
                if pop_increase > 0 {
                    holding.population += pop_increase;
                    self.events.push(SimulationEvent::PopulationGrown {
                        holding_id: holding.id.0,
                        increase: pop_increase,
                    });
                }
            }
        }

        // Apply doctrine tax modifier
        let tax_mod_pct = self.faction.doctrine.tax_modifier_pct();
        let adjusted_tax = daily_tax_revenue + (daily_tax_revenue * tax_mod_pct as i64 / 100);
        self.treasury += adjusted_tax;

        // Resource accumulation
        self.stockpile.food += total_food_produced - total_food_consumed;
        self.stockpile.metal += total_metal_produced;
        self.stockpile.wood += total_wood_produced;
        self.stockpile.trade_docks += total_docks_produced;

        // Famine cascade: if food drops negative, citizens starve and loyalty falls
        if self.stockpile.food < 0 {
            for holding in &mut self.holdings {
                if holding.owner == FactionId::Player {
                    holding.loyalty = (holding.loyalty - 10).max(0);
                    if holding.loyalty < 40 {
                        self.events.push(SimulationEvent::UnrestTriggered {
                            holding_id: holding.id.0,
                            loyalty: holding.loyalty,
                        });
                    }
                }
            }
        }

        if adjusted_tax > 0 {
            self.events.push(SimulationEvent::TaxesCollected {
                total_gold: adjusted_tax,
            });
        }

        self.events.push(SimulationEvent::FoodDistributed {
            total_consumed: total_food_consumed,
            net_surplus: self.stockpile.food,
        });
    }

    /// Deposit gold into national treasury.
    pub fn deposit_treasury(&mut self, amount: i64) -> Result<()> {
        EconomyManager::deposit_treasury(&mut self.treasury, amount)
    }

    /// Withdraw gold from national treasury.
    pub fn withdraw_treasury(&mut self, amount: i64) -> Result<()> {
        EconomyManager::withdraw_treasury(&mut self.treasury, amount)
    }

    /// Upgrade a facility within an owned holding.
    pub fn upgrade_facility(
        &mut self,
        holding_id: HoldingId,
        facility: FacilityType,
    ) -> Result<i32> {
        let idx = holding_id.0 as usize;
        if idx >= self.holdings.len() {
            return Err(SimulationError::InvalidHoldingId(holding_id.0));
        }

        EconomyManager::upgrade_facility(
            &mut self.holdings[idx],
            &mut self.treasury,
            &mut self.stockpile,
            facility,
        )
    }

    /// Conscript troops from holding's unemployed labor reserve.
    pub fn conscript_troops(&mut self, holding_id: HoldingId, percentage: f64) -> i32 {
        let idx = holding_id.0 as usize;
        if idx >= self.holdings.len() {
            return 0;
        }

        // Apply doctrine conscription modifier
        let doc_mod = self.faction.doctrine.conscription_modifier_pct() as f64 / 100.0;
        let effective_pct = (percentage + doc_mod).max(0.0).min(1.0);

        let recruits = conscript_troops(&mut self.holdings[idx], effective_pct);
        if recruits > 0 {
            self.events.push(SimulationEvent::TroopsRecruited {
                holding_id: holding_id.0,
                count: recruits,
            });
        }
        recruits
    }

    /// Transfer ownership of a holding atomically.
    pub fn transfer_holding(
        &mut self,
        target_id: HoldingId,
        new_owner: FactionId,
        has_staging_camp: bool,
    ) -> Result<()> {
        self.territory.transfer_ownership(
            &mut self.holdings,
            target_id,
            new_owner,
            has_staging_camp,
        )?;

        self.events.push(SimulationEvent::HoldingCaptured {
            holding_id: target_id.0,
            new_owner: new_owner.name(),
        });

        Ok(())
    }

    /// Appoint a governor / Jarl to a held capital.
    pub fn appoint_governor(
        &mut self,
        holding_id: HoldingId,
        governor_name: impl Into<String>,
    ) -> Result<()> {
        let day = self.time.day;
        self.territory.appoint_governor(holding_id, governor_name, day)
    }

    /// Assert core domain invariants. Returns Err if invariants are violated.
    pub fn check_invariants(&self) -> Result<()> {
        if self.holdings.len() != HoldingId::COUNT {
            return Err(SimulationError::InvalidStateTransition(
                "Holding count must equal exactly 15",
            ));
        }
        if self.treasury < 0 {
            return Err(SimulationError::InsufficientFunds {
                required: 0,
                available: self.treasury,
            });
        }
        Ok(())
    }
}
