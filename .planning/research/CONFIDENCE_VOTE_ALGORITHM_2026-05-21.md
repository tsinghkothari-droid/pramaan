# Confidence Vote Algorithm Research - 2026-05-21

Purpose: improve Pramaan's GSD phases with a mathematically defensible,
auditable confidence vote for AI-authored pull request evidence.

## Recommendation

Add Phase 28.5 between real property/fuzz replay and attestation. The
confidence vote must be built from receipts, then signed or attested in Phase
29. It should be decomposed and auditable, never a magic percentage.

## Sources to Copy From

| Source | Link | Pramaan use |
| --- | --- | --- |
| Snorkel LabelModel | https://snorkel.readthedocs.io/en/v0.9.6/packages/_autosummary/labeling/snorkel.labeling.model.label_model.LabelModel.html | Treat stages as noisy labeling functions that can vote risky, safe, or abstain. |
| Snorkel weak supervision paper | https://link.springer.com/article/10.1007/s00778-019-00552-1 | Combine overlapping and conflicting weak signals into probabilistic labels instead of naive majority vote. |
| Crowd-Kit Dawid-Skene aggregation | https://crowd-kit.readthedocs.io/en/latest/classification/ | Model stage voters as noisy sources with different reliability. |
| scikit-learn calibration docs | https://scikit-learn.org/stable/modules/calibration.html | Evaluate probability quality with Brier score, log loss, and reliability diagrams. |
| Wilson score interval | https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval | Report mutation confidence with small-sample-safe intervals. |
| Rule of three | https://en.wikipedia.org/wiki/Rule_of_three_(statistics) | Report upper bound on unseen failure rate after zero fuzz/property failures. |
| SLSA VSA | https://slsa.dev/spec/v1.2/verification_summary | Bind verifier decisions to policy and attestable artifact digests. |
| OpenSSF Scorecard | https://github.com/ossf/scorecard | Borrow decomposed check scoring and caveats, not a single opaque score. |

## Algorithm v0.1

Hard gates run first. If any hard gate fires, the confidence artifact records a
failed decision even if other stages look clean.

Hard gates:

- weakened or deleted tests
- new skip/ignore markers
- bundle tamper
- invalid signature or attestation
- untrusted plugin
- unsupported critical evidence path

Non-gated evidence uses a deterministic weighted vote:

```text
risk_logit =
  repo_prior_risk
  + sum(stage_weight * reliability * evidence_strength * dependency_discount)
  + skipped_stage_penalty

risk_probability = sigmoid(risk_logit)
confidence = 1 - risk_probability
```

Each stage emits one of:

- `risky`
- `safe`
- `abstain`

Dependency clusters prevent double-counting:

- test quality: oracle, mutation, property/fuzz
- supply chain: sandbox, signing, attestation
- semantic review: claim scope, optional critic
- static correctness: typecheck, lint, import binding

## Statistical Evidence

Mutation:

```text
mutation_score = killed_mutants / valid_mutants
mutation_confidence = WilsonLower(killed_mutants, valid_mutants, 95%)
```

Fuzz/property with zero failures:

```text
residual_failure_rate_upper_bound_95 = 3 / generated_cases
```

Calibration after pilots:

- Brier score
- log loss
- reliability diagram
- expected calibration error

## Artifact Shape

`confidence.json` should include:

- algorithm version
- policy version
- decision
- confidence
- risk probability
- hard gates triggered
- stage votes
- stage weights
- dependency clusters
- statistical intervals
- skipped/abstained evidence
- top risk drivers
- top confidence drivers
- calibration dataset and method
- residual-risk explanation

`confidence.md` should explain the same result in reviewer language.

## Product Claim

Use:

> Pramaan computes a calibrated, auditable evidence vote from verification
> receipts and records exactly how that vote was produced.

Do not use:

> Pramaan proves this PR is correct.
