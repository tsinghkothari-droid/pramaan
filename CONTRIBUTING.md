# Contributing

Pramaan is being designed as an evidence-first verification system for AI-generated code.

## Contribution Principles

- Evidence beats opinion.
- Receipts must be auditable.
- Every stage should emit a receipt, including failures and skips.
- Do not introduce claims that Pramaan proves code correctness.
- Prefer narrow, reliable checks over broad but vague scoring.
- Map findings to stable risk IDs where possible.

## Good First Areas

- JSON schema refinement.
- Fixture repositories for oracle tampering.
- Python assertion weakening detection.
- TypeScript assertion weakening detection.
- Risk taxonomy examples.
- Bundle verification tests.

## Language Direction

The planned core is Rust, with per-language plugin adapters for Python, TypeScript, and Rust first.
