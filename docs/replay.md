# Replay

Phase 28 adds a first replay contract for property/fuzz evidence:

```powershell
pramaan replay target/pramaan/fuzz --case "<stable-id>#<input-index>"
```

The command reads `differential-fuzz.json`, finds the recorded divergence, and
prints the input, base output, head output, classification, and rationale.

Current replay mode is `metadata_replay`: Pramaan replays the recorded generated
case from the evidence bundle so a reviewer or agent can inspect the exact
failing input.

Phase 28.1 adds bounded Hypothesis/fast-check harness execution when the
property-testing tool is actually available and Pramaan found safe pure-function
candidates. The replay command still prints recorded cases; the harness output
is preserved as audit metadata and raw-output digests in
`differential-fuzz.json`.

## Case IDs

Case IDs are deterministic:

```text
<stable_id>#<input.index>
```

For convenience, `--case` also accepts the stable ID or function name when that
matches a unique recorded divergence.

## Honest Limits

- `tool_backed=false` remains deterministic differential evidence, not a real
  Hypothesis or fast-check campaign.
- `tool_backed=true` means a bounded generated harness ran; it still does not
  prove correctness.
- `mode: metadata_replay` means the bundle preserved the case; it does not mean
  the original repository was re-executed by `pramaan replay`.
- Missing property-testing tools must stay visible as skipped or residual risk,
  never as mitigation.
