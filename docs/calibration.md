# Calibration and Drift

Pramaan's confidence vote is intentionally not a correctness proof. Phase 34
adds local feedback artifacts so teams can see whether a finding is unusual for
their repository and whether the confidence model is becoming overconfident.

## Baseline File

Start with a repo-local JSON baseline:

```powershell
cargo run -p pramaan-cli -- feedback analyze `
  --bundle target/pramaan `
  --baseline examples/fixtures/repo-baseline.synthetic.json `
  --observations examples/fixtures/calibration-observations.synthetic.json `
  --out target/pramaan-feedback
```

The baseline records the repository noise floor for mutation survivors, oracle
residual risks, skipped stages, static findings, runtime p95, and confidence
floor. Treat the checked-in fixture as a contract example, not a real baseline.

## Outputs

`feedback analyze` writes:

- `feedback-report.json`: machine-readable metrics, drift warnings, and
  calibration metrics.
- `feedback-metrics.csv`: dashboard-ready rows that do not require a hosted
  dashboard.

Calibration uses supplied labeled outcomes only. With no labels, the report
uses `no_labeled_outcomes`. With labels, Pramaan reports Brier score, log loss,
expected calibration error, and simple reliability buckets.

## Honest Interpretation

- A high confidence score is still evidence for review prioritization, not merge
  authorization.
- Drift warnings mean "unusual for this repo," not "definitely wrong."
- Reviewer overrides are allowed, but they must be recorded as evidence with
  accepted risk IDs and rationale.
