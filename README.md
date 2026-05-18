# Pramaan

Pramaan is a verification system for AI-generated code changes.

It does not claim that code is correct. It creates an auditable proof bundle that shows what was checked, what evidence was produced, and what risks remain.

## Why This Exists

AI coding agents can make pull requests that look green while hiding real problems:

- tests were weakened or skipped;
- the original bug was never reproduced;
- a fake API or import was invented;
- a shallow test was added that does not catch the bug;
- the fix works for one path but breaks another;
- CI passed, but the evidence is impossible to audit later.

Pramaan is built for that gap.

## What Pramaan Does

For a pull request, Pramaan should:

1. Capture what the PR claims to change.
2. Create isolated base/head worktrees.
3. Run static and hallucination checks.
4. Detect test-oracle tampering.
5. Run diff-scoped mutation checks.
6. Run replayable property/fuzz/differential checks.
7. Emit signed or signable receipts for every stage.
8. Summarize mitigated and residual risks using stable risk IDs.

Example summary:

```text
Claim: Fix invoice rounding for mixed tax rates.

Evidence:
- Original failing test now passes unchanged.
- No assertions were weakened.
- Static checks found no invented imports or undefined symbols.
- Mutation killed 91% of changed-line mutants.
- Differential property checks found no unexpected divergence.

Residual risks:
- R-049 concurrency not exercised.
- R-057 performance not benchmarked.
- R-081 formal verification not applicable.
```

## Product Boundary

Pramaan says:

> Here is the evidence for this code change.

Pramaan does not say:

> This code is definitely correct.

That distinction is the product.

## Planned v1 Pipeline

```text
PR diff
  -> Sandbox
  -> Claim Scope
  -> Static/Hallucination
  -> Oracle Integrity
  -> Delta Mutation
  -> Property + Differential Fuzz
  -> Bundle Signing / Verification
  -> GitHub Action Summary
```

## Repository State

This repository currently contains the planning system for Pramaan:

- [.planning/PROJECT.md](.planning/PROJECT.md)
- [.planning/REQUIREMENTS.md](.planning/REQUIREMENTS.md)
- [.planning/ROADMAP.md](.planning/ROADMAP.md)
- [.planning/research/](.planning/research)
- [.planning/phases/](.planning/phases)
- [.planning/AUTONOMOUS_BUILD_COMMAND.md](.planning/AUTONOMOUS_BUILD_COMMAND.md)

## Autonomous Build

The planned autonomous build sequence is documented here:

[.planning/AUTONOMOUS_BUILD_COMMAND.md](.planning/AUTONOMOUS_BUILD_COMMAND.md)

## License

License decision is pending. The intended direction is open source for the core verification engine and hosted services for enterprise execution, managed attestations, and adapter certification.
