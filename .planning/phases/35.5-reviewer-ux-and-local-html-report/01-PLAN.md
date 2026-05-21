# Phase 35.5: Reviewer UX and Local HTML Report

## Goal

Make a proof bundle understandable in under 30 seconds for both humans and
agents.

## Reviewer Shape

The first screen should answer:

```text
Blockers
Warnings
What Ran
What Skipped
What Changed In Tests
Replay Commands
Human Override
```

No dashboard-first detour. Start with a static local report and GitHub PR
summary improvements.

## Files To Change

- `crates/pramaan-cli/src/main.rs`
- `action/render-summary.mjs`
- `docs/reviewer-ux.md`
- `examples/reports/`
- `assets/`

## Implementation Steps

1. Add `pramaan report html --bundle <path> --out <report.html>`.
2. Add `pramaan report markdown --bundle <path>` for PR comment rendering.
3. Group findings by blocker, warning, skipped, and replayable case.
4. Surface oracle diffs before lower-signal evidence.
5. Include copyable replay commands for fuzz/property/mutation failures.
6. Include human override form fields or markdown template:
   accepted risk IDs, reason, reviewer identity source, timestamp.
7. Add visual examples for pass/warn/fail states.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `node --test action/render-summary.test.mjs`
- HTML snapshot smoke test confirms blocker/warning/replay sections render.
- Manual check: weakened-test demo report is understandable in under 30 seconds.

## Exit Criteria

Pramaan has a reviewer-facing artifact that sells the product without requiring
a hosted dashboard.
