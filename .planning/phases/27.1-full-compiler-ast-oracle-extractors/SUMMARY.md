# Phase 27.1 Summary: Parser Metadata and Full-AST Decision

Date: 2026-05-21

Status: completed with explicit residual risk

## What Landed

- Added parser metadata fields to every extracted oracle test:
  `parser_version`, `fallback_reason`, `unsupported_syntax`, and
  `disagreement_count`.
- Added tests proving parser metadata is present and that the full-AST fallback
  reason remains visible.
- Added `scripts/check-oracle-parser-metadata.mjs` to validate generated
  `oracle-diff.json` files.
- Added metadata fixtures for decorated/parametrized Python tests, TypeScript
  computed names, and Rust macro-generated tests under
  `examples/oracle-integrity/full-parser-metadata-fixtures/`.
- Added `docs/oracle-parser-decision.md` explaining why CPython `ast`,
  TypeScript compiler API, and rust-analyzer/syn paths remain deferred until
  subprocess and dependency costs are justified.
- Updated `docs/oracle-integrity.md`, `STATUS.md`, `TASKS.md`,
  `.planning/ROADMAP.md`, `.planning/STATE.md`, and `docs/claim-audit.md`.

## Evidence

- Unit coverage: `cargo test -p pramaan-core oracle_fixture`
- Metadata validator:
  `node scripts/check-oracle-parser-metadata.mjs target/pramaan-minimum-lovable/oracle-diff.json`

## Deferred

- This phase does not ship true compiler AST extraction.
- Python `ast`, TypeScript compiler API, and Rust analyzer/syn integration are
  deferred to a heavier follow-up because replacing the current subset requires
  pinned subprocess/runtime behavior and parity fixtures.
- Public docs still mark full compiler AST extraction as planned.

## Self-Check

- [x] Parser version evidence exists.
- [x] Unsupported syntax is explicit.
- [x] Fallback reason says full AST is not enabled.
- [x] Claim audit keeps full compiler AST extraction narrowed.
- [x] No public copy relabels the subset as complete compiler AST proof.
