# Phase 1: Receipt-First CLI Skeleton - Context

**Gathered:** 2026-05-18
**Status:** Ready for planning
**Source:** Revised from project research and improvement research

<domain>
## Phase Boundary

Phase 1 creates Pramaan's executable foundation: a Rust workspace, CLI command shape, schema files, receipt-writing contract, claim-scope model, and minimal test fixtures. It does not need to run real sandbox/static/oracle/mutation stages yet; it must prove that later stages can plug into a stable receipt and bundle model.

The phase should leave the repo able to run:

```text
pramaan verify --base <ref> --head <ref> --out <dir>
```

and produce at least one schema-valid synthetic receipt plus a concise terminal summary.
</domain>

<decisions>
## Implementation Decisions

### Locked Decisions

- Use a Rust workspace with `crates/pramaan-cli`, `crates/pramaan-core`, `crates/pramaan-sandbox`, and `crates/pramaan-bundle`.
- Use JSON receipts and JSON Schema under `schemas/`.
- Include a `claim_scope` schema in Phase 1 because downstream oracle integrity depends on knowing what behavior the PR claims to change.
- Use content-addressed artifact references in schema design even if Phase 1 only emits synthetic artifacts.
- The CLI should distinguish `passed`, `failed`, `skipped`, `not_applicable`, `timed_out`, and `error`.
- Do not add real language plugins in Phase 1 beyond directory/protocol placeholders.
- Do not market or encode any "code is correct" claim.

### Claude's Discretion

- Exact Rust crate module layout, as long as boundaries are clear and future plugins can call into core types.
- Whether JSON Schema is hand-written first or generated from Rust types later; Phase 1 must commit stable schema files regardless.
- Test framework details, as long as `cargo test` validates schema fixtures and CLI smoke behavior.
</decisions>

<specifics>
## Specific Ideas

- Receipt fields should include `schema_version`, `stage`, `status`, `tool`, `started_at`, `ended_at`, `inputs`, `outputs`, `summary`, `artifacts`, and `limitations`.
- Claim scope fields should include `source_refs`, `expected_behavior`, `out_of_scope_behavior`, `touched_public_apis`, `confidence`, and `extraction_method`.
- Bundle manifest should anticipate later Sigstore/GitHub attestation metadata without requiring live signing in Phase 1.
- CLI output should be boring and auditable: stage table plus bundle path.
</specifics>

<deferred>
## Deferred Ideas

- Real sandbox worktree execution moves to Phase 2.
- Real oracle weakening detection moves to Phase 3.
- Real mutation/property execution moves to Phase 4.
- Sigstore/GitHub artifact attestation execution moves to Phase 5/6.
</deferred>

---

*Phase: 01-receipt-first-cli-skeleton*
*Context gathered: 2026-05-18 via revised GSD planning*
