# Agent Completion Rules

Pramaan is an evidence gate for AI-authored code changes. Coding agents should
run the gate before claiming that a task is done.

## Required Gate

Use:

```powershell
pramaan agent done-gate --base <base-ref> --head <head-ref> --out target/pramaan-agent
```

Then read:

```text
target/pramaan-agent/agent-decision.json
```

## Agent Behavior

- `pass`: you may summarize the Pramaan evidence, but do not claim the code is
  proven correct.
- `warn`: do not present the work as cleanly verified. Report every warning and
  residual risk, then ask for human acceptance if those risks matter.
- `block`: stop. Do not claim completion. Fix the blocking stage or ask the
  human for an explicit override.

The agent cannot override Pramaan by saying the code "looks right." Only a human
reviewer can accept residual or blocking risk, and that acceptance should be
recorded in the bundle once reviewer-override capture is enabled.

## Existing Bundle

If a bundle already exists, use:

```powershell
pramaan agent explain --bundle target/pramaan-agent
```

This prints the same deterministic JSON decision without rerunning verification.
