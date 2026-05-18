---
phase: 12
plan: 1
title: Oracle Integrity Engine
wave: 1
depends_on:
  - ../03-oracle-integrity-and-killer-demo/03-VALIDATION
  - ../11-sandbox-claim-static-depth/01-PLAN
files_modified:
  - crates/pramaan-cli/src/oracle.rs
  - crates/pramaan-core/
  - examples/fixtures/oracle/
  - docs/demo.md
  - docs/risk-taxonomy.md
autonomous: true
priority: P1
---

# Plan 01 - Oracle Integrity Engine

## Objective

Make test-oracle tampering detection deep enough to catch realistic AI-agent failure modes across Python, TypeScript, and Rust.

## Tasks

<task id="12-01-01">Implement Python AST diff for pytest assertions, skips, xfails, raises, and parametrized cases.</task>
<task id="12-01-02">Implement TypeScript AST diff for Jest, Vitest, and common `expect` patterns.</task>
<task id="12-01-03">Implement Rust test diff checks for `assert!`, `assert_eq!`, panic tests, and snapshot changes.</task>
<task id="12-01-04">Detect deleted and renamed tests through stable body fingerprints.</task>
<task id="12-01-05">Classify fixture and snapshot diffs as oracle-sensitive with before/after hashes.</task>
<task id="12-01-06">Detect removed boundary cases, error cases, and parameter values.</task>
<task id="12-01-07">Emit reviewer-facing summaries that name the exact weakened assertion or oracle artifact.</task>

## Verification

Add positive and negative fixtures for each language. Confirm ordinary CI can pass in the weakened-test scenario while Pramaan fails with a specific oracle receipt. Run full workspace tests and demo bundle verification.

