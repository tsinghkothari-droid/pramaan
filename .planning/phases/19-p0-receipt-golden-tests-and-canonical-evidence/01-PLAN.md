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

## Completion Summary

Completed on 2026-05-21.

Landed:

- Added canonical JSON byte serialization in `pramaan-core`.
- Switched bundle manifest digest calculation to canonical JSON bytes.
- Added core round-trip tests for canonical receipt serialization.
- Added a normalized golden contract assertion for the generated claim-scope
  receipt in the CLI smoke test.
- Documented golden/canonical evidence discipline in `docs/receipt-model.md`.
- Marked the P0 golden-test task complete in `TASKS.md`.

Deferred:

- Full JSON Schema validation for every generated artifact remains future
  hardening.
- Receipt artifact digests still hash the exact emitted bytes, by design.

Risks discovered:

- Existing receipt artifacts are display JSON and signable evidence at the same
  time. Future signing work should explicitly split human-readable rendering
  from canonical signing payloads if that becomes necessary.
