---
phase: 4
plan: 1
title: Diff-Scoped Mutation Adapters
wave: 1
depends_on:
  - ../02-sandbox-and-static-checks/02-PLAN
  - ../03-oracle-integrity-and-killer-demo/01-PLAN
files_modified:
  - plugins/python/mutation/
  - plugins/typescript/mutation/
  - plugins/rust/mutation/
  - crates/pramaan-core/src/lib.rs
  - examples/fixtures/mutation/
autonomous: true
requirements:
  - MUTN-01
  - MUTN-02
  - MUTN-03
  - MUTN-04
  - MUTN-05
---

# Plan 01 - Diff-Scoped Mutation Adapters

## Objective

Run practical mutation testing on changed source, preserving enough evidence to avoid false confidence.

## Tasks

<task id="4-01-01">Implement Python mutmut adapter with changed-file and coverage/filter metadata.</task>
<task id="4-01-02">Implement TypeScript StrykerJS adapter with mutate pattern and incremental metadata.</task>
<task id="4-01-03">Implement Rust cargo-mutants adapter or not-applicable fallback based on project/tool availability.</task>
<task id="4-01-04">Normalize killed/survived/timed-out/unviable/skipped counts into receipts.</task>
<task id="4-01-05">Classify survivors as review/test-gap/likely-equivalent where possible.</task>

## Verification

Run mutation fixtures and ensure receipts include risk IDs R-068 through R-072.
