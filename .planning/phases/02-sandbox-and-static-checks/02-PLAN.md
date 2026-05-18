---
phase: 2
plan: 2
title: Static Check Plugin Adapters
wave: 2
depends_on:
  - 01-PLAN
files_modified:
  - plugins/python/
  - plugins/typescript/
  - plugins/rust/
  - crates/pramaan-core/src/lib.rs
  - crates/pramaan-cli/src/main.rs
  - examples/fixtures/static/
autonomous: true
requirements:
  - STAT-01
  - STAT-02
  - STAT-03
  - STAT-04
  - STAT-05
---

# Plan 02 - Static Check Plugin Adapters

## Objective

Add first real language adapters for static evidence: Python compile/lint/type, TypeScript type/lint, and Rust cargo checks.

## Tasks

<task id="2-02-01">Implement Python adapter command discovery for `compileall`, `ruff`, and `mypy` when configured.</task>
<task id="2-02-02">Implement TypeScript adapter command discovery for package manager, `tsc --noEmit`, and lint script when configured.</task>
<task id="2-02-03">Implement Rust adapter for `cargo check` and `cargo test --no-run`.</task>
<task id="2-02-04">Normalize missing tool/config as skipped/not-applicable receipts with residual risks.</task>
<task id="2-02-05">Classify static failures into hallucination categories where evidence supports it.</task>

## Verification

Run `cargo test` plus fixture CLI runs for Python, TypeScript, and Rust sample repos.
