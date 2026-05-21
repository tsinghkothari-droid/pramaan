# Phase 27: Parser-Backed Oracle Extractors

## Goal

Replace the remaining structured-text oracle extractor risk with parser-backed
Python, TypeScript, and Rust oracle evidence.

## Research Drivers

- Test-oracle weakening is Pramaan's strongest killer use case.
- Research on AI coding regressions keeps pointing to hidden or inadequate
  tests as a major failure mode.

## Tasks Covered

- Full compiler/parser AST integrations for Python, TypeScript, and Rust.
- Golden negative fixtures for parser edge cases.
- Dependency justifications for any parser libraries or subprocess tools.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `examples/oracle-integrity/`
- `docs/oracle-integrity.md`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Choose parser strategy for each language: standard library or subprocess
   first, new crates only with justification.
2. Parse base/head test files and extract assertion, skip, ignore, xfail,
   snapshot, and fixture references.
3. Preserve existing risk IDs and receipt shapes while adding parser evidence.
4. Add negative fixtures for comments, strings, multiline assertions, macros,
   generated tests, and renamed test bodies.
5. Compare parser results with current structured extractor output and mark
   disagreements explicitly.

## Verification

- Golden fixtures cover Python, TypeScript, and Rust parser positives and
  negatives.
- `cargo test --workspace` passes.
- Claim audit labels full parser-backed oracle support accurately.

## Exit Criteria

Pramaan can honestly claim parser-backed oracle integrity for the supported
language subset, while unsupported syntax remains visible residual risk.
