# Phase 26 Execution Summary

Date: 2026-05-21

## Landed

- Added `scripts/run-phase26-pilots.ps1` to rerun the external local pilot set.
- Ran Pramaan against three public repositories:
  - `pypa/packaging` for Python.
  - `sindresorhus/is` for TypeScript.
  - `dtolnay/itoa` for Rust.
- Recorded commit SHAs, changed files, stage runtimes, skipped-tool profiles,
  noisy findings, residual risks, and reviewer time-to-understand in
  `.planning/reports/phase-26-external-alpha-pilots.md`.

## Decision

Public Alpha remains **no-go**. The three local external pilots exist, but the
live GitHub Action proof on a real PR did not run in this environment.

## Split

The live GitHub Action proof is split to Phase 26.1. Continue later hardening
work only with public copy kept honest: private technical preview yes, public
Alpha no.

## Verification

- `cargo build -p pramaan-cli`
- `powershell -ExecutionPolicy Bypass -File scripts/run-phase26-pilots.ps1`

Full workspace verification completed before the phase commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
