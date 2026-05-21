# Phase 26: External Alpha Pilots and Live Action Proof

## Goal

Run Pramaan on real repositories before public Alpha and prove that the GitHub
Action produces useful, reviewable evidence on a live PR.

## Research Drivers

- SWE-Lancer and SWE-Bench Pro show that real-world coding tasks fail in ways
  small synthetic fixtures miss.
- GitHub artifact-attestation and CI-hardening docs make the live Action path
  part of the product, not packaging polish.

## Tasks Covered

- Public Alpha blocker: run three external repositories.
- Public Alpha blocker: prove the GitHub Action on a live PR.
- Track runtime, skipped stages, false positives, false negatives, and reviewer
  time-to-understand.

## Files to Change

- `.planning/research/`
- `.planning/reports/`
- `docs/demo.md`
- `docs/github-action.md`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Select one Python, one TypeScript, and one Rust repository with ordinary CI.
2. Run Pramaan locally against representative PR-style diffs.
3. Open or simulate a real GitHub PR and run the composite Action.
4. Save bundles, Action summaries, screenshots or logs, and reviewer notes.
5. Record stage runtime, skipped-stage profile, observed false positives, known
   false negatives, and time-to-understand.
6. Update the Alpha decision with go/no-go evidence.

## Verification

- Pilot report includes repo URLs or local paths, commands, bundle paths,
  runtime, policy result, and reviewer-facing summary.
- At least one live Action run produces an uploaded bundle and PR summary.
- Any missing tools or unsupported language features are recorded as residual
  risk, not pass evidence.

## Exit Criteria

Public Alpha either remains blocked with measured reasons or proceeds with
external evidence instead of internal fixture confidence.
