# Plugin And Adapter Evidence

Pramaan v0.1 keeps plugins intentionally boring: subprocess adapters may emit
receipts and artifacts, but skipped or missing adapters must remain visible.

## Mutation Adapters

The current mutation adapters are orchestrator wrappers around existing tools:

| Language | Tool | Evidence mode |
| --- | --- | --- |
| Python | `mutmut` | `tool_executed`, `missing_tool`, or `not_applicable` |
| TypeScript/JavaScript | StrykerJS | `tool_executed`, `missing_tool`, or `not_applicable` |
| Rust | `cargo-mutants` | `tool_executed`, `missing_tool`, or `not_applicable` |

When a tool is missing, Pramaan writes a skipped receipt with the command it
would have run, the changed-file filter, the timeout, and the still-open risk
IDs. It does not place mutation risk IDs in `mitigated_risks`.

## Property And Differential Fuzz Adapters

The differential fuzz stage discovers conservative pure-function candidates and
compares base/head behavior on a deterministic generated corpus. The evidence
records seeds, corpus hash, replay path, generated input count, and divergence
classification.

The receipt also records adapter availability:

- `hypothesis_available`
- `fast_check_available`
- `tool_backed`
- `adapter`

`tool_backed=false` means Pramaan produced deterministic replay evidence, not a
real Hypothesis or fast-check campaign. That distinction is part of the product
surface: absence of a tool is evidence the reviewer needs, not a pass.

## Trust Rules

- Plugins should never edit prior receipts or bundle manifests.
- Plugin identity and permissions belong in receipts before third-party plugins
  are accepted.
- Risky parsers, test runners, mutation engines, and fuzzers should run behind
  stronger sandbox boundaries before enterprise use.
- Adapter output should be hash-linked when it is used for a policy decision.

## Agent Harness Boundary

Agent harnesses are consumers of Pramaan evidence, not privileged plugins. A
Claude Code/Codex/Cursor-style wrapper may call:

```powershell
pramaan agent done-gate --base <base-ref> --head <head-ref> --out target/pramaan-agent
```

or:

```powershell
pramaan agent explain --bundle target/pramaan-agent
```

The returned `agent_decision` JSON is derived from receipts and policy results.
Harnesses must not rewrite receipts, manifests, plugin identity, or policy
outcomes. If the decision is `block`, the agent should stop and repair the
blocking stage or ask a human for explicit override.
