# Phase 19: P0 Receipt Golden Tests and Canonical Evidence

## Goal

Make receipt and bundle evidence stable enough that fixture drift, schema drift,
and hash drift are caught early.

## P0 Tasks Covered

- Add golden tests that diff generated receipts against approved fixtures.
- Strengthen schema/runtime consistency guardrails.
- Make canonical serialization and hashing explicit.
- Prevent accidental changes to fixture receipts from passing unnoticed.

## Files to Change

- `crates/pramaan-core/src/lib.rs`
- `crates/pramaan-bundle/src/lib.rs`
- `crates/pramaan-cli/tests/` or existing CLI smoke tests
- `schemas/receipt.schema.json`
- `schemas/bundle.schema.json`
- `examples/**/proof-bundle/**`
- `docs/receipt-model.md`

## Implementation Steps

1. Inventory checked-in generated receipts and classify which should become golden fixtures.
2. Add tests that regenerate deterministic receipts for selected fixtures and compare normalized output.
3. Add canonical serialization helpers or document current deterministic boundaries before broader refactor.
4. Fail tests when required schema/runtime fields drift.
5. Document how to intentionally update golden fixtures.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Fixture regeneration command documented in the phase summary.

## Exit Criteria

Receipt drift is intentional, reviewable, and test-visible.
