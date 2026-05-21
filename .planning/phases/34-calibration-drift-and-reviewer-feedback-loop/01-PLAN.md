# Phase 34: Calibration, Drift, and Reviewer Feedback Loop

## Goal

Prevent alert fatigue by recording repo baselines, reviewer overrides, and
quality drift over time.

## Research Drivers

- CI signal fatigue makes uncalibrated warnings easy to ignore.
- Agent-author attribution only becomes valuable when correlated across PRs and
  repositories.

## Tasks Covered

- Reviewer override persistence.
- Per-repo baselines for mutation survival, oracle warnings, skipped stages,
  runtime, and static findings.
- Trend export for agent-code quality drift.
- Agent-author attribution analysis.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `schemas/`
- `docs/calibration.md`
- `docs/reviewer-overrides.md`
- `.planning/reports/`
- `TASKS.md`

## Implementation Steps

1. Define local baseline storage and export format.
2. Add reviewer override import/capture command or receipt path.
3. Compare current bundle metrics to repo baseline.
4. Emit drift warnings for mutation survivors, oracle risks, skipped stages,
   runtime, and agent-author clusters.
5. Keep dashboards optional by exporting JSON/CSV first.

## Verification

- Baseline fixtures cover no-baseline, healthy baseline, noisy baseline, and
  drift cases.
- Override decisions persist with accepted risk IDs and rationale.
- Trend export is deterministic and redaction-compatible.

## Exit Criteria

Reviewers see whether a finding is exceptional for this repo instead of a raw
global score with no context.
