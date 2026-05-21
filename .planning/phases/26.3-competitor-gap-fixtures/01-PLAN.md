# Phase 26.3: Competitor-Gap Fixtures

## Goal

Convert "Pramaan catches what adjacent tools miss" into executable fixtures
with expected receipts, risk IDs, and policy decisions.

## Why This Phase Exists

Comparisons are weak unless they are backed by scenarios. The best demo is not
"we reviewed the diff better." It is a reproducible case where a normal PR
review or ordinary CI looks green while Pramaan shows exactly what evidence was
weakened, skipped, unsigned, or missing.

## Fixture Categories

- Weakened assertion that ordinary CI still passes.
- Added skip/ignore marker hiding a broken test.
- Fixture or snapshot churn that blesses wrong behavior.
- Hallucinated API/import that a prose reviewer may miss.
- False-green CI with missing required evidence.
- Unsigned aggregate report with no receipt-level provenance.
- Hidden skipped verification stage presented as success.

## Files To Change

- `examples/`
- `corpus/`
- `docs/competitive-benchmark.md`
- `docs/demo.md`
- `docs/risk-taxonomy.md`
- `scripts/`
- `.planning/reports/`

## Implementation Steps

1. Inventory existing demos and reuse them where they already match a gap.
2. Add missing fixtures with base/head or before/after examples.
3. Define expected Pramaan receipts and risk IDs for every fixture.
4. Add a competitor-gap report that maps fixture category to adjacent tool
   category, not brittle claims about one project's current implementation.
5. Add validation that fixtures cannot go stale without being noticed.

## Verification

- Fixture runner produces expected pass/warn/fail decisions.
- Every fixture maps to at least one stable risk ID.
- Duplicate or unmapped fixtures fail validation.

## Exit Criteria

Pramaan can demonstrate its differentiated value with executable evidence,
not just roadmap breadth.
