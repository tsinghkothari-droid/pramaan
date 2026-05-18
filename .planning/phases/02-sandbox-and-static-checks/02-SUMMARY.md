# Plan 02 Summary - Static Check Plugin Adapters

## Completed

- Added `pramaan static-checks --repo <path> --out <path>` for static receipt
  generation without changing the existing synthetic `verify` flow.
- Implemented Python static discovery for:
  - `python -m compileall -q .` when Python files exist;
  - `ruff check .` when Ruff config exists;
  - `mypy .` when Mypy config exists.
- Implemented TypeScript static discovery for:
  - package manager selection from lockfiles;
  - `<pm> exec tsc --noEmit` when TypeScript files, `package.json`, and
    `tsconfig.json` exist;
  - `<pm> run lint` when a lint script exists.
- Implemented Rust static discovery for:
  - `cargo check`;
  - `cargo test --no-run`.
- Normalized non-applicable and missing-tool paths as Pramaan receipts with
  conservative residual/not-applicable risk IDs.
- Added static hallucination classification for broken imports and undefined
  symbols where command diagnostics support it.
- Added fixture projects under `examples/fixtures/static/` for Python,
  TypeScript, and Rust.
- Updated plugin adapter READMEs to document Phase 2 discovery and receipt
  behavior.

## Verification

- `cargo fmt` passed.
- `cargo test -p pramaan-core` passed: 5 tests.
- `cargo test -p pramaan-cli --test smoke` passed: 3 tests.
- Fixture CLI run passed:
  - `cargo run -p pramaan-cli -- static-checks --repo examples/fixtures/static/python --out target/pramaan-static-python`
  - `cargo run -p pramaan-cli -- static-checks --repo examples/fixtures/static/typescript --out target/pramaan-static-typescript`
  - `cargo run -p pramaan-cli -- static-checks --repo examples/fixtures/static/rust --out target/pramaan-static-rust`

## Notes

- Full workspace `cargo test` was attempted, but it is currently blocked by
  `crates/pramaan-sandbox` tests referencing missing files under
  `examples/fixtures/sandbox/`. This summary does not modify the sandbox crate
  because another worker may own that area in parallel.
- The TypeScript fixture emitted skipped receipts in this environment because
  the package-manager executable was not available to the static runner.
