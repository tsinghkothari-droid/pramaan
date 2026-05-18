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

Run from the repository root:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/base -p "test_*.py"
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

The first command is expected to fail. The second command is expected to pass,
which is the point of the demo.
