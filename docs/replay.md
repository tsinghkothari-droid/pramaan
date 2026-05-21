# Replay

Phase 28 adds a first replay contract for property/fuzz evidence:

```powershell
pramaan replay target/pramaan/fuzz --case "<stable-id>#<input-index>"
```

The command reads `differential-fuzz.json`, finds the recorded divergence, and
prints the input, base output, head output, classification, and rationale.

Current mode is `metadata_replay`: Pramaan replays the recorded generated case
from the evidence bundle so a reviewer or agent can inspect the exact failing
input. It does not yet re-execute a generated Hypothesis or fast-check harness.
That fuller execution path is split to the safe harness work because it needs
language-specific sandboxing, dependency installation, and timeout controls.

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
- `mode: metadata_replay` means the bundle preserved the case; it does not mean
  the original repository was re-executed.
- Missing property-testing tools must stay visible as skipped or residual risk,
  never as mitigation.
