# Plan 01 Summary - Oracle Diff Engine

## Completed

- Added a core oracle snapshot and diff engine for Python and TypeScript/JavaScript test files.
- Implemented stable per-test fingerprints, test discovery, deleted-test detection, skip/xfail/todo detection, parametrized case reduction detection, and assertion weakening heuristics.
- Classified fixtures and snapshots as oracle-sensitive artifacts and flagged changed or deleted artifacts.
- Added `pramaan oracle --base-repo <path> --head-repo <path> --out <path>` with an `oracle_integrity` receipt and `oracle-diff.json` artifact.
- Mapped oracle receipts to mitigated risk coverage `R-004` through `R-020` and `R-087` through `R-089`, with residual risks populated from findings.
- Added Python and TypeScript oracle plugin notes plus fixture repositories under `examples/fixtures/oracle/`.

## Verification

- `cargo test`
- `cargo run -p pramaan-cli -- oracle --base-repo examples\fixtures\oracle\base --head-repo examples\fixtures\oracle\head --out target\pramaan-oracle-manual`

The manual fixture run produced a failed oracle receipt with deleted test, added skip/todo, weakened assertion, parametrized case reduction, fixture change, and snapshot change findings.

## Notes

- The current engine is intentionally heuristic and deterministic. It is suitable for receipts and review gating, while future parser-backed adapters can replace individual language scanners without changing the receipt contract.
- This plan did not modify `examples/vulnerable-*` or `docs/demo.md`.
