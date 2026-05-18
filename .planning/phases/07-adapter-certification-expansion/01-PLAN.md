---
phase: 7
plan: 1
title: Adapter Certification Research Pack
wave: 1
depends_on:
  - ../06-github-action-and-public-demo-loop/02-PLAN
files_modified:
  - docs/adapter-certification.md
  - docs/PRODUCT_FAMILY_NOTES.md
  - schemas/adapter_certification.schema.json
  - examples/fixtures/adapter_certification.synthetic.json
  - .planning/research/ADAPTER_CERTIFICATION_RISKS_2026-05-18.md
autonomous: true
requirements:
  - ADPT-01
  - ADPT-02
  - ADPT-03
  - ADPT-04
---

# Plan 01 - Adapter Certification Research Pack

## Objective

Add the adapter-certification expansion as a well-bounded Pramaan mode: certify MCP/agent tools through typed, auditable, risk-mapped evidence.

## Tasks

<task id="7-01-01">Create `docs/adapter-certification.md` explaining the mode, checks, and non-goals.</task>
<task id="7-01-02">Update product-family notes to state registry/Sutra are deferred and Pramaan Adapter Certification is the first adjacent expansion.</task>
<task id="7-01-03">Create `schemas/adapter_certification.schema.json` for adapter certification receipts.</task>
<task id="7-01-04">Create `examples/fixtures/adapter_certification.synthetic.json` mapped to adapter risk IDs.</task>
<task id="7-01-05">Create `.planning/research/ADAPTER_CERTIFICATION_RISKS_2026-05-18.md` with starter adapter failure modes.</task>

## Verification

Parse the schema and fixture as JSON. Confirm the risk register uses stable `A-###` IDs and the docs keep Pramaan as the current product focus.
