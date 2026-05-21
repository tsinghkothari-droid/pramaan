# Phase 26.3 Competitor-Gap Fixture Report

Date: 2026-05-21

This report turns the Phase 26.2 benchmark into concrete fixture categories.
It deliberately compares against adjacent tool categories rather than making
brittle claims about one vendor's latest implementation.

## Fixture Set

| Fixture | Adjacent category | Ordinary surface | Pramaan signal | Decision |
| --- | --- | --- | --- | --- |
| `CG-001` weakened assertion | Review assistant | Passing tests can hide a weaker assertion. | Failed oracle receipt. | fail |
| `CG-002` added skip marker | Test-change monitor | CI passes because the regression is disabled. | Failed oracle receipt. | fail |
| `CG-003` fixture/snapshot drift | CI quality aggregator | Dashboard shows green tests while expected artifacts changed. | Sensitive artifact finding. | fail |
| `CG-004` hallucinated import | Review assistant | Plausible code depends on nonexistent API. | Static receipt with hallucination risk. | fail |
| `CG-005` false-green CI | CI status check | Unit-test check succeeds, but required evidence is absent. | Policy/confidence residual risk. | warn |
| `CG-006` unsigned aggregate report | CI quality aggregator | Summary exists without receipt provenance. | Bundle digest/tamper model required. | warn |
| `CG-007` hidden skipped stage | Status dashboard | Skipped mutation/fuzz looks green. | Skipped required evidence cannot improve confidence. | fail |

## Validation

Run:

```powershell
node scripts/check-competitor-gap-fixtures.mjs
```

The validator checks fixture count, unique IDs, non-empty reviewer-facing fields,
safe repo-relative fixture paths, risk ID shapes, required adjacent categories,
and required gap categories.

Executable demo commands remain stored in
`corpus/competitor-gap-fixtures.v0.1.json`. Metadata-only fixtures are not proof
that Pramaan detected a live bug; they are risk-mapped scenarios that should be
promoted to executable fixtures as the verifier deepens.

## Public Claim Boundary

Phase 26.3 supports this claim:

> Pramaan has a checked competitor-gap fixture corpus describing the evidence
> gaps it targets across adjacent tool categories.

It does not yet support this stronger claim:

> Pramaan empirically outperforms every named review assistant or quality tool.

That stronger claim would require running named tools under pinned versions on
the same fixture set and publishing the results.
