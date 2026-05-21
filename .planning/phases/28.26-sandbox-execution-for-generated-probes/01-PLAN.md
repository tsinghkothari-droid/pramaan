# Phase 28.26: Sandbox Execution for Generated Probes

## Goal

Execute AI/provider/agent-generated probe candidates in isolated temporary test
locations, preserving accepted and rejected execution evidence.

## Why This Split Exists

Phase 28.25 can safely produce provider-neutral probe plans. It cannot honestly
claim mitigation because generated probes are not yet compiled, run, bound to
changed behavior, or mutation/differential validated.

## Files To Change

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `docs/ai-probe-generator.md`
- `examples/ai-probes/`
- `schemas/probe.schema.json` if execution fields need extension

## Implementation Steps

1. Add `pramaan probe execute --plan <path> --bundle <path>`.
2. Materialize each probe in a temp test location outside the repository source
   tree.
3. Execute language-native test commands with strict timeout, no network by
   default, and captured stdout/stderr.
4. Reject probes that fail to compile/run or do not bind to changed behavior.
5. Preserve rejected probes and rejection reasons in the probe artifact.
6. Mark accepted probes only after sandbox execution and, where practical,
   mutation or differential evidence.
7. Emit an updated `ai_probe_generation` receipt with real accepted/rejected
   counts and execution artifact hashes.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Golden fixtures for accepted, rejected compile-fail, rejected no-binding, and
  timeout cases.
