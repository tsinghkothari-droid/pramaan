# Benchmark Integrity Plan

Pramaan's adversarial corpus can become a target. Agents may overfit fixture
names, hidden-test assumptions, known replay commands, or expected receipt
strings. Benchmark-integrity work exists to detect those gaming behaviors.

## Planned Checks

- Mutate fixture names, paths, and comments while preserving behavior.
- Mutate hidden-test assumptions so hard-coded corpus answers stop working.
- Run fixture variants where ordinary CI still passes but oracle evidence
  should remain stable.
- Compare Pramaan findings before and after corpus perturbation.
- Flag agents or patches that only pass the public fixture shape.

## Current Status

The current repo has scenario specs for benchmark overfitting and verifier abuse
under `corpus/adversarial-scenarios-v0.1.json` and
`corpus/verifier-abuse-fixtures.v0.1.json`. A full benchmark-integrity mutation
harness is not implemented yet and remains a Phase 40 hardening item.
