# Pramaan Confidence Vote

Decision: **fail**

Confidence score: **38/100**  
Residual risk score: **62/100**  
Calibration: **uncalibrated**

This is auditable residual-risk evidence, not a proof that the code is correct.

## Hard Gates

- `HG-ORACLE-001` from `oracle_integrity`: Oracle integrity reported weakened, deleted, skipped, or sensitive test evidence. (`R-014`)

## Votes

| Stage | Status | Vote | Cluster | Weight | Why |
|---|---:|---:|---|---:|---|
| `oracle_integrity` | `failed` | `risky` | `test_quality` | 270 | oracle_integrity is failed and reports residual risks R-014. |
| `static_hallucination` | `passed` | `safe` | `static_semantic` | 140 | static_hallucination passed with 1 mitigated risks and no residual risks. |

## Top Risk Drivers

- `oracle_integrity` impact 270: oracle_integrity reported status failed with residual risks R-014. (`R-014`)

## Top Confidence Drivers

- `static_hallucination` impact 140: static_hallucination passed with 1 mitigated risk references. (`R-038`)

## Statistical Notes

- `mutation_python_mutmut` `mutation_kill_rate`: estimate 1000000 ppm, conservative bound 342372 ppm over 2/2 (`wilson_lower_bound_95_percent`)

## Limitations

- Confidence uses deterministic starter weights and is marked uncalibrated until Phase 34 supplies labeled outcomes.
- The artifact aggregates receipt evidence; it is not a proof that the PR is correct.
