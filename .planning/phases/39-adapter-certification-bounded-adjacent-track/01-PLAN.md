# Phase 39: Adapter Certification as a Bounded Adjacent Track

## Goal

Keep MCP/agent adapter certification useful without letting it distract from
Pramaan's PR-verification core.

## Research Drivers

- Agent ecosystems need typed, auditable, reliable adapters.
- Pramaan's current moat is evidence-bundle verification; adapter work should
  reuse that engine instead of becoming a second product too early.

## Tasks Covered

- Adapter certification checks for names, descriptions, schemas, auth scopes,
  idempotency, retry behavior, rate limits, and auditability.
- Adapter proof-bundle examples.
- Agent-tool adapter failure taxonomy.

## Files to Change

- `docs/adapter-certification.md`
- `schemas/adapter-certification.schema.json`
- `examples/adapter-certification/`
- `corpus/`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Define adapter certification as an optional mode, not Alpha scope.
2. Reuse receipt and bundle primitives for adapter checks.
3. Add a small proof-bundle example for one adapter-like fixture.
4. Map adapter risks to a separate taxonomy section.
5. Add clear docs explaining when to defer adapter work.

## Verification

- Adapter certification examples run without affecting PR-verification gates.
- Docs keep adapter work adjacent and optional.
- No public copy implies the adapter registry exists in v0.1.

## Exit Criteria

Adapter certification has a safe place in the roadmap without stealing focus
from public Alpha and Serious v1.
