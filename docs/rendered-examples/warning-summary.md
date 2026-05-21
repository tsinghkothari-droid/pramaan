# Warning Summary Example

```text
Pramaan Proof Bundle

Final status: inconclusive
Policy profile: open-source-maintainer

Actionable warnings:
- mutation: skipped because mutmut was not installed.
- fuzz: tool_backed=false; deterministic replay evidence exists, but no
  Hypothesis/fast-check campaign executed.

Residual risk families:
- R-068 mutation_survivor_risk
- R-074 generated_input_coverage_risk

Reviewer action:
- Install missing tools or record why the team accepts this evidence gap.
- Do not treat skipped stages as mitigation.
```
