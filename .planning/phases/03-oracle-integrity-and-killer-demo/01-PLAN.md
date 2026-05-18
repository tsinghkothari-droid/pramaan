---
phase: 3
plan: 1
title: Oracle Diff Engine
wave: 1
depends_on:
  - ../02-sandbox-and-static-checks/01-PLAN
files_modified:
  - crates/pramaan-core/src/lib.rs
  - crates/pramaan-cli/src/main.rs
  - plugins/python/oracle/
  - plugins/typescript/oracle/
  - examples/fixtures/oracle/
autonomous: true
requirements:
  - SCOP-03
  - ORCL-01
  - ORCL-02
  - ORCL-03
  - ORCL-04
---

# Plan 01 - Oracle Diff Engine

## Objective

Detect when tests, fixtures, or snapshots are weakened or changed in ways that reduce trust.

## Tasks

<task id="3-01-01">Implement test file discovery and stable test fingerprints.</task>
<task id="3-01-02">Detect deleted tests, skip/xfail/todo additions, and parametrized case reductions.</task>
<task id="3-01-03">Implement Python assertion AST weakening heuristics.</task>
<task id="3-01-04">Implement TypeScript/JavaScript expect/assert weakening heuristics.</task>
<task id="3-01-05">Classify fixture and snapshot changes as oracle-sensitive artifacts.</task>
<task id="3-01-06">Emit oracle receipts with R-004 through R-020 and R-087 through R-089 coverage.</task>

## Verification

Run oracle fixtures covering deletion, skip, weakened assertion, and snapshot update.
