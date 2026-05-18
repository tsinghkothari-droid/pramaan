# Phase 16a Execution Brief

## Phase

Phase 16a: Schema-impact subset of Attribution, Feedback, Calibration, and Security

## Objective

Add the receipt and bundle schema hooks that must exist before Phase 9 freezes the receipt/bundle contract. This is intentionally not the full Phase 16 implementation.

## In Scope

- Agent-author attribution.
- Reviewer override shape.
- Multi-agent provenance shape.
- Plugin identity and permissions shape.
- Evidence sensitivity and redaction manifest shape.
- Policy decision shape.
- Stage budget shape.
- Generated `pramaan verify` evidence that exercises at least one non-empty trust-hook set.

## Out of Scope

- Full schema freeze.
- Full JSON Schema validation harness.
- Redaction scrubber implementation.
- Plugin sandbox enforcement.
- Calibration and drift export implementation.
- Non-GitHub provider support.

## Known Risks

- Runtime receipt structs still use the older flat receipt shape while public JSON Schema already describes a richer structured receipt contract. Phase 9 must reconcile this before freeze.
- Bundle verification remains integrity-oriented local verification, not CI-backed authenticity proof.
- Redaction and plugin permissions are declarative metadata in this subset; they are not enforced yet.
