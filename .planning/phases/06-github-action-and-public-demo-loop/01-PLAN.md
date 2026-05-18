---
phase: 6
plan: 1
title: GitHub Action Wrapper
wave: 1
depends_on:
  - ../05-bundle-signing-and-verification/02-PLAN
files_modified:
  - action/
  - action.yml
  - .github/workflows/pramaan.yml
  - docs/github-action.md
autonomous: true
requirements:
  - GHAC-01
  - GHAC-02
  - GHAC-03
  - GHAC-04
---

# Plan 01 - GitHub Action Wrapper

## Objective

Make Pramaan run naturally on pull requests with artifact upload, risk summary, and optional GitHub artifact attestation.

## Tasks

<task id="6-01-01">Create GitHub Action wrapper that installs/runs the Pramaan CLI against PR base/head refs.</task>
<task id="6-01-02">Upload the proof bundle as a workflow artifact.</task>
<task id="6-01-03">Render PR/check summary focused on failed stages and residual risk families.</task>
<task id="6-01-04">Document minimal permissions and optional artifact attestation flow.</task>

## Verification

Run action wrapper tests or local dry-run and inspect generated summary markdown.
