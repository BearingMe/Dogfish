//! Standalone state persistence: complete serialization, deserialization, and data validation.
//!
//! Grounded in Tier 5 persistence contracts from `docs/reconstruction-testing.md`.
//! Resolves DEF-PERS-01, DEF-PERS-02, DEF-PERS-03, and DEF-PERS-04.
//! Explicitly decoupled from Skyrim binary `.ess` files (zero engine savegame dependency).

use std::collections::BTreeMap;
use crate::core::{DeterministicRng, Result, SimulationError, SimulationTime};
use crate::diplomacy::ForeignFactionRelation;
use crate::faction::{BannerColor, GovernmentDoctrine};
use crate::government::{CabinetMinister, CabinetRole};
use crate::military::{ArmyOrder, SiegeOperation, SiegePhase, TroopClass};
use crate::state::GameState;
use crate::territory::AppointedGovernor;
use crate::types::{DiplomaticStance, DivisionId, FactionId, HoldingId};

/// Escape a string for valid JSON encoding (resolves DEF-PERS-04).
pub fn escape_json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out
}

/// Minimal zero-dependency JSON value representation for standalone persistence.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleJson {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<SimpleJson>),
    Object(BTreeMap<String, SimpleJson>),
}

impl SimpleJson {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SimpleJson::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            SimpleJson::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.as_f64().map(|n| n as i64)
    }

    pub fn as_i32(&self) -> Option<i32> {
        self.as_f64().map(|n| n as i32)
    }

    pub fn as_u32(&self) -> Option<u32> {
        self.as_f64().map(|n| n as u32)
    }

    pub fn as_u8(&self) -> Option<u8> {
        self.as_f64().map(|n| n as u8)
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.as_f64().map(|n| n as u64)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SimpleJson::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&SimpleJson> {
        match self {
            SimpleJson::Object(map) => map.get(key),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<SimpleJson>> {
        match self {
            SimpleJson::Array(arr) => Some(arr),
            _ => None,
        }
    }
}

/// Lightweight tokenizer and recursive descent parser for SimpleJson.
pub struct JsonParser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> JsonParser<'a> {
    pub fn parse(input: &'a str) -> Result<SimpleJson> {
        let mut parser = Self {
            chars: input.chars().peekable(),
        };
        parser.skip_whitespace();
        let val = parser.parse_value()?;
        parser.skip_whitespace();
        if parser.chars.peek().is_some() {
            return Err(SimulationError::InvalidStateTransition(
                "Trailing characters after valid JSON value",
            ));
        }
        Ok(val)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Result<SimpleJson> {
        self.skip_whitespace();
        match self.chars.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => self.parse_string(),
            Some('t') | Some('f') => self.parse_bool(),
            Some('n') => self.parse_null(),
            Some(&c) if c == '-' || c.is_ascii_digit() => self.parse_number(),
            Some(_) => Err(SimulationError::InvalidStateTransition(
                "Unexpected character in JSON",
            )),
            None => Err(SimulationError::InvalidStateTransition("Unexpected EOF in JSON")),
        }
    }

    fn parse_object(&mut self) -> Result<SimpleJson> {
        self.chars.next(); // eat '{'
        let mut map = BTreeMap::new();
        self.skip_whitespace();

        if let Some(&'}') = self.chars.peek() {
            self.chars.next();
            return Ok(SimpleJson::Object(map));
        }

        loop {
            self.skip_whitespace();
            let key = match self.parse_value()? {
                SimpleJson::String(k) => k,
                _ => {
                    return Err(SimulationError::InvalidStateTransition(
                        "Expected string key in object",
                    ))
                }
            };

            self.skip_whitespace();
            if self.chars.next() != Some(':') {
                return Err(SimulationError::InvalidStateTransition("Expected ':' after key"));
            }

            let value = self.parse_value()?;
            map.insert(key, value);

            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => continue,
                Some('}') => break,
                _ => return Err(SimulationError::InvalidStateTransition("Expected ',' or '}' in object")),
            }
        }

        Ok(SimpleJson::Object(map))
    }

    fn parse_array(&mut self) -> Result<SimpleJson> {
        self.chars.next(); // eat '['
        let mut list = Vec::new();
        self.skip_whitespace();

        if let Some(&']') = self.chars.peek() {
            self.chars.next();
            return Ok(SimpleJson::Array(list));
        }

        loop {
            let value = self.parse_value()?;
            list.push(value);

            self.skip_whitespace();
            match self.chars.next() {
                Some(',') => continue,
                Some(']') => break,
                _ => return Err(SimulationError::InvalidStateTransition("Expected ',' or ']' in array")),
            }
        }

        Ok(SimpleJson::Array(list))
    }

    fn parse_string(&mut self) -> Result<SimpleJson> {
        self.chars.next(); // eat '"'
        let mut s = String::new();
        while let Some(c) = self.chars.next() {
            if c == '"' {
                return Ok(SimpleJson::String(s));
            } else if c == '\\' {
                if let Some(esc) = self.chars.next() {
                    match esc {
                        '"' => s.push('"'),
                        '\\' => s.push('\\'),
                        '/' => s.push('/'),
                        'b' => s.push('\x08'),
                        'f' => s.push('\x0C'),
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        'u' => {
                            let mut hex = String::with_capacity(4);
                            for _ in 0..4 {
                                if let Some(h) = self.chars.next() {
                                    hex.push(h);
                                }
                            }
                            if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                if let Some(ch) = char::from_u32(code) {
                                    s.push(ch);
                                }
                            }
                        }
                        _ => s.push(esc),
                    }
                }
            } else {
                s.push(c);
            }
        }
        Err(SimulationError::InvalidStateTransition("Unterminated string literal"))
    }

    fn parse_number(&mut self) -> Result<SimpleJson> {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '-' || c == '.' || c.is_ascii_digit() || c == 'e' || c == 'E' || c == '+' {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        let num: f64 = s
            .parse()
            .map_err(|_| SimulationError::InvalidStateTransition("Failed to parse number"))?;
        Ok(SimpleJson::Number(num))
    }

    fn parse_bool(&mut self) -> Result<SimpleJson> {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_alphabetic() {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        if s == "true" {
            Ok(SimpleJson::Bool(true))
        } else if s == "false" {
            Ok(SimpleJson::Bool(false))
        } else {
            Err(SimulationError::InvalidStateTransition("Invalid boolean literal"))
        }
    }

    fn parse_null(&mut self) -> Result<SimpleJson> {
        for _ in 0..4 {
            self.chars.next();
        }
        Ok(SimpleJson::Null)
    }
}

/// Standalone persistence engine for serializing and restoring complete GameState.
pub struct StatePersistence;

impl StatePersistence {
    /// Serialize complete GameState into a standardized JSON string.
    pub fn serialize(state: &GameState) -> String {
        let mut out = String::with_capacity(8192);
        out.push_str("{\n");
        out.push_str("  \"version\": 1,\n");
        out.push_str(&format!("  \"day\": {},\n", state.time.day));
        out.push_str(&format!("  \"hour\": {},\n", state.time.hour));
        out.push_str(&format!("  \"total_ticks\": {},\n", state.time.total_ticks));
        out.push_str(&format!("  \"rng_state\": {},\n", state.rng.state()));
        out.push_str(&format!("  \"treasury\": {},\n", state.treasury));

        // Stockpile
        out.push_str(&format!(
            "  \"stockpile\": {{\"food\": {}, \"metal\": {}, \"wood\": {}, \"docks\": {}}},\n",
            state.stockpile.food, state.stockpile.metal, state.stockpile.wood, state.stockpile.trade_docks
        ));

        // Faction Profile (with string escaping)
        let banner_str = match state.faction.banner_color {
            BannerColor::Crimson => "Crimson",
            BannerColor::RoyalBlue => "RoyalBlue",
            BannerColor::EmeraldGreen => "EmeraldGreen",
            BannerColor::ImperialGold => "ImperialGold",
            BannerColor::ObsidianBlack => "ObsidianBlack",
            BannerColor::NordicWhite => "NordicWhite",
        };
        let doctrine_str = match state.faction.doctrine {
            GovernmentDoctrine::Autocracy => "Autocracy",
            GovernmentDoctrine::Feudalism => "Feudalism",
            GovernmentDoctrine::Republic => "Republic",
            GovernmentDoctrine::Meritocracy => "Meritocracy",
        };
        out.push_str(&format!(
            "  \"faction\": {{\"name\": \"{}\", \"army\": \"{}\", \"banner\": \"{}\", \"doctrine\": \"{}\", \"declared\": {}, \"day\": {}}},\n",
            escape_json_str(&state.faction.name),
            escape_json_str(&state.faction.army_name),
            banner_str,
            doctrine_str,
            state.faction.is_declared,
            state.faction.declaration_day
        ));

        // Holdings (15 holdings with all 5 facility levels)
        out.push_str("  \"holdings\": [\n");
        for (i, h) in state.holdings.iter().enumerate() {
            let owner_str = match h.owner {
                FactionId::Player => "Player",
                FactionId::Imperial => "Imperial",
                FactionId::Stormcloak => "Stormcloak",
                FactionId::Neutral => "Neutral",
            };
            out.push_str(&format!(
                "    {{\"id\": {}, \"owner\": \"{}\", \"pop\": {}, \"unemployed\": {}, \"jobs\": {}, \"loyalty\": {}, \"tax\": {}, \"guards\": {}, \"facilities\": {{\"farm\": {}, \"mine\": {}, \"mill\": {}, \"docks\": {}, \"barracks\": {}}}}}",
                h.id.0, owner_str, h.population, h.unemployed, h.max_jobs, h.loyalty, h.tax_rate, h.guards,
                h.facilities.farm, h.facilities.mine, h.facilities.lumber_mill, h.facilities.docks, h.facilities.barracks
            ));
            if i + 1 < state.holdings.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");

        // Governors (resolves DEF-PERS-02)
        out.push_str("  \"governors\": [\n");
        for (i, gov_opt) in state.territory.governors.iter().enumerate() {
            match gov_opt {
                Some(gov) => {
                    out.push_str(&format!(
                        "    {{\"holding_id\": {}, \"name\": \"{}\", \"loyalty\": {}, \"day\": {}}}",
                        gov.holding_id.0, escape_json_str(&gov.name), gov.loyalty, gov.appointment_day
                    ));
                }
                None => {
                    out.push_str("    null");
                }
            }
            if i + 1 < state.territory.governors.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");

        // Camp cooldowns and crime gold (resolves DEF-PERS-03)
        out.push_str("  \"camp_cooldowns\": [");
        for (i, cd) in state.territory.camp_cooldowns.iter().enumerate() {
            out.push_str(&format!("{}", cd));
            if i + 1 < state.territory.camp_cooldowns.len() {
                out.push_str(", ");
            }
        }
        out.push_str("],\n");

        out.push_str("  \"crime_gold\": [");
        for (i, cg) in state.territory.crime_gold.iter().enumerate() {
            out.push_str(&format!("{}", cg));
            if i + 1 < state.territory.crime_gold.len() {
                out.push_str(", ");
            }
        }
        out.push_str("],\n");

        // Divisions (10 divisions, resolves DEF-PERS-02)
        out.push_str("  \"divisions\": [\n");
        for (i, d) in state.military.divisions.iter().enumerate() {
            let class_str = match d.troop_class {
                TroopClass::Infantry => "Infantry",
                TroopClass::ShockWarrior => "ShockWarrior",
                TroopClass::Archer => "Archer",
                TroopClass::Crossbowman => "Crossbowman",
                TroopClass::Battlemage => "Battlemage",
                TroopClass::Automaton => "Automaton",
            };
            let cmd_str = match &d.commander_name {
                Some(n) => format!("\"{}\"", escape_json_str(n)),
                None => "null".to_string(),
            };
            let order_str = match &d.order {
                ArmyOrder::FollowPlayer => "{\"type\": \"Follow\"}".to_string(),
                ArmyOrder::PatrolHold(hid) => format!("{{\"type\": \"Patrol\", \"target\": {}}}", hid.0),
                ArmyOrder::GarrisonFort(hid) => format!("{{\"type\": \"Fort\", \"target\": {}}}", hid.0),
                ArmyOrder::GarrisonCity(hid) => format!("{{\"type\": \"City\", \"target\": {}}}", hid.0),
                ArmyOrder::CampAt(hid) => format!("{{\"type\": \"Camp\", \"target\": {}}}", hid.0),
                ArmyOrder::HuntBounties => "{\"type\": \"Hunt\"}".to_string(),
                ArmyOrder::AssaultHolding(hid) => format!("{{\"type\": \"Assault\", \"target\": {}}}", hid.0),
                ArmyOrder::StandBy => "{\"type\": \"StandBy\"}".to_string(),
            };
            out.push_str(&format!(
                "    {{\"id\": {}, \"name\": \"{}\", \"commander\": {}, \"class\": \"{}\", \"count\": {}, \"power\": {}, \"order\": {}}}",
                d.id.0, escape_json_str(&d.name), cmd_str, class_str, d.troop_count, d.power_level, order_str
            ));
            if i + 1 < state.military.divisions.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ],\n");

        // Active Siege (resolves DEF-PERS-01)
        match &state.military.active_siege {
            Some(s) => {
                let phase_str = match s.phase {
                    SiegePhase::StagingCamp => "StagingCamp",
                    SiegePhase::OuterGateBreach => "OuterGateBreach",
                    SiegePhase::UrbanStreetCombat => "UrbanStreetCombat",
                    SiegePhase::PalaceConfrontation => "PalaceConfrontation",
                    SiegePhase::Capitulated => "Capitulated",
                };
                out.push_str(&format!(
                    "  \"active_siege\": {{\"target\": {}, \"phase\": \"{}\", \"pincer\": {}, \"artillery\": {}, \"ram\": {}, \"att_cas\": {}, \"def_cas\": {}, \"wave\": {}, \"cap\": {}}},\n",
                    s.target_holding.0, phase_str, s.pincer_assault, s.artillery_enabled, s.battering_ram,
                    s.attacker_casualties, s.defender_casualties, s.reinforcement_wave, s.battle_concurrency_cap
                ));
            }
            None => {
                out.push_str("  \"active_siege\": null,\n");
            }
        }

        // Government & Cabinet
        let coronation_str = match state.government.coronation_day {
            Some(d) => format!("{}", d),
            None => "null".to_string(),
        };
        out.push_str(&format!(
            "  \"government\": {{\"high_king\": {}, \"coronation\": {}, \"prisoners\": {}, \"cabinet\": [\n",
            state.government.is_high_king, coronation_str, state.government.prisoners_in_dungeon
        ));
        for (i, m) in state.government.cabinet.iter().enumerate() {
            let role_str = match m.role {
                CabinetRole::Steward => "Steward",
                CabinetRole::Commander => "Commander",
                CabinetRole::Blacksmith => "Blacksmith",
                CabinetRole::Spymaster => "Spymaster",
                CabinetRole::CourtMage => "CourtMage",
                CabinetRole::CourtBard => "CourtBard",
                CabinetRole::Executioner => "Executioner",
            };
            out.push_str(&format!(
                "    {{\"role\": \"{}\", \"name\": \"{}\", \"day\": {}}}",
                role_str, escape_json_str(&m.name), m.appointed_day
            ));
            if i + 1 < state.government.cabinet.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]},\n");

        // Diplomacy (resolves DEF-PERS-03)
        out.push_str("  \"diplomacy\": [\n");
        for (i, r) in state.diplomacy.relations.iter().enumerate() {
            let fac_str = match r.faction_id {
                FactionId::Player => "Player",
                FactionId::Imperial => "Imperial",
                FactionId::Stormcloak => "Stormcloak",
                FactionId::Neutral => "Neutral",
            };
            let stance_str = match &r.stance {
                DiplomaticStance::Neutral => "{\"type\": \"Neutral\"}".to_string(),
                DiplomaticStance::War => "{\"type\": \"War\"}".to_string(),
                DiplomaticStance::Truce { days_remaining } => {
                    format!("{{\"type\": \"Truce\", \"days\": {}}}", days_remaining)
                }
                DiplomaticStance::Defeated => "{\"type\": \"Defeated\"}".to_string(),
            };
            out.push_str(&format!(
                "    {{\"faction\": \"{}\", \"stance\": {}, \"score\": {}, \"troops\": {}, \"pows\": {}}}",
                fac_str, stance_str, r.relationship_score, r.estimated_troops, r.held_pow_count
            ));
            if i + 1 < state.diplomacy.relations.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]\n");

        out.push_str("}\n");
        out
    }

    /// Deserialize complete GameState from a JSON string, enforcing domain invariants.
    pub fn deserialize(json_str: &str, fallback_seed: u64) -> Result<GameState> {
        let root = JsonParser::parse(json_str)?;

        let version = root.get("version").and_then(|v| v.as_u32()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'version' field in savegame"),
        )?;
        if version != 1 {
            return Err(SimulationError::InvalidStateTransition(
                "Unsupported savegame version (expected version 1)",
            ));
        }

        let day = root.get("day").and_then(|v| v.as_u32()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'day' field in savegame"),
        )?;
        let hour = root.get("hour").and_then(|v| v.as_u8()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'hour' field in savegame"),
        )?;
        let total_ticks = root.get("total_ticks").and_then(|v| v.as_u64()).unwrap_or((day as u64 - 1) * 24 + hour as u64);
        let rng_state = root.get("rng_state").and_then(|v| v.as_u64()).unwrap_or(fallback_seed);

        let treasury = root.get("treasury").and_then(|v| v.as_i64()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'treasury' field in savegame"),
        )?;

        if treasury < 0 {
            return Err(SimulationError::InsufficientFunds {
                required: 0,
                available: treasury,
            });
        }

        // Initialize state
        let mut state = GameState::new(rng_state);
        state.time = SimulationTime {
            day,
            hour,
            total_ticks,
        };
        state.rng = DeterministicRng::from_state(rng_state);
        state.treasury = treasury;

        // Stockpile
        if let Some(stock_obj) = root.get("stockpile") {
            if let Some(f) = stock_obj.get("food").and_then(|v| v.as_i32()) {
                state.stockpile.food = f;
            }
            if let Some(m) = stock_obj.get("metal").and_then(|v| v.as_i32()) {
                state.stockpile.metal = m;
            }
            if let Some(w) = stock_obj.get("wood").and_then(|v| v.as_i32()) {
                state.stockpile.wood = w;
            }
            if let Some(d) = stock_obj.get("docks").and_then(|v| v.as_i32()) {
                state.stockpile.trade_docks = d;
            }
        }

        // Faction
        if let Some(fac_obj) = root.get("faction") {
            if let Some(n) = fac_obj.get("name").and_then(|v| v.as_str()) {
                state.faction.name = n.to_string();
            }
            if let Some(a) = fac_obj.get("army").and_then(|v| v.as_str()) {
                state.faction.army_name = a.to_string();
            }
            if let Some(b) = fac_obj.get("banner").and_then(|v| v.as_str()) {
                state.faction.banner_color = match b {
                    "Crimson" => BannerColor::Crimson,
                    "RoyalBlue" => BannerColor::RoyalBlue,
                    "EmeraldGreen" => BannerColor::EmeraldGreen,
                    "ImperialGold" => BannerColor::ImperialGold,
                    "ObsidianBlack" => BannerColor::ObsidianBlack,
                    _ => BannerColor::NordicWhite,
                };
            }
            if let Some(doc) = fac_obj.get("doctrine").and_then(|v| v.as_str()) {
                state.faction.doctrine = match doc {
                    "Autocracy" => GovernmentDoctrine::Autocracy,
                    "Republic" => GovernmentDoctrine::Republic,
                    "Meritocracy" => GovernmentDoctrine::Meritocracy,
                    _ => GovernmentDoctrine::Feudalism,
                };
            }
            if let Some(dec) = fac_obj.get("declared").and_then(|v| v.as_bool()) {
                state.faction.is_declared = dec;
            }
            if let Some(d) = fac_obj.get("day").and_then(|v| v.as_u32()) {
                state.faction.declaration_day = d;
            }
        }

        // Holdings
        if let Some(hold_arr) = root.get("holdings").and_then(|v| v.as_array()) {
            if hold_arr.len() != HoldingId::COUNT {
                return Err(SimulationError::InvalidStateTransition(
                    "Savegame holding count does not match modeled count (15)",
                ));
            }

            for item in hold_arr {
                let id = item.get("id").and_then(|v| v.as_u8()).ok_or(
                    SimulationError::InvalidStateTransition("Corrupted holding id in savegame"),
                )?;
                if id as usize >= HoldingId::COUNT {
                    return Err(SimulationError::InvalidHoldingId(id));
                }

                let owner = match item.get("owner").and_then(|v| v.as_str()).unwrap_or("Neutral") {
                    "Player" => FactionId::Player,
                    "Imperial" => FactionId::Imperial,
                    "Stormcloak" => FactionId::Stormcloak,
                    _ => FactionId::Neutral,
                };

                let pop = item.get("pop").and_then(|v| v.as_i32()).unwrap_or(30);
                if pop < 0 {
                    return Err(SimulationError::InvalidStateTransition(
                        "Holding population cannot be negative in savegame",
                    ));
                }

                let idx = id as usize;
                state.holdings[idx].owner = owner;
                state.holdings[idx].population = pop;
                if let Some(u) = item.get("unemployed").and_then(|v| v.as_i32()) {
                    state.holdings[idx].unemployed = u;
                }
                if let Some(j) = item.get("jobs").and_then(|v| v.as_i32()) {
                    state.holdings[idx].max_jobs = j;
                }
                if let Some(loy) = item.get("loyalty").and_then(|v| v.as_i32()) {
                    state.holdings[idx].loyalty = loy;
                }
                if let Some(tax) = item.get("tax").and_then(|v| v.as_i32()) {
                    state.holdings[idx].tax_rate = tax;
                }
                if let Some(g) = item.get("guards").and_then(|v| v.as_i32()) {
                    state.holdings[idx].guards = g;
                }

                if let Some(fac) = item.get("facilities") {
                    if let Some(farm) = fac.get("farm").and_then(|v| v.as_u8()) {
                        state.holdings[idx].facilities.farm = farm;
                    }
                    if let Some(mine) = fac.get("mine").and_then(|v| v.as_u8()) {
                        state.holdings[idx].facilities.mine = mine;
                    }
                    if let Some(mill) = fac.get("mill").and_then(|v| v.as_u8()) {
                        state.holdings[idx].facilities.lumber_mill = mill;
                    }
                    if let Some(docks) = fac.get("docks").and_then(|v| v.as_u8()) {
                        state.holdings[idx].facilities.docks = docks;
                    }
                    if let Some(barracks) = fac.get("barracks").and_then(|v| v.as_u8()) {
                        state.holdings[idx].facilities.barracks = barracks;
                    }
                }
            }
        }

        // Governors (resolves DEF-PERS-02)
        if let Some(gov_arr) = root.get("governors").and_then(|v| v.as_array()) {
            for (i, item) in gov_arr.iter().enumerate() {
                if i >= HoldingId::COUNT {
                    break;
                }
                match item {
                    SimpleJson::Object(obj) => {
                        let name = obj.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
                        let loyalty = obj.get("loyalty").and_then(|v| v.as_i32()).unwrap_or(80);
                        let day = obj.get("day").and_then(|v| v.as_u32()).unwrap_or(1);
                        state.territory.governors[i] = Some(AppointedGovernor {
                            name,
                            holding_id: HoldingId(i as u8),
                            loyalty,
                            appointment_day: day,
                        });
                    }
                    _ => {
                        state.territory.governors[i] = None;
                    }
                }
            }
        }

        // Cooldowns and Crime Gold (resolves DEF-PERS-03)
        if let Some(cd_arr) = root.get("camp_cooldowns").and_then(|v| v.as_array()) {
            for (i, val) in cd_arr.iter().enumerate() {
                if i < HoldingId::COUNT {
                    state.territory.camp_cooldowns[i] = val.as_u32().unwrap_or(0);
                }
            }
        }

        if let Some(cg_arr) = root.get("crime_gold").and_then(|v| v.as_array()) {
            for (i, val) in cg_arr.iter().enumerate() {
                if i < HoldingId::COUNT {
                    state.territory.crime_gold[i] = val.as_u32().unwrap_or(0);
                }
            }
        }

        // Divisions (resolves DEF-PERS-02)
        if let Some(div_arr) = root.get("divisions").and_then(|v| v.as_array()) {
            for item in div_arr {
                let id = item.get("id").and_then(|v| v.as_u8()).unwrap_or(0);
                if id as usize >= DivisionId::MAX_DIVISIONS {
                    continue;
                }
                let idx = id as usize;

                if let Some(n) = item.get("name").and_then(|v| v.as_str()) {
                    state.military.divisions[idx].name = n.to_string();
                }
                state.military.divisions[idx].commander_name = item.get("commander").and_then(|v| v.as_str()).map(|s| s.to_string());
                if let Some(cls) = item.get("class").and_then(|v| v.as_str()) {
                    state.military.divisions[idx].troop_class = match cls {
                        "ShockWarrior" => TroopClass::ShockWarrior,
                        "Archer" => TroopClass::Archer,
                        "Crossbowman" => TroopClass::Crossbowman,
                        "Battlemage" => TroopClass::Battlemage,
                        "Automaton" => TroopClass::Automaton,
                        _ => TroopClass::Infantry,
                    };
                }
                if let Some(cnt) = item.get("count").and_then(|v| v.as_i32()) {
                    state.military.divisions[idx].troop_count = cnt;
                }
                if let Some(pwr) = item.get("power").and_then(|v| v.as_u8()) {
                    state.military.divisions[idx].power_level = pwr;
                }

                if let Some(ord_obj) = item.get("order") {
                    if let Some(t) = ord_obj.get("type").and_then(|v| v.as_str()) {
                        let target = ord_obj.get("target").and_then(|v| v.as_u8()).map(HoldingId).unwrap_or(HoldingId(0));
                        state.military.divisions[idx].order = match t {
                            "Follow" => ArmyOrder::FollowPlayer,
                            "Patrol" => ArmyOrder::PatrolHold(target),
                            "Fort" => ArmyOrder::GarrisonFort(target),
                            "City" => ArmyOrder::GarrisonCity(target),
                            "Camp" => ArmyOrder::CampAt(target),
                            "Hunt" => ArmyOrder::HuntBounties,
                            "Assault" => ArmyOrder::AssaultHolding(target),
                            _ => ArmyOrder::StandBy,
                        };
                    }
                }
            }
        }

        // Active Siege (resolves DEF-PERS-01)
        if let Some(siege_val) = root.get("active_siege") {
            match siege_val {
                SimpleJson::Object(obj) => {
                    let target = obj.get("target").and_then(|v| v.as_u8()).map(HoldingId).unwrap_or(HoldingId(0));
                    let phase_str = obj.get("phase").and_then(|v| v.as_str()).unwrap_or("StagingCamp");
                    let phase = match phase_str {
                        "OuterGateBreach" => SiegePhase::OuterGateBreach,
                        "UrbanStreetCombat" => SiegePhase::UrbanStreetCombat,
                        "PalaceConfrontation" => SiegePhase::PalaceConfrontation,
                        "Capitulated" => SiegePhase::Capitulated,
                        _ => SiegePhase::StagingCamp,
                    };
                    let pincer = obj.get("pincer").and_then(|v| v.as_bool()).unwrap_or(false);
                    let artillery = obj.get("artillery").and_then(|v| v.as_bool()).unwrap_or(false);
                    let ram = obj.get("ram").and_then(|v| v.as_bool()).unwrap_or(false);
                    let att_cas = obj.get("att_cas").and_then(|v| v.as_i32()).unwrap_or(0);
                    let def_cas = obj.get("def_cas").and_then(|v| v.as_i32()).unwrap_or(0);
                    let wave = obj.get("wave").and_then(|v| v.as_u8()).unwrap_or(0);
                    let cap = obj.get("cap").and_then(|v| v.as_i32()).unwrap_or(40);

                    state.military.active_siege = Some(SiegeOperation {
                        target_holding: target,
                        phase,
                        pincer_assault: pincer,
                        artillery_enabled: artillery,
                        battering_ram: ram,
                        attacker_casualties: att_cas,
                        defender_casualties: def_cas,
                        reinforcement_wave: wave,
                        battle_concurrency_cap: cap,
                    });
                }
                _ => {
                    state.military.active_siege = None;
                }
            }
        }

        // Government
        if let Some(gov_obj) = root.get("government") {
            if let Some(hk) = gov_obj.get("high_king").and_then(|v| v.as_bool()) {
                state.government.is_high_king = hk;
            }
            state.government.coronation_day = gov_obj.get("coronation").and_then(|v| v.as_u32());
            if let Some(p) = gov_obj.get("prisoners").and_then(|v| v.as_u32()) {
                state.government.prisoners_in_dungeon = p;
            }

            if let Some(cab_arr) = gov_obj.get("cabinet").and_then(|v| v.as_array()) {
                state.government.cabinet.clear();
                for m_item in cab_arr {
                    let role_str = m_item.get("role").and_then(|v| v.as_str()).unwrap_or("Steward");
                    let role = match role_str {
                        "Commander" => CabinetRole::Commander,
                        "Blacksmith" => CabinetRole::Blacksmith,
                        "Spymaster" => CabinetRole::Spymaster,
                        "CourtMage" => CabinetRole::CourtMage,
                        "CourtBard" => CabinetRole::CourtBard,
                        "Executioner" => CabinetRole::Executioner,
                        _ => CabinetRole::Steward,
                    };
                    let name = m_item.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
                    let day = m_item.get("day").and_then(|v| v.as_u32()).unwrap_or(1);
                    state.government.cabinet.push(CabinetMinister {
                        role,
                        name,
                        appointed_day: day,
                    });
                }
            }
        }

        // Diplomacy (resolves DEF-PERS-03)
        if let Some(dip_arr) = root.get("diplomacy").and_then(|v| v.as_array()) {
            state.diplomacy.relations.clear();
            for r_item in dip_arr {
                let fac_str = r_item.get("faction").and_then(|v| v.as_str()).unwrap_or("Imperial");
                let faction_id = match fac_str {
                    "Stormcloak" => FactionId::Stormcloak,
                    "Player" => FactionId::Player,
                    "Neutral" => FactionId::Neutral,
                    _ => FactionId::Imperial,
                };
                let score = r_item.get("score").and_then(|v| v.as_i32()).unwrap_or(-50);
                let troops = r_item.get("troops").and_then(|v| v.as_i32()).unwrap_or(150);
                let pows = r_item.get("pows").and_then(|v| v.as_u32()).unwrap_or(0);

                let stance = if let Some(st_obj) = r_item.get("stance") {
                    let st_type = st_obj.get("type").and_then(|v| v.as_str()).unwrap_or("War");
                    match st_type {
                        "Neutral" => DiplomaticStance::Neutral,
                        "Truce" => {
                            let days = st_obj.get("days").and_then(|v| v.as_u32()).unwrap_or(30);
                            DiplomaticStance::Truce { days_remaining: days }
                        }
                        "Defeated" => DiplomaticStance::Defeated,
                        _ => DiplomaticStance::War,
                    }
                } else {
                    DiplomaticStance::War
                };

                state.diplomacy.relations.push(ForeignFactionRelation {
                    faction_id,
                    stance,
                    relationship_score: score,
                    estimated_troops: troops,
                    held_pow_count: pows,
                });
            }
        }

        state.check_invariants()?;
        Ok(state)
    }
}
