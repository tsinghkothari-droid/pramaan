# Phase 28.15: Fuzz Harness Truthfulness Review Gate

## Goal

Close the truthfulness gaps found by an independent local review of Phase 28.1
before Pramaan relies on tool-backed Hypothesis or fast-check evidence in
confidence, policy, or public claims.

## Why This Insert Exists

Phase 28.1 added generated Hypothesis and fast-check harness execution, but the
review found that green tests do not prove the harness evidence is wired into
the actual differential-fuzz verdict. This is exactly the class of issue
Pramaan exists to catch: a repository can be technically green while still
emitting misleading verification evidence.

## Review Findings To Resolve

1. Tool-backed harness failures are written to raw output but not promoted into
   `divergences`, replay data, counterexamples, residual risks, or policy
   decisions.
2. The advertised harness timeout is ignored by `run_with_timeout`, so Python or
   Node subprocesses can hang CI indefinitely.
3. Harness process errors currently abort the whole fuzz command instead of
   emitting structured failed/skipped tool evidence.
4. The generated JavaScript harness uses dynamic `Function(...)` evaluation over
   expressions from changed code; this should be replaced or strictly isolated.
5. `tool_version` is parsed from a human-readable reason string instead of a
   structured field.
6. Deterministic corpus input count and tool-generated case count are conflated
   in reviewer-facing metadata.

## Tasks

1. Parse Hypothesis and fast-check raw-output `failures` and convert them into
   canonical divergence/counterexample/replay entries.
2. Implement a real subprocess timeout with kill-on-expiry behavior and timeout
   evidence.
3. Convert harness nonzero exits into structured receipt evidence unless policy
   explicitly chooses to hard-fail the stage.
4. Remove dynamic JS `Function(...)` evaluation or run it behind a stricter
   isolated evaluator with a documented residual risk.
5. Carry `tool_version`, `generated_cases`, `raw_output_digest`, and timeout
   status as structured fields.
6. Split `deterministic_input_count` from `tool_generated_case_count`.
7. Add positive and negative tests where installed-tool harness failures affect
   the final fuzz receipt and confidence/policy interpretation.

## Exit Criteria

- A failing generated Hypothesis/fast-check case changes the fuzz receipt
  verdict and appears in replay/counterexample artifacts.
- Timeout behavior is enforced by an executable test.
- Tool subprocess failure produces auditable evidence, not an unstructured
  panic-like command abort.
- Public/task wording no longer claims Phase 28.1 is fully closed until these
  checks pass.
