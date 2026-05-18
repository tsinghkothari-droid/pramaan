# Demo - CI Green, Pramaan Red

The first Pramaan demo is a tiny Python PR where ordinary CI passes only because
the assertion was weakened.

## Scenario

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

Expected Pramaan behavior once the Phase 3 oracle engine is wired:

```powershell
cargo run -p pramaan-cli -- verify --base examples/vulnerable-python-pr/base --head examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo
```

Expected result: oracle integrity fails and writes a receipt equivalent to
`examples/vulnerable-python-pr/expected-oracle-integrity.receipt.json`.

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
