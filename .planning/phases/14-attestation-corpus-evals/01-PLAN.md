---
phase: 14
plan: 1
title: Attestation, Corpus, and Evals
wave: 1
depends_on:
  - ../09-receipt-bundle-trust-hardening/01-PLAN
  - ../10-github-action-production-readiness/01-PLAN
  - ../13-mutation-fuzz-adapters/01-PLAN
files_modified:
  - crates/pramaan-bundle/
  - corpus/
  - docs/attestation.md
  - docs/adversarial-corpus.md
  - schemas/bundle.schema.json
autonomous: true
priority: P2
---

# Plan 01 - Attestation, Corpus, and Evals

## Objective

Make Pramaan evidence durable and measurable: signed/attested bundles plus an adversarial corpus that tracks false positives, false negatives, runtime, and reviewer usefulness.

## Tasks

<task id="14-01-01">Add Sigstore keyless signing path for local and CI runs.</task>
<task id="14-01-02">Add GitHub artifact attestation integration.</task>
<task id="14-01-03">Map bundle manifest fields to in-toto/SLSA-compatible predicates.</task>
<task id="14-01-04">Add offline verification mode for downloaded bundles.</task>
<task id="14-01-05">Expand adversarial corpus to 25, then 75, then 100+ scenarios mapped to risk IDs.</task>
<task id="14-01-06">Add real-world replay cases from open-source bug-fix PRs.</task>
<task id="14-01-07">Track false positives, false negatives, runtime, and reviewer time-to-understand.</task>
<task id="14-01-08">Create a benchmark report template.</task>

## Verification

Verify signed and unsigned bundles offline. Confirm attestation metadata is present when running in GitHub Actions. Run corpus fixtures and generate a benchmark report without manual editing.

