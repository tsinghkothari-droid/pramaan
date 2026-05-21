# Pramaan proof bundle

Final status: **inconclusive**

Compared refs: `HEAD~1` -> `3b089eedf6ec27d216351e2566839a7543008490`

Bundle: `bundle_1779371136`

Manifest digest: `sha256:3cae6f3ca2d09c46526d2668df23ce1ba9f19419b8311d3b3aea790ced37a28b`

Policy decision: **warning**

## Failed, skipped, or incomplete stages

| Stage | Status | Residual risk families | Mitigated risk families |
| --- | --- | --- | --- |
| none | none | none | none |

## Risk families

| Bucket | Families |
| --- | --- |
| mitigated | claim_scope (8), oracle_integrity (10), sandbox_reproducibility (8), static_hallucination (3), property_fuzz (8), bundle_integrity (3) |
| residual | sandbox_reproducibility (2), static_hallucination (5), property_fuzz (2), bundle_integrity (1) |
| skipped | none |
| not_applicable | static_hallucination (1), bundle_integrity (1) |

## Bundle evidence

- Receipts: 13
- Artifacts: 14
- Artifact attestation: slsa: present
- Residual risk note: Included receipts report residual risk IDs: R-028, R-029, R-031, R-032, R-033, R-039, R-040, R-073, R-077, R-090.

## Policy

Policy: `pramaan-default-v0`

Hard failures:

none

Warnings:

- not_applicable_risk:claim_scope:R-081
- not_applicable_risk:static_python_mypy:R-038
- not_applicable_risk:static_python_pyright:R-038
- not_applicable_risk:static_python_ruff:R-038
- not_applicable_risk:static_typescript_lint:R-038
- not_applicable_risk:static_typescript_tsc:R-038
- partial_evidence:claim_scope
- residual_risk:claim_scope:R-090
- residual_risk:differential_fuzz:R-073,R-077
- residual_risk:sandbox_setup:R-028,R-029,R-033,R-031,R-032
- residual_risk:static_python_compileall:R-039,R-040
- residual_risk:static_python_mypy:R-031,R-032
- residual_risk:static_python_pyright:R-031,R-032
- residual_risk:static_python_ruff:R-031,R-032
- residual_risk:static_rust_cargo_check:R-039,R-040
- residual_risk:static_rust_cargo_clippy:R-039,R-040
- residual_risk:static_rust_cargo_test_no_run:R-039,R-040
- residual_risk:static_typescript_lint:R-031,R-032
- residual_risk:static_typescript_tsc:R-031,R-032
- stage_incomplete:static_python_mypy:not_applicable
- stage_incomplete:static_python_pyright:not_applicable
- stage_incomplete:static_python_ruff:not_applicable
- stage_incomplete:static_typescript_lint:not_applicable
- stage_incomplete:static_typescript_tsc:not_applicable

## CLI tail

```text
  - residual_risk:static_rust_cargo_check:R-039,R-040
  - residual_risk:static_rust_cargo_clippy:R-039,R-040
  - residual_risk:static_rust_cargo_test_no_run:R-039,R-040
  - residual_risk:static_typescript_lint:R-031,R-032
  - residual_risk:static_typescript_tsc:R-031,R-032
  - stage_incomplete:static_python_mypy:not_applicable
  - stage_incomplete:static_python_pyright:not_applicable
  - stage_incomplete:static_python_ruff:not_applicable
  - stage_incomplete:static_typescript_lint:not_applicable
  - stage_incomplete:static_typescript_tsc:not_applicable
waived:
  none
```
