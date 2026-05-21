# Machine Verification

Required commands:

```powershell
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
node scripts/check-phase36-5-repo-health.mjs
```

Expected result: all pass.

Observed on 2026-05-21:

- `cargo fmt --check`: pass.
- `cargo test --workspace`: pass, 86 workspace tests plus doctests.
- `cargo clippy --workspace -- -D warnings`: pass.
- `node scripts/check-claim-audit.mjs`: pass, 66 claims and 34 STATUS rows covered.
- `node --test action/render-summary.test.mjs`: pass, 4 tests.
- `node scripts/check-phase36-5-repo-health.mjs`: pass.
