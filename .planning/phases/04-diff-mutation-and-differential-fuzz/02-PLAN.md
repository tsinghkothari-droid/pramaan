---
phase: 4
plan: 2
title: Differential Property and Fuzz Adapters
wave: 2
depends_on:
  - 01-PLAN
files_modified:
  - plugins/python/fuzz/
  - plugins/typescript/fuzz/
  - crates/pramaan-core/src/lib.rs
  - examples/fixtures/fuzz/
autonomous: true
requirements:
  - FUZZ-01
  - FUZZ-02
  - FUZZ-03
  - FUZZ-04
---

# Plan 02 - Differential Property and Fuzz Adapters

## Objective

Compare pre-patch and post-patch behavior on generated/shared inputs, with replayable evidence.

## Tasks

<task id="4-02-01">Implement safe pure-function discovery for Python and not-applicable fallback when discovery is unsafe.</task>
<task id="4-02-02">Implement Hypothesis differential runner with seed/example database/counterexample fields.</task>
<task id="4-02-03">Implement TypeScript fast-check runner with seed/path/replayPath capture.</task>
<task id="4-02-04">Classify divergences against claim scope as expected, unexpected, or needs review.</task>
<task id="4-02-05">Record corpus hashes and replay artifacts.</task>

## Verification

Run differential fixtures that include expected change, unexpected regression, and not-applicable discovery.
