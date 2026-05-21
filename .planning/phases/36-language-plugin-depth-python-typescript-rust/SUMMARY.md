# Phase 36 Summary - Language Plugin Depth

Status: `PASS_WITH_RISKS`

Completed on: 2026-05-21

## What Landed

- Added `docs/languages.md` with a concrete Python, TypeScript/JavaScript, and
  Rust support matrix across static checks, oracle integrity, mutation,
  property/fuzz, diff scoping, fingerprints, fixtures, and residual risks.
- Tightened plugin README files for Python, TypeScript, and Rust to mention the
  current deeper gates and missing-tool evidence behavior.
- Added a Rust property/fuzz placeholder README that explicitly states Rust is
  not at Hypothesis/fast-check parity.
- Added `scripts/check-phase36-language-depth.mjs` to verify language docs stay
  aligned with actual code/plugin evidence.

## Deferred / Residual Risk

- Full compiler-AST oracle integrations remain planned.
- Rust property/fuzz support is partial and not tool-backed like
  Hypothesis/fast-check.
- Stronger sandboxing for risky parsers, test runners, mutation engines, and
  fuzzers remains open.
- Go and Java remain blocked.

## Verification

- `node scripts/check-phase36-language-depth.mjs`
- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
