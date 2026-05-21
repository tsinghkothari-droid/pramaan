# Phase 28.25 Execution Summary

Date: 2026-05-21

## Landed

- Added `pramaan probe plan --bundle <bundle>` with default in-bundle output at
  `probes/ai-probe-plan.json`.
- Added `schemas/probe.schema.json` and runtime `pramaan.probe.v1` structs.
- Probe plans record provider-neutral metadata, prompt hash, risk IDs, probe
  kind, language, target files, candidate skeletons, and
  `trusted_for_decision=false`.
- Added `ai_probe_generation` receipts that preserve accepted/rejected/pending
  counts and keep targeted risks residual until execution exists.
- Added smoke coverage proving probe plans are emitted, folded back into the
  bundle manifest, and remain bundle-verifiable.
- Added docs, examples, and corpus taxonomy for probe categories.

## Split

Generated probes are not yet compiled or executed. That work is split to Phase
28.26 because it needs isolated temp test locations, language-specific
subprocess harnesses, and rejection evidence for non-compiling/non-binding
probes.

## Verification

Completed before commit:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
