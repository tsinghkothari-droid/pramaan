# Phase 28.1 Fuzz Harness Truthfulness Review

Date: 2026-05-21

## Verdict

FAIL_FOR_CLAIM_TRUTHFULNESS.

The workspace is green, but the Phase 28.1 implementation currently overstates
what the tool-backed property/fuzz harness contributes to Pramaan's evidence.
This is not a general Rust quality failure; it is a verifier-truthfulness
failure.

## Tooling Context

This was a local pull-request-style review rather than a hosted review-bot run.
At review time this repository had no open pull requests, and the local shell
did not expose `OPENAI_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, or
`GITHUB_TOKEN`, so the review was grounded in the active diff, local commands,
and repository files.

## Findings

1. **High: tool-backed failures do not affect the fuzz verdict.**
   - Evidence: `crates/pramaan-cli/src/fuzz.rs:54` computes `divergences`
     before the tool-backed harness runs at `fuzz.rs:70`.
   - The generated Python harness writes `failures` at `fuzz.rs:1076`, and the
     generated JavaScript harness writes `failures` at `fuzz.rs:1118`, but
     `run_hypothesis_harness` and `run_fast_check_harness` only retain tool
     version, generated count, path, and digest.
   - Risk: Pramaan can say `tool_backed=true` while the final verdict is still
     based only on deterministic replay inputs.

2. **High: the harness timeout is a no-op.**
   - Evidence: `crates/pramaan-cli/src/fuzz.rs:1127` accepts `_timeout` and
     immediately calls `command.output()`.
   - Risk: Python or Node harnesses can hang CI indefinitely despite the
     10-second budget shown at `fuzz.rs:926` and `fuzz.rs:982`.

3. **Medium: harness errors abort instead of becoming structured evidence.**
   - Evidence: nonzero Python and Node exits call `anyhow::bail!` at
     `fuzz.rs:931` and `fuzz.rs:987`.
   - Risk: verifier-tool failure becomes an opaque command failure instead of a
     signed receipt with stderr, timeout/error category, and policy outcome.

4. **Medium: the JS harness uses dynamic `Function(...)` over generated
   expressions.**
   - Evidence: `crates/pramaan-cli/src/fuzz.rs:1096`.
   - Risk: the expression filter is conservative, but Pramaan runs against
     untrusted PRs; verifier execution should avoid dynamic evaluation or
     isolate it more strongly.

5. **Medium: tool version is parsed from prose.**
   - Evidence: `crates/pramaan-cli/src/fuzz.rs:281` extracts `tool_version`
     from `adapter_availability.reason`.
   - Risk: changing the reason text breaks structured evidence.

6. **Low: generated input counts are ambiguous.**
   - Evidence: `crates/pramaan-cli/src/fuzz.rs:87` stores deterministic corpus
     length as `generated_input_count` even when Hypothesis/fast-check also
     report generated cases.
   - Risk: reviewers may overread the amount of executed tool-backed evidence.

## Positive Evidence

- `cargo fmt --check`: PASS.
- `cargo test --workspace`: PASS.
- `cargo clippy --workspace -- -D warnings`: PASS.
- The repository is clean at review time.

## Recommendation

Do not market Phase 28.1 as fully closed. Insert and execute Phase 28.15 before
letting confidence, policy, reports, or public copy treat tool-backed
Hypothesis/fast-check evidence as a reliable gate.
