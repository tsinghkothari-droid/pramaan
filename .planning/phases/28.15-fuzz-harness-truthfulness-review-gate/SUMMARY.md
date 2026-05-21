# Phase 28.15 Summary: Fuzz Harness Truthfulness Review Gate

**Status:** PASS_WITH_RISKS

**Completed:** 2026-05-21

## What Landed

- Harness-discovered Hypothesis/fast-check failures now become canonical fuzz
  divergences before replay and counterexample artifacts are written.
- Python and Node harness subprocesses now use bounded execution with
  kill-on-timeout behavior.
- Harness nonzero exits, spawn failures, and timeouts are converted into
  structured adapter evidence instead of aborting the whole fuzz command.
- Generated JavaScript harnesses no longer use dynamic `Function(...)`
  evaluation; the private-preview path uses a small bounded arithmetic
  evaluator.
- Fuzz receipts now separate deterministic corpus inputs from tool-generated
  case counts and carry structured tool version, raw-output digest, harness
  path, raw-output path, execution status, and timeout metadata.
- `scripts/check-fuzz-harness-evidence.mjs` now validates truthful evidence
  shape for both tool-backed and deterministic fallback fuzz receipts.

## Verification

- `cargo fmt --check`
- `cargo test -p pramaan-cli fuzz`
- `cargo test -p pramaan-core fuzz_receipt_types_preserve_replay_and_classification_fields`
- `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base --head-repo examples\fixtures\fuzz\head --claim-scope examples\fixtures\fuzz\claim_scope.json --out target\pramaan-fuzz-phase28.15 --seed 1337`
- `node scripts/check-fuzz-harness-evidence.mjs target\pramaan-fuzz-phase28.15\differential-fuzz.json`

Full workspace verification is recorded in the final public-review readiness
report for this closeout.

## Deferred Risks

- Hypothesis and fast-check execution is still a bounded private-preview path;
  it only runs when the relevant tools and safe pure-function candidates are
  available.
- Missing tools remain explicit deterministic fallback evidence, not successful
  tool-backed campaigns.
- Stronger container isolation for property/fuzz tools remains future
  verifier-security hardening.
- Pramaan still produces evidence bundles, not correctness proof.
