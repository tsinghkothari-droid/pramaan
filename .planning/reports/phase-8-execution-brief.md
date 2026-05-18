# Phase 8 Execution Brief

## Phase

Phase 8: Killer Demo and Proof Bundles

## Objective

Create public demos where ordinary CI or superficial review can look green while
Pramaan emits concrete evidence for weakened assertions, oracle artifact drift,
or hallucinated code.

## Planned Workstreams

- Extend existing weakened-test demo documentation.
- Add snapshot/fixture drift demo.
- Add static hallucination demo.
- Generate example Pramaan outputs for all three demos.
- Update adversarial corpus and demo docs.

## Known Risks

- Current stage commands emit receipts and artifacts but not full signed bundle
  manifests for oracle/static demo commands.
- Static hallucination classification depends on Rust diagnostics available in
  the local toolchain.
- Generated example outputs include timestamps and local path evidence, so they
  are examples of evidence shape rather than deterministic golden fixtures.

