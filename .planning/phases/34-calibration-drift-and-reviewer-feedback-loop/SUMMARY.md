# Phase 34 Summary: Calibration, Drift, and Reviewer Feedback Loop

## Landed

- Added `pramaan.feedback.v1` feedback evidence types for reviewer overrides,
  repo baselines, bundle feedback metrics, drift warnings, calibration
  observations, and calibration reports.
- Added `pramaan feedback override` to persist accepted risk IDs, reviewer
  identity, rationale, linked outcome, and calibration intent as bundle-local
  evidence.
- Added `pramaan feedback analyze` to aggregate one or more bundles into
  dashboard-ready `feedback-report.json` and `feedback-metrics.csv` exports.
- Added local repo baseline comparison for mutation survivors, oracle residual
  risks, skipped stages, static residual risks, runtime, and confidence score.
- Added calibration scoring with Brier score, log loss, expected calibration
  error, and reliability buckets when labeled outcomes exist.
- Added docs for calibration and reviewer overrides plus synthetic baseline and
  labeled-outcome fixtures.

## Deferred

- Hosted trend API, persistent database storage, and dashboard UI remain future
  work.
- Automatic correlation between override decisions and later reverts, incidents,
  or defects remains future work.
- Real pilot-calibrated thresholds remain blocked on external pilot data.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`

## Residual Risk

Phase 34 provides local, auditable feedback evidence and exports. It does not
yet prove that the confidence model is calibrated on real production outcomes.
