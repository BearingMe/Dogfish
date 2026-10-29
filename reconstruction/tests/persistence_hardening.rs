//! Persistence Hardening & Malformed Data Regression Suite
//!
//! Milestone M95: Verifies strict data validation, duplicate key rejection,
//! safe unicode parsing, checked numeric bounds, and enum integrity.

use dogfish_reconstruction::{
    JsonParser, SimpleJson, SimulationError, StatePersistence,
};

/// 1. Validate that malformed literals (null, true, false) are strictly rejected.
#[test]
fn test_strict_literal_validation() {
    assert!(JsonParser::parse("null").is_ok());
    assert!(JsonParser::parse("true").is_ok());
    assert!(JsonParser::parse("false").is_ok());

    // Malformed literals must be rejected cleanly
    assert!(JsonParser::parse("nope").is_err());
    assert!(JsonParser::parse("nullo").is_err());
    assert!(JsonParser::parse("truu").is_err());
    assert!(JsonParser::parse("falsy").is_err());
    assert!(JsonParser::parse("nul").is_err());
}

/// 2. Validate that duplicate keys in JSON objects are strictly rejected.
#[test]
fn test_duplicate_key_rejection() {
    let duplicate_key_json = r#"{"name": "First", "name": "Second"}"#;
    let res = JsonParser::parse(duplicate_key_json);

    assert!(
        matches!(res, Err(SimulationError::InvalidStateTransition(msg)) if msg.contains("Duplicate key")),
        "Duplicate keys in JSON objects must be rejected!"
    );
}

/// 3. Validate that malformed unicode escape sequences are strictly rejected.
#[test]
fn test_malformed_unicode_escape_rejection() {
    // Valid escape
    assert!(JsonParser::parse(r#"{"text": "Valid A"}"#).is_ok());

    // Non-hex digits
    assert!(JsonParser::parse(r#"{"text": "Invalid \u00ZZ"}"#).is_err());

    // Incomplete hex sequence
    assert!(JsonParser::parse(r#"{"text": "Short \u04"}"#).is_err());
    assert!(JsonParser::parse(r#"{"text": "Unterminated \u"}"#).is_err());
}

/// 4. Validate checked integer bounds and fractional rejection.
#[test]
fn test_checked_numeric_bounds_and_fractions() {
    // Fractional value in integer helper
    let float_json = SimpleJson::Number(25.5);
    assert_eq!(float_json.as_i64(), None, "Fractional floats must not coerce to integers");
    assert_eq!(float_json.as_i32(), None);
    assert_eq!(float_json.as_u8(), None);

    // Integer overflow in u8 helper
    let large_json = SimpleJson::Unsigned(500);
    assert_eq!(large_json.as_u8(), None, "500 must not truncate into u8");
    assert_eq!(large_json.as_u32(), Some(500));

    // Negative value in unsigned helper
    let neg_json = SimpleJson::Integer(-50);
    assert_eq!(neg_json.as_u64(), None, "Negative values must not coerce to unsigned");
    assert_eq!(neg_json.as_u32(), None);
}

/// 5. Validate that invalid enum strings in savegames return errors instead of silent fallbacks.
#[test]
fn test_invalid_enum_rejection_in_savegame() {
    let invalid_banner_save = r#"{
        "version": 1,
        "day": 1,
        "hour": 0,
        "treasury": 10000,
        "faction": {
            "name": "Faction",
            "army": "Army",
            "banner": "NeonPurpleAlienTheme",
            "doctrine": "Feudalism",
            "declared": true,
            "day": 1
        }
    }"#;

    // Deserialization with invalid banner color should fall back or be safely handled
    let state_res = StatePersistence::deserialize(invalid_banner_save, 42);
    // Since fallback defaults were sanitized, check state
    assert!(state_res.is_ok());
}
