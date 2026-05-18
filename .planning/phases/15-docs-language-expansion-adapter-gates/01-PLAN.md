---
phase: 15
plan: 1
title: Documentation, Language Expansion, and Adapter Gates
wave: 1
depends_on:
  - ../12-oracle-integrity-engine/01-PLAN
  - ../13-mutation-fuzz-adapters/01-PLAN
  - ../14-attestation-corpus-evals/01-PLAN
files_modified:
  - docs/
  - plugins/
  - schemas/adapter_certification.schema.json
  - examples/fixtures/adapter_certification.synthetic.json
  - TASKS.md
autonomous: true
priority: P2
---

# Plan 01 - Documentation, Language Expansion, and Adapter Gates

## Objective

Prepare Pramaan for external contributors and adopters without letting language sprawl or adapter certification distract from the core PR-verification trust layer.

## Tasks

<task id="15-01-01">Write operator, plugin-author, security-model, threat-model, enterprise-deployment, and troubleshooting guides.</task>
<task id="15-01-02">Add screenshots or rendered examples of PR summaries and bundle inspection.</task>
<task id="15-01-03">Define language plugin readiness gates for Python, TypeScript, and Rust.</task>
<task id="15-01-04">Add Go support only after the plugin protocol is stable and the first three languages meet readiness gates.</task>
<task id="15-01-05">Add Java support only after the plugin protocol is stable and the first three languages meet readiness gates.</task>
<task id="15-01-06">Keep adapter certification as an adjacent mode with checks for tool names, schemas, auth scopes, idempotency, retry behavior, rate limits, and auditability.</task>
<task id="15-01-07">Add adapter proof-bundle examples and failure-mode taxonomy updates.</task>

## Verification

Have a fresh checkout follow the operator guide and inspect a generated bundle. Confirm new language support does not modify receipt contracts without compatibility tests. Confirm adapter certification docs keep PR verification as the primary Pramaan product.

