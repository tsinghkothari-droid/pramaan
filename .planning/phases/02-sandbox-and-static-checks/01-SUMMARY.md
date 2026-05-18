# Plan 01 Summary - Sandbox Evidence and Worktree Runner

## Completed

- Implemented `pramaan-sandbox` isolated base/head worktree setup with cleanup guards.
- Captured base/head commit SHAs, source and worktree dirty state, recognized lockfile hashes, config hashes, OS/runtime identity, and optional `PRAMAAN_IMAGE_DIGEST`.
- Added sandbox evidence receipts through `pramaan verify`, emitted as `receipts/sandbox-setup.receipt.json` with a `sandbox/sandbox-evidence.json` artifact.
- Mapped sandbox receipt risks across `R-021` through `R-033`, with non-hermetic and missing-lockfile cases recorded as residual risk.
- Added sandbox fixtures for clean, dirty, missing-lockfile, and non-hermetic runs under `examples/fixtures/sandbox/`.
- Added focused sandbox tests that create temporary Git repos, materialize worktrees, verify evidence, and confirm cleanup.

## Verification

- `cargo fmt --check` passed.
- `cargo test -p pramaan-sandbox` passed: 5 tests.
- `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-static-smoke` passed and emitted `sandbox_setup`.
- `cargo test` against the default target dir first hit Windows linker error `LNK1104` on `target/debug/deps/smoke-...exe`, consistent with an artifact lock.
- `$env:CARGO_TARGET_DIR='target/pramaan-phase2-test'; cargo test -p pramaan-cli --test smoke` passed: 3 tests.
- `$env:CARGO_TARGET_DIR='target/pramaan-phase2-test'; cargo test` passed: all workspace tests.

## Notes

- Worktrees are isolated Git worktrees and are removed on `SandboxRun` drop.
- Runs without `PRAMAAN_IMAGE_DIGEST` are intentionally marked non-hermetic while still producing usable evidence.
- No Git staging, committing, branch switching, or reverting was performed.
