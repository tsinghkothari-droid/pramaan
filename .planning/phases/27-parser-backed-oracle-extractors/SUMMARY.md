# Phase 27 Execution Summary

Date: 2026-05-21

## Landed

- Hardened Python, TypeScript, and Rust oracle extractors into parser-backed
  subsets with explicit `*_v2` engine labels.
- Added comment/string filtering before skip-marker and assertion detection.
- Added multiline assertion grouping for Python assertions, TypeScript chained
  matchers, and Rust assertion macros.
- Added deleted-test boundary/error signal findings so deleting a boundary test
  is not only reported as a generic deletion.
- Added parser-negative fixtures for comments, strings, and multiline
  assertions under `examples/oracle-integrity/`.
- Added `docs/oracle-integrity.md` and updated receipt/risk docs plus claim
  audit wording.

## Split

Full compiler AST-backed extractor support did not land in this phase. It is
split to Phase 27.1 with parser dependency and runtime justification required.
Public docs continue to call the current implementation a parser-backed subset,
not a full compiler AST proof.

## Verification

Completed before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
