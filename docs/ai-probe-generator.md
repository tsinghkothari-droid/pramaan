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

Phase 28.26 adds a bounded execution step:

```powershell
pramaan probe execute --plan target/pramaan/probes/ai-probe-plan.json --bundle target/pramaan
```

The command materializes each candidate under `probes/executed/sandbox/`, runs a
language-native command with a timeout, captures stdout/stderr digests, and
writes:

- `probes/executed/ai-probe-plan.executed.json`
- `probes/executed/ai-probe-execution.json`
- an updated `ai_probe_generation` receipt

Accepted probes must pass all of these checks:

- contain the `pramaan-accepted-probe` marker;
- avoid blocked network/process/filesystem tokens;
- bind to changed behavior through a risk ID, target basename, or
  `pramaan-bind`;
- compile or run successfully under the language command.

Rejected probes stay in the artifact with `rejection_reason`. This is
intentional: a rejected generated probe is useful reviewer evidence about what
Pramaan refused to trust.

## Probe Kinds

- `regression_assertion`
- `property_invariant`
- `differential_input`
- `security_sink_source_check`
- `mutation_targeted_test`
- `fixture_snapshot_challenge`

## Honest Limits

- Phase 28.25 does not call a hosted model.
- Phase 28.26 executes only safe-marker bounded probes; arbitrary generated code
  is rejected rather than run.
- Rust probes are compile-checked first; deeper test-harness integration remains
  future language-depth work.
- A probe with `requires_execution` is residual risk, not mitigation.
- A rejected probe is evidence, not failure of the whole bundle.
- Provider output must remain reproducible enough to audit through prompt hash,
  model/provider metadata, and recorded candidate code.
