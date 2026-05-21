# Phase 36 Machine Verification

Status: passed on 2026-05-21.

Required commands:

```powershell
node scripts/check-phase36-language-depth.mjs
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```

This phase is `PASS_WITH_RISKS`: the first three language lanes have a checked
support matrix, but full compiler AST, Rust property/fuzz parity, and stronger
sandboxing remain future work.

Result:

- `node scripts/check-phase36-language-depth.mjs` passed.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace -- -D warnings` passed.
- `node scripts/check-claim-audit.mjs` passed.
- `node --test action/render-summary.test.mjs` passed.
