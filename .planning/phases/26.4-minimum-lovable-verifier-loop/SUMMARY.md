# Phase 26.4 Summary: Minimum Lovable Verifier Loop

Date: 2026-05-21

Status: completed with residual risk

## What Landed

- Added `docs/quickstart.md` as the canonical first-run path.
- Added `scripts/run-minimum-lovable-loop.ps1`, which runs ordinary CI on the
  weakened-test demo, runs Pramaan oracle integrity, emits confidence evidence,
  verifies the bundle, captures policy output, and writes
  `minimum-lovable-report.md`.
- Updated the oracle command to write `bundle.manifest.json`, making standalone
  oracle demos verifiable proof bundles instead of loose receipts.
- Added smoke-test coverage that standalone oracle output can be verified with
  `pramaan bundle verify`.
- Updated README, demo docs, GitHub Action docs, STATUS, TASKS, ROADMAP, STATE,
  and claim audit.
- Captured the manual UAT result in
  `.planning/reports/phase-26.4-minimum-lovable-loop-uat.md`.

## Evidence

- Command: `powershell -ExecutionPolicy Bypass -File scripts/run-minimum-lovable-loop.ps1`
- Generated report path: `target/pramaan-minimum-lovable/minimum-lovable-report.md`
- Verification from the UAT run:
  - ordinary CI on the weakened branch passed;
  - Pramaan found one `weakened_assertion`;
  - confidence decision was `fail`;
  - bundle verification checked 2 receipts and 3 artifacts;
  - policy explanation failed on oracle/confidence plus missing required stages.

## Deferred

- This is an oracle-focused local demo, not a full PR run.
- Phase 35.5 still owns Rust-native `pramaan report html` and
  `pramaan report markdown`.
- Full public review still needs Phase 27.1, Phase 28.1, Phase 28.26,
  Phase 32.75, and Phase 35.5.

## Self-Check

- [x] One command exists.
- [x] One blockers-first report exists.
- [x] Bundle manifest is verifiable.
- [x] Skipped/missing required stages show as policy failure, not success.
- [x] Public copy remains evidence-first, not correctness-proof language.
