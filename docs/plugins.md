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

Recorded divergences can be inspected with:

```powershell
pramaan replay target/pramaan/fuzz --case "<stable-id>#<input-index>"
```

This is metadata replay of the recorded case. Real Hypothesis/fast-check
re-execution remains split to the safe generated-harness phase.

## Trust Rules

- Plugins run as subprocess JSON adapters for v0.1. A plugin receives a JSON
  request on stdin and emits JSON receipts/artifacts on stdout or into its
  assigned output directory.
- Plugin receipts must include `plugin_identity` and `plugin_permissions`.
- Plugins may emit receipts and artifacts, but they must not modify prior
  receipts or bundle manifests.
- Plugin output paths must be relative bundle paths; parent traversal,
  backslashes, and absolute paths are rejected.
- Third-party plugins should use `subprocess`, `container`, or future `wasm`
  isolation. `in_process` is reserved for workspace-owned internal code.
- Adapter output should be hash-linked when it is used for a policy decision.

The protocol shape is captured in `schemas/plugin_protocol.schema.json`.
Bundle construction now rejects plugin receipts with high/critical trust
findings such as missing identity, dangerous write permissions, untrusted
unsigned provenance, no sandbox boundary, or path escape attempts.

Allowed v0.1 permissions:

```json
{
  "may_emit_receipts": true,
  "may_emit_artifacts": true,
  "may_read_previous_receipts": false,
  "may_modify_previous_receipts": false,
  "may_modify_manifest": false
}
```

Any plugin that needs broader permissions is out of scope until there is a
separate signed plugin registry and review process.

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
