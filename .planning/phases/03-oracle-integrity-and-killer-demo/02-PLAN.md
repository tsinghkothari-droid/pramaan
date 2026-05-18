---
phase: 3
plan: 2
title: Killer Demo Repository
wave: 2
depends_on:
  - 01-PLAN
files_modified:
  - examples/vulnerable-python-pr/
  - examples/vulnerable-typescript-pr/
  - docs/demo.md
autonomous: true
requirements:
  - ORCL-05
---

# Plan 02 - Killer Demo Repository

## Objective

Create a demo where ordinary CI passes because a test was weakened, while Pramaan fails with a clean oracle-integrity receipt.

## Tasks

<task id="3-02-01">Create a small Python demo with a real bug, original failing test, and AI-style weakened-test branch.</task>
<task id="3-02-02">Optionally mirror the same scenario in TypeScript if Phase 3 time allows.</task>
<task id="3-02-03">Document the demo flow: normal CI green, Pramaan red, receipt names weakened assertion.</task>
<task id="3-02-04">Map demo scenarios to risk IDs R-010, R-011, R-014, R-087, and R-100.</task>

## Verification

Run demo CI command and Pramaan command; confirm CI green and Pramaan red.
