# Demo - CI Green, Pramaan Red

Pramaan's public demo set is deliberately small and brutal. Each scenario shows
ordinary review or CI looking acceptable while Pramaan emits concrete evidence
that the result is not trustworthy.

For the shortest end-to-end path, use the quickstart loop:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/run-minimum-lovable-loop.ps1
```

It writes a bundle plus `minimum-lovable-report.md` under
`target/pramaan-minimum-lovable/`.

## Demo 1: Weakened Test Assertion

The checkout function should apply a 10 percent discount:

```text
discounted_total(10000, 10) == 9000
```

The implementation validates the discount but returns the original subtotal. In
the base branch, the regression test catches that bug. In the AI-style PR branch,
the code is still wrong, but the test changes from an exact value check to a
positive-value check.

## Commands

Run from the repository root.

Original failing oracle:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/base -p "test_*.py"
```

Expected result: fails with `10000 != 9000`.

Ordinary CI on the weakened PR:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

Expected result: passes, even though the bug is still present.

Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

Expected result: oracle integrity completes with findings and writes a failed
receipt at:

```text
target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json
```

The checked-in expected receipt is:

```text
examples/vulnerable-python-pr/expected-oracle-integrity.receipt.json
```

Use the checked-in receipt as the stable public-demo expectation. Use the
generated receipt as the local proof that the current engine still finds the
weakened assertion.

## 30-Second Proof Bundle Inspection

After running the Pramaan command above, inspect these files:

| File | What to check |
| --- | --- |
| `target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json` | `stage` is `oracle_integrity`, `status` is `failed`, and `residual_risks` contains the finding risk IDs. |
| `target/pramaan-demo/oracle/oracle-diff.json` | The finding kind is `weakened_assertion` for `test_applies_percentage_discount`; discovered tests include extractor labels and assertion-signal hashes. |
| `examples/vulnerable-python-pr/risk-map.json` | The public demo maps normal CI green and Pramaan red to stable risk IDs. |
| `corpus/starter-adversarial-scenarios.json` | `ADV-001` links the implemented demo to the broader adversarial corpus. |

If the local receipt path is absent, rerun the Pramaan oracle command. If the
normal CI command fails, the demo no longer proves the intended "CI green,
Pramaan red" contrast.

## Risk Map

| Risk ID | Demo mapping |
| --- | --- |
| `R-010` | The oracle stage must inspect test-control changes such as skips, xfails, and todos. This scenario does not add a skip, so the expected receipt keeps it residual while still documenting the guardrail. |
| `R-011` | The regression assertion is weakened from exact discounted output to a broad positivity assertion. |
| `R-014` | The new test checks only a truthy/non-zero style property and misses the wrong value. |
| `R-087` | Code and test are edited together, so passing CI cannot distinguish a real fix from oracle tampering. |
| `R-100` | The scenario is now a reusable adversarial corpus entry under `examples/vulnerable-python-pr/`. |

## Reviewer Read

The proof is visible in two files:

- `examples/vulnerable-python-pr/base/test_checkout.py`
- `examples/vulnerable-python-pr/weakened-test/test_checkout.py`

The production code is intentionally identical and still wrong in both branches.
Only the test oracle changes.

The adversarial corpus index lives in
`corpus/starter-adversarial-scenarios.json`, with a reviewer guide in
`docs/adversarial-corpus.md`.

## Demo 2: Snapshot and Fixture Drift

This demo shows a PR where the test still passes because the expected artifact
was changed.

Ordinary CI:

```powershell
python -m unittest discover -s examples/snapshot-fixture-drift-pr/head -p "test_*.py"
```

Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/snapshot-fixture-drift-pr/base --head-repo examples/snapshot-fixture-drift-pr/head --out target/pramaan-demo/snapshot-fixture-drift
```

Inspect:

```text
target/pramaan-demo/snapshot-fixture-drift/receipts/oracle-integrity.receipt.json
target/pramaan-demo/snapshot-fixture-drift/oracle-diff.json
examples/snapshot-fixture-drift-pr/risk-map.json
```

Expected result: normal CI passes, while Pramaan reports
`sensitive_artifact_changed` findings for fixture and snapshot files.

## Demo 3: Static Hallucination

This demo shows a generated Rust patch that imports a plausible but nonexistent
helper crate.

Pramaan static checks:

```powershell
cargo run -p pramaan-cli -- static-checks --repo examples/hallucinated-rust-pr --out target/pramaan-demo/hallucinated-rust
```

Inspect:

```text
target/pramaan-demo/hallucinated-rust/receipts/static/rust-cargo-check.receipt.json
examples/hallucinated-rust-pr/risk-map.json
```

Expected result: Pramaan emits a failed static receipt with hallucination-style
evidence for unresolved imports or missing dependencies.

## Checked-In Example Outputs

Generated example outputs for the public demos live under:

```text
examples/proof-bundles/
```

These are not claims that the demo code is correct. They are stable artifacts
showing the evidence shape a reviewer should inspect.

## Competitor-Gap Fixtures

Phase 26.3 adds category-level fixtures for places where ordinary review
surfaces can look acceptable while Pramaan should still fail or warn:

```text
corpus/competitor-gap-fixtures.v0.1.json
examples/competitor-gaps/
```

Validate the fixture manifest:

```powershell
node scripts/check-competitor-gap-fixtures.mjs
```

Run the added skipped-test oracle fixture:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/competitor-gaps/skipped-test/base --head-repo examples/competitor-gaps/skipped-test/head --out target/pramaan-gap/skipped-test
```

Expected result: Pramaan reports oracle risk for the newly skipped regression
test. Metadata-only competitor-gap fixtures are not proof of named-tool
performance; they are checked risk scenarios that should be promoted into
executable demos as the verifier matures.

## External Pilot Evidence

Phase 26 ran Pramaan locally against public Python, TypeScript, and Rust
repositories and recorded runtime, skipped-tool, noisy-finding, and residual
risk notes in:

```text
.planning/reports/phase-26-external-alpha-pilots.md
```

Those pilots are evidence that Pramaan can inspect real repositories, not proof
that public Alpha is ready. The live GitHub Action proof remains a separate
Phase 26.1 gate.
