//! Global simulation state container and deterministic execution loop.

use crate::core::{DeterministicRng, Result, SimulationError, SimulationEvent, SimulationTime};
use crate::diplomacy::DiplomacyManager;
use crate::economy::{calculate_gross_production, EconomyManager, FacilityType};
use crate::faction::FactionProfile;
use crate::government::{CabinetRole, GovernmentManager, JudicialSentence, LegalOffense, MootOutcome};
use crate::military::{ArmyOrder, MilitaryManager, SiegeOperation, SiegePhase};
use crate::population::{allocate_holding_labor, calculate_population_growth, conscript_troops};
use crate::territory::TerritoryManager;
use crate::types::{DivisionId, FactionId, HoldingId, HoldingTier};

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
    pub military: MilitaryManager,
    pub government: GovernmentManager,
    pub diplomacy: DiplomacyManager,
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
            military: MilitaryManager::new(),
            government: GovernmentManager::new(),
            diplomacy: DiplomacyManager::new(),
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

    /// Execute the complete 24-hour daily economic, demographic, and diplomatic simulation cycle.
    pub fn tick_day(&mut self) {
        self.events.push(SimulationEvent::DayAdvanced { day: self.time.day });
        self.territory.advance_day();
        self.diplomacy.advance_day();

        let mut daily_tax_revenue = 0i64;
        let mut total_food_consumed = 0i32;
        let mut total_food_produced = 0i32;
        let mut total_metal_produced = 0i32;
        let mut total_wood_produced = 0i32;
        let mut total_docks_produced = 0i32;

        total_food_consumed += self.military.total_soldiers();

        for holding in &mut self.holdings {
            if holding.owner == FactionId::Player {
                daily_tax_revenue += holding.calculate_tax();

                let labor = allocate_holding_labor(holding);
                holding.unemployed = labor.unemployed;

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

                total_food_consumed += holding.food_required();

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

        let tax_mod_pct = self.faction.doctrine.tax_modifier_pct();
        let adjusted_tax = daily_tax_revenue + (daily_tax_revenue * tax_mod_pct as i64 / 100);
        self.treasury += adjusted_tax;

        self.stockpile.food += total_food_produced - total_food_consumed;
        self.stockpile.metal += total_metal_produced;
        self.stockpile.wood += total_wood_produced;
        self.stockpile.trade_docks += total_docks_produced;

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

    /// Assign order to division.
    pub fn assign_army_order(&mut self, div_id: DivisionId, order: ArmyOrder) -> Result<()> {
        self.military.assign_order(div_id, order)
    }

    /// Upgrade division power level.
    pub fn upgrade_division_power(&mut self, div_id: DivisionId) -> Result<()> {
        if !div_id.is_valid() {
            return Err(SimulationError::InvalidDivisionId(div_id.0));
        }
        self.military.divisions[div_id.0 as usize].upgrade_power(
            &mut self.treasury,
            &mut self.stockpile,
        )
    }

    /// Assign recruited soldiers to division.
    pub fn reinforce_division(&mut self, div_id: DivisionId, count: i32) -> Result<()> {
        self.military.reinforce_division(div_id, count)
    }

    /// Launch a siege operation against a target holding.
    pub fn start_siege(
        &mut self,
        target_id: HoldingId,
        pincer: bool,
        artillery: bool,
    ) -> Result<()> {
        if !target_id.is_capital() {
            return Err(SimulationError::InvalidStateTransition(
                "Siege operations can only target Hold Capitals",
            ));
        }
        if self.holdings[target_id.0 as usize].owner == FactionId::Player {
            return Err(SimulationError::InvalidStateTransition(
                "Cannot siege an already owned holding",
            ));
        }

        self.military.active_siege = Some(SiegeOperation::new(target_id, pincer, artillery));
        Ok(())
    }

    /// Advance active siege to next phase.
    pub fn advance_siege(&mut self) -> Result<SiegePhase> {
        let siege = self.military.active_siege.as_mut().ok_or(
            SimulationError::InvalidStateTransition("No active siege operation underway"),
        )?;

        let next_phase = siege.advance_phase()?;
        if next_phase == SiegePhase::Capitulated {
            let target = siege.target_holding;
            self.transfer_holding(target, FactionId::Player, true)?;
        }

        Ok(next_phase)
    }

    /// Appoint a minister to a cabinet position (FEAT-GOV-01).
    pub fn appoint_cabinet_minister(&mut self, role: CabinetRole, name: impl Into<String>) {
        let day = self.time.day;
        self.government.appoint_minister(role, name, day);
    }

    /// Attempt a covert coup via Chief Spymaster.
    pub fn attempt_covert_coup(&mut self, target: HoldingId) -> Result<bool> {
        let success = self.government.attempt_covert_coup(
            target,
            &mut self.treasury,
            &mut self.rng,
        )?;
        if success {
            self.transfer_holding(target, FactionId::Player, true)?;
        }
        Ok(success)
    }

    /// Judge a court case in royal throne session (FEAT-LAW-02).
    pub fn judge_court_case(
        &mut self,
        offense: LegalOffense,
        sentence: JudicialSentence,
    ) -> Result<i64> {
        self.government.judge_case(offense, sentence, &mut self.treasury)
    }

    /// Negotiate a bilateral non-aggression truce (FEAT-DIPL-02).
    pub fn negotiate_truce(
        &mut self,
        target_faction: FactionId,
        compatible_doctrine: bool,
        truce_days: u32,
    ) -> Result<bool> {
        let player_troops = self.military.total_soldiers();
        let success = self.diplomacy.negotiate_truce(
            target_faction,
            compatible_doctrine,
            player_troops,
            truce_days,
            &mut self.rng,
        )?;
        if success {
            self.events.push(SimulationEvent::TreatySigned {
                other_faction: target_faction.name(),
                duration_days: truce_days,
            });
        }
        Ok(success)
    }

    /// Ransom prisoners of war for gold (FEAT-DIPL-03).
    pub fn ransom_pows(&mut self, target_faction: FactionId, gold_per_pow: i64) -> Result<u32> {
        self.diplomacy.ransom_pows(target_faction, &mut self.treasury, gold_per_pow)
    }

    /// Convene The Moot to elect the High King (FEAT-VICT-02).
    pub fn convene_the_moot(&mut self, player_capital: HoldingId) -> Result<MootOutcome> {
        let day = self.time.day;
        self.government.convene_the_moot(
            &self.holdings,
            &self.territory.governors,
            player_capital,
            day,
        )
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
