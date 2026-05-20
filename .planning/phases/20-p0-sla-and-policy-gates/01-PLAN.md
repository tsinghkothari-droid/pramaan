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

## Completion Summary

Completed on 2026-05-21.

Landed:

- Added a default policy profile with required stages, hard gate statuses,
  warning statuses, security-sensitive path hints, and small/medium/large SLA
  classes.
- Added core policy evaluation with tests for pass, warning, failed stage,
  skipped required stage, and budget exhaustion.
- Added `pramaan policy explain <bundle>`.
- Updated the GitHub Action to run policy explanation before rendering the
  summary.
- Updated the summary renderer to show policy decision, hard failures, and
  warnings.
- Documented SLA and policy behavior in GitHub Action and bundle verification
  docs.

Deferred:

- Custom user-supplied policy files remain future work.
- Policy decisions are computed by `policy explain`; embedding the final
  computed decision back into every manifest remains future hardening.

Risks discovered:

- Current manifests often carry the first stage's informational policy decision.
  Reviewers should prefer `pramaan policy explain` until bundle-level policy
  embedding is implemented.
