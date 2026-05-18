---
phase: 11
plan: 1
title: Sandbox, Claim Scope, and Static Depth
wave: 1
depends_on:
  - ../02-sandbox-and-static-checks/02-PLAN
  - ../09-receipt-bundle-trust-hardening/01-PLAN
files_modified:
  - crates/pramaan-sandbox/
  - crates/pramaan-core/
  - crates/pramaan-cli/
  - plugins/python/
  - plugins/typescript/
  - plugins/rust/
  - docs/receipt-model.md
autonomous: true
priority: P1
---

# Plan 01 - Sandbox, Claim Scope, and Static Depth

## Objective

Turn the environment, claim, and static-check receipts into practical evidence for real repositories.

## Tasks

<task id="11-01-01">Capture OS, architecture, shell, timezone, locale, and toolchain versions.</task>
<task id="11-01-02">Record base/head commit IDs plus dirty and untracked file state.</task>
<task id="11-01-03">Hash dependency manifests and lockfiles, and flag lockfile drift.</task>
<task id="11-01-04">Capture container image name and digest when available.</task>
<task id="11-01-05">Add network policy evidence: disabled, allowed, observed, or unknown.</task>
<task id="11-01-06">Parse PR title/body and linked issue context from GitHub Actions when available.</task>
<task id="11-01-07">Detect changed public APIs for Python, TypeScript, and Rust.</task>
<task id="11-01-08">Integrate real Python, TypeScript, and Rust static tools when configured.</task>
<task id="11-01-09">Classify hallucination failures using stable categories and risk IDs.</task>

## Verification

Run fixture repositories for Python, TypeScript, and Rust. Confirm skipped tools produce skipped receipts rather than silent green output. Confirm claim-scope and lockfile risks appear in bundle summaries.

