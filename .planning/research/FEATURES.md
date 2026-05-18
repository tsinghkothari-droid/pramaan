# Feature Research: Pramaan

## Table Stakes

- CLI that runs locally and in CI.
- GitHub Action integration for PR verification.
- Structured JSON receipts for every stage.
- Machine-readable schemas for receipts and bundles.
- Human-readable PR summary.
- Deterministic artifact hashing.
- Stage timeouts and failure receipts.
- Clear distinction between failed, skipped, not-applicable, and passed checks.
- Oracle integrity checks for test tampering.
- Diff-scoped mutation testing.
- Static/typecheck/name-binding checks.
- Differential pre/post behavior checks where feasible.

## Differentiators

- Signed, auditable proof bundle rather than a plain CI log.
- Explicit detection of weakened tests and skipped assertions.
- Per-language plugin depth and adversarial corpora.
- Receipt replay and bundle verification.
- Scope-aware differential testing.
- Short 30-second human audit view.
- Later: optional formal verification receipts where specs exist.
- Later: dual critic as non-gating narrative signal.

## Anti-Features

- "AI says this PR is safe" as a gate.
- "Code is correct" proof claims.
- Opaque scoring without receipts.
- Running mutation/fuzz globally when a diff-scoped check gives better latency and relevance.
- Treating generated tests as trustworthy without oracle integrity checks.

## Dependencies Between Features

- CLI and schemas must exist before plugins can emit stable receipts.
- Sandbox artifact capture must precede meaningful stage receipts.
- Oracle integrity should precede mutation and fuzz in the demo because it catches the most legible failure mode.
- Bundle signing depends on stable manifest shape.
