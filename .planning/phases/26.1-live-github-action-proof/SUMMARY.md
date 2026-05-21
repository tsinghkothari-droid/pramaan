# Phase 26.1 Summary: Live GitHub Action Proof

## Landed

- Ran the live `Pramaan` GitHub Actions workflow on `main`.
- Captured the successful run URL, job URL, uploaded artifact ID, artifact
  digest, rendered summary, and downloaded proof bundle.
- Fixed Action-manifest YAML parsing for the attestation input description.
- Fixed composite Action helper script paths for local `uses: ./` execution.
- Fixed receipt artifact declarations so command labels are not treated as
  files and oracle/fuzz/mutation evidence paths are bundle-relative.
- Added `.planning/reports/phase-26.1-live-action-proof.md`.

## Live Evidence

- Run URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652
- Job URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652/job/77187017437
- Artifact URL: https://github.com/tsinghkothari-droid/pramaan/actions/runs/26229890652/artifacts/7137454243
- Downloaded artifact copy: `.planning/reports/phase-26.1-live-action-artifact/`
- Final status: `inconclusive`
- Policy decision: `warning`

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- live GitHub Actions run `26229890652`

## Deferred

- A pull-request event demo remains useful for Phase 26.4 / public-review
  readiness.
- Production Sigstore/cosign identity remains future hardening.

## Residual Risk

The live proof confirms the Action can build Pramaan, run verification, upload
the bundle, and render residual risk. It does not make Pramaan production-ready
or prove correctness of the repository.
