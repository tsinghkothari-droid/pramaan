# Phase 35.5 Summary: Reviewer UX and Local HTML Report

## Status

Completed 2026-05-21 as `PASS_WITH_RISKS`.

## What Landed

- Added `pramaan report markdown --bundle <path> [--out <path>]`.
- Added `pramaan report html --bundle <path> --out <report.html>`.
- Reports use the same reviewer-first hierarchy:
  - Blockers
  - Warnings
  - What Ran
  - What Skipped
  - What Changed In Tests
  - Replay Commands
  - Human Override
- Oracle-integrity evidence appears before lower-signal replay and integrity
  details.
- Replay guidance is included when fuzz/property or mutation stages appear.
- Human override fields are explicit: accepted risks, reason, reviewer identity
  source, and timestamp.
- GitHub Action summary now mirrors the same blocker/warning/ran/skipped/override
  structure.
- Added `docs/reviewer-ux.md` and static pass/warn/fail report examples under
  `examples/reports/`.
- Added a CLI smoke test for markdown and HTML reports.

## Verification

Full phase-close verification was run before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`

## Deferred

- The HTML report is intentionally static and local; hosted dashboards remain
  future work.
- The HTML renderer wraps the canonical markdown evidence for v0.1 rather than
  introducing a separate templating system.
