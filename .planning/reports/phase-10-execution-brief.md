# Phase 10 Execution Brief

## Phase

Phase 10: GitHub Action Production Readiness

## Objective

Make Pramaan usable as a serious pull-request GitHub Action with deterministic CLI build, stable inputs, bundle upload, reviewer summary, and clear fork/permission guidance.

## Scope

- Add production-shaped action inputs: `base-ref`, `head-ref`, `out-dir`, `fail-on`, and `upload-bundle`.
- Preserve deprecated aliases for older docs/users: `bundle-path` and `upload-artifact`.
- Build the CLI with `cargo build --locked -p pramaan-cli` before execution.
- Upload the proof bundle before applying `fail-on`, so red runs still preserve evidence.
- Add Python, TypeScript, and Rust workflow examples.
- Update docs for permissions, forked PR behavior, and failure policy.

## Known Risks

- The composite action is tested statically and through renderer/unit tests, not with a live GitHub Actions runner in this phase.
- The action still builds from source in the checked-out repository; release binary download remains a future distribution improvement.
