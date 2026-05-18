# Pitfalls Research: Pramaan

## Pitfall: Overclaiming Correctness

Warning signs:

- Marketing says Pramaan proves code is correct.
- One aggregate score hides which checks actually ran.

Prevention:

- Use receipt-backed claims only.
- Keep "not a correctness proof" in product docs and bundle summary.

Phase:

- Phase 1 and Phase 5.

## Pitfall: Flaky Receipts

Warning signs:

- Same PR produces different stage status without source changes.
- Receipts omit seeds, tool versions, or corpus hashes.

Prevention:

- Require seeds, versions, hashes, timeouts, and environment evidence in schemas.
- Add replay checks early.

Phase:

- Phase 1.

## Pitfall: Test Tampering Detection Too Shallow

Warning signs:

- Only detects deleted test files, not weakened assertions or skipped cases.
- Snapshot or fixture changes bypass the oracle stage.

Prevention:

- AST-based comparison for test assertions.
- Explicit snapshot/fixture diff classification.
- Demo target should include weakened assertion and skipped-test variants.

Phase:

- Phase 3.

## Pitfall: Mutation Testing Too Slow

Warning signs:

- Whole-repo mutation exceeds useful CI budgets.
- Developers disable the stage.

Prevention:

- Scope mutation to changed files and directly affected tests.
- Emit partial/timeout receipts rather than hiding timeouts.

Phase:

- Phase 4.

## Pitfall: Correlated Blind Spots

Warning signs:

- Fuzz, mutation, and critic all depend on the same weakened oracle or wrong issue scope.
- Product copy claims independent probabilities multiply.

Prevention:

- Phrase stages as diverse failure modes, not statistically independent.
- Include scope and oracle-quality warnings in bundle summary.

Phase:

- Phase 5.

## Pitfall: Plugin Sprawl Before Protocol Stability

Warning signs:

- New language support changes receipt format.
- Plugins duplicate orchestration logic.

Prevention:

- Freeze minimal plugin protocol before adding Go/Java.
- Build Python/TS/Rust deeply first.

Phase:

- Phase 1 and Phase 6.
