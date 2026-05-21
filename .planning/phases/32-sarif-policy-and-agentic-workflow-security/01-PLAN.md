# Phase 32: SARIF, Policy, and Agentic Workflow Security

## Goal

Integrate Pramaan findings into existing security review systems and harden the
agentic workflow path from untrusted PR text to tools and CI sinks.

## Research Drivers

- GitHub code scanning accepts SARIF from existing CI systems.
- OPA is a common portable policy engine for CI/CD guardrails.
- Agentic workflow injection research expands the threat model beyond code diff
  contents into PR text, issue text, and comments.

## Tasks Covered

- SARIF export for Pramaan findings.
- OPA/Conftest policy parity or export.
- CI hardening expansion and CodeQL/security-scanner integration.
- Agentic workflow-injection checks.

## Files to Change

- `crates/pramaan-cli/`
- `crates/pramaan-core/`
- `schemas/`
- `.github/workflows/`
- `docs/policy.md`
- `docs/github-action.md`
- `docs/threat-model.md`
- `TASKS.md`

## Implementation Steps

1. Map risk IDs and receipt findings into SARIF rules and results.
2. Add `pramaan export sarif` and GitHub Action upload guidance.
3. Add OPA/Rego policy export or parity tests for default Pramaan policy.
4. Add checks for untrusted PR/issue/comment text flowing into agent prompts,
   workflow commands, shell commands, or tool arguments.
5. Add optional CodeQL/security-scanner evidence ingestion as warning-only
   correlation.

## Verification

- SARIF validates and imports into GitHub code scanning.
- OPA parity tests match the Rust policy decision for representative bundles.
- Agentic workflow-injection fixtures produce stable risk IDs.

## Exit Criteria

Pramaan evidence appears in the review surfaces security teams already use, and
workflow-injection risks are visible before external adoption expands.
