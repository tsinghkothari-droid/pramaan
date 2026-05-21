# Phase 32.75 Summary: Anti-Gaming and Verifier-Abuse Hardening

## Status

Completed 2026-05-21 as `PASS_WITH_RISKS`.

## What Landed

- Added `R-094` (`VERIFIER_SURFACE_CHANGED`) and `R-095`
  (`VERIFIER_STAGE_LAUNDERING`) named risk constants.
- Added `detect_verifier_abuse_paths` in core to flag changes to:
  - `.github/workflows/`
  - `action.yml` / `action.yaml`
  - verifier scripts under `scripts/`
  - schemas under `schemas/`
  - fixtures/corpus under `examples/`, `corpus/`, and rendered examples
  - GSD readiness evidence under `.planning/`
- Integrated the detector into claim-scope evidence so verifier-surface changes
  become receipt limitations and residual risk.
- Updated the `security-sensitive` and inherited `fintech-strict` policy
  profiles to hard-fail `R-094` and `R-095`.
- Added `corpus/verifier-abuse-fixtures.v0.1.json` and
  `scripts/check-verifier-abuse-fixtures.mjs` with six anti-gaming scenarios.
- Updated policy, threat-model, risk taxonomy, status, roadmap, and task docs.

## Verification

Full phase-close verification was run before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `node scripts/check-verifier-abuse-fixtures.mjs`

## Deferred

- The new corpus file is scenario-level, not a complete executable exploit
  suite.
- Phase 40 still owns larger executable exploit fixtures and benchmark
  integrity stress tests.
