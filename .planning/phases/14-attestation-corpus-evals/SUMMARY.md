# Phase 14 Summary - Attestation, Corpus, and Evals

Status: `PASS_WITH_RISKS`

Completed on: 2026-05-21

## What Landed

- Verified that Pramaan has local/offline VSA-style attestation output and an
  in-toto statement wrapper through `pramaan bundle attest`.
- Verified that `pramaan bundle verify-offline` checks manifest integrity,
  VSA result consistency, manifest digest, and confidence artifact references.
- Verified that the GitHub Action can request artifact attestation and emits
  local/offline VSA material before upload.
- Verified that the maintained adversarial corpus has 25 risk-mapped scenarios
  with implemented demos and scenario specs.
- Added `scripts/check-phase14-attestation-corpus-evidence.mjs` so this phase
  can be rechecked without hand inspection.

## Deferred / Residual Risk

- Production Sigstore/cosign keyless signing is still planned, not shipped.
- GitHub artifact attestations are wired as an Action path, but the repo still
  treats GitHub identity verification as separate from local VSA consistency.
- The corpus has 25 scenarios, not 75 or 100+.
- Real-world replay cases and measured false-positive/false-negative benchmark
  reports remain future work.

## Verification

- `node scripts/check-phase14-attestation-corpus-evidence.mjs`
- `node scripts/check-adversarial-corpus.mjs`
- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
