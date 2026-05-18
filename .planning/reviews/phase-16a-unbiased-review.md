# Phase 16a Unbiased Review

## Verdict

PASS_WITH_RISKS

## Roadmap Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Agent-author attribution hook exists | PASS | `crates/pramaan-core/src/lib.rs`, `schemas/receipt.schema.json`, generated claim receipt |
| Reviewer override hook exists | PASS_WITH_RISKS | Struct/schema/fixture exist; no reviewer workflow yet |
| Multi-agent provenance hook exists | PASS_WITH_RISKS | Struct/schema/fixture exist; no commit-chain validation yet |
| Plugin identity and permissions hooks exist | PASS_WITH_RISKS | Struct/schema/fixture exist; permissions are not enforced yet |
| Redaction manifest and sensitivity hooks exist | PASS_WITH_RISKS | Struct/schema/fixture exist; no scrubber proof yet |
| Policy decision hook exists | PASS_WITH_RISKS | Struct/schema/generated evidence exist; no policy engine yet |
| Stage budget hook exists | PASS_WITH_RISKS | Struct/schema/generated evidence exist; no SLA enforcement yet |

## What Was Built

- Added optional trust hooks to receipt structs.
- Added bundle and stage manifest aggregation for those hooks.
- Added non-empty generated trust hooks in `pramaan verify` claim-scope receipts.
- Added fixture coverage and tests proving serialization and aggregation.
- Updated public schemas so Phase 9 can freeze the contract without retrofitting these fields later.

## Tests and Stress Tests

- `cargo fmt --check`: PASS.
- `cargo test --workspace`: PASS, 27 tests.
- `node --test action\render-summary.test.mjs`: PASS, 3 tests.
- JSON parse check for changed schemas and fixtures: PASS.
- Generated bundle hook assertion: PASS.
- Trust-hook receipt tamper gate: PASS; digest mismatch detected.

## Evidence For Completion

- At least one real generated receipt path emits non-empty trust-hook evidence.
- Bundle manifest aggregation preserves agent attribution, plugin identity, redaction profile, policy decision, and stage budget.
- Tampering with a trust-hook-bearing receipt is detected by bundle verification.

## Evidence Against Completion

- This is not schema-freeze-ready. Runtime receipt JSON and public receipt schema still have older-vs-richer contract mismatch.
- There is no JSON Schema validation harness for all generated receipts.
- Plugin permissions and redaction are declarative only.
- Local bundle verification remains integrity evidence, not signed provenance.

## Missing Tests

- Generated receipt validation against JSON Schema.
- Negative schema fixtures for malformed trust hooks.
- Redaction leak tests for secrets, private paths, and internal endpoints.
- Malicious plugin permission tests.
- Manifest recomputation and signing-metadata tamper tests.

## Security and Trust Risks

- A malicious plugin can still claim permissions honestly without enforcement.
- A malicious actor who can rewrite a whole local bundle can still recompute local integrity metadata until signing/attestation verification is hardened.
- Redaction profiles can claim a secret was removed without a scrubber proving it.

## False Confidence Risks

- Seeing `agent_author` or `policy_decision` in a receipt may imply a stronger verified identity or policy engine than currently exists.
- The hooks should be described as schema hooks until Phase 9/16/17 implement validation and enforcement.

## Files Changed

- `crates/pramaan-core/src/lib.rs`
- `crates/pramaan-bundle/src/lib.rs`
- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-cli/src/oracle.rs`
- `crates/pramaan-cli/src/static_checks.rs`
- `crates/pramaan-cli/src/mutation.rs`
- `crates/pramaan-cli/src/fuzz.rs`
- `crates/pramaan-cli/tests/smoke.rs`
- `schemas/receipt.schema.json`
- `schemas/bundle.schema.json`
- `examples/fixtures/receipt.synthetic.json`
- `examples/fixtures/bundle.synthetic.json`

## Commit

ba0ab70821eb463583d04c6f625fc51ee39f2ad9

## Next Action

Proceed to Phase 9, but treat schema compatibility and generated JSON Schema validation as hard blockers before declaring receipt schema v0.1 frozen.
