//! Domain types and identifiers for the Conquest of Skyrim behavioral simulation.
//!
//! Grounded in the behavioral contracts established in `docs/behavioral-contracts.md`.

use std::fmt;

/// Unique identifier for provincial holdings (0..14).
///
/// 0..8 represent the 9 Hold Capitals:
/// - 0: Whiterun
/// - 1: Falkreath
/// - 2: Solitude
/// - 3: Windhelm
/// - 4: Dawnstar
/// - 5: Riften
/// - 6: Morthal
/// - 7: Markarth
/// - 8: Winterhold
///
/// 9..14 represent the 6 major villages:
/// - 9: Riverwood
/// - 10: Rorikstead
/// - 11: Dragon Bridge
/// - 12: Ivarstead
/// - 13: Shor's Stone
/// - 14: Karthwasten
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HoldingId(pub u8);

impl HoldingId {
    pub const WHITERUN: HoldingId = HoldingId(0);
    pub const FALKREATH: HoldingId = HoldingId(1);
    pub const SOLITUDE: HoldingId = HoldingId(2);
    pub const WINDHELM: HoldingId = HoldingId(3);
    pub const DAWNSTAR: HoldingId = HoldingId(4);
    pub const RIFTEN: HoldingId = HoldingId(5);
    pub const MORTHAL: HoldingId = HoldingId(6);
    pub const MARKARTH: HoldingId = HoldingId(7);
    pub const WINTERHOLD: HoldingId = HoldingId(8);

    pub const RIVERWOOD: HoldingId = HoldingId(9);
    pub const RORIKSTEAD: HoldingId = HoldingId(10);
    pub const DRAGON_BRIDGE: HoldingId = HoldingId(11);
    pub const IVARSTEAD: HoldingId = HoldingId(12);
    pub const SHORS_STONE: HoldingId = HoldingId(13);
    pub const KARTHWASTEN: HoldingId = HoldingId(14);

    /// Total number of valid modeled holdings in Conquest of Skyrim.
    pub const COUNT: usize = 15;

    pub fn is_capital(&self) -> bool {
        self.0 <= 8
    }

    pub fn is_village(&self) -> bool {
        self.0 >= 9 && self.0 <= 14
    }

    pub fn is_valid(&self) -> bool {
        (self.0 as usize) < Self::COUNT
    }

    pub fn name(&self) -> &'static str {
        match self.0 {
            0 => "Whiterun",
            1 => "Falkreath",
            2 => "Solitude",
            3 => "Windhelm",
            4 => "Dawnstar",
            5 => "Riften",
            6 => "Morthal",
            7 => "Markarth",
            8 => "Winterhold",
            9 => "Riverwood",
            10 => "Rorikstead",
            11 => "Dragon Bridge",
            12 => "Ivarstead",
            13 => "Shor's Stone",
            14 => "Karthwasten",
            _ => "Unknown Holding",
        }
    }
}

impl fmt::Display for HoldingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name(), self.0)
    }
}

/// Structural tier of a territorial holding.
///
/// Determines taxation rates and base economic yields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HoldingTier {
    /// Walled provincial capitals (Solitude, Windhelm, Whiterun, Riften, Markarth).
    CapitalMajor,
    /// Open provincial capitals (Falkreath, Dawnstar, Morthal, Winterhold).
    CapitalMinor,
    /// Dependent rural settlements (Riverwood, Rorikstead, etc.).
    Village,
    /// Military fortifications (e.g. Fort Greymoor).
    Fort,
    /// Wilderness staging camps.
    Camp,
}

impl HoldingTier {
    pub fn for_holding(id: HoldingId) -> Self {
        match id.0 {
            0 | 2 | 3 | 5 | 7 => HoldingTier::CapitalMajor,
            1 | 4 | 6 | 8 => HoldingTier::CapitalMinor,
            9..=14 => HoldingTier::Village,
            _ => HoldingTier::Camp,
        }
    }

    /// Base taxation constant per citizen.
    pub fn base_tax(&self) -> i64 {
        match self {
            HoldingTier::CapitalMajor => 500,
            HoldingTier::CapitalMinor => 300,
            HoldingTier::Village => 200,
            HoldingTier::Fort | HoldingTier::Camp => 0,
        }
    }

    /// Tax rate multiplier constant per citizen per tax level unit.
    pub fn rate_multiplier(&self) -> i64 {
        match self {
            HoldingTier::CapitalMajor => 250,
            HoldingTier::CapitalMinor => 150,
            HoldingTier::Village => 100,
            HoldingTier::Fort | HoldingTier::Camp => 0,
        }
    }
}

/// Faction identity within the strategic simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FactionId {
    Player,
    Imperial,
    Stormcloak,
    Neutral,
}

impl FactionId {
    pub fn name(&self) -> &'static str {
        match self {
            FactionId::Player => "Player Faction",
            FactionId::Imperial => "Imperial Legion",
            FactionId::Stormcloak => "Stormcloak Rebellion",
            FactionId::Neutral => "Neutral Local Authorities",
        }
    }
}

/// Economic resource types tracked in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Gold,
    Food,
    Metal,
    Wood,
    Docks,
}

/// 10 Field Divisions (0..9) supported by Conquest of Skyrim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DivisionId(pub u8);

impl DivisionId {
    pub const MAX_DIVISIONS: usize = 10;

    pub fn is_valid(&self) -> bool {
        (self.0 as usize) < Self::MAX_DIVISIONS
    }
}

impl fmt::Display for DivisionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Division-{}", self.0)
    }
}

/// Diplomatic treaty stance between two factions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiplomaticStance {
    Neutral,
    War,
    Truce { days_remaining: u32 },
    Defeated,
}
