# TypeScript Differential Fuzz Adapter

Phase 4 Plan 02 records a fast-check-compatible receipt model for TypeScript and
JavaScript pure functions. Until a repository has fast-check wiring available,
Pramaan uses the deterministic simulated runner and emits the same replay,
seed, corpus, and counterexample fields expected from the real adapter.

Discovery is deliberately conservative:

- eligible functions are inline `function` declarations with generated
  parameters;
- the body must contain one `return` expression;
- expressions are limited to integer arithmetic over parameters;
- calls, attribute access, imports, I/O, async/yield, globals, containers, and
  complex bodies become `not_applicable` evidence instead of unsafe execution.

Receipt fields map cleanly to a future fast-check runner:

- `seed` maps to `fc.assert(..., { seed })`;
- `replay_path` maps to fast-check replay path capture;
- `counterexample_path` records minimized failing examples;
- `corpus_hash` hashes the generated corpus used for base/head comparison.
