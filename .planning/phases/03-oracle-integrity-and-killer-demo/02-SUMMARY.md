# Plan 02 Summary - Killer Demo Repository

## Completed

- Added a self-contained Python vulnerable PR demo under
  `examples/vulnerable-python-pr/`.
- Created a base branch fixture where the original regression test fails because
  `discounted_total(10000, 10)` returns `10000` instead of `9000`.
- Created an AI-style weakened-test branch where the production bug remains but
  ordinary CI passes because the assertion only checks a positive result.
- Added `expected-oracle-integrity.receipt.json` to document the Phase 3 oracle
  behavior expected from the engine worker: fail oracle integrity and name the
  weakened assertion.
- Added `risk-map.json` and `docs/demo.md` mapping the demo to `R-010`, `R-011`,
  `R-014`, `R-087`, and `R-100`.

## Validation

- `python -m unittest discover -s examples/vulnerable-python-pr/base -p "test_*.py"`
  failed as expected with `10000 != 9000`.
- `python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"`
  passed, demonstrating ordinary CI green on the weakened PR.

## Notes

- TypeScript mirror was not added because the Python demo fully satisfies the
  Plan 02 requirement and keeps the owned surface small.
- No core, CLI, schema, or plugin engine files were modified. The expected
  Pramaan failure is documented as a receipt fixture because another worker owns
  oracle-engine implementation.
- Git was not touched.
