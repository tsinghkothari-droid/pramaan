# Machine Verification

Required commands:

```powershell
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
node scripts/check-phase29-35-runtime-gap-slices.mjs
```

Expected result: all pass.
