---
phase: 10
plan: 1
title: GitHub Action Production Readiness
wave: 1
depends_on:
  - ../06-github-action-and-public-demo-loop/02-PLAN
  - ../09-receipt-bundle-trust-hardening/01-PLAN
files_modified:
  - action.yml
  - action/
  - .github/workflows/
  - docs/github-action.md
  - examples/
autonomous: true
priority: P0
---

# Plan 01 - GitHub Action Production Readiness

## Objective

Make Pramaan installable and understandable as a real pull-request gate through GitHub Actions.

## Tasks

<task id="10-01-01">Make the action install or download the Pramaan CLI deterministically.</task>
<task id="10-01-02">Add `base-ref`, `head-ref`, `out-dir`, `fail-on`, and `upload-bundle` inputs.</task>
<task id="10-01-03">Upload the proof bundle as a GitHub Actions artifact.</task>
<task id="10-01-04">Render a concise PR summary focused on failed stages and residual risks.</task>
<task id="10-01-05">Document permissions and forked-PR behavior.</task>
<task id="10-01-06">Add minimal workflow examples for Python, TypeScript, and Rust repositories.</task>

## Verification

Run action unit tests, execute the action workflow locally where possible, and validate the rendered PR summary against fixture bundles. Confirm missing permissions produce clear failure messages.

