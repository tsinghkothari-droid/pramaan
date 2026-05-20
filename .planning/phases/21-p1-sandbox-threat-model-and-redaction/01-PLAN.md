# Phase 21: P1 Sandbox, Threat Model, and Redaction

## Goal

Treat the verifier itself as an attack surface and harden the evidence boundary
around untrusted PR code.

## P1 Tasks Covered

- Auto-detect OCI/container identity when CI does not provide it explicitly.
- Detect source changes after a stage runs.
- Threat-model malicious PR authors and compromised tools.
- Add PII/secrets scrubbing rules.
- Add CI hardening checks for untrusted PR execution.

## Files to Change

- `crates/pramaan-sandbox/src/lib.rs`
- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `docs/threat-model.md`
- `docs/github-action.md`
- `docs/receipt-model.md`
- `schemas/receipt.schema.json`

## Implementation Steps

1. Add best-effort OCI/container identity detection from common CI/runtime metadata.
2. Add stage source-state fingerprints and dirty-after-run detection.
3. Add redaction profile model for environment evidence, logs, paths, endpoints, and CI metadata.
4. Add GitHub workflow hardening checks for token permissions, `pull_request_target`, unpinned actions, cache poisoning, artifact retention, and self-hosted runners.
5. Document what Pramaan can and cannot protect when the runner is compromised.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Redaction fixture tests for secret-like values and private paths.

## Exit Criteria

Bundles become safer to share, and unsafe CI/verifier conditions are visible
instead of invisible assumptions.
