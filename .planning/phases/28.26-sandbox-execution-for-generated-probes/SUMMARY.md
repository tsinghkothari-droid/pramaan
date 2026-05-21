# Phase 28.26 Summary: Sandbox Execution for Generated Probes

## Status

Completed 2026-05-21 as `PASS_WITH_RISKS`.

## What Landed

- Added `pramaan probe execute --plan <path> --bundle <path>`.
- Materializes probe candidates under `probes/executed/sandbox/`.
- Runs bounded language-native commands with timeout handling:
  - Python: `python test_probe.py`
  - JavaScript/TypeScript probe artifact: `node probe.test.js`
  - Rust: `rustc --crate-type lib --emit metadata probe_test.rs`
- Keeps only probes that:
  - include `pramaan-accepted-probe`;
  - avoid blocked network/process/filesystem tokens;
  - bind to a risk ID, target basename, or `pramaan-bind`;
  - compile/run successfully.
- Preserves rejected probes with stable rejection reasons for skeleton,
  dangerous-token, no-binding, timeout, execution-error, and compile/run
  failures.
- Emits `ai-probe-plan.executed.json`, `ai-probe-execution.json`, and an
  updated `ai_probe_generation` receipt with accepted/rejected/pending counts
  and artifact hashes.
- Added `scripts/check-ai-probe-execution.mjs` and a CLI smoke test covering
  accepted, compile-failed, and static-rejected candidates.

## Verification

- `cargo test --workspace` passed.
- `node scripts/check-ai-probe-execution.mjs target/pramaan-probe-execute-tests/15004/probes/executed/ai-probe-plan.executed.json` passed.

Full phase-close verification was run after docs/task updates before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `node scripts/check-ai-probe-execution.mjs <executed-plan>`

## Deferred

- Accepted probes are not yet mutation-tested or differential-tested.
- Arbitrary generated code is not executed; unmarked or dangerous candidates
  are rejected.
- Stronger process/container isolation remains future hardening.
