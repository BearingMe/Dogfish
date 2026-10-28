# Adversarial Regression Baseline: Reproducible Defect Evidence

- **Document**: Adversarial Regression Suite & Defect Reproduction Report
- **Milestone**: M65 — Adversarial Regression Baseline
- **Phase**: Phase 16 (Baseline Verification and Defect Reproduction)
- **Subject**: Execution of `reconstruction/tests/adversarial_regressions.rs`
- **Investigation Date**: 2026-10-28
- **Status**: EXECUTED & DEFECTS REPRODUCED (9 Failures / 3 Passes)

---

## 1. Executive Summary & Objective

In strict compliance with Rule 1.1 (*"Test Before Fixing"*), Milestone M65 establishes an automated adversarial test harness specifically designed to expose suspected architectural weaknesses, unverified assumptions, and state corruption vectors in the current reconstruction implementation before attempting any code corrections.

The test suite [`reconstruction/tests/adversarial_regressions.rs`](../../reconstruction/tests/adversarial_regressions.rs) was executed against the unpatched implementation.

### Summary of Results:
$$\text{Total Test Cases} = 12 \quad\Big|\quad \mathbf{3\ Passed} \quad\Big|\quad \mathbf{9\ Failed\ (Defects\ Confirmed)}$$

---

## 2. Granular Results by Test Case

| Case ID | Adversarial Test Scenario | Target Subsystem | Result | Observed Defect Behavior |
| :---: | :--- | :--- | :---: | :--- |
| **Case 01** | Save/load with active siege | `reconstruction::persistence` | **FAILED** | `active_siege` is completely omitted from JSON serialization; deserialized state has `active_siege: None`. |
| **Case 02** | Save/load with armies & governors | `reconstruction::persistence` | **FAILED** | Division troop counts, orders, and appointed Jarls are omitted from serialization and reset to defaults. |
| **Case 03** | Faction names with quotes & backslashes | `reconstruction::persistence` | **FAILED** | Strings are injected directly into JSON without escaping; unescaped quotes break the `JsonParser`. |
| **Case 04** | Facility upgrade in enemy holding | `reconstruction::economy` | **FAILED** | Missing ownership invariant: player can spend gold and wood upgrading facilities in enemy-owned holds. |
| **Case 05** | Governor appointment in enemy holding| `reconstruction::territory` | **FAILED** | Missing ownership invariant: player can appoint Jarls to unowned or enemy provincial capitals. |
| **Case 06** | Repeated siege initiation | `reconstruction::military` | **FAILED** | Calling `start_siege` while a siege is underway silently overwrites the previous siege operation. |
| **Case 07** | Siege progression without troops | `reconstruction::military` | **FAILED** | `advance_siege` allows player with 0 troops to advance all 5 phases and capitulate a capital in 0 seconds. |
| **Case 08** | Negative court fines and ransoms | `reconstruction::government` | **FAILED** | Negative judicial fines are accepted without validation, deducting treasury gold. |
| **Case 09** | Resource integer overflow | `reconstruction::economy` | **FAILED** | `deposit_treasury` uses unchecked `*treasury += amount`, triggering an unhandled arithmetic panic on overflow. |
| **Case 10** | Invalid holding ownership transitions | `reconstruction::territory` | **PASSED** | Correctly rejects transferring a holding to its current owning faction. |
| **Case 11** | Savegame idempotency cycles | `reconstruction::persistence` | **PASSED** | Serializing and restoring default state produces identical JSON output across consecutive cycles. |
| **Case 12** | Interrupted operation resource safety | `reconstruction::economy` | **PASSED** | Facility upgrade checks both gold and wood before mutating; insufficient wood aborts without deducting gold. |

---

## 3. Verbatim Test Execution Output

```text
$ cargo test --test adversarial_regressions --manifest-path reconstruction/Cargo.toml --offline

running 12 tests
test test_case_10_invalid_holding_transitions ... ok
test test_case_12_interrupted_operation_resource_conservation ... ok
test test_case_11_savegame_idempotency_cycles ... ok
test test_case_05_governor_appointment_in_enemy_holding ... FAILED
test test_case_03_faction_names_with_quotes_and_backslashes ... FAILED
test test_case_06_repeated_siege_initiation ... FAILED
test test_case_02_save_load_with_modified_armies_and_governors ... FAILED
test test_case_01_save_load_with_active_siege ... FAILED
test test_case_07_siege_progression_without_participating_army ... FAILED
test test_case_09_resource_overflow_protection ... FAILED
test test_case_04_facility_upgrade_in_enemy_holding ... FAILED
test test_case_08_negative_court_fines_and_ransom ... FAILED

failures:

---- test_case_05_governor_appointment_in_enemy_holding stdout ----
thread 'test_case_05_governor_appointment_in_enemy_holding' panicked at tests\adversarial_regressions.rs:134:5:
DEFECT CONFIRMED: Player can appoint governors in enemy-owned holdings!

---- test_case_03_faction_names_with_quotes_and_backslashes stdout ----
thread 'test_case_03_faction_names_with_quotes_and_backslashes' panicked at tests\adversarial_regressions.rs:92:5:
DEFECT CONFIRMED: JSON serializer produces invalid JSON when string contains quotes!

---- test_case_06_repeated_siege_initiation stdout ----
thread 'test_case_06_repeated_siege_initiation' panicked at tests\adversarial_regressions.rs:153:5:
DEFECT CONFIRMED: Starting a second siege silently overwrites existing active siege!

---- test_case_02_save_load_with_modified_armies_and_governors stdout ----
thread 'test_case_02_save_load_with_modified_armies_and_governors' panicked at tests\adversarial_regressions.rs:60:5:
assertion `left == right` failed: DEFECT CONFIRMED: Division troop_count is lost across serialization!
  left: 0
 right: 50

---- test_case_01_save_load_with_active_siege stdout ----
thread 'test_case_01_save_load_with_active_siege' panicked at tests\adversarial_regressions.rs:30:5:
DEFECT CONFIRMED: active_siege is lost across serialization!

---- test_case_07_siege_progression_without_participating_army stdout ----
thread 'test_case_07_siege_progression_without_participating_army' panicked at tests\adversarial_regressions.rs:174:5:
DEFECT CONFIRMED: Sieges can be advanced to victory without any participating troops!

---- test_case_09_resource_overflow_protection stdout ----
thread 'test_case_09_resource_overflow_protection' panicked at src\economy.rs:60:9:
attempt to add with overflow

---- test_case_04_facility_upgrade_in_enemy_holding stdout ----
thread 'test_case_04_facility_upgrade_in_enemy_holding' panicked at tests\adversarial_regressions.rs:114:5:
DEFECT CONFIRMED: Player can upgrade facilities in enemy-owned holdings!

---- test_case_08_negative_court_fines_and_ransom stdout ----
thread 'test_case_08_negative_court_fines_and_ransom' panicked at tests\adversarial_regressions.rs:191:5:
DEFECT CONFIRMED: Negative court fines are accepted and corrupt treasury!

test result: FAILED. 3 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 4. Significance for Phases 17–19

These 9 reproducible test failures provide the objective, hard evidence needed for the repair program:
1. **M67 & M68 (Persistence Repair)**: Cases 01, 02, and 03 prove that the custom serializer is severely incomplete and lacks string escaping.
2. **M69 & M70 (Invariants & Safety)**: Cases 04, 05, 08, and 09 prove that domain authorization checks and checked arithmetic are missing.
3. **M74 (Military Fidelity)**: Cases 06 and 07 prove that siege lifecycle rules lack physical prerequisite validation.
