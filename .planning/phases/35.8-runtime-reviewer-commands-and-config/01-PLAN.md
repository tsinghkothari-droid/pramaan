# Phase 35.8: Runtime Reviewer Commands and Config

## Goal

Turn the Phase 35.6 reviewer-interface contract into a small runtime slice:
local config loading plus a diagnostic command reviewers can run before trying
Pramaan in a repository.

## Scope

- Add `pramaan doctor`.
- Add `.pramaan.toml` loading for `pramaan verify`.
- Support stage skips, mutation enablement, fuzz seed, policy/redaction labels,
  and configured local reports.
- Add smoke tests and docs.

## Out Of Scope

- `pramaan verify-pr --url`.
- Persistent forge comment updates.
- External policy-file loading.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `node scripts/check-phase29-35-runtime-gap-slices.mjs`
