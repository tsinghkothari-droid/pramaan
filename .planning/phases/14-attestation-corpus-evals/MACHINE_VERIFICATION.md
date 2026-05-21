# Phase 14 Machine Verification

Status: passed on 2026-05-21.

Required commands:

```powershell
node scripts/check-phase14-attestation-corpus-evidence.mjs
node scripts/check-adversarial-corpus.mjs
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```

The phase is only a `PASS_WITH_RISKS`: local/offline VSA and in-toto evidence
are implemented, but production Sigstore/cosign identity and the 100+ scenario
corpus remain open.

Result:

- `node scripts/check-phase14-attestation-corpus-evidence.mjs` passed.
- `node scripts/check-adversarial-corpus.mjs` passed with 25 scenarios.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace -- -D warnings` passed.
- `node scripts/check-claim-audit.mjs` passed.
- `node --test action/render-summary.test.mjs` passed.
