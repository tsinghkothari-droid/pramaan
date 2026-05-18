# State: Pramaan

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-18)

**Core value:** Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

**Current focus:** Phase 6 - GitHub Action and Public Demo Loop

## Current Phase

Phase 6: GitHub Action and Public Demo Loop

## Status

Phase 5 is implemented and validated: bundle manifests hash receipts/artifacts, `pramaan bundle verify` catches tampering, and signing/attestation metadata plus risk summaries are in place. Proceeding to Phase 6.

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
