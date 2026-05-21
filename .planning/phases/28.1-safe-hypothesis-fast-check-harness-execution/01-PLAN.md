# Phase 28.1: Safe Hypothesis and fast-check Harness Execution

## Goal

Execute real Hypothesis and fast-check campaigns from generated harnesses where
eligible pure functions and project dependencies make that safe.

## Why This Split Exists

Phase 28 added a replay CLI for recorded generated cases and preserved the
deterministic fallback evidence. It did not honestly ship sandboxed generated
Hypothesis/fast-check execution. That needs stricter language-specific harness
generation and dependency/runtime isolation.

## Tasks

1. Generate harnesses only for pure-function candidates with no ambient IO,
   network, subprocess, imports with side effects, async/yield, or complex body
   shapes.
2. Run Python Hypothesis with bounded examples, deadline, seed, and example
   database path.
3. Run TypeScript fast-check with bounded runs, seed, timeout, and generated
   case capture.
4. Record tool versions, generated counts, shrink/counterexample metadata,
   timeout status, and raw-output digests.
5. Preserve deterministic replay fallback when tools or safe harness conditions
   are missing.
6. Add passing, failing, timeout, missing-tool, and no-eligible-code fixtures.

## Exit Criteria

- Tool-backed property/fuzz receipts use `tool_backed=true` only after real
  Hypothesis or fast-check execution.
- Failing generated cases can be replayed or re-run from recorded metadata.
- Missing tools remain visible residual risk.
