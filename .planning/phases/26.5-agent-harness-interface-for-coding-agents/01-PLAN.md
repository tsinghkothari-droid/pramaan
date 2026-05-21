# Phase 26.5: Agent Harness Interface for Coding Agents

## Goal

Make Pramaan directly usable by Claude Code, Codex, Cursor-style coding agents,
and custom agent harnesses before they claim a task is complete.

## Why This Phase Exists

Pramaan should not only be a CI-after-the-fact verifier. The strongest product
shape is an evidence gate that agents call during their own workflow:

```text
agent edits code -> pramaan agent done-gate -> pass/warn/block JSON
```

The final gate remains deterministic. AI agents may ask Pramaan what to fix,
but they cannot self-certify completion.

## Files To Change

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `schemas/agent_decision.schema.json`
- `docs/agent-harness.md`
- `docs/plugins.md`
- `AGENTS.md`
- `.claude/commands/pramaan-done.md`
- `.claude/settings.json` or example hook docs, if safe to add as template
- `examples/agent-harness/`

## Implementation Steps

1. Add `pramaan agent done-gate --base <ref> --head <ref> --out <dir>`.
2. Add `pramaan agent explain --bundle <path>` for machine-readable next
   actions.
3. Define `agent_decision.schema.json`:
   `decision`, `reason`, `bundle_path`, `blocking_stages`, `warnings`,
   `required_actions`, `agent_message`, `human_override_allowed`.
4. Add deterministic decisions:
   - `block`: hard policy failure, oracle weakening, tampered bundle, missing
     required stage, exhausted budget, unsafe skipped security stage.
   - `warn`: residual risks, missing optional tools, partial evidence.
   - `pass`: required stages present, no hard failures, no unaccepted blockers.
5. Add `AGENTS.md` instructions telling Codex-style agents not to claim done
   unless `done-gate` passes or the human explicitly overrides.
6. Add Claude Code command/hook templates that run the done gate at completion.
7. Add tests proving blocked decisions emit actionable repair instructions.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- JSON schema validates sample `agent_decision` fixtures.
- A blocked oracle fixture returns `decision=block` and a clear agent message.

## Exit Criteria

Claude Code, Codex, or any coding agent can call Pramaan as a completion gate
and receive a compact JSON answer: pass, warn, or block.
