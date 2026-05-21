# Phase 23 Summary

## Status

Completed 2026-05-21 as `PASS_WITH_RISKS`.

## Landed

- Added a language-neutral oracle evidence model for extractor profile,
  assertion-signal kind, assertion strength, signal hash, and skip markers.
- Preserved deterministic Python, TypeScript, and Rust oracle behavior.
- Added tests that assert structured extractor evidence is present in oracle
  fixtures.
- Updated docs to say the shipped path is structured extractor evidence, not
  full compiler AST proof.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo run -p pramaan-cli -- oracle --base-repo examples\fixtures\oracle\base --head-repo examples\fixtures\oracle\head --out target\phase23-oracle`

## Residual Risk

Full compiler/parser-backed AST integrations for Python, TypeScript, and Rust
remain follow-up work with dependency justification and golden negative
fixtures.
