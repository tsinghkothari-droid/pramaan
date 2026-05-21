# Phase 28.1 Summary: Safe Hypothesis and fast-check Harness Execution

Date: 2026-05-21

Status: completed with environment-dependent execution

## What Landed

- Differential fuzz now attempts tool-backed generated harness execution when:
  - conservative pure-function candidates exist in both base and head;
  - Python Hypothesis or TypeScript fast-check is installed;
  - the candidate expression passes Pramaan's existing no-IO/no-call/no-complex
    grammar gate.
- Added generated Hypothesis and fast-check harness writers with bounded cases,
  deterministic seed/settings, short runtime limit, tool-version capture,
  generated-case count, harness path, raw-output path, and raw-output digest.
- Missing tools still select deterministic replay evidence and keep
  `tool_backed=false`.
- Added `scripts/check-fuzz-harness-evidence.mjs`.
- Updated README, STATUS, plugin/replay docs, claim audit, TASKS, ROADMAP, and
  STATE.

## Evidence

- Existing fuzz smoke tests pass.
- `node scripts/check-fuzz-harness-evidence.mjs <differential-fuzz.json>`
  validates both tool-backed and deterministic fallback evidence shape.
- In this local environment, Hypothesis and fast-check were not installed, so
  verification exercised the visible fallback path. The code path for installed
  tools is implemented as a subprocess harness and remains environment-gated.

## Deferred

- Stronger sandbox boundaries for running property tools remain future
  hardening.
- `pramaan replay` still replays recorded metadata; it does not re-execute the
  generated harness.
- Positive CI coverage with Hypothesis/fast-check installed should be added when
  a toolchain image is introduced.

## Self-Check

- [x] Tool-backed evidence is only set after a generated harness runs.
- [x] Missing tools remain residual/fallback evidence.
- [x] Tool version and raw-output digest are recorded in adapter metadata.
- [x] Public docs avoid claiming property/fuzz proof of correctness.
