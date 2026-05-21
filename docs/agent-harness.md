# Agent Harness

Pramaan's agent harness lets Claude Code, Codex, Cursor-style agents, and custom
automation call the same evidence gate before they claim a pull request is done.

The command is deterministic:

```powershell
pramaan agent done-gate --base <base-ref> --head <head-ref> --out target/pramaan-agent
```

It runs `pramaan verify`, reads the generated bundle manifest, evaluates the
default policy, and writes:

```text
target/pramaan-agent/agent-decision.json
```

The JSON follows `schemas/agent_decision.schema.json`:

```json
{
  "schema_version": "pramaan.agent_decision.v1",
  "decision": "block",
  "reason": "Pramaan policy blocked completion with 1 hard failure(s).",
  "bundle_path": "target/pramaan-agent",
  "blocking_stages": ["oracle_integrity"],
  "warnings": [],
  "required_actions": [
    "oracle_integrity: Restore or strengthen the changed tests/fixtures, then rerun Pramaan oracle and the agent done gate."
  ],
  "agent_message": "Stop. Do not claim the task is done. Fix the blocking Pramaan findings or ask the human for an explicit override.",
  "human_override_allowed": true
}
```

## Decisions

| Decision | Meaning | Agent behavior |
| --- | --- | --- |
| `pass` | Required stages passed and there are no policy warnings. | Summarize evidence, but never say Pramaan proved correctness. |
| `warn` | Required stages exist, but residual/skipped/partial evidence remains. | Report the warnings and ask for human acceptance where material. |
| `block` | A hard gate failed, a required stage is missing, or a budget was exhausted. | Stop and fix the stage before claiming completion. |

`pramaan agent explain --bundle <path>` reads an existing bundle and prints the
same decision JSON without rerunning verification.

## Trust Boundary

The harness is not a second AI reviewer. It does not judge code style or infer
correctness. It maps Pramaan receipts and policy outcomes into completion
instructions that an agent can follow. Human override is intentionally explicit:
an agent may surface the override option, but it cannot grant the override to
itself.
