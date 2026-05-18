---
phase: 1
plan: 3
title: Smoke Tests, Summary Output, and Phase Docs
wave: 2
depends_on:
  - 01-PLAN
  - 02-PLAN
files_modified:
  - crates/pramaan-cli/src/main.rs
  - crates/pramaan-cli/tests/smoke.rs
  - crates/pramaan-core/src/lib.rs
  - README.md
  - docs/receipt-model.md
  - docs/risk-taxonomy.md
autonomous: true
requirements:
  - CLI-03
  - RCPT-03
  - RISK-01
---

# Plan 03 - Smoke Tests, Summary Output, and Phase Docs

## Objective

Make the Phase 1 skeleton demonstrably usable: schema fixtures validate, the CLI smoke path works, and the terminal summary makes the evidence model clear.

## Must Haves

- CLI prints a compact summary with stage name, status, and output bundle path.
- CLI summary shows risk families as mitigated/residual/skipped without a single opaque score.
- Smoke test exercises the CLI verify path.
- README shows the intended command and carefully describes Pramaan as an auditable proof bundle, not a correctness proof.
- `docs/receipt-model.md` explains how receipts, claim scope, and bundle manifest relate.
- `docs/risk-taxonomy.md` explains how the top-100 flaw register maps to receipt risk IDs.
- Phase 1 validation commands are documented and green.

## Tasks

<task id="1-03-01">
Add CLI summary rendering for synthetic verification results.
</task>

<task id="1-03-02">
Add a smoke/integration test for `pramaan verify --base HEAD --head HEAD --out <tmp>`.
</task>

<task id="1-03-03">
Add schema fixture validation tests if they were not completed in Plan 02.
</task>

<task id="1-03-04">
Create README quickstart and `docs/receipt-model.md` with claim-disciplined language.
</task>

<task id="1-03-05">
Create `docs/risk-taxonomy.md` explaining the top-100 risk model and how stage receipts reference risk IDs.
</task>

<task id="1-03-06">
Run the full validation suite and update Phase 1 notes if any commands differ from the validation strategy.
</task>

## Verification

<automated>
Run `cargo fmt --check; cargo test`.
</automated>

<manual>
Read README and CLI output to ensure Pramaan says what was checked and does not claim correctness.
</manual>
