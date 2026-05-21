# Phase 28.5 Execution Summary

Date: 2026-05-21

## Landed

- Added a core confidence artifact builder with schema version
  `pramaan.confidence.v1` and algorithm version
  `pramaan-confidence-v0.1-uncalibrated`.
- Added `pramaan confidence explain <bundle>`, which writes `confidence.json`,
  `confidence.md`, and a `confidence_vote` receipt, then rebuilds the bundle
  manifest so the confidence artifacts have digest links for Phase 29 signing.
- Added hard-gate handling for failed oracle-integrity evidence, failed
  bundle/attestation-style receipts, explicitly untrusted plugin provenance,
  and exhausted stage budgets.
- Added weak-signal votes, dependency discounts for correlated evidence
  clusters, skipped-stage uncertainty penalties, Wilson lower bounds for
  mutation kill-rate evidence, and rule-of-three residual-risk bounds for
  zero-failure property/fuzz evidence.
- Added `schemas/confidence.schema.json`, `docs/confidence.md`, checked
  confidence fixtures, smoke coverage, and claim-audit updates.

## Deferred

- Full JSON Schema validator coverage for unknown algorithm versions and
  forward-compatible optional fields.
- Dedicated bundle-tamper and invalid-attestation confidence fixtures.
- Unsupported critical evidence path hard gates beyond the current budget and
  failed-attestation signals.
- Calibration using external pilot outcomes, Brier score, log loss, expected
  calibration error, and reliability diagrams. That remains Phase 34.

## Verification

- `cargo test -p pramaan-core confidence -- --nocapture`
- Full workspace fmt/test/clippy should be run before commit.

## Status

Phase 28.5 is partially executed and now has a working v0.1 confidence bridge.
It should not be described as calibrated, signed, or production merge authority.
