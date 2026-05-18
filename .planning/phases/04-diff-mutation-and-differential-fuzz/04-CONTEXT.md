# Phase 4: Diff Mutation and Differential Fuzz - Context

**Status:** Ready for execution after Phase 3

Phase 4 proves test quality and regression resistance. It adds mutation adapters and differential property/fuzz checks, scoped to changed files and bounded by strict budgets. The goal is useful CI evidence, not exhaustive testing.

Locked decisions:

- Mutation is diff-scoped by default.
- Receipts must preserve skipped, unviable, timeout, cache reuse, and survivor classifications.
- Property/fuzz receipts must include seeds, replay paths where possible, corpus hashes, and divergence scope.
