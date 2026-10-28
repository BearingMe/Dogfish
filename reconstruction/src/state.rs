//! Global simulation state container and deterministic execution loop.

use crate::core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
use crate::faction::FactionProfile;
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

    /// Execute the complete 24-hour daily economic and demographic simulation cycle.
    pub fn tick_day(&mut self) {
        self.events.push(SimulationEvent::DayAdvanced { day: self.time.day });
        self.territory.advance_day();

        let mut daily_tax_revenue = 0i64;
        let mut total_food_consumed = 0i32;

        for holding in &mut self.holdings {
            if holding.owner == FactionId::Player {
                daily_tax_revenue += holding.calculate_tax();
                total_food_consumed += holding.food_required();
            }
        }

        self.treasury += daily_tax_revenue;
        self.stockpile.food -= total_food_consumed;

        if daily_tax_revenue > 0 {
            self.events.push(SimulationEvent::TaxesCollected {
                total_gold: daily_tax_revenue,
            });
        }

        self.events.push(SimulationEvent::FoodDistributed {
            total_consumed: total_food_consumed,
            net_surplus: self.stockpile.food,
        });
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
