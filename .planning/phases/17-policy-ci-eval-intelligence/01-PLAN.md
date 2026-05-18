---
phase: 17
plan: 1
title: Policy, CI Hardening, and Evaluation Intelligence
wave: 1
depends_on:
  - ../09-receipt-bundle-trust-hardening/01-PLAN
  - ../10-github-action-production-readiness/01-PLAN
  - ../14-attestation-corpus-evals/01-PLAN
  - ../16-attribution-feedback-calibration-security/01-PLAN
files_modified:
  - TASKS.md
  - .planning/research/NEXT_LEVEL_RESEARCH_2026-05-19.md
  - .planning/ROADMAP.md
  - .planning/STATE.md
  - docs/
  - schemas/
  - corpus/
autonomous: true
priority: P0-P2
---

# Plan 01 - Policy, CI Hardening, and Evaluation Intelligence

## Objective

Turn Pramaan from a receipt generator into a policy-driven evidence layer: repo-specific gates, CI attack resistance, performance SLAs, verification summary attestations, redaction-safe bundles, and a benchmark corpus that measures real AI-code risk.

## Tasks

<task id="17-01-01">Define policy-as-code profile format for hard gates, warning gates, waiver rules, stage requirements, security-sensitive paths, and override requirements.</task>

<task id="17-01-02">Implement `pramaan policy explain` plan: given a bundle, show final decision, failed gates, warning gates, accepted overrides, and policy rule IDs.</task>

<task id="17-01-03">Add default policy profile: hard fail on bundle tamper, missing base/head evidence, configured static failure, test skip/weakening, policy weakening, and security-sensitive validation removal.</task>

<task id="17-01-04">Add CI hardening checks for GitHub Actions: least-privilege token permissions, forked PR behavior, `pull_request_target` hazards, cache poisoning, unpinned actions, artifact retention, and self-hosted runner warnings.</task>

<task id="17-01-05">Define non-GitHub CI abstraction for artifacts, identity, comments, refs, merge requests, and OIDC signing, with GitLab as first target.</task>

<task id="17-01-06">Add SLSA Verification Summary Attestation output plan for Pramaan's final verifier decision, policy identity, verifier identity, input bundle hash, and result.</task>

<task id="17-01-07">Define redaction profiles: internal-full, reviewer-redacted, public-demo, and summary-only.</task>

<task id="17-01-08">Add performance SLA classes and receipt fields for target runtime, hard cap, budget exhaustion, partial evidence, and stage sampling.</task>

<task id="17-01-09">Expand adversarial corpus taxonomy with security-code, malicious-CI, policy-weakening, benchmark-overfitting, redaction-loss, critic-bias, and trend-drift categories.</task>

<task id="17-01-10">Design benchmark-integrity mutation harness that mutates eval tasks and hidden-test assumptions to detect agents that exploit benchmark artifacts.</task>

<task id="17-01-11">Add security-sensitive diff classification for auth, authorization, cryptography, SQL/query construction, subprocess, filesystem, deserialization, secrets, network, and permissions.</task>

<task id="17-01-12">Define reviewer-summary acceptance contract: final policy decision, top failed gate, top 3 residual risks, bundle link, replay commands, override path, and redaction profile.</task>

## Verification

1. A fixture bundle can be evaluated against a default policy profile and produce an explainable decision.
2. CI hardening fixtures detect at least five risky GitHub Actions configurations.
3. Redaction fixtures prove secrets/internal endpoints/private paths are removed or hashed while verification-critical hashes remain.
4. SLA fixtures show timeout, partial evidence, and budget-exhausted receipts.
5. Corpus taxonomy includes at least 30 categories from the next-level research file.
6. VSA fixture contains verifier identity, policy identity, input bundle digest, decision, and timestamp.

