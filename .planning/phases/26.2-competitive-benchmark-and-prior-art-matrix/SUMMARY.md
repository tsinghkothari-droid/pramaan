# Phase 26.2 Summary: Competitive Benchmark and Prior-Art Matrix

Date: 2026-05-21

Status: completed

## What Landed

- Added `docs/competitive-benchmark.md` with a public positioning matrix for
  AI PR reviewers, reviewdog-style aggregators, test-change monitors,
  Pynguin/EvoSuite-style test generators, mutation/property engines, GitHub
  artifact attestations, SLSA VSA, Sigstore, and in-toto.
- Linked the benchmark from `README.md` and narrowed the differentiation claim:
  Pramaan is an evidence-bundle verifier, not a blanket replacement for
  reviewers, CI, SAST, generated tests, or supply-chain attestation systems.
- Updated `TASKS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`, and
  `docs/claim-audit.md` to mark Phase 26.2 complete and keep "catches what X
  misses" claims blocked until Phase 26.3 executable fixtures exist.

## Evidence

- Public benchmark doc: `docs/competitive-benchmark.md`
- Claim audit row: `CLAIM-POSITIONING-001`
- Live Action proof from prior phase remains linked in
  `.planning/reports/phase-26.1-live-action-proof.md`

## Deferred

- Phase 26.3 must turn the strongest comparison claims into executable
  competitor-gap fixtures.
- Phase 26.4 must package the minimum lovable loop so a reviewer can inspect
  one command, one report, and one bundle quickly.
- This benchmark should refresh before public Alpha and Serious v1.

## Self-Check

- [x] Competitive matrix exists and cites each compared category.
- [x] Competitors and reusable primitives are separated.
- [x] README language remains evidence-first and avoids correctness-proof or
  superiority claims.
- [x] Tracking files updated honestly.
