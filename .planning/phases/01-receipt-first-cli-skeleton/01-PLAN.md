---
phase: 1
plan: 1
title: Schema and Evidence Contracts
wave: 0
depends_on: []
files_modified:
  - schemas/receipt.schema.json
  - schemas/claim_scope.schema.json
  - schemas/risk_taxonomy.schema.json
  - schemas/bundle.schema.json
  - examples/fixtures/receipt.synthetic.json
  - examples/fixtures/claim_scope.synthetic.json
  - examples/fixtures/risk_taxonomy.synthetic.json
  - examples/fixtures/bundle.synthetic.json
autonomous: true
requirements:
  - RCPT-01
  - RCPT-02
  - RCPT-03
  - RCPT-05
  - RISK-01
  - RISK-02
  - SCOP-01
  - SCOP-02
---

# Plan 01 - Schema and Evidence Contracts

## Objective

Define Pramaan's v1 evidence model before implementation details harden. The receipt, claim-scope, and bundle schemas are the contract every later stage must obey.

## Must Haves

- Receipt schema supports all normalized statuses: `passed`, `failed`, `skipped`, `not_applicable`, `timed_out`, `error`.
- Receipt schema includes tool identity, inputs, outputs, artifact references, timing, summary, limitations, and risk ID arrays.
- Claim-scope schema supports PR/issue source refs, expected behavior, out-of-scope behavior, touched public APIs, extraction method, and confidence.
- Risk taxonomy schema supports stable `R-###` IDs, family, flaw, mitigation, phase owner, and severity.
- Bundle schema references receipts/artifacts by digest and has a placeholder for future signing/attestation metadata.
- Synthetic fixtures validate against schemas.

## Tasks

<task id="1-01-01">
Create `schemas/receipt.schema.json` with a stable v1 receipt structure and explicit enum values for stage status.
</task>

<task id="1-01-02">
Create `schemas/claim_scope.schema.json` that models expected behavior, excluded behavior, touched public APIs, source refs, extraction method, and confidence.
</task>

<task id="1-01-03">
Create `schemas/risk_taxonomy.schema.json` using the top-100 flaw register as the source of stable risk IDs.
</task>

<task id="1-01-04">
Create `schemas/bundle.schema.json` that aggregates receipts, artifacts, risk summaries, tool versions, final status, and optional signing or artifact-attestation metadata.
</task>

<task id="1-01-05">
Add synthetic JSON fixtures under `examples/fixtures/` for receipt, claim scope, risk taxonomy, and bundle.
</task>

<task id="1-01-06">
Document schema intent in fixture comments or adjacent README text without claiming code correctness or emitting a single opaque risk score.
</task>

## Verification

<automated>
After Rust test harness exists, run `cargo test -p pramaan-core schema_fixtures`.
</automated>

<manual>
Inspect schema names and fixture fields for claim discipline: receipts should prove what ran, not that code is correct.
</manual>
