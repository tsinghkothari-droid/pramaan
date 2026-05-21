---
phase: 27.2
title: Full AST Oracle Integration Slice
priority: P1 hardening
status: planned
gap_closure: true
depends_on:
  - ../27.1-full-compiler-ast-oracle-extractors/01-PLAN.md
---

# Phase 27.2 - Full AST Oracle Integration Slice

## Objective

Replace one high-value parser-backed subset path with a real compiler/parser
subprocess integration without changing public claims for the other languages.

## Scope

Start with Python because CPython `ast` is available wherever Python is
installed and can be isolated as a subprocess helper. TypeScript compiler API
and Rust rust-analyzer/syn integrations remain follow-up slices until the
Python path proves the protocol.

## Tasks

1. Add a subprocess helper that parses Python test files with `ast` and emits
   normalized test nodes: stable ID, decorators, assertion calls, skip/xfail,
   `pytest.raises`, unittest assertions, parametrized cases, and source spans.
2. Add a Rust-side adapter that invokes the helper with a strict timeout and
   converts helper failures into oracle metadata, not panics.
3. Add fixture parity tests comparing current parser-backed subset evidence
   against Python AST evidence.
4. Add disagreement fields to the oracle diff when subset and AST extraction
   disagree.
5. Keep TypeScript and Rust labeled as parser-backed subsets until separate
   executable integrations land.

## Acceptance Criteria

- Python AST extraction is represented as `compiler_ast_subprocess` evidence.
- Helper timeout, invalid syntax, missing Python, and helper JSON errors become
  structured residual risk.
- Existing oracle tests remain green.
- `STATUS.md`, `docs/oracle-parser-decision.md`, and `docs/claim-audit.md`
  distinguish Python AST support from TypeScript/Rust subset support.

## Verification

```powershell
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```
