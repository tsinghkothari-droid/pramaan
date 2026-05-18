# Plan 01 Summary - Diff-Scoped Mutation Adapters

## Scope

Implemented Phase 4 Plan 01 mutation work only. Fuzz-owned files were not edited.

## Completed

- Added shared mutation normalization types/helpers in `crates/pramaan-core/src/lib.rs`.
- Added `pramaan mutation` CLI wiring in `crates/pramaan-cli/src/main.rs`.
- Added mutation adapter implementation in `crates/pramaan-cli/src/mutation.rs`.
- Added adapter documentation:
  - `plugins/python/mutation/README.md`
  - `plugins/typescript/mutation/README.md`
  - `plugins/rust/mutation/README.md`
- Added mutation fixture projects under `examples/fixtures/mutation/`.
- Added smoke coverage for mutation receipt emission in `crates/pramaan-cli/tests/smoke.rs`.

## Adapter Behavior

- Python adapter targets changed `.py` files through `mutmut run --paths-to-mutate`.
- TypeScript adapter targets changed JS/TS files through StrykerJS `--mutate` with incremental mode requested.
- Rust adapter targets changed `.rs` files through `cargo mutants --file` when `cargo-mutants` is available.
- All adapters emit receipts when tools are missing or not applicable, preserving skipped counts and reason metadata.
- Receipts record normalized `killed`, `survived`, `timed_out`, `unviable`, and `skipped` counts.
- Receipts record timeout, filter mode, coverage-filter note, cache/incremental mode, changed-test awareness, kill threshold, survivor classification counts, and R-068..R-072 metadata.

## Verification

- `cargo test -p pramaan-core` passed: 8 tests passed.
- `cargo test -p pramaan-cli --test smoke mutation_emits_diff_scoped_receipts_with_budget_metadata` passed before a parallel fuzz edit changed the CLI compile state.
- Full `cargo test` is currently blocked by an out-of-scope compile error in `crates/pramaan-cli/src/fuzz.rs`: missing `BTreeSet` import for the parallel fuzz worker's code path.

## Notes

- Skipped/not-applicable mutation receipts intentionally keep uncovered mutation risks visible instead of claiming mitigation from a tool that did not run.
- The mutation fixture is designed to validate adapter discovery and receipt shape even when mutmut, StrykerJS, and cargo-mutants are not installed.
