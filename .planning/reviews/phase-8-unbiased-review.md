# Phase 8 Unbiased Review

## Verdict

PASS_WITH_RISKS

## Roadmap Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Standalone weakened assertion demo | PASS | `examples/vulnerable-python-pr/`, `docs/demo.md`, `examples/proof-bundles/weakened-assertion/` |
| Snapshot/fixture drift demo | PASS | `examples/snapshot-fixture-drift-pr/`, `examples/proof-bundles/snapshot-fixture-drift/` |
| Static hallucination demo | PASS | `examples/hallucinated-rust-pr/`, `examples/proof-bundles/hallucinated-rust/` |
| Example Pramaan outputs for all three demos | PASS_WITH_RISKS | `examples/proof-bundles/`; outputs are stage-specific receipt directories |
| 30-second reviewer walkthrough | PASS | `docs/demo.md` |
| Corpus entries with risk mappings | PASS | `corpus/starter-adversarial-scenarios.json`, `corpus/README.md`, `docs/adversarial-corpus.md` |

## What Was Built

- Added a snapshot/fixture drift adversarial demo where normal Python tests pass after fixture and snapshot updates, but Pramaan flags sensitive artifact changes.
- Added a Rust hallucination demo where generated code imports an undeclared crate/API and Pramaan classifies it as `broken_import` with residual risk `R-038`.
- Checked in example stage evidence for weakened assertion, snapshot/fixture drift, and hallucinated Rust demos.
- Updated public demo docs, corpus docs, and P0 task status.

## Tests and Stress Tests

- `cargo fmt --check`: PASS.
- `$env:CARGO_TARGET_DIR='target/pramaan-phase8-test'; cargo test --workspace`: PASS, 26 tests.
- `node --test action\render-summary.test.mjs`: PASS, 3 tests.
- Demo Python CI checks: PASS for weakened assertion and snapshot/fixture drift.
- Pramaan demo checks: PASS command execution with expected failed receipts.
- Bundle clean verify plus tamper verify: PASS; tampered artifact failed digest verification.
- Markdown link check, JSON parse check, and corpus path check: PASS.

## Evidence For Completion

- The three public demo failure modes are now represented by real repo fixtures and checked-in evidence outputs.
- The demo docs explain the reviewer workflow without claiming correctness proof.
- The adversarial corpus maps each implemented demo to stable risk IDs.
- Tamper verification confirms that current bundle integrity checks fail on modified artifacts.

## Evidence Against Completion

- The example outputs are not full signed proof bundles with manifest-backed attestations for every demo stage.
- There is no automated CI job yet that regenerates and verifies these demo outputs on every PR.
- The copied example receipts include machine-local path evidence and timestamps.

## Missing Tests

- No golden-file test currently asserts that every checked-in proof-bundle example remains structurally valid.
- No CI workflow yet runs all three demo scenarios as a public demo regression suite.
- No cross-platform check has validated these examples on Linux and Windows.

## Security and Trust Risks

- The demo outputs should not be marketed as CI-backed provenance until later attestation phases are complete.
- Local path evidence in example receipts may reveal developer machine paths if copied directly into public bundles.
- Static hallucination classification can drift if compiler diagnostic wording changes.

## False Confidence Risks

- A reviewer may read `examples/proof-bundles/` as cryptographically complete proof bundles. The README now clarifies this, but the name is still stronger than the current implementation.
- Passing demo fixtures do not prove broad oracle integrity coverage; they only prove the selected adversarial examples are detected.

## Files Changed

- `TASKS.md`
- `corpus/README.md`
- `corpus/starter-adversarial-scenarios.json`
- `docs/adversarial-corpus.md`
- `docs/demo.md`
- `examples/hallucinated-rust-pr/`
- `examples/proof-bundles/`
- `examples/snapshot-fixture-drift-pr/`
- `.planning/reports/phase-8-execution-brief.md`
- `.planning/reports/phase-8-workstreams.md`
- `.planning/reports/phase-8-verification.md`

## Commit

42111924bb74c6761765861fb171f77b2edf8b10

## Next Action

Execute Phase 16a before Phase 9 so schema-impact attribution, override, provenance, plugin identity, redaction, policy decision, and stage-budget hooks land before the receipt schema freezes.
