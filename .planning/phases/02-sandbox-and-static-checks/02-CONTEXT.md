# Phase 2: Sandbox and Static Checks - Context

**Status:** Ready for execution after Phase 1

Phase 2 turns the synthetic receipt skeleton into real environment and static-analysis evidence. It owns base/head worktrees, runtime/tool identity, dependency/config hashing, and Python/TypeScript/Rust static check adapters.

Locked decisions:

- Worktree creation and environment evidence belong in `crates/pramaan-sandbox`.
- Static plugin adapters should emit receipts through the Phase 1 core contract.
- Missing tools must produce `skipped` or `not_applicable` receipts with residual risk IDs, never quiet green.
- Hallucination categories follow the risk taxonomy: invented API, nonexistent import, undefined symbol, invalid parameter, resource mismatch, logic mismatch.

Deferred:

- Test oracle diffing moves to Phase 3.
- Mutation/fuzz execution moves to Phase 4.
