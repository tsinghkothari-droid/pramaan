# Phase 26.2: Competitive Benchmark and Prior-Art Matrix

## Goal

Make Pramaan's public positioning evidence-grounded by comparing it against
adjacent GitHub tools and supply-chain primitives before claiming it is more
comprehensive.

## Why This Phase Exists

The public market already has AI PR reviewers, structural risk triage tools,
test-change monitors, quality aggregators, and attestation primitives. Pramaan
should not pretend those do not exist. The stronger claim is narrower:
Pramaan combines execution-grounded PR evidence, oracle integrity, policy
decisions, and proof-bundle trust in one auditable loop.

## Tools To Compare

- PR-Agent and similar AI PR reviewers.
- OpenReview-style sandboxed AI review loops.
- inspect-style structural risk triage.
- Testomatio/check-tests-style test-change detection.
- quality-monitor/reviewdog-style report aggregation.
- actions/attest, SLSA verifier, Sigstore, and in-toto primitives.

## Files To Change

- `docs/competitive-benchmark.md`
- `README.md`
- `STATUS.md`
- `docs/claim-audit.md`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Create a benchmark matrix with columns for problem solved, evidence type,
   execution grounding, oracle-integrity depth, signing/provenance, policy
   decisions, reviewer UX, and agent-harness support.
2. Separate competitors from reusable primitives so Pramaan can build on SLSA,
   Sigstore, in-toto, and GitHub attestations instead of rebranding them.
3. Add "what Pramaan should not duplicate" notes for each adjacent tool.
4. Audit README and marketing claims that say or imply "most comprehensive."
5. Add a refresh date and maintenance trigger before public Alpha and Serious
   v1.

## Verification

- `docs/competitive-benchmark.md` exists and cites every compared tool.
- Claim audit contains entries for any superiority or differentiation claims.
- README language stays honest: evidence bundle, not correctness proof.

## Exit Criteria

Pramaan has a public, defensible competitor map that explains exactly where it
is different, where it reuses primitives, and where it is still not shipped.
