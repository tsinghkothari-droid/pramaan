# AI Evidence-Seeking Probe Generator

Pramaan can use AI as a search assistant for missing evidence, but never as the
judge. The boundary is:

```text
bundle receipts -> probe plan -> sandbox execution -> receipt evidence
```

Phase 28.25 adds the provider-neutral planning step:

```powershell
pramaan probe plan --bundle target/pramaan
```

The command reads the bundle manifest and receipts, groups residual and skipped
risk IDs, and writes `probes/ai-probe-plan.json` plus an
`ai_probe_generation` receipt. The plan records:

- `prompt_hash` over the evidence used to ask for probes;
- provider metadata with `trusted_for_decision=false`;
- probe kind, language, risk IDs, target files, and candidate skeleton;
- `sandbox_status=requires_execution`;
- `kept_or_rejected=pending_execution`.

No AI-generated probe mitigates risk until a later sandbox stage compiles/runs
it and records the result. The current plan is useful because reviewers and
agents can see exactly which missing evidence should be pursued next, without
turning model output into proof.

## Probe Kinds

- `regression_assertion`
- `property_invariant`
- `differential_input`
- `security_sink_source_check`
- `mutation_targeted_test`
- `fixture_snapshot_challenge`

## Honest Limits

- Phase 28.25 does not call a hosted model.
- Phase 28.25 does not execute generated tests.
- A probe with `requires_execution` is residual risk, not mitigation.
- Provider output must remain reproducible enough to audit through prompt hash,
  model/provider metadata, and recorded candidate code.
