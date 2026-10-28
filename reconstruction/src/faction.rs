//! Faction identity, sovereign customization, and governance doctrine.
//!
//! Grounded in contracts FEAT-FACT-01 through FEAT-FACT-04.

use crate::types::FactionId;

/// Sovereign heraldic color and banner theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BannerColor {
    Crimson,
    RoyalBlue,
    EmeraldGreen,
    ImperialGold,
    ObsidianBlack,
    NordicWhite,
}

impl BannerColor {
    pub fn name(&self) -> &'static str {
        match self {
            BannerColor::Crimson => "Crimson Red",
            BannerColor::RoyalBlue => "Royal Blue",
            BannerColor::EmeraldGreen => "Emerald Green",
            BannerColor::ImperialGold => "Imperial Gold",
            BannerColor::ObsidianBlack => "Obsidian Black",
            BannerColor::NordicWhite => "Nordic White",
        }
    }
}

/// Political doctrine chosen upon sovereign declaration.
///
/// Modifies taxation yield, conscription rates, and stability thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GovernmentDoctrine {
    /// Autocracy: Higher conscription, lower citizen loyalty base.
    Autocracy,
    /// Feudalism: Standard balanced economy and Jarl loyalty bonus.
    Feudalism,
    /// Republic: Higher taxation tolerance, slower military draft.
    Republic,
    /// Meritocracy: Bonus officer combat power, higher wage costs.
    Meritocracy,
}

impl GovernmentDoctrine {
    /// Canonical source-verified tax modifier (0% distortion).
    ///
    /// NOTE (DEF-FIDEL-01): In Conquest of Skyrim v0.3.9, doctrine yield modifiers were
    /// unreleased stubs in cs_factiongovernmentscript.psc. Canonical simulation runs
    /// strictly on verified tiered taxation without artificial distortion.
    pub fn tax_modifier_pct(&self) -> i32 {
        0
    }

    /// Canonical source-verified conscription modifier (0% distortion).
    pub fn conscription_modifier_pct(&self) -> i32 {
        0
    }

    /// Experimental non-canonical tax multiplier modifier (isolated for research testing).
    pub fn experimental_tax_modifier_pct(&self) -> i32 {
        match self {
            GovernmentDoctrine::Autocracy => 15,
            GovernmentDoctrine::Feudalism => 0,
            GovernmentDoctrine::Republic => 20,
            GovernmentDoctrine::Meritocracy => -5,
        }
    }

    /// Experimental non-canonical conscription modifier (isolated for research testing).
    pub fn experimental_conscription_modifier_pct(&self) -> i32 {
        match self {
            GovernmentDoctrine::Autocracy => 25,
            GovernmentDoctrine::Feudalism => 0,
            GovernmentDoctrine::Republic => -15,
            GovernmentDoctrine::Meritocracy => 10,
        }
    }
}

/// Sovereign faction identity and organizational profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactionProfile {
    pub id: FactionId,
    pub name: String,
    pub army_name: String,
    pub banner_color: BannerColor,
    pub doctrine: GovernmentDoctrine,
    pub is_declared: bool,
    pub declaration_day: u32,
}

impl FactionProfile {
    /// Create default player faction profile before formal declaration.
    pub fn new_player() -> Self {
        Self {
            id: FactionId::Player,
            name: "The New Kingdom".to_string(),
            army_name: "Royal Legion".to_string(),
            banner_color: BannerColor::RoyalBlue,
            doctrine: GovernmentDoctrine::Feudalism,
            is_declared: false,
            declaration_day: 0,
        }
    }

    /// Formally declare sovereign factionhood (FEAT-FACT-01).
    pub fn declare_faction(
        &mut self,
        name: impl Into<String>,
        army_name: impl Into<String>,
        banner_color: BannerColor,
        doctrine: GovernmentDoctrine,
        current_day: u32,
    ) {
        self.name = name.into();
        self.army_name = army_name.into();
        self.banner_color = banner_color;
        self.doctrine = doctrine;
        self.is_declared = true;
        self.declaration_day = current_day;
    }
}
