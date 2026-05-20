# State: Pramaan

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-18)

**Core value:** Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

**Current focus:** P0/P1 completion track. Phase 19 is complete; the next
execution phase is Phase 20: P0 SLA and Policy Gates.

## Current Phase

Phase 20: P0 SLA and Policy Gates

## Status

Phases 1-12 and the Phase 16a schema-impact subset are implemented and
validated. Phases 13-17 remain the broader Serious v1 execution path. Phases
18-25 are now the focused P0/P1 completion track: product honesty, golden
evidence, SLA/policy gates, sandbox/threat/redaction hardening, claim/static
security signals, AST oracle extractors, real mutation/fuzz adapters, and a
pilot gate before P2 expansion.

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
| 2026-05-19 | Completed Phase 16a schema-impact hooks. | Receipt and bundle schemas now have hooks for agent attribution, reviewer override, multi-agent provenance, plugin identity, redaction, policy decision, and stage budgets before Phase 9 schema hardening. |
| 2026-05-19 | Completed Phase 9 receipt and bundle trust hardening. | v0.1 receipt schema now matches runtime output, checked-in receipts parse as current fixtures, and bundle verification rejects missing artifacts, path escapes, ambiguous artifact refs, and signing metadata tamper. |
| 2026-05-19 | Completed Phase 10 GitHub Action production readiness. | The composite action now has stable PR inputs, deterministic source build, bundle upload before failure policy, fork-permission docs, and Python/TypeScript/Rust workflow examples. |
| 2026-05-19 | Completed Phase 11 sandbox, claim, and static depth. | Pramaan now records richer environment and lockfile-drift evidence, PR-grounded claim scope, deterministic public API scans, configured pyright/clippy receipts, and expanded static hallucination categories. |
| 2026-05-19 | Completed Phase 12 oracle integrity engine. | Oracle receipts now detect Python, TypeScript, and Rust oracle weakening patterns, renamed/deleted tests, boundary/error removals, and fixture/snapshot hash drift with reviewer-facing details. |
| 2026-05-21 | Added P0/P1 GSD completion track for Phases 18-25. | Remaining P0/P1 tasks are now mapped to execution phases, with a Phase 25 pilot gate before P2/P3 expansion. |
| 2026-05-21 | Completed Phase 18 product honesty and direction. | `STATUS.md`, README status language, non-goals, ICP, killer workflow, research sufficiency, and pivot criteria now keep the public surface ambitious but honest. |
| 2026-05-21 | Completed Phase 19 receipt golden tests and canonical evidence. | Generated claim-scope receipts now have a normalized golden contract test, bundle manifest digests use canonical JSON bytes, and receipt docs explain fixture update discipline. |
