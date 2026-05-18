# State: Pramaan

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-18)

**Core value:** Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

**Current focus:** Phase 8 killer demo completed; next safe-order step is Phase 16a schema-impact hooks before Phase 9 schema hardening.

## Current Phase

Phase 16a: Schema-impact subset of Attribution, Feedback, Calibration, and Security

## Status

Phases 1-8 are implemented and validated. Phases 9-17 remain the Serious v1 execution path: receipt/bundle hardening, GitHub Action readiness, sandbox/claim/static depth, oracle integrity, mutation/fuzz adapters, attestation/corpus/evals, adoption/language/adapter gates, blind-spot closure for attribution/feedback/calibration/security, and next-level policy/CI/evaluation intelligence. The safe order requires the schema-impact subset of Phase 16 before Phase 9 freezes receipt schema v0.1.

## Open Questions

- Exact v1 demo language: Python first is recommended, TypeScript also viable.
- Whether to create a real monorepo skeleton immediately in Phase 1 or keep planning docs first until phase discussion.
- Whether initial signing should be pure local signing metadata or a Sigstore-compatible placeholder.
- How much claim/scope extraction should be deterministic in v1 versus LLM-assisted with a reviewable receipt.

## Recent Decisions

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-05-18 | Narrow v1 to CLI, receipts, sandbox, static, oracle, mutation, differential fuzz, and bundle signing. | Reliability beats stage breadth for first trustable release. |
| 2026-05-18 | Delay adversarial amplification, formal, and dual critic to v2. | They are valuable but should not block a usable proof bundle. |
| 2026-05-18 | Make the first demo about test weakening. | It is the clearest user-facing proof that ordinary CI is insufficient. |
| 2026-05-18 | Add claim/scope receipts before oracle integrity. | Current benchmark research shows hidden tests and task descriptions can be misaligned, too narrow, or too wide. |
| 2026-05-18 | Split Phase 1 into schema, CLI/orchestrator, and smoke/docs plans. | This keeps the receipt contract stable before real verification stages are added. |
| 2026-05-18 | Add top-100 flaw register and risk-ID references to receipts. | Reviewers need risk-family evidence and residual risk, not a single opaque score. |
| 2026-05-18 | Planned all six v1 phases and added autonomous build command. | The project now has an end-to-end build path from schemas through GitHub Action and demo corpus. |
| 2026-05-18 | Completed Phase 1 implementation. | The receipt-first skeleton is green under `cargo fmt --check`, `cargo test`, and synthetic CLI smoke verification. |
| 2026-05-18 | Completed Phase 2 implementation. | Sandbox and static-check paths are green under `cargo fmt --check`, full workspace tests, and CLI smoke verification. |
| 2026-05-18 | Completed Phase 3 implementation. | Oracle integrity catches the weakened-test demo while the weakened PR's normal unit test passes. |
| 2026-05-18 | Completed Phase 4 implementation. | Mutation and differential fuzz commands emit budgeted, replayable, risk-mapped receipts and pass workspace tests. |
| 2026-05-18 | Completed Phase 5 implementation. | Bundle verification and signing/attestation metadata are green under full workspace tests and CLI manifest verification. |
| 2026-05-18 | Completed Phase 6 implementation. | GitHub Action, demo docs, and starter adversarial corpus are green under Node tests, Rust tests, and demo oracle verification. |
| 2026-05-18 | Completed Phase 7 implementation. | Adapter certification docs, schema, fixture, and risk register are in place while registry/Sutra remain deferred. |
| 2026-05-19 | Made the GitHub repository public. | Pramaan needs public proof, public research grounding, and public demo credibility. |
| 2026-05-19 | Added Serious v1 GSD plans for Phases 8-15. | The marketing README now has an execution map behind it, with P0/P1/P2/P3 tasks converted into phase plans. |
| 2026-05-19 | Added Phase 16 blind-spot plan. | Agent attribution, reviewer override capture, performance SLAs, baseline calibration, drift tracking, plugin trust, bundle redaction, multi-agent provenance, and non-GitHub support must be designed before Serious v1 hardens. |
| 2026-05-19 | Added next-level research pass and Phase 17. | Current benchmark, CI-security, supply-chain, and policy-as-code research shows Pramaan needs policy gates, VSA output, redaction profiles, CI hardening, secure-code corpus categories, and benchmark-integrity mutation. |
| 2026-05-19 | Completed Phase 8 killer demo and proof-bundle examples. | Pramaan now has three public demo scenarios where ordinary CI or superficial review misses weakened assertions, sensitive fixture/snapshot drift, or hallucinated Rust imports while Pramaan emits concrete failed receipts. |
