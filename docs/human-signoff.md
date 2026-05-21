# Machine Verification and Human Sign-Off

Pramaan is built by agents, but it must not ask agents to be the final judge of
their own work. Every meaningful phase should separate mechanical verification
from human approval.

## Rule

Codex or CI can write code, run tests, produce receipts, scan for known risks,
and summarize the evidence. A human must approve meaning, usefulness, release
claims, risk severity, policy defaults, security boundaries, legal concerns,
and public readiness.

## Required Phase Artifacts

Each completed GSD phase should include two artifacts:

1. `MACHINE_VERIFICATION.md`
   - written by the coding agent;
   - records changed files, commands run, receipts generated, fixture results,
     known gaps, failed checks, residual risks, and next actions;
   - links to bundle/report artifacts where applicable.

2. `HUMAN_SIGNOFF.md`
   - prepared by the coding agent;
   - completed by a human reviewer;
   - records approval, rejection, required changes, public-claim approval,
     security acceptance, UX usefulness, and override rationale.

## Machine-Checkable Examples

- formatting, linting, tests, and smoke checks;
- schema validation and canonical hash reproducibility;
- receipt completeness and risk-ID uniqueness;
- oracle-integrity fixture outcomes;
- mutation/fuzz timeout and failure propagation;
- redaction and forbidden-name scans;
- report rendering smoke tests;
- bundle tamper detection;
- policy pass/warn/block fixtures;
- performance budget measurements.

## Human-Required Examples

- whether a phase actually satisfies the product intent;
- whether README or release copy overclaims;
- whether the report is understandable in 30 seconds;
- whether warning/block severity matches real reviewer expectations;
- whether a sandbox boundary is acceptable for untrusted code;
- whether false positives are tolerable;
- whether a red result can be overridden;
- whether a release, Marketplace listing, or public demo should ship.

## Merge Discipline

A phase can be mechanically complete without being human-approved. Public
claims, release tags, Marketplace publishing, and Serious v1 gates require a
human-approved `HUMAN_SIGNOFF.md`.
