# Phase 28.5: Auditable Confidence Vote and Calibration Schema

## Goal

Add an auditable confidence-vote artifact that aggregates Pramaan receipts into
a decomposed risk decision without pretending to prove code correctness.

## Research Drivers

- Weak supervision systems such as Snorkel combine overlapping, conflicting,
  and abstaining signals into probabilistic labels instead of using naive
  majority vote.
- Dawid-Skene style aggregation treats voters as noisy sources with different
  reliability.
- Wilson score intervals reduce fake confidence in small mutation samples.
- The rule of three gives an honest residual-risk bound when fuzz/property
  campaigns find zero failures.
- SLSA VSA shows that verifier decisions should be policy-bound and auditable.

## Tasks Covered

- `pramaan-confidence` module or equivalent core subsystem.
- `schemas/confidence.schema.json`.
- `confidence.json` and `confidence.md` artifacts.
- Hard-gate rules that cannot be averaged away.
- Weak-signal aggregation over receipts.
- Dependency discounts for correlated stages.
- Statistical intervals for mutation and fuzz/property evidence.
- Calibration metadata for Phase 34.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-bundle/`
- `crates/pramaan-cli/`
- `schemas/confidence.schema.json`
- `docs/confidence.md`
- `examples/`
- `TASKS.md`
- `.planning/STATE.md`

## Algorithm Shape

1. Convert each stage receipt into a vote:
   - `risky`
   - `safe`
   - `abstain`
2. Apply hard gates before scoring:
   - weakened/deleted tests
   - skip/ignore additions
   - bundle tamper
   - invalid signature or attestation
   - untrusted plugin
   - unsupported critical evidence path
3. Aggregate non-gated evidence with deterministic weights:
   - stage reliability
   - evidence strength
   - dependency discount
   - skipped-stage uncertainty penalty
4. Add statistical evidence:
   - Wilson lower bound for mutation kill confidence
   - rule-of-three residual-risk upper bound for zero-failure fuzz/property runs
5. Emit:
   - decision: `fail`, `warn`, or `pass`
   - confidence score
   - risk probability
   - top risk drivers
   - top confidence drivers
   - skipped/abstained evidence
   - calibration dataset and method metadata

## Verification

- Golden fixtures cover hard fail, warning, pass, skipped-tool uncertainty,
  contradictory evidence, correlated evidence, and small-sample mutation cases.
- Confidence artifacts are deterministic under stable receipt inputs.
- Public docs state that the confidence vote is risk evidence, not proof of
  correctness.
- Phase 29 can sign or attest the confidence artifact digest.

## Exit Criteria

Pramaan can show reviewers why a PR received a confidence vote, which evidence
drove it, which evidence was missing, and why the score should or should not be
trusted.
