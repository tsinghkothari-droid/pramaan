# Phase 32.5: Policy Pack Library and Enterprise Profiles

## Goal

Turn Pramaan policy from one default profile into selectable policy packs for
different teams and risk environments.

## Policy Packs

- `startup-fast`: fast feedback, warnings for optional depth.
- `open-source-maintainer`: visible skipped tools, public-safe summaries.
- `security-sensitive`: hard gates for auth, crypto, SQL, deserialization,
  secrets, subprocess, network, permissions, and workflow changes.
- `fintech-strict`: stricter mutation/fuzz/security budgets and override
  capture.
- `private-preview`: honest residual-risk mode for early pilots.

## Files To Change

- `crates/pramaan-core/src/lib.rs`
- `crates/pramaan-cli/src/main.rs`
- `schemas/policy_profile.schema.json`
- `policy/`
- `docs/policy-packs.md`
- `docs/github-action.md`

## Implementation Steps

1. Add built-in policy profile loading by ID.
2. Add `pramaan policy list` and `pramaan policy explain --profile <id>`.
3. Add policy profile schema and checked-in policy fixtures.
4. Add hard-gate parity tests so Rust policy output and exported OPA/Rego
   policy behavior match for representative bundles.
5. Make GitHub Action accept a `policy-profile` input.
6. Document which policies are appropriate for pilots, private repos, public
   OSS, and high-risk security-sensitive code.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Policy fixture tests for pass, warn, fail, waiver, and security-sensitive
  path escalation.

## Exit Criteria

Teams can adopt Pramaan with a policy that matches their risk tolerance without
editing code.
