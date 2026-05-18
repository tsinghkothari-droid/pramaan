# Plan 02 Summary - Differential Property and Fuzz Adapters

## Completed

- Added shared differential fuzz evidence types in `crates/pramaan-core/src/lib.rs`:
  - safe and unsafe discovery models;
  - adapter mode model;
  - seed, corpus hash, replay path, example database path, counterexample path;
  - divergence records with `expected`, `unexpected`, and `needs_review` classification;
  - property/fuzz risk helpers for R-073 through R-080.
- Added `pramaan fuzz` in `crates/pramaan-cli/src/fuzz.rs` and wired it through `crates/pramaan-cli/src/main.rs`.
- Implemented conservative pure-function discovery for Python and TypeScript/JavaScript:
  - accepts single-return integer arithmetic functions;
  - marks calls, attribute access, imports, I/O, async/yield, globals, containers, empty parameters, and complex bodies unsafe;
  - emits `not_applicable` receipts when safe discovery is unavailable.
- Implemented deterministic simulated differential execution with the same receipt fields expected from Hypothesis/fast-check adapters when those tools are not wired into a target project.
- Added claim-scope divergence classification:
  - expected when the function matches expected behavior or touched public API scope;
  - unexpected when it diverges outside a scoped public API claim or matches out-of-scope behavior;
  - needs review when no claim scope is available.
- Added replay and corpus artifacts:
  - `fuzz-corpus.json`;
  - `fuzz-replay.json`;
  - `counterexamples.json` when divergences exist;
  - `differential-fuzz.json`;
  - `receipts/differential-fuzz.receipt.json`.
- Added plugin adapter documentation:
  - `plugins/python/fuzz/README.md`;
  - `plugins/typescript/fuzz/README.md`.
- Added fixtures under `examples/fixtures/fuzz/`:
  - Python expected change plus unexpected regression;
  - TypeScript fast-check-compatible fixture;
  - unsafe/not-applicable discovery fixture;
  - claim-scope fixture files.

## Verification

- `cargo test -p pramaan-cli fuzz` passed.
- `cargo test -p pramaan-core fuzz_receipt_types_preserve_replay_and_classification_fields` passed.

## Notes

- Full `cargo test` was run and the fuzz tests passed, but the suite currently fails in mutation-only coverage outside Plan 02 ownership:
  - `mutation_emits_diff_scoped_receipts_with_budget_metadata`
  - `mutation_normalizers_preserve_counts_and_risk_family`
- The mutation failures appear related to concurrent Phase 4 Plan 01 work and were not changed as part of this Plan 02 slice.
