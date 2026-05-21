# Language Readiness Gates

Pramaan should deepen Python, TypeScript, and Rust before adding Go or Java.
This page defines the minimum gate a language must satisfy before public docs
describe it as more than partial/private-preview support.

## Gate Matrix

| Gate | Python | TypeScript / JavaScript | Rust |
| --- | --- | --- | --- |
| Static tool receipt | `compileall`, configured `ruff`, `mypy`, `pyright` | package-manager detection, `tsc --noEmit`, configured ESLint | `cargo check`, `cargo test --no-run`, configured `cargo clippy` |
| Oracle integrity | pytest/unittest assertions, skips, xfails, raises, parametrized cases | Jest/Vitest `test`/`it`, `expect`, `assert`, skips/todos, snapshots | `#[test]`, `#[ignore]`, `#[should_panic]`, assertion macros, snapshots |
| Parser evidence | parser-backed subset with parser version and fallback reason | parser-backed subset with parser version and fallback reason | parser-backed subset with parser version and fallback reason |
| Mutation | `mutmut` when installed; skipped receipt when missing | StrykerJS when installed; skipped receipt when missing | `cargo-mutants` when installed; skipped receipt when missing |
| Property/fuzz | bounded Hypothesis harness when safe and installed | bounded fast-check harness when safe and installed | deterministic/replay evidence only today |
| Fixtures | static, oracle, mutation, fuzz/property where applicable | static, oracle, mutation, fuzz/property where applicable | static, oracle, mutation, oracle fixtures; property/fuzz remains residual |
| Residual risk | dynamic imports, monkeypatching, generated tests, fixture churn | computed names, transpiler config, generated tests, snapshot churn | macro-generated tests, feature-gated modules, workspace layout |

## Promotion Rules

- A language cannot be called `implemented` unless each gate has executable
  tests or a checked fixture.
- Missing tools must emit skipped or residual-risk receipts, never silent
  success.
- Full compiler-AST parsing is a separate hardening gate. Current parser-backed
  subset evidence must stay labeled as partial.
- Go and Java remain blocked until the first three languages pass the matrix
  above without relying on undocumented heuristics.

## Adapter Boundary

Language support is separate from adapter certification. Adapter certification
may reuse receipts and bundles, but it must remain an adjacent mode until the
core PR-verification flow is trusted.
