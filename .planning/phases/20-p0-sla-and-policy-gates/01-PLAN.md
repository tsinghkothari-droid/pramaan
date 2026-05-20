# Phase 20: P0 SLA and Policy Gates

## Goal

Make Pramaan usable in pull-request CI by giving reviewers clear runtime budgets
and explainable policy outcomes.

## P0 Tasks Covered

- Define hard performance SLA targets.
- Add default policy-as-code profile.
- Add `pramaan policy explain`.
- Make stage-budget exhaustion visible in receipts and GitHub summaries.

## Files to Change

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `action/render-summary.mjs`
- `action.yml`
- `docs/github-action.md`
- `docs/bundle-verification.md`
- `TASKS.md`

## Implementation Steps

1. Define small/medium/large PR SLA classes by changed-line count and language mix.
2. Add default policy profile with hard gates, warning gates, waivers, required stages, and security-sensitive paths.
3. Implement `pramaan policy explain <bundle>`.
4. Surface policy outcome in CLI and GitHub Action summary.
5. Add tests for pass, warning, fail, skipped-required-stage, and budget-exhausted decisions.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node --test action/render-summary.test.mjs`

## Exit Criteria

A reviewer can tell why a bundle passed, warned, or failed without reading raw
JSON.
