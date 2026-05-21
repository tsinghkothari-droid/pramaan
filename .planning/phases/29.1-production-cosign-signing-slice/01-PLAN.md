# Phase 29.1: Production Cosign Signing Slice

## Goal

Close the first concrete gap between local/offline Phase 29 attestations and a
future production Sigstore/cosign identity flow without claiming production
identity proof.

## Scope

- Add a runtime command that inspects a bundle and emits a cosign signing
  readiness plan.
- Record manifest digest, cosign availability, suggested signing command, and
  residual risks.
- Document that this is not a production CI identity proof.
- Add smoke tests and a validation script.

## Out Of Scope

- Live OIDC signing.
- Fulcio/Rekor certificate verification.
- Failing CI on absent production signature.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `node scripts/check-phase29-35-runtime-gap-slices.mjs`
