# Pramaan Adversarial Corpus

This directory contains public demo and evaluation scenarios for AI-code trust failures that ordinary CI can miss.

The starter manifest is:

```text
corpus/starter-adversarial-scenarios.json
```

Each scenario records:

- a stable scenario ID;
- the failure mode;
- implementation status;
- demo or fixture paths when they already exist;
- the command Pramaan should run when the scenario is executable;
- expected Pramaan signal;
- risk IDs mapped back to the top-100 register.

## Starter Scenarios

| Scenario | Failure mode | Status | Primary risk IDs |
| --- | --- | --- | --- |
| `ADV-001` | Weakened assertion | Implemented demo | `R-011`, `R-014`, `R-087`, `R-100` |
| `ADV-002` | Skipped test | Starter spec | `R-010`, `R-012`, `R-087`, `R-100` |
| `ADV-003` | Invented import | Starter spec | `R-038`, `R-039`, `R-040`, `R-100` |
| `ADV-004` | Mutation survivor | Starter spec | `R-068`, `R-071`, `R-072`, `R-100` |
| `ADV-005` | Unexpected differential divergence | Starter spec | `R-073`, `R-075`, `R-080`, `R-100` |

`ADV-001` is the public Python demo under `examples/vulnerable-python-pr/`. The other four entries are intentionally starter specs so future phases can add executable fixtures without changing the scenario IDs.

## Public Demo Path

Run normal CI on the implemented vulnerable PR:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

Then run Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

The inspection path for reviewers is:

```text
target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json
target/pramaan-demo/oracle/oracle-diff.json
examples/vulnerable-python-pr/risk-map.json
corpus/starter-adversarial-scenarios.json
```
