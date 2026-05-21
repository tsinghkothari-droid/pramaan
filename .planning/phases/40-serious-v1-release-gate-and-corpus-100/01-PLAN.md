# Phase 40: Serious v1 Release Gate and Corpus 100

## Goal

Make an evidence-based Serious v1 decision after the core product has real
pilots, signed/verifiable bundles, redaction, plugin security, calibration, and
100 adversarial scenarios.

## Research Drivers

- Benchmark contamination and long-horizon coding benchmarks make release
  claims dangerous unless backed by fresh, diverse evaluation evidence.
- Serious v1 should mean repeatable trust evidence, not feature volume.

## Tasks Covered

- 100+ adversarial scenarios mapped to risk IDs.
- Benchmark-integrity mutation harness.
- Cross-platform CI.
- Final claim audit.
- Serious v1 go/no-go report.

## Files to Change

- `corpus/`
- `.github/workflows/`
- `.planning/reports/`
- `docs/claim-audit.md`
- `STATUS.md`
- `README.md`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Expand the corpus from 25 to 75, then to 100+ only with non-duplicate risk
   coverage.
2. Add benchmark-integrity checks for overfit fixtures and stale expected
   outputs.
3. Run cross-platform CI for supported binaries and core fixtures.
4. Re-run the claim audit against README, STATUS, docs, schemas, examples, and
   Action behavior.
5. Produce Serious v1 release report with open risks, unsupported claims,
   runtime data, false-positive/false-negative notes, and adoption guidance.

## Verification

- Corpus runner validates 100+ mapped scenarios.
- Cross-platform CI is green or residual failures are accepted explicitly.
- Claim audit has zero false-or-stale public claims.
- Serious v1 report names go, no-go, or conditional release.

## Exit Criteria

Pramaan either ships Serious v1 with honest evidence or refuses to ship until
the remaining trust gaps are closed.
