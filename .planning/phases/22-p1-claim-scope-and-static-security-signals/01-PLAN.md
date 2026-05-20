# Phase 22: P1 Claim Scope and Static Security Signals

## Goal

Improve the bridge between what the PR claims, what changed, and which
static/security risks should raise gate severity.

## P1 Tasks Covered

- Ingest linked issue text when available.
- Add low-confidence warnings for vague or missing PR descriptions.
- Allow maintainer scope notes.
- Map claim-scope warnings to stable risk IDs.
- Add bounded semantic claim-implementation mismatch detection.
- Detect relaxed static-check configuration.
- Classify security-sensitive diffs.

## Files to Change

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-cli/src/static_checks.rs`
- `crates/pramaan-core/src/lib.rs`
- `schemas/claim_scope.schema.json`
- `docs/receipt-model.md`
- `docs/risk-taxonomy.md`

## Implementation Steps

1. Add optional issue/context ingestion from environment or file input.
2. Add `.pramaan-scope.md` or equivalent scope-note support.
3. Emit stable risk IDs for vague, missing, overbroad, and low-confidence claim scope.
4. Detect changes that relax mypy/pyright/tsconfig/eslint/clippy or similar static gates.
5. Add security-sensitive diff classifier for auth, authorization, crypto, SQL/query construction, subprocess, filesystem, deserialization, secrets, network, and permissions.
6. Keep semantic mismatch as advisory evidence unless policy elevates it.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Fixture receipts for vague claims, scope notes, relaxed config, and security-sensitive diffs.

## Exit Criteria

Claim scope and static risk receipts become useful enough for policy gates
without pretending to know developer intent perfectly.
