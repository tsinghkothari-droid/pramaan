# Confidence Vote

Pramaan's confidence vote is a reviewer-facing summary of receipt evidence. It
does not prove that a pull request is correct. It answers a narrower question:
"given the receipts we have, which risks are still visible, which evidence is
missing, and should a reviewer fail, warn, or continue?"

## Artifact

`pramaan confidence explain <bundle>` writes:

- `confidence.json`: schema-versioned machine-readable evidence using
  `pramaan.confidence.v1`.
- `confidence.md`: the same decision in reviewer-readable form.
- `receipts/confidence-vote.receipt.json`: a receipt that links the two
  artifacts so the bundle manifest can hash and later sign them.

The command rebuilds `bundle.manifest.json` when writing into a bundle
directory, so Phase 29 can sign or attest the confidence artifact digests.

## Algorithm v0.1

The initial algorithm is `pramaan-confidence-v0.1-uncalibrated`. It uses
deterministic starter weights until Phase 34 has enough labeled outcomes for
calibration.

1. Convert every non-confidence receipt into a vote: `safe`, `risky`, or
   `abstain`.
2. Apply hard gates before score aggregation.
3. Group receipts into dependency clusters such as `test_quality`,
   `supply_chain`, `static_semantic`, and `scope`.
4. Discount later votes from the same cluster so oracle, mutation, and
   property/fuzz evidence do not get counted as independent proof.
5. Penalize skipped and not-applicable stages as uncertainty.
6. Add statistical notes where receipts expose enough metadata.

## Hard Gates

Hard gates dominate the final decision and cannot be averaged away. v0.1
detects:

- failed oracle-integrity receipts, including weakened/deleted/skipped tests
  and sensitive fixture churn;
- failed bundle-integrity or attestation-style receipts;
- required attestation/signature metadata that is explicitly invalid, failed,
  missing-required, or tied to an untrusted identity;
- unsupported or unexecuted critical evidence paths such as required
  property/fuzz execution for a high-risk diff;
- explicitly untrusted plugin provenance;
- stages that exhausted their maximum evidence budget.

If a hard gate fires, the decision is `fail` even if other stages look clean.

## Statistical Notes

Mutation evidence records a Wilson 95 percent lower bound for kill rate when
`mutants_killed` and `mutants_total` metadata are present. This prevents a tiny
sample from looking stronger than it is.

Property/fuzz evidence records a rule-of-three residual-risk upper bound when a
zero-failure run includes `generated_input_count`. A clean fuzz run only bounds
the generated input distribution; it is not evidence that every input is safe.

## Calibration Status

All v0.1 confidence artifacts are marked `uncalibrated`. They are useful for
audit and review prioritization, but they are not merge authority. Phase 34 is
responsible for replacing or validating starter weights using reviewer
overrides, labeled outcomes, Brier score, log loss, and reliability diagrams.

## Reviewer Rule

Treat `pass` as "no hard gate and enough current receipt evidence to continue
normal review." Treat `warn` as "review the missing or weak evidence before
merging." Treat `fail` as "do not merge until the hard-gated evidence is fixed
or explicitly overridden with a recorded human reason."
