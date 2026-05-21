# Phase 32.5 Summary: Policy Pack Library and Enterprise Profiles

## Status

Completed the built-in policy pack slice on 2026-05-21.

## Landed

- Added built-in profile loading for:
  - `startup-fast`
  - `open-source-maintainer`
  - `security-sensitive`
  - `fintech-strict`
  - `private-preview`
- Added `hard_gate_risk_ids` to policy evaluation so stricter profiles can
  escalate security-sensitive residual risks.
- Added `pramaan policy list`.
- Added `pramaan policy explain <bundle> --profile <id>`.
- Added `policy-profile` input to the GitHub Action.
- Added `schemas/policy_profile.schema.json`.
- Added checked-in policy fixtures under `policy/`.
- Added `docs/policy-packs.md`.
- Added tests for profile selection and security-sensitive hard-risk
  escalation.

## Deferred Honestly

- External policy-file loading is not implemented.
- Rego execution/parity through OPA or Conftest remains future hardening.
- Action job failure is still controlled by `fail-on`; `policy-profile`
  currently controls policy explanation.

## Verification

- Targeted policy profile tests passed during implementation.
- Full required phase verification is recorded in the phase commit workflow.
