# Plan 03 Summary - Smoke Tests, Summary Output, and Phase Docs

## Tasks Completed

- Added CLI summary rendering for Phase 1 synthetic verification output.
- Summary now shows bundle output path, per-stage status, receipt paths, and risk families split into mitigated, residual, and skipped buckets.
- Added an integration smoke test for `pramaan verify --base HEAD --head HEAD --out <tmp>`.
- Added core tests that parse the example fixtures, check schema version fields, preserve claim-disciplined language, and validate stable `R-000` risk ID shape.
- Added risk-family mapping used by the CLI summary.
- Updated README with quickstart, validation commands, and explicit proof-bundle language.
- Added receipt-model and risk-taxonomy docs with conservative claim language.

## Files Changed

- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-cli/tests/smoke.rs`
- `crates/pramaan-core/src/lib.rs`
- `README.md`
- `docs/receipt-model.md`
- `docs/risk-taxonomy.md`
- `.planning/phases/01-receipt-first-cli-skeleton/03-SUMMARY.md`

## Validation Notes

- `cargo fmt --check` passed.
- `cargo test` passed.
- `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke` passed.

The smoke command wrote:

- `target/pramaan-smoke/claim_scope.synthetic.json`
- `target/pramaan-smoke/receipts/claim-scope.receipt.json`
- `target/pramaan-smoke/receipts/synthetic-verification.receipt.json`

The CLI summary included stage statuses and these risk-family buckets:

- mitigated: `claim_scope`, `oracle_integrity`
- residual: `bundle_integrity`, `public_api_compatibility`, `runtime_behavior`
- skipped: `bundle_integrity`

## Deviations

- No Git operations were performed.
- Schema fixture validation is lightweight in core tests and does not use a full JSON Schema engine yet; it confirms fixture parseability, schema-version fields, stable risk ID shape, and claim-discipline language.
