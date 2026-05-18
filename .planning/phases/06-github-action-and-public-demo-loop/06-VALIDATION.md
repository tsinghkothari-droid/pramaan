---
phase: 6
slug: github-action-and-public-demo-loop
status: draft
nyquist_compliant: true
created: 2026-05-18
---

# Phase 6 - Validation Strategy

## Required Commands

- `npm test` or action wrapper tests if TypeScript action is used
- `cargo fmt --check; cargo test`
- Local action dry-run where possible

## Verification Map

| Requirement | Verification |
|-------------|--------------|
| GHAC-01 | Action invokes CLI on PR refs |
| GHAC-02 | Action uploads bundle artifact |
| GHAC-03 | Action renders risk-aware PR summary |
| GHAC-04 | Optional attestation path documented/configured |
| RISK-04 | Corpus scenarios map to risk IDs |
| DEMO-01 | Vulnerable demo repo/fixture exists |
| DEMO-02 | Demo shows CI green and Pramaan red |
| DEMO-03 | Demo bundle receipt names weakened assertion |
