---
phase: 4
slug: diff-mutation-and-differential-fuzz
status: draft
nyquist_compliant: true
created: 2026-05-18
---

# Phase 4 - Validation Strategy

## Required Commands

- `cargo fmt --check`
- `cargo test`
- Fixture runs for Python mutmut/Hypothesis and TypeScript StrykerJS/fast-check where tools are available

## Verification Map

| Requirement | Verification |
|-------------|--------------|
| MUTN-01 | Python changed-file mutation fixture |
| MUTN-02 | TS changed-file mutation fixture |
| MUTN-03 | Rust cargo-mutants fixture or not-applicable receipt |
| MUTN-04 | Receipt reports mutant counts and threshold |
| MUTN-05 | Receipt reports timeout/cache/filter/skipped rationale |
| FUZZ-01 | Hypothesis differential fixture |
| FUZZ-02 | fast-check differential fixture |
| FUZZ-03 | Seeds/corpus/replay fields present |
| FUZZ-04 | Safe discovery failure becomes not-applicable receipt |
