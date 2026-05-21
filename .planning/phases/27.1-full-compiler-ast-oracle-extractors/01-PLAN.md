# Phase 27.1: Full Compiler AST Oracle Extractors

## Goal

Replace the Phase 27 parser-backed subset with full compiler/parser AST
integrations where the dependency and runtime costs are justified.

## Why This Split Exists

Phase 27 hardened Pramaan's deterministic parser subset and added negative
fixtures for comments, strings, and multiline assertions. It did not honestly
ship full compiler AST support for Python, TypeScript, and Rust. That requires
language-specific parser dependencies or subprocess tools, fixture coverage, and
dependency justifications.

## Tasks

1. Evaluate parser choices:
   - Python: standard-library `ast` subprocess or embedded parser strategy.
   - TypeScript: TypeScript compiler API subprocess or a Rust parser with
     pinned version.
   - Rust: `syn`/`ra_ap_syntax`/rust-analyzer path with dependency and runtime
     justification.
2. Add golden fixtures for comments, strings, generated tests, multiline
   assertions, macros, renamed bodies, decorators/attributes, and skipped tests.
3. Record parser version, fallback reason, unsupported syntax, and disagreement
   counts in oracle evidence.
4. Keep existing risk IDs stable.
5. Update claim audit from `accepted-risk` only after executable tests prove
   full compiler/parser coverage for the scoped subset.

## Exit Criteria

- Full parser-backed extractor evidence is executable for the selected language
  subset.
- Unsupported syntax is explicit residual risk.
- Public docs stop saying full compiler AST extraction is planned only after
  fixtures pass.
