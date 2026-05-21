# Phase 17 Machine Verification

Status: passed on 2026-05-21.

Required commands:

```powershell
node scripts/check-phase17-policy-ci-eval.mjs
node scripts/check-verifier-abuse-fixtures.mjs
node scripts/check-adversarial-corpus.mjs
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```

This phase is `PASS_WITH_RISKS`: policy, CI, VSA, redaction, and corpus
taxonomy evidence exists, but empirical benchmark reporting and non-GitHub
execution remain future work.

Result:

- `node scripts/check-phase17-policy-ci-eval.mjs` passed.
- `node scripts/check-verifier-abuse-fixtures.mjs` passed.
- `node scripts/check-adversarial-corpus.mjs` passed.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace -- -D warnings` passed.
- `node scripts/check-claim-audit.mjs` passed.
- `node --test action/render-summary.test.mjs` passed.
