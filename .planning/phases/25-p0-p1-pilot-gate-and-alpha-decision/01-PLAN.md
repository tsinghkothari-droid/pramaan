# Phase 25: P0/P1 Pilot Gate and Alpha Decision

## Goal

Prove whether the P0/P1 loop is actually useful before expanding into P2/P3
features.

## P0/P1 Tasks Covered

- Evaluate Alpha MVP gates.
- Run Pramaan on at least three selected real repositories.
- Measure runtime, skipped stages, false positives, false negatives, and reviewer time-to-understand.
- Produce unresolved-risk and go/no-go reports.

## Files to Change

- `.planning/STATE.md`
- `.planning/ROADMAP.md`
- `.planning/research/`
- `docs/demo.md`
- `docs/github-action.md`
- `TASKS.md`

## Implementation Steps

1. Select three pilot repositories: one Python, one TypeScript, one Rust where possible.
2. Run the P0/P1 Pramaan loop and collect bundles.
3. Record runtime and skipped-stage profiles.
4. Review findings for false positives/false negatives.
5. Evaluate Alpha MVP release gates.
6. Decide whether to proceed to P2 signing/attestation expansion, repeat P0/P1 hardening, or pivot.

## Verification

- Pilot report includes commands, bundle paths, metrics, and reviewer-facing summaries.
- All remaining P0/P1 tasks are either completed, split into new scoped phases, or accepted as explicit alpha residual risk.

## Exit Criteria

The project has an honest Alpha MVP decision instead of drifting into endless
research or feature expansion.
