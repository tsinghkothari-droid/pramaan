# Phase 28.25: AI Evidence-Seeking Probe Generator

## Goal

Use AI to generate better tests, properties, differential inputs, and security
probes, while counting only evidence that executes in Pramaan's sandbox.

## Core Rule

AI may propose probes. AI may not decide that the PR is safe.

```text
diff + claim + receipts -> AI proposes probes -> sandbox executes probes
-> mutation/fuzz/differential evidence validates them -> receipts decide
```

## Files To Change

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `schemas/probe.schema.json`
- `docs/ai-probe-generator.md`
- `examples/ai-probes/`
- `corpus/`

## Implementation Steps

1. Add `pramaan probe plan --bundle <path>` to create a probe plan from claim,
   diff, risk IDs, and skipped/residual evidence.
2. Add a provider-neutral probe schema:
   `probe_id`, `risk_ids`, `kind`, `language`, `target_files`, `prompt_hash`,
   `candidate_code`, `sandbox_status`, `execution_result`, `kept_or_rejected`,
   `rejection_reason`.
3. Support probe kinds:
   - regression assertion;
   - property-based invariant;
   - differential input;
   - security sink/source test;
   - mutation-targeted test;
   - fixture/snapshot challenge.
4. Run generated probes in an isolated temporary test location.
5. Keep only probes that compile/run and bind to changed behavior.
6. Mutation-test generated probes where possible; weak probes become rejected
   evidence, not mitigation.
7. Emit `ai_probe_generation` receipts with both accepted and rejected probes.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- Golden fixtures show:
  - AI-generated assertion accepted after execution;
  - non-compiling generated probe rejected;
  - probe that does not touch changed behavior rejected;
  - accepted probe improves mutation/differential evidence.

## Exit Criteria

Pramaan can use AI to search for missing evidence without letting AI become the
judge. The repo gains a real "evidence-seeking" loop.
