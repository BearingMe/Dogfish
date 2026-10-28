//! Royal cabinet, judicial administration, and The Moot coronation loop.
//!
//! Grounded in contracts FEAT-GOV-01 through FEAT-GOV-03, FEAT-LAW-01 through FEAT-LAW-03,
//! and FEAT-VICT-01 through FEAT-VICT-03.
//! Addresses the Moot deadlock vulnerability identified in M31 and M38.

use crate::core::{DeterministicRng, Result, SimulationError};
use crate::territory::AppointedGovernor;
use crate::types::{FactionId, HoldingId};

/// The 7 official ministerial roles in the Royal Cabinet (FEAT-GOV-01..03).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CabinetRole {
    Steward,
    Commander,
    Blacksmith,
    Spymaster,
    CourtMage,
    CourtBard,
    Executioner,
}

impl CabinetRole {
    pub fn title(&self) -> &'static str {
        match self {
            CabinetRole::Steward => "Royal Steward",
            CabinetRole::Commander => "Grand Commander",
            CabinetRole::Blacksmith => "Master Blacksmith",
            CabinetRole::Spymaster => "Chief Spymaster",
            CabinetRole::CourtMage => "High Court Mage",
            CabinetRole::CourtBard => "Royal Court Bard",
            CabinetRole::Executioner => "High Executioner",
        }
    }
}

/// Appointed royal minister.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabinetMinister {
    pub name: String,
    pub role: CabinetRole,
    pub appointed_day: u32,
}

/// Judicial offense categories heard in royal court sessions (FEAT-LAW-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegalOffense {
    Theft,
    Assault,
    Murder,
    SpeakingOut,
    IllegalWorship,
    Treason,
    Contraband,
}

/// Judicial sentences handed down from the throne.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JudicialSentence {
    Pardon,
    Fine(i64),
    DungeonImprisonment { days: u32 },
    Execution,
}

/// Outcome of The Moot grand convocation (FEAT-VICT-02).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MootOutcome {
    pub is_elected: bool,
    pub votes_in_favor: u8,
    pub votes_against: u8,
    pub total_jarls_voting: u8,
}

/// Government manager administering the cabinet, judiciary, and high kingship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernmentManager {
    pub cabinet: Vec<CabinetMinister>,
    pub prisoners_in_dungeon: u32,
    pub is_high_king: bool,
    pub coronation_day: Option<u32>,
}

impl GovernmentManager {
    pub fn new() -> Self {
        Self {
            cabinet: Vec::new(),
            prisoners_in_dungeon: 0,
            is_high_king: false,
            coronation_day: None,
        }
    }

    /// Appoint a minister to a cabinet position (FEAT-GOV-01).
    pub fn appoint_minister(
        &mut self,
        role: CabinetRole,
        name: impl Into<String>,
        day: u32,
    ) {
        self.cabinet.retain(|m| m.role != role);
        self.cabinet.push(CabinetMinister {
            name: name.into(),
            role,
            appointed_day: day,
        });
    }

    /// Check if a specific cabinet role is currently filled.
    pub fn has_minister(&self, role: CabinetRole) -> bool {
        self.cabinet.iter().any(|m| m.role == role)
    }

    /// Execute a Spymaster covert coup against an unowned capital (CS_IntelReportScript).
    ///
    /// Requires Chief Spymaster appointed and 10,000 gold bribe cost.
    /// If roll succeeds, the local garrison surrenders without siege bloodshed.
    pub fn attempt_covert_coup(
        &self,
        target: HoldingId,
        treasury: &mut i64,
        rng: &mut DeterministicRng,
    ) -> Result<bool> {
        if !self.has_minister(CabinetRole::Spymaster) {
            return Err(SimulationError::InvalidStateTransition(
                "Covert coups require an appointed Chief Spymaster",
            ));
        }

        if !target.is_capital() {
            return Err(SimulationError::InvalidStateTransition(
                "Covert coups can only be launched against Hold Capitals",
            ));
        }

        let bribe_cost = 10000i64;
        if *treasury < bribe_cost {
            return Err(SimulationError::InsufficientFunds {
                required: bribe_cost,
                available: *treasury,
            });
        }

        *treasury -= bribe_cost;

        // 40% base success chance for a covert coup
        let roll = rng.gen_percent();
        let success = roll < 40;

        Ok(success)
    }

    /// Judge a legal offense in royal court session (FEAT-LAW-02).
    pub fn judge_case(
        &mut self,
        offense: LegalOffense,
        sentence: JudicialSentence,
        treasury: &mut i64,
    ) -> Result<i64> {
        let mut gold_collected = 0i64;

        match sentence {
            JudicialSentence::Pardon => {}
            JudicialSentence::Fine(amt) => {
                gold_collected = amt;
                *treasury += amt;
            }
            JudicialSentence::DungeonImprisonment { .. } => {
                self.prisoners_in_dungeon += 1;
            }
            JudicialSentence::Execution => {
                if !self.has_minister(CabinetRole::Executioner) && offense != LegalOffense::Treason {
                    return Err(SimulationError::InvalidStateTransition(
                        "Public executions require an appointed High Executioner",
                    ));
                }
            }
        }

        Ok(gold_collected)
    }

    /// Convene The Moot to elect the High King of Skyrim (FEAT-VICT-02).
    ///
    /// Recovered Invariant & Deadlock Prevention:
    /// In the original Papyrus code (CS_MeetingScript.psc), GetJarls() loops through 0..8
    /// and expects non-null actor references in GovFaction. If replacement Jarls were not
    /// recruited for all 8 provincial holds, the script deadlocks indefinitely.
    ///
    /// This reconstruction explicitly validates that all 8 non-capital provincial holds
    /// have an appointed Jarl before voting can commence.
    pub fn convene_the_moot(
        &mut self,
        holdings: &[crate::state::HoldingState],
        governors: &[Option<AppointedGovernor>],
        player_capital: HoldingId,
        day: u32,
    ) -> Result<MootOutcome> {
        // 1. Verify player owns all 9 Hold Capitals (0..8)
        for i in 0..9u8 {
            if holdings[i as usize].owner != FactionId::Player {
                return Err(SimulationError::InvalidStateTransition(
                    "The Moot cannot be convened until all 9 Hold Capitals are conquered",
                ));
            }
        }

        // 2. Validate deadlock prevention: all 8 non-capital hold capitals must have an appointed Jarl
        let mut voting_jarls = 0u8;
        let mut votes_in_favor = 0u8;
        let mut votes_against = 0u8;

        for i in 0..9u8 {
            let hid = HoldingId(i);
            if hid == player_capital {
                continue; // The player's seat does not vote for itself
            }

            match &governors[i as usize] {
                Some(gov) => {
                    voting_jarls += 1;
                    if gov.loyalty >= 50 {
                        votes_in_favor += 1;
                    } else {
                        votes_against += 1;
                    }
                }
                None => {
                    return Err(SimulationError::InvalidStateTransition(
                        "Moot Deadlock Prevention: An appointed provincial Jarl is required for every conquered hold",
                    ));
                }
            }
        }

        // 3. Majority rule: At least 5 of the 8 Jarls must vote in favor
        let is_elected = votes_in_favor >= 5;

        if is_elected {
            self.is_high_king = true;
            self.coronation_day = Some(day);
        }

        Ok(MootOutcome {
            is_elected,
            votes_in_favor,
            votes_against,
            total_jarls_voting: voting_jarls,
        })
    }
}

impl Default for GovernmentManager {
    fn default() -> Self {
        Self::new()
    }
}
