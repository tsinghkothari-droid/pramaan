# Phase 32.75: Anti-Gaming and Verifier-Abuse Hardening

## Goal

Make Pramaan resistant to PRs, agents, or plugins that try to game the
verification surface itself.

## Why This Phase Exists

Once Pramaan becomes a gate, agents and malicious contributors have incentives
to optimize around it. The verifier must detect changes that weaken tests,
relax config, hide skipped stages, forge tool output, or tamper with receipts.

## Threat Categories

- Careless AI weakens the test oracle without noticing.
- Overfitted AI edits code to satisfy known fixtures while missing behavior.
- Malicious contributor tampers with verification configuration or artifacts.
- Malicious or compromised plugin poisons receipts across runs.

## Files To Change

- `examples/`
- `corpus/`
- `docs/threat-model.md`
- `docs/risk-taxonomy.md`
- `docs/policy.md`
- `schemas/`
- `crates/pramaan-core/src/lib.rs`
- `crates/pramaan-cli/src/main.rs`

## Implementation Steps

1. Add malicious PR fixtures for relaxed config, removed hooks, skipped tests,
   altered fixtures, poisoned snapshots, and changed verification scripts.
2. Add verifier-abuse fixtures for artifact path escape, receipt tampering,
   hidden skipped stages, fake tool output, timeout laundering, and benchmark
   overfitting.
3. Add policy rules that escalate verifier-surface changes by profile.
4. Ensure skipped required stages cannot improve confidence.
5. Ensure plugins and PR code cannot overwrite earlier receipts or manifests.
6. Feed accepted scenarios into Phase 33 and Phase 40 corpus accounting.

## Verification

- Anti-gaming fixtures emit stable risk IDs and expected policy outcomes.
- Bundle verification rejects receipt/manifest tampering.
- Confidence output discounts or blocks missing required evidence.

## Exit Criteria

Pramaan's own gate is harder to game, and the adversarial corpus has concrete
verifier-abuse scenarios instead of only application-code bugs.
