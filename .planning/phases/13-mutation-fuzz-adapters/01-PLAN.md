---
phase: 13
plan: 1
title: Mutation and Differential Fuzz Adapters
wave: 1
depends_on:
  - ../04-diff-mutation-and-differential-fuzz/04-VALIDATION
  - ../11-sandbox-claim-static-depth/01-PLAN
files_modified:
  - crates/pramaan-cli/src/mutation.rs
  - crates/pramaan-cli/src/fuzz.rs
  - plugins/python/
  - plugins/typescript/
  - plugins/rust/
  - examples/fixtures/mutation/
  - examples/fixtures/fuzz/
autonomous: true
priority: P1
---

# Plan 01 - Mutation and Differential Fuzz Adapters

## Objective

Wrap production mutation and property-testing tools with strict budgets, replay metadata, and honest skipped/timeout receipts.

## Tasks

<task id="13-01-01">Run `mutmut` on changed Python files and directly affected tests.</task>
<task id="13-01-02">Run StrykerJS in TypeScript diff-scoped mode where possible.</task>
<task id="13-01-03">Run `cargo-mutants` on changed Rust crates or modules.</task>
<task id="13-01-04">Record mutants created, killed, survived, timed out, skipped, and unviable.</task>
<task id="13-01-05">Record threshold, timeout policy, incremental-cache state, filtering mode, and equivalent-mutant classification.</task>
<task id="13-01-06">Auto-discover eligible pure functions for Hypothesis and fast-check differential checks.</task>
<task id="13-01-07">Record seeds, replay data, minimized counterexamples, corpus hashes, generated counts, and divergence scope classification.</task>

## Verification

Run mutation and fuzz fixtures for Python, TypeScript, and Rust. Confirm timeouts and missing tools produce honest receipts. Confirm every failing generated case has a replay command or enough replay metadata.

