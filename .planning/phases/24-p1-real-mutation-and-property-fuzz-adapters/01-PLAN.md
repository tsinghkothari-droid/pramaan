# Phase 24: P1 Real Mutation and Property/Fuzz Adapters

## Goal

Replace simulated/advisory execution with real tool-backed mutation and
property/fuzz checks where tools are installed.

## P1 Tasks Covered

- Python `mutmut` on changed files and directly affected tests.
- TypeScript StrykerJS in diff-scoped mode.
- Rust cargo-mutants on changed crates/modules.
- Python Hypothesis differential checks.
- TypeScript fast-check differential checks.
- Seeds, replay data, minimized counterexamples, corpus hashes, generated input counts, and divergence classifications.

## Files to Change

- `crates/pramaan-cli/src/mutation.rs`
- `crates/pramaan-cli/src/fuzz.rs`
- `crates/pramaan-core/src/lib.rs`
- `docs/plugins.md`
- `docs/receipt-model.md`
- `examples/fixtures/mutation/**`
- `examples/fixtures/fuzz/**`

## Implementation Steps

1. Keep current honest receipt statuses: skipped, not_applicable, timed_out, failed, passed.
2. Bind parsed mutation evidence to raw artifact digests.
3. Add structured parsing for mutmut, StrykerJS JSON, and cargo-mutants outputs.
4. Add Hypothesis and fast-check subprocess adapters with deterministic seeds and replay metadata.
5. Record unsupported/unsafe pure-function candidates rather than silently ignoring them.
6. Ensure skipped/missing tools never satisfy a mitigation gate.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Fixtures for missing tools, timeout, survived mutants, and replayable divergence.

## Exit Criteria

P1 mutation/fuzz receipts become real execution evidence where tools are
available, and honest residual risk where they are not.
