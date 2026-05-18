# Adapter Certification Risk Register

This starter register defines stable adapter risk IDs for Pramaan Adapter Certification. These IDs are separate from the core code-change `R-###` taxonomy so adapter receipts can map MCP and tool behavior without overloading PR-verification risks.

Pramaan remains the focus. The registry and Sutra concepts are deferred product-family ideas; this register exists only to support typed Pramaan evidence bundles for adapters.

## Stable ID Rules

- Use `A-###` IDs for adapter certification risks.
- Do not renumber existing IDs after a receipt references them.
- Mark uncovered checks as residual instead of implying adapter safety.
- Keep skipped, failed, and timed-out certification checks in the bundle.

## Starter Risks

| ID | Family | Failure mode | Trust break | Starter mitigation |
| --- | --- | --- | --- | --- |
| A-001 | surface_contract | Tool list is incomplete, unstable, or generated from undocumented runtime state. | Reviewers cannot know what the adapter actually exposes to agents. | Capture a versioned tool manifest with names, descriptions, input schema refs, output schema refs, and digests. |
| A-002 | type_contract | Inputs or outputs are weakly typed, ambiguous, or missing structured error shapes. | Agents guess parameters or misread results, creating unsafe calls or false evidence. | Require machine-readable input and output contracts for every declared tool. |
| A-003 | permission_scope | Tools request broader auth scopes or filesystem access than their behavior requires. | A narrow task grants the adapter excessive authority. | Map each tool to least-required permissions and record scope evidence in the receipt. |
| A-004 | auth_boundary | Credential handling, token refresh, revocation, or tenant boundaries are undocumented or untested. | A tool may leak secrets, cross tenants, or keep working after access should end. | Record auth mode, secret redaction rules, and credentialed-test status; leave untested auth behavior residual. |
| A-005 | replayability | Calls cannot be replayed from fixtures or normalized traces. | Reviewers cannot distinguish actual behavior from narrative claims. | Store safe fixtures, normalized call traces, timing, and output digests where calls can be replayed. |
| A-006 | side_effects | Side effects are hidden, irreversible, or not classified per tool. | An agent can mutate external state without clear reviewer awareness. | Classify tools as read-only, write-once, update, delete, external-send, privileged, or unknown. |
| A-007 | idempotency | Retried calls duplicate writes, sends, payments, tickets, or other external actions. | Agent retries can amplify harm while appearing like normal recovery behavior. | Require idempotency keys, dry-run mode, or explicit residual risk for side-effecting calls. |
| A-008 | failure_semantics | Rate limits, timeouts, upstream failures, and invalid inputs return unstructured or misleading errors. | Agents may continue with bad assumptions after the adapter should have failed closed. | Exercise negative fixtures and record structured error contracts. |
| A-009 | audit_integrity | Logs, artifacts, or receipts omit call inputs, outputs, actor identity, or digests. | Later audit cannot prove what was called or what evidence was produced. | Emit receipt evidence refs with paths, media types, digests, and redaction notes. |
| A-010 | supply_chain | Adapter source, package version, build provenance, or transitive dependencies are not pinned. | Certification may apply to a different adapter than the one deployed. | Record source kind, version, lockfile or package digest, and provenance status. |

## Current Use

Phase 7 Plan 01 uses this register for the synthetic adapter certification fixture only. Future phases can promote these IDs into a schema-backed taxonomy once the Pramaan adapter mode moves beyond docs and fixtures.
