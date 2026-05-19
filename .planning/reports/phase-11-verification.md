# Phase 11 Verification

## Commands

```powershell
cargo fmt --check
cargo test --workspace
node --test action\render-summary.test.mjs
```

Result: PASS.

```powershell
$env:PRAMAAN_IMAGE_NAME='ghcr.io/pramaan/dev:phase11'
$env:PRAMAAN_IMAGE_DIGEST='sha256:phase11'
$env:PRAMAAN_NETWORK_POLICY='disabled'
$env:PRAMAAN_PR_TITLE='Phase 11 sandbox claim static depth'
$env:PRAMAAN_PR_BODY='Fixes #11. Capture environment, claim scope, public APIs, pyright and clippy receipts.'
cargo run -q -- verify --base HEAD --head HEAD --out target/pramaan/phase11-verify
```

Result: PASS.

## Evidence Checked

- Claim scope contained PR title/body evidence and linked issue reference `#11`.
- Claim scope no longer retained the old synthetic public API placeholder when no changed API was detected.
- Sandbox evidence contained image name, image digest, network policy, source dirty state, toolchain versions, and Git blob lockfile digests.
- Static smoke tests required `broken_import,nonexistent_import` for the Rust hallucinated import fixture.
- Bundle manifest generation succeeded with relative artifact paths.

## Residual Verification Risks

- The verification run occurred while Phase 11 files were dirty, so sandbox evidence correctly reported a non-hermetic local checkout.
- No live GitHub Actions run was executed in this phase.
- The public API scanner is line-oriented and should be replaced or supplemented with AST extraction in a later hardening pass.
