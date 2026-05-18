---
phase: 5
slug: bundle-signing-and-verification
status: draft
nyquist_compliant: true
created: 2026-05-18
---

# Phase 5 - Validation Strategy

## Required Commands

- `cargo fmt --check`
- `cargo test`
- `cargo run -p pramaan-cli -- bundle verify target/pramaan-smoke/bundle.json`

## Verification Map

| Requirement | Verification |
|-------------|--------------|
| RCPT-04 | Bundle references all receipts/artifacts by digest |
| RISK-03 | Summary shows risk families and residuals |
| BNDL-01 | Manifest includes tool/stage/status/seed/corpus fields |
| BNDL-02 | Local dev signing/signable output exists |
| BNDL-03 | Bundle verify validates hashes and metadata |
| BNDL-04 | GitHub attestation metadata fields supported |
