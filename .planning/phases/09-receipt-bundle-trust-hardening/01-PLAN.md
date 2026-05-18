---
phase: 9
plan: 1
title: Receipt and Bundle Trust Hardening
wave: 1
depends_on:
  - ../05-bundle-signing-and-verification/02-PLAN
  - ../08-killer-demo-and-proof-bundles/01-PLAN
files_modified:
  - schemas/
  - crates/pramaan-core/
  - crates/pramaan-bundle/
  - examples/fixtures/
  - docs/receipt-model.md
  - docs/bundle-verification.md
autonomous: true
priority: P0
---

# Plan 01 - Receipt and Bundle Trust Hardening

## Objective

Make receipts and bundles stable enough that future stages can evolve without breaking auditability.

## Tasks

<task id="9-01-01">Freeze receipt schema version `0.1` and document compatibility rules.</task>
<task id="9-01-02">Add schema compatibility tests for every checked-in receipt and bundle fixture.</task>
<task id="9-01-03">Add golden tests that compare generated receipts with approved fixture outputs.</task>
<task id="9-01-04">Add artifact graph support so receipts can reference hashed logs, corpora, and tool output files.</task>
<task id="9-01-05">Add bundle-level verification summary grouped by mitigated, residual, skipped, and not-applicable risk families.</task>
<task id="9-01-06">Add tamper tests for missing artifacts, modified receipts, modified manifests, and changed signing metadata.</task>

## Verification

Run `cargo fmt --check`, `cargo test --workspace`, schema fixture validation, and explicit bundle tamper tests. Confirm old `0.1` fixtures remain readable and invalid schema changes fail loudly.

