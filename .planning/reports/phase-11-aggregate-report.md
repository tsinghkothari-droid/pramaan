# Phase 11 Aggregate Report

## Status

PASS_WITH_RISKS

## Commit

`76b73ac` - `Phase 11: deepen sandbox claim and static evidence`

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `node --test action\render-summary.test.mjs`
- `cargo run -q -- verify --base HEAD --head HEAD --out target/pramaan/phase11-verify` with PR metadata, image metadata, and network policy environment variables

## Completed Tasks

- Captured richer sandbox environment evidence.
- Recorded base/head commit IDs and dirty/untracked state.
- Hashed manifests and lockfiles.
- Detected lockfile drift and mapped it to dependency-drift risk.
- Captured supplied image name/digest and network policy evidence.
- Parsed PR title/body from GitHub context or environment.
- Captured linked issue references.
- Detected changed public API entries for Python, TypeScript/TSX, and Rust through deterministic scans.
- Added configured `pyright` and `cargo clippy` receipts.
- Expanded static hallucination categories.
- Kept missing tools visible through skipped receipts.

## Residual Risks

- Network policy is not enforced.
- Linked issue text is not ingested.
- Public API extraction is not AST-complete.
- Relaxed static-check config detection remains open.
- Enterprise redaction is still needed before broad bundle export.

## Next Action

Execute Phase 12: Oracle Integrity Engine.
