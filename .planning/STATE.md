# State: Pramaan

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-05-18)

**Core value:** Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

**Current focus:** All six v1 phases are planned; autonomous execution can start at Phase 1

## Current Phase

Phase 1: Receipt-First CLI Skeleton

## Status

All v1 GSD phase packs are created for Phases 1-6, including context, validation strategy, and execution plans. Use `.planning/AUTONOMOUS_BUILD_COMMAND.md` to run the full build sequence without further clarification prompts.

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
