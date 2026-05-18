# Phase 9 Execution Brief

## Phase

Phase 9: Receipt and Bundle Trust Hardening

## Objective

Make the first receipt/bundle contract stable enough for future stages to evolve without breaking auditability.

## Scope

- Align public receipt schema v0.1 with the compact Rust runtime receipt shape.
- Keep Phase 16a trust hooks reserved in schemas and fixtures.
- Add checked-in receipt/bundle fixture compatibility coverage.
- Harden bundle manifest path policy and evidence completeness.
- Add tamper tests for missing artifacts, path escape, ambiguous artifact references, and signing metadata edits.
- Document receipt compatibility rules and bundle verification boundaries.

## Known Risks

- Golden-file diffing for normalized generated receipts is still open.
- Local bundle verification proves local integrity, not signer identity or CI provenance.
- Full JSON Schema validation harness remains future hardening; current Rust tests use serde compatibility plus JSON parse checks.
