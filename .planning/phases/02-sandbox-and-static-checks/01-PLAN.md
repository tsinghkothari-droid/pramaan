---
phase: 2
plan: 1
title: Sandbox Evidence and Worktree Runner
wave: 1
depends_on:
  - ../01-receipt-first-cli-skeleton/01-PLAN
  - ../01-receipt-first-cli-skeleton/02-PLAN
files_modified:
  - crates/pramaan-sandbox/src/lib.rs
  - crates/pramaan-core/src/lib.rs
  - crates/pramaan-cli/src/main.rs
  - examples/fixtures/sandbox/
autonomous: true
requirements:
  - SNDB-01
  - SNDB-02
  - SNDB-03
---

# Plan 01 - Sandbox Evidence and Worktree Runner

## Objective

Create isolated base/head worktrees and emit real sandbox evidence receipts.

## Tasks

<task id="2-01-01">Implement worktree setup for base/head refs with cleanup-safe temp directories.</task>
<task id="2-01-02">Record commit SHAs, dirty state, lockfile hashes, config hashes, OS/runtime identity, and optional image digest.</task>
<task id="2-01-03">Emit sandbox receipt with hermeticity limitations and risk IDs R-021 through R-033 where relevant.</task>
<task id="2-01-04">Add fixture tests for clean, dirty, missing-lockfile, and non-hermetic runs.</task>

## Verification

Run `cargo test -p pramaan-sandbox` and a CLI smoke verify against `HEAD`.
