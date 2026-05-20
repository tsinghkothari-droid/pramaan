# Phase 23: P1 AST Oracle Extractors

## Goal

Replace the highest-risk heuristic oracle checks with AST-backed extractors and
golden fixtures.

## P1 Tasks Covered

- Replace heuristic oracle scanning with AST-backed Python, TypeScript, and Rust extractors.
- Preserve existing deterministic oracle behavior.
- Add golden fixtures for each supported weakening pattern.

## Files to Change

- `crates/pramaan-cli/src/oracle.rs`
- `crates/pramaan-core/src/lib.rs`
- `examples/oracle-integrity/**`
- `docs/demo.md`
- `docs/risk-taxonomy.md`

## Implementation Steps

1. Define a language-neutral oracle model: test identity, assertion strength, skip markers, expected error/boundary cases, fixture/snapshot sensitivity.
2. Add Python AST-backed extraction for pytest patterns.
3. Add TypeScript parser-backed extraction for Jest/Vitest patterns.
4. Add Rust parser-backed extraction for assertions, panic tests, and `#[ignore]`.
5. Keep heuristic fallback with an honest evidence label when a parser is unavailable.
6. Add positive and negative golden fixtures per language.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Oracle fixture command demonstrates weakened-test detection.

## Exit Criteria

Oracle receipts are precise enough that the weakened-test demo remains strong
while false positives are easier to inspect.
