# Phase 38: Multi-Agent Provenance and Handoff Tracking

## Goal

Represent modern agentic coding workflows where one agent writes code, another
reviews it, a third adds tests, and a human merges the result.

## Research Drivers

- Coding-agent workflows increasingly involve multiple models and tools.
- Agent-author attribution becomes more useful when handoffs and intermediate
  commits are preserved.

## Tasks Covered

- Multi-agent provenance chains.
- Intermediate commit attribution.
- Handoff metadata.
- Final human-review context.

## Files to Change

- `schemas/`
- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `docs/provenance.md`
- `examples/`
- `TASKS.md`

## Implementation Steps

1. Extend provenance docs around author, reviewer, test-author, and final-human
   roles without breaking existing receipt schema compatibility.
2. Capture commit-level attribution when available from commit metadata, PR
   labels, Action inputs, or local config.
3. Record handoff events as evidence, not as trust by default.
4. Add examples for single-agent, multi-agent, and unknown-agent PRs.

## Verification

- Fixtures preserve current single-agent behavior.
- Multi-agent examples render clear bundle summaries.
- Unknown provenance is explicit and does not silently downgrade risk.

## Exit Criteria

Pramaan can explain who or what produced each meaningful change and which human
accepted the residual risk.
