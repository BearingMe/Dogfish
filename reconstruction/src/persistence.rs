//! Standalone state persistence: serialization, deserialization, and integrity validation.
//!
//! Grounded in Tier 5 persistence contracts from `docs/reconstruction-testing.md`.
//! Explicitly decoupled from Skyrim binary `.ess` files (zero engine savegame dependency).

use std::collections::BTreeMap;
use crate::core::{Result, SimulationError};
use crate::state::GameState;
use crate::types::{FactionId, HoldingId};

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
            Some(_) => Err(SimulationError::InvalidStateTransition("Unexpected character in JSON")),
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
                _ => return Err(SimulationError::InvalidStateTransition("Expected string key in object")),
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
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
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

/// Standalone persistence engine for serializing and restoring GameState.
pub struct StatePersistence;

impl StatePersistence {
    /// Serialize GameState into a standardized JSON string.
    pub fn serialize(state: &GameState) -> String {
        let mut out = String::with_capacity(4096);
        out.push_str("{\n");
        out.push_str(&format!("  \"version\": 1,\n"));
        out.push_str(&format!("  \"day\": {},\n", state.time.day));
        out.push_str(&format!("  \"hour\": {},\n", state.time.hour));
        out.push_str(&format!("  \"treasury\": {},\n", state.treasury));
        out.push_str(&format!(
            "  \"stockpile\": {{\"food\": {}, \"metal\": {}, \"wood\": {}, \"docks\": {}}},\n",
            state.stockpile.food, state.stockpile.metal, state.stockpile.wood, state.stockpile.trade_docks
        ));
        out.push_str(&format!(
            "  \"faction\": {{\"name\": \"{}\", \"army\": \"{}\", \"declared\": {}}},\n",
            state.faction.name, state.faction.army_name, state.faction.is_declared
        ));
        out.push_str(&format!(
            "  \"government\": {{\"high_king\": {}, \"prisoners\": {}}},\n",
            state.government.is_high_king, state.government.prisoners_in_dungeon
        ));

        // Holdings
        out.push_str("  \"holdings\": [\n");
        for (i, h) in state.holdings.iter().enumerate() {
            let owner_str = match h.owner {
                FactionId::Player => "Player",
                FactionId::Imperial => "Imperial",
                FactionId::Stormcloak => "Stormcloak",
                FactionId::Neutral => "Neutral",
            };
            out.push_str(&format!(
                "    {{\"id\": {}, \"owner\": \"{}\", \"pop\": {}, \"jobs\": {}, \"loyalty\": {}, \"tax\": {}}}",
                h.id.0, owner_str, h.population, h.max_jobs, h.loyalty, h.tax_rate
            ));
            if i + 1 < state.holdings.len() {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("  ]\n");
        out.push_str("}\n");
        out
    }

    /// Deserialize GameState from a JSON string, enforcing domain invariants.
    pub fn deserialize(json_str: &str, seed: u64) -> Result<GameState> {
        let root = JsonParser::parse(json_str)?;

        let day = root.get("day").and_then(|v| v.as_u32()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'day' field in savegame"),
        )?;
        let hour = root.get("hour").and_then(|v| v.as_u8()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'hour' field in savegame"),
        )?;
        let treasury = root.get("treasury").and_then(|v| v.as_i64()).ok_or(
            SimulationError::InvalidStateTransition("Missing 'treasury' field in savegame"),
        )?;

        // Invariant check: Treasury cannot be negative
        if treasury < 0 {
            return Err(SimulationError::InsufficientFunds {
                required: 0,
                available: treasury,
            });
        }

        let mut state = GameState::new(seed);
        state.time.day = day;
        state.time.hour = hour;
        state.treasury = treasury;

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
        }

        if let Some(fac_obj) = root.get("faction") {
            if let Some(n) = fac_obj.get("name").and_then(|v| v.as_str()) {
                state.faction.name = n.to_string();
            }
            if let Some(dec) = fac_obj.get("declared").and_then(|v| v.as_bool()) {
                state.faction.is_declared = dec;
            }
        }

        if let Some(gov_obj) = root.get("government") {
            if let Some(hk) = gov_obj.get("high_king").and_then(|v| v.as_bool()) {
                state.government.is_high_king = hk;
            }
        }

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

                state.holdings[id as usize].owner = owner;
                state.holdings[id as usize].population = pop;
                if let Some(loy) = item.get("loyalty").and_then(|v| v.as_i32()) {
                    state.holdings[id as usize].loyalty = loy;
                }
                if let Some(tax) = item.get("tax").and_then(|v| v.as_i32()) {
                    state.holdings[id as usize].tax_rate = tax;
                }
            }
        }

        state.check_invariants()?;
        Ok(state)
    }
}
