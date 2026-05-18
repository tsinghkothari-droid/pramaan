---
phase: 6
plan: 2
title: Public Demo and Adversarial Corpus
wave: 2
depends_on:
  - 01-PLAN
files_modified:
  - examples/vulnerable-python-pr/
  - examples/vulnerable-typescript-pr/
  - corpus/
  - docs/demo.md
  - docs/adversarial-corpus.md
autonomous: true
requirements:
  - RISK-04
  - DEMO-01
  - DEMO-02
  - DEMO-03
---

# Plan 02 - Public Demo and Adversarial Corpus

## Objective

Create repeatable public evidence that Pramaan catches AI-code trust failures ordinary CI misses.

## Tasks

<task id="6-02-01">Finalize weakened-test demo and include commands showing normal CI green and Pramaan red.</task>
<task id="6-02-02">Create corpus metadata mapping each demo/eval scenario to risk IDs.</task>
<task id="6-02-03">Add at least five starter adversarial scenarios: weakened assertion, skipped test, invented import, mutation survivor, and unexpected differential divergence.</task>
<task id="6-02-04">Write docs so a reviewer can inspect the demo proof bundle in under 30 seconds.</task>

## Verification

Run demo end to end and confirm generated bundle includes risk mappings and failing oracle receipt.
