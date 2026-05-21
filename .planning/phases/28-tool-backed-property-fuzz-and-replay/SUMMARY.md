# Phase 28 Execution Summary

Date: 2026-05-21

## Landed

- Added `pramaan replay <bundle-or-fuzz-evidence> --case <id>`.
- Replay resolves `differential-fuzz.json` from a fuzz output directory or a
  direct evidence file.
- Replay prints recorded input, base output, head output, classification,
  rationale, and `mode: metadata_replay`.
- Added smoke coverage proving a recorded divergence from the fuzz fixture can
  be replayed by deterministic case ID.
- Added `docs/replay.md` with the honest boundary between metadata replay and
  real harness re-execution.

## Split

Safe generated Hypothesis and fast-check harness execution did not land here.
It is split to Phase 28.1 because real execution needs language-specific
sandboxing, dependency setup, timeouts, and raw-output/counterexample capture.

## Verification

Completed before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
