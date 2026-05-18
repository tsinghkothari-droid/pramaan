---
phase: 3
slug: oracle-integrity-and-killer-demo
status: draft
nyquist_compliant: true
created: 2026-05-18
---

# Phase 3 - Validation Strategy

## Required Commands

- `cargo fmt --check`
- `cargo test`
- `cargo run -p pramaan-cli -- verify --base demo/base --head demo/weakened-test --out target/pramaan-demo`

## Verification Map

| Requirement | Verification |
|-------------|--------------|
| SCOP-03 | Narrow/wide/changed/missing-regression fixture receipts |
| ORCL-01 | Deleted/skipped test fixture fails |
| ORCL-02 | Python weakened assertion fixture fails |
| ORCL-03 | TS/JS weakened assertion fixture fails |
| ORCL-04 | Snapshot/fixture change classified oracle-sensitive |
| ORCL-05 | Demo shows ordinary CI green and Pramaan red |
