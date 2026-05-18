---
phase: 5
plan: 2
title: Signing Metadata and Risk Summary
wave: 2
depends_on:
  - 01-PLAN
files_modified:
  - crates/pramaan-bundle/src/lib.rs
  - crates/pramaan-cli/src/main.rs
  - docs/attestation.md
autonomous: true
requirements:
  - RISK-03
  - BNDL-02
  - BNDL-04
---

# Plan 02 - Signing Metadata and Risk Summary

## Objective

Support local dev signing/signable output, future GitHub artifact attestation metadata, and readable risk-family summaries.

## Tasks

<task id="5-02-01">Add local dev signing/signable metadata model and mark it clearly as dev-mode when not CI-backed.</task>
<task id="5-02-02">Add optional GitHub artifact attestation fields: issuer, subject, workflow, repo, commit SHA, transparency mode.</task>
<task id="5-02-03">Generate bundle summary grouped by mitigated, residual, skipped, and not-applicable risk families.</task>
<task id="5-02-04">Remove any single opaque trust score from v1 output.</task>

## Verification

Inspect bundle summary for accurate language and run schema tests for attestation metadata.
