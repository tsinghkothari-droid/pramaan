# Public Review Readiness Report - 2026-05-21

**Decision:** READY_FOR_PUBLIC_REVIEW_WITH_RISKS

This decision means Pramaan is ready for external technical readers to inspect,
run, and try to break the current evidence-bundle loop. It does not mean
production-ready, Serious v1, or proof of correctness.

## Scope Reviewed

- Phase 26.1 live GitHub Action proof
- Phase 26.2 competitive benchmark / prior-art matrix
- Phase 26.3 competitor-gap fixtures
- Phase 26.4 minimum lovable verifier loop
- Phase 27.1 parser metadata and full-AST dependency decision
- Phase 28.1 bounded Hypothesis / fast-check execution
- Phase 28.15 fuzz harness truthfulness review gate
- Phase 28.26 sandbox execution for generated probes
- Phase 32.75 anti-gaming / verifier-abuse hardening
- Phase 35.5 local HTML / Markdown reviewer report

## Why It Is Ready With Risks

- The public-review wedge is coherent: one command can produce receipts,
  bundles, policy/confidence evidence, and a reviewer-facing report.
- The main demo remains honest: weakened tests and oracle drift are exposed as
  evidence ordinary green CI can miss.
- Phase 28.15 fixed the remaining fuzz truthfulness blocker: harness failures
  now affect canonical fuzz evidence, timeouts are enforced, harness errors are
  receipts, dynamic generated JavaScript evaluation is removed, and tool-backed
  versus deterministic case counts are distinct.
- Missing tools, skipped stages, rejected probes, and timeouts remain visible
  residual risk instead of successful evidence.

## Required Verification

Passed locally on 2026-05-21:

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base --head-repo examples\fixtures\fuzz\head --claim-scope examples\fixtures\fuzz\claim_scope.json --out target\pramaan-fuzz-phase28.15 --seed 1337`
- `node scripts/check-fuzz-harness-evidence.mjs target\pramaan-fuzz-phase28.15\differential-fuzz.json`

## Residual Risks To State Publicly

- Pramaan produces auditable evidence bundles, not correctness proof.
- Production Sigstore/cosign identity and full signed-attestation trust are not
  complete.
- Enforced container isolation for arbitrary untrusted PR code and property
  tools remains future hardening.
- Full compiler-AST oracle integrations are not complete; current parser work
  is a documented subset with metadata and fallback evidence.
- Hypothesis and fast-check execution depends on installed tools and eligible
  bounded pure-function candidates.
- The adversarial corpus is useful but not yet the 100-scenario Serious v1
  corpus.
- Serious v1 remains gated on Phase 40.

## Public Review Ask

Ask reviewers to inspect whether the receipts are truthful, whether skipped
tools and residual risks are visible enough, whether the weakened-test demo is
convincing, and whether any public copy implies correctness or production
readiness beyond what the evidence supports.
