# Language Support

Pramaan's first supported lanes are Python, TypeScript/JavaScript, and Rust.
They are private-preview lanes with explicit residual risk, not production v1
language certifications.

## Support Matrix

| Evidence lane | Python | TypeScript / JavaScript | Rust |
| --- | --- | --- | --- |
| Static checks | `compileall`, configured `ruff`, `mypy`, `pyright` | package-manager detection, `tsc --noEmit`, configured lint script / ESLint | `cargo check`, `cargo test --no-run`, configured `cargo clippy` |
| Oracle integrity | pytest/unittest assertions, skips, xfails, raises, parametrized cases | Jest/Vitest/common `expect` and `assert` patterns, skips, todos, snapshots | `#[test]`, `#[ignore]`, `#[should_panic]`, assertion macros, snapshots |
| Parser status | parser-backed subset with metadata and fallback reason | parser-backed subset with metadata and fallback reason | parser-backed subset with metadata and macro residual risk |
| Mutation | `mutmut` when installed, otherwise skipped evidence | StrykerJS when installed, otherwise skipped evidence | `cargo-mutants` when installed, otherwise skipped evidence |
| Property/fuzz | deterministic replay plus bounded Hypothesis harness when eligible and installed | deterministic replay plus bounded fast-check harness when eligible and installed | deterministic/replay evidence only today |
| Diff scoping | changed files and discovered pure-function candidates | changed files and discovered pure-function candidates | changed crates/modules for mutation and parser-backed oracle files |
| Tool fingerprint | command availability, version where available, raw output digest, timeout/budget | command availability, version where available, raw output digest, timeout/budget | command availability, version where available, raw output digest, timeout/budget |

## Fixture Accountability

| Language | Static | Oracle | Mutation | Property/fuzz | Primary proof |
| --- | --- | --- | --- | --- | --- |
| Python | yes | yes | yes | yes | `cargo test --workspace`, `scripts/check-phase36-language-depth.mjs` |
| TypeScript / JavaScript | yes | yes | yes | yes | `cargo test --workspace`, `scripts/check-phase36-language-depth.mjs` |
| Rust | yes | yes | yes | partial | `cargo test --workspace`, `scripts/check-phase36-language-depth.mjs` |

## Residual Risks

- Full compiler-AST oracle extractors are still planned. Current evidence is a
  parser-backed subset, not a full discovery proof.
- Mutation tools are subprocess adapters. Missing tools emit skipped evidence
  and do not mitigate risk.
- Python Hypothesis and TypeScript fast-check run only for safe eligible
  candidates when the tools are installed.
- Rust property/fuzz support is not yet equivalent to Python/TypeScript
  property support.
- Risky parsers, mutation engines, fuzzers, and generated harnesses still need
  stronger sandbox boundaries.

## Expansion Rule

Go and Java remain blocked until Python, TypeScript, and Rust have credible
depth across static, oracle, mutation, property/fuzz where applicable, fixtures,
and claim-audited documentation. A new language logo is not useful if skipped
evidence looks like success.
