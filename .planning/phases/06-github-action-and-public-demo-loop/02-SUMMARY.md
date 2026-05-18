# Plan 02 Summary - Public Demo and Adversarial Corpus

## Scope

Executed only Plan 02 for Phase 6. Work stayed within the public demo and corpus-owned files:

- `examples/vulnerable-python-pr/**`
- `corpus/**`
- `docs/demo.md`
- `docs/adversarial-corpus.md`
- `.planning/phases/06-github-action-and-public-demo-loop/02-SUMMARY.md`

No GitHub Action files were edited.

## Completed

- Finalized the weakened-test public demo docs with normal CI green and Pramaan oracle red commands.
- Added `corpus/starter-adversarial-scenarios.json` with five starter adversarial scenarios:
  - `ADV-001`: weakened assertion
  - `ADV-002`: skipped test
  - `ADV-003`: invented import
  - `ADV-004`: mutation survivor
  - `ADV-005`: unexpected differential divergence
- Added `corpus/README.md` for corpus conventions and quick demo commands.
- Added `docs/adversarial-corpus.md` with the scenario index and corpus maintenance rules.
- Documented the demo proof bundle inspection path:
  - `target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json`
  - `target/pramaan-demo/oracle/oracle-diff.json`
  - `examples/vulnerable-python-pr/risk-map.json`
  - `corpus/starter-adversarial-scenarios.json`

## Verification

Verified:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/base -p "test_*.py"
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

Results:

- the base regression test fails as expected with `10000 != 9000`;
- normal CI passes for the weakened PR;
- Pramaan oracle integrity emits one `weakened_assertion` finding for `test_applies_percentage_discount`;
- the generated receipt at `target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json` has `stage=oracle_integrity`, `status=failed`, and populated risk buckets;
- `corpus/starter-adversarial-scenarios.json` parses successfully and contains all five starter scenarios.
