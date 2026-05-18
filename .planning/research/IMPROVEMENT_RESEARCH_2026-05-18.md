# Improvement Research: Pramaan

**Date:** 2026-05-18
**Purpose:** Tighten Pramaan's v1 architecture against current evidence from supply-chain attestation tooling, mutation/property tooling, and recent AI-code evaluation failures.

## Executive Findings

1. **Add claim/scope receipts before oracle integrity.** OpenAI's 2026 SWE-bench Verified audit found that many remaining benchmark failures were caused by tests that were too narrow, too wide, or mismatched with task descriptions. Pramaan should therefore record the PR's claimed behavior scope and flag oracle/scope mismatch risk, not only detect weakened tests.
2. **Update supply-chain language to current SLSA.** SLSA has moved beyond the originally cited v1.1 framing; current docs present SLSA v1.2 and recommended attestation formats including provenance and verification summary attestations.
3. **Treat GitHub artifact attestation as a first-class CI path.** GitHub artifact attestations are already Sigstore-backed and map to SLSA build levels. For Pramaan, the signed proof bundle should be usable both through local/dev signing and GitHub-native artifact attestation.
4. **Mutation stages need budget receipts, not just kill-rate receipts.** StrykerJS incremental mode, mutmut coverage filtering, and cargo-mutants timeout behavior all point to the same product rule: Pramaan must record what it skipped, reused, timed out, and why.
5. **Property/fuzz receipts must capture replay data.** Hypothesis and fast-check both support deterministic/replayable workflows. Pramaan should persist seeds, replay paths, example database/corpus hashes, and minimized counterexamples.
6. **Code hallucination detection should use a taxonomy.** CodeHalu and the newer Delulu benchmark support classifying hallucinations as invented APIs, invalid parameters, undefined variables, non-existent imports, and broader mapping/naming/resource/logic errors.

## Evidence Notes

### Claim/Scope Integrity

OpenAI's SWE-bench Verified retirement analysis is directly relevant to Pramaan. It says model-generated code was evaluated against hidden tests, then reports residual issues where tests were overly strict, task descriptions underspecified, or environments flaky. In an audit of 138 difficult tasks, 59.4% had material test-design or problem-description issues, including 35.5% narrow tests and 18.8% wide tests.

Implication for Pramaan:

- Add a **Claim Scope** stage before Oracle Integrity.
- Capture PR title/body, linked issue text, changed public APIs, and developer-supplied expected behavior notes.
- Mark oracle risks as:
  - `narrow_oracle_risk`: tests enforce implementation details not present in the claim.
  - `wide_oracle_risk`: tests cover behavior outside the claim.
  - `changed_oracle_risk`: tests/fixtures/snapshots changed in the PR.
  - `missing_regression_risk`: no original failing test or no stable reproduction.

This makes Pramaan's claim more defensible: the bundle reports not just "tests passed" or "tests were not weakened," but whether the tests appear aligned with the claimed change.

### Supply Chain and Bundle Signing

Current SLSA documentation presents SLSA v1.2, with provenance and verification summary attestations as recommended attestation formats. Sigstore keyless signing binds short-lived certificates to OIDC identities and logs signing events in Rekor for public flows. GitHub artifact attestations create signed provenance for build artifacts and are generated through Sigstore; public repositories use the public Sigstore instance with transparency logging, while private repositories use GitHub's Sigstore instance without public transparency log.

Implication for Pramaan:

- Treat `pramaan bundle attest` as a v1.5/v2 hardening path, but design v1 bundle manifests to be compatible with in-toto/SLSA-style predicates.
- Add a GitHub Action path that can call GitHub artifact attestation for the proof bundle.
- Make verification explicit: signing alone is not the value; bundle verification is.

### Mutation Testing Tooling

StrykerJS supports incremental mutation testing and can reuse previous mutant results when code/tests are unchanged, but its docs list limitations around non-code files, environment changes, snapshots, and test-runner reporting detail. Mutmut supports limiting mutation to covered lines via coverage.py and can reduce incidental test coupling through stack-depth configuration. Cargo-mutants explicitly handles hangs/timeouts by killing build or test runs and continuing.

Implication for Pramaan:

- Mutation receipts must include:
  - changed files considered;
  - mutants generated/killed/survived/timed out/unviable;
  - threshold used;
  - incremental cache/reuse status if applicable;
  - timeout policy;
  - coverage/filter mode;
  - known limitations for the selected runner.
- Surviving mutants should not be a generic red badge; classify them as `requires_review`, `likely_equivalent`, or `test_gap` where the tool output permits.

### Property and Differential Testing

Hypothesis supports deterministic mode and saves failing examples to a database for replay. fast-check supports deterministic seeds and prints seeds/replay data for failures, with shrinking to smaller counterexamples.

Implication for Pramaan:

- Differential property receipts should record:
  - seed;
  - replay path where available;
  - minimized counterexample;
  - example database/corpus hash;
  - pre-patch output;
  - post-patch output;
  - scope classification of divergence.

### Code Hallucination Detection

CodeHalu classifies code hallucinations into mapping, naming, resource, and logic categories and grounds detection in execution verification. Delulu, submitted in May 2026, focuses on fill-in-the-middle hallucinations across 7 languages and 4 hallucination types, including invented APIs, invalid parameters, undefined variables, and non-existent imports.

Implication for Pramaan:

- Static/Hallucination receipts should not merely say "static failed."
- Add `hallucination_category` when evidence allows:
  - `invented_api`
  - `invalid_parameter`
  - `undefined_symbol`
  - `nonexistent_import`
  - `resource_mismatch`
  - `logic_mismatch`
  - `unknown`

## Recommended Project Changes

### Add v1 Claim Scope Stage

Insert this into the v1 pipeline:

```text
CLI + GitHub Action -> Receipts -> Sandbox -> Claim Scope ->
Static -> Oracle Integrity -> Diff Mutation -> Differential Property/Fuzz ->
Signed/Attested Bundle
```

This stage is cheap, high-leverage, and strengthens every downstream receipt.

### Tighten Phase 1

Phase 1 should define the receipt schema plus the claim/scope schema. This avoids bolting scope interpretation onto later stages.

### Tighten Phase 3

Phase 3 should detect not only weakened tests, but also oracle/scope mismatch risk. The demo can have two variants:

1. AI weakens an assertion and CI goes green.
2. AI adds a test that enforces an implementation detail not required by the issue.

### Move GitHub Attestation Into Phase 5/6 Interface

Phase 5 should keep local signing/signable output, but the schema should already support GitHub artifact attestation metadata. Phase 6 should include a GitHub Action option to attest uploaded bundles.

## Source Index

- SLSA specification v1.2: https://slsa.dev/spec/
- Sigstore keyless signing overview: https://docs.sigstore.dev/cosign/signing/overview/
- GitHub artifact attestations: https://docs.github.com/en/actions/concepts/security/artifact-attestations
- StrykerJS incremental mutation: https://stryker-mutator.io/docs/stryker-js/incremental/
- mutmut documentation: https://mutmut.readthedocs.io/en/latest/
- cargo-mutants timeouts: https://mutants.rs/timeouts.html
- Hypothesis API reference: https://hypothesis.readthedocs.io/en/latest/reference/api.html
- fast-check property-based testing: https://fast-check.dev/docs/introduction/why-property-based/
- CodeHalu paper: https://arxiv.org/abs/2405.00253
- Delulu paper: https://arxiv.org/abs/2605.07024
- OpenAI SWE-bench Verified analysis: https://openai.com/index/why-we-no-longer-evaluate-swe-bench-verified/
