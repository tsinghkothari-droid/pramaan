# Adversarial Corpus

Pramaan keeps public adversarial scenarios in `corpus/` so demos and future evaluations can reuse the same failure-mode IDs and risk mappings.

The starter corpus manifest is:

```text
corpus/starter-adversarial-scenarios.json
```

## Scenario Index

| ID | Failure mode | Status | Risk mapping |
| --- | --- | --- | --- |
| `ADV-001` | Weakened assertion | Implemented demo | `R-011`, `R-014`, `R-087`, `R-100` |
| `ADV-002` | Skipped test | Starter spec | `R-010`, `R-012`, `R-087`, `R-100` |
| `ADV-003` | Invented import | Starter spec | `R-038`, `R-039`, `R-040`, `R-100` |
| `ADV-004` | Mutation survivor | Starter spec | `R-068`, `R-071`, `R-072`, `R-100` |
| `ADV-005` | Unexpected differential divergence | Starter spec | `R-073`, `R-075`, `R-080`, `R-100` |

## Implemented Demo

`ADV-001` is implemented in `examples/vulnerable-python-pr/`.

Run ordinary CI on the weakened PR:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

Run Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

Inspect:

```text
target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json
target/pramaan-demo/oracle/oracle-diff.json
examples/vulnerable-python-pr/risk-map.json
corpus/starter-adversarial-scenarios.json
```

The local receipt should fail at `oracle_integrity`; the diff should name a `weakened_assertion` finding; the risk-map and corpus manifest should both point to stable `R-...` IDs.

## Corpus Rules

- Keep scenario IDs stable after publication.
- Add executable fixture paths when a starter spec becomes runnable.
- Keep risk mappings conservative and grounded in `.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md`.
- Do not remove old scenarios when detection improves; mark them as implemented, superseded, or retained for regression coverage.
