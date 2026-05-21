# Phase 28: Tool-Backed Property, Fuzz, and Replay

## Goal

Turn deterministic replay evidence into real budgeted Hypothesis and fast-check
campaigns where safe harness generation is possible.

## Research Drivers

- Property-generated solver work supports invariant and property discovery over
  example-only tests.
- Mutation-guided test generation research suggests surviving mutants should
  drive follow-up tests and replay cases.

## Tasks Covered

- Python Hypothesis generated harness execution.
- TypeScript fast-check generated harness execution.
- Replay command contracts for seeds and counterexamples.
- Budget, timeout, and skipped-tool evidence.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `plugins/python/`
- `plugins/typescript/`
- `examples/`
- `docs/replay.md`
- `TASKS.md`

## Implementation Steps

1. Define safe harness-generation limits: pure functions only, no ambient IO,
   no network, bounded input sizes, and explicit timeout.
2. Execute Hypothesis when Python dependencies and eligible functions exist.
3. Execute fast-check when package metadata and eligible exports exist.
4. Record seeds, generated counts, corpus hashes, counterexamples, shrink data,
   and tool version.
5. Add `pramaan replay` acceptance tests for recorded failing cases.
6. Keep deterministic replay as fallback evidence when tools are unavailable.

## Verification

- Fixtures include passing, failing, timeout, missing-tool, and no-eligible-code
  cases.
- Missing Hypothesis or fast-check emits skipped or not-applicable receipts.
- Failing generated cases can be replayed from recorded metadata.

## Exit Criteria

Pramaan can distinguish real property/fuzz campaigns from deterministic
fallbacks and replay the evidence that drove a failure.
