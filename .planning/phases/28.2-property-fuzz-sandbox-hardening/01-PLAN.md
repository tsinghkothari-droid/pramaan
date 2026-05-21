---
phase: 28.2
title: Property/Fuzz Sandbox Hardening
priority: P1 security
status: planned
gap_closure: true
depends_on:
  - ../28.1-safe-hypothesis-fast-check-harness-execution/01-PLAN.md
  - ../28.15-fuzz-harness-truthfulness-review-gate/01-PLAN.md
---

# Phase 28.2 - Property/Fuzz Sandbox Hardening

## Objective

Move bounded Hypothesis and fast-check generated harnesses from truthful
subprocess execution toward production-grade isolation.

## Tasks

1. Add a harness sandbox policy object: writable temp root, read-only source
   roots, no inherited secrets, no network by default, and command allowlist.
2. Scrub environment variables before invoking Python/Node harnesses.
3. Record sandbox policy evidence in fuzz receipts.
4. Add tests proving generated harnesses cannot write outside the temp root.
5. Add timeout/process-tree cleanup tests for child processes.
6. Add Windows-specific notes for process termination limits and residual risk.

## Acceptance Criteria

- Harness receipts include sandbox policy, env scrub policy, timeout, and
  cleanup status.
- Harness failures, timeouts, policy violations, and missing tools stay as
  structured evidence.
- Public docs still say "bounded private-preview path" until stronger
  OS/container isolation exists.

## Verification

```powershell
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-fuzz-harness-evidence.mjs <differential-fuzz.json>
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
```
