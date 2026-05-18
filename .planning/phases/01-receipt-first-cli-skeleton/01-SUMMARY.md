# Plan 01 Summary - Schema and Evidence Contracts

## Tasks Completed

- Created `schemas/receipt.schema.json` with the v1 stage receipt contract, including normalized statuses: `passed`, `failed`, `skipped`, `not_applicable`, `timed_out`, and `error`.
- Created `schemas/claim_scope.schema.json` with source refs, expected behavior, out-of-scope behavior, touched public APIs, extraction method, confidence, limitations, and risk refs.
- Created `schemas/risk_taxonomy.schema.json` for stable `R-###` IDs, risk family, flaw, trust-break description, mitigation, phase owner, severity, requirement refs, and stage owners.
- Created `schemas/bundle.schema.json` for manifest-level receipt/artifact digest references, tool versions, final status, risk summaries, and future signing/artifact-attestation metadata.
- Added synthetic fixtures for receipt, claim scope, risk taxonomy, and bundle under `examples/fixtures/`.
- Added `examples/fixtures/README.md` to document fixture intent without putting comments inside JSON.

## Files Created

- `schemas/receipt.schema.json`
- `schemas/claim_scope.schema.json`
- `schemas/risk_taxonomy.schema.json`
- `schemas/bundle.schema.json`
- `examples/fixtures/receipt.synthetic.json`
- `examples/fixtures/claim_scope.synthetic.json`
- `examples/fixtures/risk_taxonomy.synthetic.json`
- `examples/fixtures/bundle.synthetic.json`
- `examples/fixtures/README.md`

## Validation Notes

- JSON syntax was checked for all schemas and fixtures.
- Local Python `jsonschema` validation passed for:
  - `examples/fixtures/receipt.synthetic.json` against `schemas/receipt.schema.json`
  - `examples/fixtures/claim_scope.synthetic.json` against `schemas/claim_scope.schema.json`
  - `examples/fixtures/risk_taxonomy.synthetic.json` against `schemas/risk_taxonomy.schema.json`
  - `examples/fixtures/bundle.synthetic.json` against `schemas/bundle.schema.json`
- The planned Rust command `cargo test -p pramaan-core schema_fixtures` was not run because the Rust workspace/crates are owned by adjacent plans/workers and are not present in this scoped Plan 01 slice.

## Deviations

- Added one minimal adjacent README at `examples/fixtures/README.md` because JSON does not support comments and Plan 01 requested schema intent documentation in fixture comments or adjacent README text.
- The synthetic risk taxonomy fixture contains a representative subset from the top-100 flaw register rather than duplicating the full register. The full register already exists at `.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md`; the fixture's purpose is schema validation.
- Digest values in synthetic fixtures are deterministic placeholder hashes for contract validation only. Real content hashes should be produced by the bundle implementation in later plans.
