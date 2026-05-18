# Vulnerable Python PR Demo

This demo is intentionally small:

- `base/` contains the original regression test. It fails because the checkout
  code ignores the discount.
- `weakened-test/` contains the AI-style PR. The production bug is still
  present, but the test assertion was weakened from an exact discounted total to
  a truthy/non-negative check, so ordinary CI passes.
- `expected-oracle-integrity.receipt.json` documents the Pramaan behavior
  expected from the Phase 3 oracle engine: fail the PR and name the weakened
  assertion.
- `risk-map.json` maps the public demo to stable risk IDs.

Run from the repository root:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/base -p "test_*.py"
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

The first command is expected to fail. The second command is expected to pass,
which is the point of the demo.

Run Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

The generated proof bundle inspection path is:

```text
target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json
target/pramaan-demo/oracle/oracle-diff.json
```

For the public corpus index, see `corpus/starter-adversarial-scenarios.json`.
