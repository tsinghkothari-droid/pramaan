---
phase: 8
plan: 1
title: Killer Demo and Proof Bundles
wave: 1
depends_on:
  - ../06-github-action-and-public-demo-loop/02-PLAN
  - ../07-adapter-certification-expansion/01-PLAN
files_modified:
  - examples/
  - docs/demo.md
  - corpus/starter-adversarial-scenarios.json
  - TASKS.md
autonomous: true
priority: P0
---

# Plan 01 - Killer Demo and Proof Bundles

## Objective

Create the public proof that makes Pramaan obvious: normal CI goes green, but Pramaan fails the PR because the AI agent weakened the test oracle or approved the wrong behavior.

## Tasks

<task id="8-01-01">Create a standalone demo fixture where a bug is "fixed" by weakening a test assertion.</task>
<task id="8-01-02">Create a snapshot/fixture drift demo where ordinary tests pass after approving wrong behavior.</task>
<task id="8-01-03">Create a static/hallucination demo where an invented import/API is classified with a stable risk ID.</task>
<task id="8-01-04">Generate example Pramaan bundles for all three demos.</task>
<task id="8-01-05">Update `docs/demo.md` with a 30-second reviewer walkthrough.</task>
<task id="8-01-06">Add each demo to the adversarial corpus with risk-ID mappings.</task>

## Verification

Run the normal demo test command and confirm it passes. Run Pramaan and confirm the relevant receipt fails or warns with the exact weakened assertion, snapshot/fixture drift, or hallucination category. Verify all generated bundles pass hash verification unless the scenario intentionally demonstrates tampering.

