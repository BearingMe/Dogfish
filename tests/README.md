# Tests and Validation

This directory contains automated validation procedures and regression tests for research artifacts, documentation integrity, and technical tools.

## Purpose

Automated checks help maintain consistency across long-running research milestones:
- **Research Document Validation**: Verifying that document schemas, citations, and milestone questions are complete.
- **Evidence Reference Checks**: Verifying that linked artifact paths, line numbers, and sources exist and match.
- **Data Consistency Checks**: Ensuring extracted records, glossary terms, and cross-references remain aligned.
- **Parser Regression Tests**: Ensuring scripts in `scripts/` produce deterministic results across runs.

## Rules for Tests

- **Naming**: Use descriptive names indicating the behavior being validated (e.g., `validate_research_links.py`, `test_record_parser.py`). Avoid vague names (`test1.py`, `check.py`).
- **Scope**: Each test should validate a specific invariant or behavior and run deterministically.
- **Frameworks**: Do not introduce heavy testing frameworks (e.g., pytest, custom runners) unless actual tests require them. Standard library scripts or simple runners are preferred.
- **Outputs**: Test outputs must be routed to `artifacts/` or temporary locations, never left in the source or research trees.
