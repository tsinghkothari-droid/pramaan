# Phase 32 Summary: SARIF, Policy, and Agentic Workflow Security

## Status

Completed the first security-review integration slice on 2026-05-21.

## Landed

- Added `pramaan export sarif <bundle> --out <file>`.
- Added `pramaan export rego --out <file>`.
- Added a minimal `schemas/sarif_export.schema.json` contract.
- SARIF output maps residual and skipped/not-applicable risk IDs to SARIF
  rules/results with locations pointing back to stage receipts.
- Added a starter Rego policy that mirrors the default hard-fail/warn policy
  shape.
- Added agentic workflow-injection detection for untrusted PR title/body and
  issue text, mapping suspicious prompt/tooling text to `R-093`.
- Documented SARIF, Rego, GitHub code-scanning upload, and workflow-injection
  limits in `docs/policy.md`, `docs/github-action.md`, and
  `docs/threat-model.md`.

## Deferred Honestly

- Full OPA/Conftest execution is not bundled; the Rego export is a starter
  policy artifact.
- CodeQL/security-scanner ingestion remains future warning-only correlation.
- SARIF import was not live-tested against GitHub code scanning in this phase.

## Verification

- Targeted SARIF/Rego smoke coverage and workflow-injection unit tests passed
  during implementation.
- Full required phase verification is recorded in the phase commit workflow.
