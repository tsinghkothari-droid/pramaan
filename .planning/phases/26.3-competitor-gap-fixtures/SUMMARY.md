# Phase 26.3 Summary: Competitor-Gap Fixtures

Date: 2026-05-21

Status: completed with residual risk

## What Landed

- Added `corpus/competitor-gap-fixtures.v0.1.json` with seven category-level
  scenarios: weakened assertion, added skip marker, fixture/snapshot drift,
  hallucinated import, false-green CI, unsigned aggregate report, and hidden
  skipped verification stage.
- Added `examples/competitor-gaps/` fixtures, including a runnable skipped-test
  base/head pair plus metadata fixtures for green CI without required evidence,
  unsigned aggregate reports, and hidden skipped stages.
- Added `scripts/check-competitor-gap-fixtures.mjs` to validate fixture count,
  unique IDs, required categories, safe repo-relative fixture paths, expected
  decisions, and risk ID shapes.
- Updated `docs/competitive-benchmark.md`, `docs/demo.md`,
  `docs/risk-taxonomy.md`, `corpus/README.md`, `TASKS.md`,
  `.planning/ROADMAP.md`, `.planning/STATE.md`, and `docs/claim-audit.md`.
- Added `.planning/reports/phase-26.3-competitor-gap-fixtures.md` to keep the
  public claim boundary explicit.

## Evidence

- Validator: `node scripts/check-competitor-gap-fixtures.mjs`
- Manifest: `corpus/competitor-gap-fixtures.v0.1.json`
- Added runnable oracle fixture:
  `examples/competitor-gaps/skipped-test/base` ->
  `examples/competitor-gaps/skipped-test/head`

## Deferred

- Metadata fixtures should be promoted into fully executable demos over time.
- This phase does not benchmark named tools under pinned versions. It compares
  adjacent tool categories and describes Pramaan's evidence gap.
- Phase 26.4 must package the loop into one command, one report, and one
  reviewer-friendly bundle.

## Self-Check

- [x] Every fixture maps to at least one stable `R-...` risk ID.
- [x] Duplicate or unmapped fixtures fail validation.
- [x] Public language avoids "Pramaan outperforms every named tool" claims.
- [x] Tracking docs updated honestly.
