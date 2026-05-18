# Phase 8 Verification

## Verdict

PASS_WITH_RISKS

Phase 8 meets the demo success criteria: ordinary demo tests pass while Pramaan emits failed receipts for weakened assertions, snapshot/fixture drift, and a hallucinated Rust import/API. The residual risk is that the checked-in outputs under `examples/proof-bundles/` are stage-specific receipt directories, not full CI-attested signed bundles.

## Commands Run

| Check | Command | Result |
| --- | --- | --- |
| Weakened assertion normal CI | `python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"` | PASS, 1 test |
| Snapshot/fixture normal CI | `python -m unittest discover -s examples/snapshot-fixture-drift-pr/head -p "test_*.py"` | PASS, 1 test |
| Weakened assertion Pramaan oracle | `cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/weakened-assertion` | PASS command; failed oracle receipt emitted |
| Snapshot/fixture Pramaan oracle | `cargo run -p pramaan-cli -- oracle --base-repo examples/snapshot-fixture-drift-pr/base --head-repo examples/snapshot-fixture-drift-pr/head --out target/pramaan-demo/snapshot-fixture-drift` | PASS command; failed oracle receipt emitted |
| Hallucinated Rust Pramaan static check | `cargo run -p pramaan-cli -- static-checks --repo examples/hallucinated-rust-pr --out target/pramaan-demo/hallucinated-rust` | PASS command; failed static receipt emitted |
| Demo receipt assertions | PowerShell assertions over `target/pramaan-demo/*` receipts and artifacts | PASS |
| Bundle clean verification | `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-phase8/synthetic-bundle`; `cargo run -p pramaan-cli -- bundle verify target/pramaan-phase8/synthetic-bundle` | PASS |
| Bundle tamper gate | Modified `claim_scope.synthetic.json`, then ran `cargo run -p pramaan-cli -- bundle verify target/pramaan-phase8/synthetic-bundle-tamper` | PASS; verification failed with digest mismatch |
| Rust formatting | `cargo fmt --check` | PASS |
| Rust workspace tests | `$env:CARGO_TARGET_DIR='target/pramaan-phase8-test'; cargo test --workspace` | PASS, 26 tests |
| GitHub Action summary tests | `node --test action\render-summary.test.mjs` | PASS, 3 tests |
| JSON parsing | Parsed corpus and new demo risk maps with `ConvertFrom-Json` | PASS |
| Markdown links | Checked repo-local links in README, TASKS, planning, corpus, and demo docs | PASS |
| Corpus demo paths | Validated all `implemented_demo` `demo_paths` exist | PASS, 3 implemented demos |

## Demo Evidence

- `examples/proof-bundles/weakened-assertion/receipts/oracle-integrity.receipt.json` records a failed oracle-integrity stage.
- `examples/proof-bundles/weakened-assertion/oracle-diff.json` records a `weakened_assertion` finding.
- `examples/proof-bundles/snapshot-fixture-drift/oracle-diff.json` records `sensitive_artifact_changed` findings for `fixtures/order.json` and `tests/__snapshots__/order.snap`.
- `examples/proof-bundles/hallucinated-rust/receipts/static/rust-cargo-check.receipt.json` records `status: failed`, `broken_import`, and residual risk `R-038`.

## Residual Risks

- Stage-specific demo commands do not yet emit full signed bundle manifests.
- Example receipts include timestamps and local path evidence, so they should be treated as evidence-shape examples rather than deterministic golden fixtures.
- The hallucinated Rust classification depends on cargo diagnostics from the local Rust toolchain.
