# Phase 17 Summary - Policy, CI Hardening, and Evaluation Intelligence

Status: `PASS_WITH_RISKS`

Completed on: 2026-05-21

## What Landed

- Verified policy profiles, `pramaan policy explain`, SARIF/Rego export paths,
  CI hardening signals, SLA stage-budget behavior, VSA schemas, redaction
  profiles, and security-sensitive policy escalation.
- Added `docs/benchmark-report-template.md` for future pilot/corpus reports.
- Added `docs/benchmark-integrity.md` to define the anti-overfitting harness
  without claiming it is implemented.
- Added `scripts/check-phase17-policy-ci-eval.mjs` so the phase closure is
  repeatable.

## Deferred / Residual Risk

- External policy-file loading and full OPA parity testing remain future work.
- GitLab/Gitea/Bitbucket execution remains future work.
- Benchmark-integrity mutation is designed but not implemented.
- 75/100+ adversarial corpus scale and measured false-positive/false-negative
  reports remain Phase 40 work.

## Verification

- `node scripts/check-phase17-policy-ci-eval.mjs`
- `node scripts/check-verifier-abuse-fixtures.mjs`
- `node scripts/check-adversarial-corpus.mjs`
- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
