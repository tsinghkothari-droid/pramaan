# Python Differential Fuzz Adapter

Phase 4 Plan 02 records a Hypothesis-compatible receipt model for Python pure
functions. The current adapter starts with safe discovery and deterministic
simulation so Pramaan can emit replayable evidence even when a project does not
vendor Hypothesis or an example database.

Discovery is deliberately conservative:

- eligible functions are top-level `def` functions with generated parameters;
- the body must be a single `return` expression;
- expressions are limited to integer arithmetic over parameters;
- calls, attribute access, imports, I/O, async/yield, globals, containers, and
  complex bodies become `not_applicable` evidence instead of unsafe execution.

Receipt fields map cleanly to a future Hypothesis runner:

- `seed` maps to `derandomize` or an explicit Hypothesis seed;
- `example_database_path` records the Hypothesis example database location;
- `counterexample_path` records minimized failing examples;
- `replay_path` records deterministic inputs for local reproduction;
- `corpus_hash` hashes the generated corpus used for base/head comparison.
