---
phase: 16
plan: 1
title: Attribution, Feedback, Calibration, and Verifier Security
wave: 1
depends_on:
  - ../09-receipt-bundle-trust-hardening/01-PLAN
  - ../10-github-action-production-readiness/01-PLAN
  - ../14-attestation-corpus-evals/01-PLAN
files_modified:
  - TASKS.md
  - schemas/
  - crates/pramaan-core/
  - crates/pramaan-bundle/
  - docs/
  - corpus/
autonomous: true
priority: P0-P2
---

# Plan 01 - Attribution, Feedback, Calibration, and Verifier Security

## Objective

Close the serious blind spots that were not covered by the first Serious v1 task list: agent-author attribution, reviewer override feedback, baseline calibration, drift tracking, plugin/verifier trust, PII/secrets scrubbing, multi-agent provenance, and non-GitHub expansion.

## Why This Phase Exists

The initial Pramaan roadmap correctly focuses on killer demos, receipts, static/oracle/mutation/fuzz evidence, and signed bundles. But a serious production trust layer also needs to learn over time, resist poisoning, protect sensitive bundle data, and support workflows where AI code is authored and reviewed by multiple agents.

Several of these items affect receipt schema shape. They must be designed before schema v0.1 is frozen or every future bundle will need migration.

## Tasks

<task id="16-01-01">Add agent-author attribution fields to receipt and bundle schemas: agent product, model family/version when available, execution mode, prompt/context hash, commit provenance, and confidence/source of attribution.</task>

<task id="16-01-02">Add reviewer override capture: override decision, accepted risk IDs, reviewer identity source, timestamp, free-text reason, linked PR/merge outcome, and whether the override should update calibration data.</task>

<task id="16-01-03">Define hard performance SLA targets: target runtime, max runtime, per-stage budget, diff-size classes, timeout behavior, and summary language when budgets are exhausted.</task>

<task id="16-01-04">Add repo-level baseline calibration model for mutation survival, oracle warnings, skipped stages, static/hallucination findings, and runtime noise floor.</task>

<task id="16-01-05">Design trend/drift export format for weekly and monthly agent-code quality metrics: mutation survival drift, oracle-risk drift, skipped-stage drift, runtime drift, and agent-specific failure patterns.</task>

<task id="16-01-06">Threat-model Pramaan as a target: malicious PR code exploiting mutation engines, fuzzers, parsers, test runners, artifact collection, plugin hooks, or CI credentials.</task>

<task id="16-01-07">Define plugin trust model: plugin identity, version, provenance, optional signature, sandbox boundary, allowed receipt-writing permissions, and prohibition on editing previous receipts or bundle manifests.</task>

<task id="16-01-08">Add PII/secrets scrubbing rules for receipts and artifacts: environment variables, internal hostnames, private paths, network endpoints, logs, toolchain names, and CI metadata.</task>

<task id="16-01-09">Add semantic claim-implementation mismatch as a bounded signal by comparing stated intent, changed public APIs, tests, and behavioral evidence without making an LLM critic the sole gate.</task>

<task id="16-01-10">Model multi-agent provenance chains: author agent, reviewer agent, test-writing agent, patching agent, final human reviewer, intermediate commits, and handoff metadata.</task>

<task id="16-01-11">Define provider-neutral VCS/CI abstraction before GitLab support: refs, PR/MR metadata, comments, artifacts, attestations, and permissions.</task>

<task id="16-01-12">Document GitLab, Gitea, and Bitbucket roadmap boundaries, with GitLab as the first non-GitHub target after GitHub Action readiness.</task>

## Verification

1. Schema fixtures include agent attribution and reviewer override examples before receipt v0.1 freeze.
2. Bundle verification rejects plugin-emitted receipts that exceed declared permissions.
3. Redaction tests prove secrets/internal endpoints/private paths do not appear in exported bundles.
4. SLA fixtures show budget-exhausted stages are visible and do not look green.
5. Calibration fixtures can distinguish a noisy repo baseline from a newly risky PR.
6. Drift export fixtures can aggregate multiple bundles without needing a dashboard.
7. Threat model documents malicious PR and malicious plugin attack paths.

