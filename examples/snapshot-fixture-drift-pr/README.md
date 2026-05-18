# Snapshot and Fixture Drift Demo

This demo shows a subtler oracle attack than a deleted or weakened assertion.
The tests still look meaningful and ordinary CI passes, but the expected
behavior is silently redefined by changing fixture and snapshot artifacts.

## Scenario

The order summary should preserve the approved total and status label. The
base branch and head branch both run the same style of test, but the head branch
changes:

- `fixtures/order.json`
- `tests/__snapshots__/order.snap`

Pramaan treats both files as oracle-sensitive artifacts.

## Ordinary CI

```powershell
python -m unittest discover -s examples/snapshot-fixture-drift-pr/head -p "test_*.py"
```

Expected result: passes.

## Pramaan

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/snapshot-fixture-drift-pr/base --head-repo examples/snapshot-fixture-drift-pr/head --out target/pramaan-demo/snapshot-fixture-drift
```

Expected result: `oracle_integrity` fails with `sensitive_artifact_changed`
findings for the fixture and snapshot.

