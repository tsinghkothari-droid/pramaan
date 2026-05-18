---
phase: 1
slug: receipt-first-cli-skeleton
status: draft
nyquist_compliant: true
wave_0_complete: false
created: 2026-05-18
---

# Phase 1 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust `cargo test` plus JSON Schema fixture validation |
| **Config file** | `Cargo.toml`, `schemas/*.schema.json` |
| **Quick run command** | `cargo test -p pramaan-core` |
| **Full suite command** | `cargo fmt --check; cargo test` |
| **Estimated runtime** | ~20-60 seconds after initial build |

---

## Sampling Rate

- **After every task commit:** Run `cargo test -p pramaan-core` once crate exists.
- **After every plan wave:** Run `cargo fmt --check; cargo test`.
- **Before `$gsd-verify-work`:** Full suite and CLI smoke command must be green.
- **Max feedback latency:** 60 seconds after initial dependency compile.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 1-01-01 | 01 | 0 | RCPT-03 | schema | `cargo test -p pramaan-core schema_fixtures` | W0 | pending |
| 1-01-02 | 01 | 0 | SCOP-02 | schema | `cargo test -p pramaan-core schema_fixtures` | W0 | pending |
| 1-01-03 | 01 | 0 | RISK-02 | schema | `cargo test -p pramaan-core schema_fixtures` | W0 | pending |
| 1-02-01 | 02 | 1 | CLI-01 | smoke | `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke` | W1 | pending |
| 1-02-02 | 02 | 1 | RCPT-01 | unit | `cargo test -p pramaan-core receipt_writer` | W1 | pending |
| 1-03-01 | 03 | 2 | CLI-03 | smoke | `cargo test -p pramaan-cli summary` | W2 | pending |
| 1-03-02 | 03 | 2 | RCPT-02 | fixture | `cargo test -p pramaan-core schema_fixtures` | W2 | pending |

---

## Wave 0 Requirements

- [ ] `schemas/receipt.schema.json`
- [ ] `schemas/claim_scope.schema.json`
- [ ] `schemas/risk_taxonomy.schema.json`
- [ ] `schemas/bundle.schema.json`
- [ ] `examples/fixtures/receipt.synthetic.json`
- [ ] `examples/fixtures/claim_scope.synthetic.json`
- [ ] `examples/fixtures/risk_taxonomy.synthetic.json`
- [ ] `Cargo.toml` workspace scaffold

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| CLI summary is understandable in 30 seconds | CLI-03 | Readability is partly product judgment | Run smoke command and inspect stage table and bundle path |
| Claims avoid correctness-proof language | PROJECT constraint | Marketing/ethics copy is semantic | Inspect README/CLI messages for "proves correct" style claims |

---

## Validation Sign-Off

- [x] All tasks have automated verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 60s after initial build
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-05-18
