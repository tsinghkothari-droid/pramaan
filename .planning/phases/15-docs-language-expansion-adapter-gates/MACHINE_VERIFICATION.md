# Phase 15 Machine Verification

Status: passed on 2026-05-21.

Required commands:

```powershell
node scripts/check-phase15-docs-language-gates.mjs
node scripts/check-phase35-docs.mjs
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```

This phase is `PASS_WITH_RISKS`: adoption docs and readiness gates exist, but
full language depth and adapter certification product work remain future phases.

Result:

- `node scripts/check-phase15-docs-language-gates.mjs` passed.
- `node scripts/check-phase35-docs.mjs` passed.
- `cargo fmt --check` passed.
- `cargo test --workspace` passed.
- `cargo clippy --workspace -- -D warnings` passed.
- `node scripts/check-claim-audit.mjs` passed.
- `node --test action/render-summary.test.mjs` passed.
