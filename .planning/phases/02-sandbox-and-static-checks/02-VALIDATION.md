---
phase: 2
slug: sandbox-and-static-checks
status: draft
nyquist_compliant: true
created: 2026-05-18
---

# Phase 2 - Validation Strategy

## Test Infrastructure

| Property | Value |
|----------|-------|
| Framework | Rust `cargo test`, fixture repos for Python/TS/Rust |
| Quick run | `cargo test -p pramaan-sandbox -p pramaan-core` |
| Full run | `cargo fmt --check; cargo test; cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-static-smoke` |

## Verification Map

| Requirement | Verification |
|-------------|--------------|
| SNDB-01 | Unit/integration test creates base/head worktrees |
| SNDB-02 | Fixture receipt contains SHAs, lockfile/config hashes, image digest field |
| SNDB-03 | Fixture receipt distinguishes hermetic/non-hermetic run |
| STAT-01 | Python fixture emits compile/lint/type receipt or skipped receipt |
| STAT-02 | TypeScript fixture emits type/lint receipt or skipped receipt |
| STAT-03 | Rust fixture emits cargo check/test-build receipt |
| STAT-04 | Broken import/undefined symbol fixture fails with categorized finding |
| STAT-05 | Hallucination fixture maps finding to risk IDs |

## Required Commands

- `cargo fmt --check`
- `cargo test`
- `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-static-smoke`
