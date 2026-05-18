# Next-Level Research: Pramaan

**Date:** 2026-05-19  
**Purpose:** Convert current research papers, benchmarks, and production security/supply-chain tools into concrete GSD upgrades for Pramaan.

## Executive Upgrade

Pramaan should evolve from "PR verification receipts" into a **policy-driven evidence operating system for AI-authored code**.

The next-level version is not only a bundle generator. It should:

1. verify evidence;
2. enforce repo-specific merge policy;
3. protect itself from malicious PR code;
4. calibrate warnings against repo baselines;
5. attribute failures to agents/workflows;
6. redact sensitive evidence safely;
7. produce VSA/SLSA/in-toto-compatible attestations;
8. evaluate itself against adversarial corpora and real benchmark tasks.

## Source Index and Impact

| Source | What It Suggests | Pramaan Change |
| --- | --- | --- |
| SWE-Lancer: https://openai.com/index/swe-lancer/ | Real freelance-style software tasks expose agent failure beyond toy tests. | Add real-repo and long-horizon PR scenarios to eval corpus. |
| SWE-bench Verified: https://openai.com/index/introducing-swe-bench-verified/ | Human-verified task curation matters because issue/test quality is a bottleneck. | Keep claim-scope and oracle-quality receipts as first-class stages. |
| SWE-bench Verified retirement analysis: https://openai.com/index/why-we-no-longer-evaluate-swe-bench-verified/ | Hidden tests can be narrow, wide, flaky, or misaligned. | Add oracle alignment categories and low-confidence scope warnings. |
| SWE-bench Pro: https://huggingface.co/papers/2509.16941 | Harder, long-context engineering tasks need more realistic evaluation. | Add large-diff, multi-file, dependency-change, and long-context scenarios. |
| Saving SWE-Bench: https://www.microsoft.com/en-us/research/publication/saving-swe-bench-a-benchmark-mutation-approach-for-realistic-agent-evaluation/ | Benchmark mutation can evaluate whether agents exploit benchmark artifacts. | Add benchmark-integrity mutation tests to Pramaan's eval harness. |
| SecureAgentBench: https://arxiv.org/abs/2509.22097 | Secure-code failures need dedicated scenarios, not only functional correctness tests. | Add security-regression corpus categories and security-sensitive diff gates. |
| CodeHalu: https://arxiv.org/abs/2405.00253 | Code hallucinations have taxonomy-level structure. | Keep hallucination categories in static receipts. |
| Delulu: https://arxiv.org/abs/2605.07024 | Fill-in-the-middle hallucinations include invented APIs, invalid params, undefined variables, and imports. | Add FIM-specific hallucination fixtures. |
| Don't Judge Code by Its Cover: https://arxiv.org/abs/2505.16222 | LLM judges are unreliable as sole gates. | Critic stages remain warning-only unless corroborated by execution evidence. |
| Position bias in LLM judges: https://arxiv.org/html/2406.07791v9 | Judge output changes with order/presentation. | Any optional critic receipt must record prompt order and position-swap result. |
| Self-preference bias: https://arxiv.org/abs/2410.21819 | Models may prefer their own style. | Agent-author attribution must be available when interpreting critic output. |
| Just et al. FSE 2014 mutation paper: https://homes.cs.washington.edu/~mernst/pubs/mutation-effectiveness-fse2014.pdf | Mutation testing correlates with real fault detection. | Mutation remains a core execution stage. |
| Papadakis et al. ICSE 2018: https://dl.acm.org/doi/pdf/10.1145/3180155.3180183 | Mutation is useful but imperfect and can be expensive. | Use diff scope, budgets, equivalent-mutant classification, and warnings. |
| Fuzz4All: https://arxiv.org/abs/2308.04748 | LLM-assisted fuzzing can raise coverage in broad systems. | Add fuzz-corpus growth and replay artifacts. |
| CodaMosa: https://dl.acm.org/doi/10.1109/ICSE48619.2023.00085 | Search-based test generation can target missing branches. | Add optional adversarial test-amplification roadmap after core v1. |
| Hypothesis docs: https://hypothesis.readthedocs.io/en/latest/reference/api.html | Replay and deterministic failure examples are core product features. | Persist seeds, example DB hashes, minimized examples, and replay commands. |
| fast-check docs: https://fast-check.dev/docs/introduction/why-property-based/ | Deterministic seeds and replay paths are available in JS/TS. | Persist fast-check seed/path and shrunken counterexamples. |
| SLSA spec: https://slsa.dev/spec/ | Provenance should be structured and verifiable. | Map bundle manifest to SLSA-compatible predicate fields. |
| SLSA Verification Summary Attestation: https://slsa.dev/verification_summary | A VSA summarizes a verifier's decision over an artifact. | Add `verification_summary` as a bundle output mode. |
| Sigstore cosign attestations: https://docs.sigstore.dev/cosign/verifying/attestation/ | in-toto attestations can be signed and verified with identities. | Add Sigstore/in-toto attestation path and identity metadata. |
| GitHub artifact attestations: https://docs.github.com/en/actions/concepts/security/artifact-attestations | GitHub can sign artifact provenance using Sigstore-backed flows. | Treat GitHub attestation as first CI-native signing path. |
| GitHub Actions security hardening: https://docs.github.com/en/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions | Untrusted input and tokens require strict workflow design. | Add CI hardening rules for untrusted PR code. |
| GitLab CI job artifacts docs: https://docs.gitlab.com/ci/jobs/job_artifacts/ | GitLab has a separate artifact model and retention semantics. | Add provider-neutral artifact abstraction before GitLab support. |
| GitLab ID tokens/OIDC docs: https://docs.gitlab.com/ci/secrets/id_token_authentication/ | GitLab supports OIDC identity for CI jobs. | Plan non-GitHub signing identity abstraction. |
| Open Policy Agent docs: https://www.openpolicyagent.org/docs/latest/ | Policy-as-code enables explainable, repo-specific gates. | Add policy engine for hard gates, warning gates, and override rules. |
| CUE language docs: https://cuelang.org/docs/ | CUE can validate structured config and schemas. | Consider CUE/JSON Schema policy profiles for repo-specific Pramaan config. |

## 35 Product and Design Improvements

### Must Add Before Receipt Schema v0.1 Freezes

1. Add `agent_author` object: product, model, version, agent mode, source, confidence, and prompt/context hash.
2. Add `human_override` object: decision, reason, accepted risks, reviewer identity source, timestamp, and merge outcome.
3. Add `policy_decision` object: hard fail, warning, informational, waived, not applicable.
4. Add `evidence_sensitivity` labels on receipts and artifacts: public, internal, secret-derived, redacted.
5. Add `redaction_manifest` to show what was removed or hashed before bundle export.
6. Add `stage_budget` fields: requested budget, consumed time, timeout reason, partial evidence flag.
7. Add `verification_summary` object compatible with SLSA VSA concepts.
8. Add `plugin_identity` and `plugin_permissions` to every plugin-emitted receipt.
9. Add `trust_boundary` metadata for each stage: local process, container, VM, remote service, unknown.
10. Add `critic_prompt_order` and `position_swap` fields for optional LLM critic receipts.

### Must Add Before Alpha MVP

11. Add policy-as-code config for merge gates and warning-only signals.
12. Add default policy profile: fail on oracle weakening, bundle tamper, static compile failure, missing required base/head evidence.
13. Add `pramaan policy explain` to explain why a PR failed or warned.
14. Add CI hardening checks for GitHub Actions: token permissions, fork behavior, pull_request_target hazards, cache poisoning risks.
15. Add security-sensitive diff classifier for auth, crypto, SQL, filesystem, subprocess, network, deserialization, and permissions.
16. Add redaction tests for secrets, internal hosts, private paths, tokens, and CI metadata.
17. Add generated replay commands to every failing fuzz/property receipt.
18. Add fixture for optional critic bias: same diff presented in swapped order.
19. Add FIM hallucination fixtures for invented APIs and invalid params.
20. Add a minimum reviewer summary contract: top failed gate, top 3 residual risks, replay command, override URL/path.

### Must Add Before Real MVP

21. Add repo-level calibration: mutation survival baseline, skipped-stage baseline, runtime baseline, oracle-warning baseline.
22. Add drift export: weekly trend by agent, repo, language, stage, risk family, and runtime.
23. Add real-repo benchmark suite: at least 10 open-source PR reproductions, with permissions-safe fixtures.
24. Add long-horizon corpus cases: multi-file changes, dependency changes, generated files, migrations, flaky tests.
25. Add security-regression corpus inspired by SecureAgentBench categories.
26. Add malicious PR corpus: test runner escape attempts, parser bombs, huge fixtures, symlink tricks, path traversal, cache poisoning.
27. Add plugin trust tests: malicious plugin tries to edit previous receipt, forge tool version, or omit skipped evidence.
28. Add provider-neutral artifact abstraction for GitHub/GitLab/local CI.
29. Add offline verification mode that does not require network access.
30. Add benchmark-integrity mutation: mutate eval tasks to detect shallow benchmark overfitting.

### Serious v1 Hardening

31. Add SLSA VSA-style output for verification decisions.
32. Add GitLab OIDC identity/signing path after GitHub path stabilizes.
33. Add cross-platform sandbox profile: Linux container, Windows runner, macOS runner, and self-hosted runner warning.
34. Add enterprise retention/export policy: full bundle, redacted bundle, summary-only bundle.
35. Add trend dashboard as optional consumer of drift exports, not as a CLI dependency.

## 24 New Failure Modes to Add

1. Agent identity missing, preventing failure-pattern learning.
2. Agent identity spoofed or inferred with low confidence but shown as certain.
3. Human override accepted risk but reason is not captured.
4. Override decision cannot be linked to merge/revert outcome.
5. Policy config changed in PR to make a failure warning-only.
6. Policy engine unavailable and PR appears green.
7. Receipt includes sensitive internal endpoint or path.
8. Redaction removes evidence needed for verification without marking it.
9. Plugin forges a receipt after seeing another plugin's output.
10. Plugin edits bundle manifest directly.
11. Malicious PR exploits test runner or fuzzer.
12. Malicious PR poisons dependency cache.
13. Malicious PR creates huge fixture or parser bomb to force timeouts.
14. Mutation stage times out and is interpreted as pass.
15. Fuzz stage finds counterexample but replay data is absent.
16. Baseline mutation survival is naturally high and causes alert fatigue.
17. Risk trend drifts upward but individual PRs remain below thresholds.
18. Optional critic is biased by diff order.
19. Generated code fixes benchmark-specific hidden tests without solving issue.
20. Security-sensitive code path changes but only functional tests run.
21. Non-GitHub CI artifact retention deletes proof bundle.
22. Self-hosted runner has unknown trust boundary but bundle looks normal.
23. Claim scope says one thing while public API change implies broader impact.
24. Artifact attestation signs the bundle but verifier identity/policy is not captured.

## Vague Tasks Needing Measurable Acceptance Criteria

| Existing Task | Problem | Replacement Acceptance Criteria |
| --- | --- | --- |
| "stage budgets strict enough" | No number. | Define small/medium/large diff budgets and fail/partial behavior. |
| "practical CI budgets" | Not measurable. | PR target: P50 under 5 min, P90 under 12 min for small/medium diffs on GitHub hosted runners; hard cap 20 min. |
| "useful PR summary" | Subjective. | Summary must include final policy decision, failed gates, top 3 residual risks, bundle link, replay commands, and override path. |
| "real tool integrations" | Too broad. | Each language must run configured static tool, emit skipped receipt if missing, and pass 5 positive/5 negative fixtures. |
| "security model complete" | Too broad. | Must cover malicious PR, malicious plugin, compromised tool cache, self-hosted runner, secret leakage, artifact tampering. |
| "corpus reaches 100+" | Needs distribution. | 25 oracle, 20 static/hallucination, 20 mutation/fuzz, 15 security, 10 CI/sandbox, 10 supply-chain/policy. |
| "dashboard-ready metrics" | Too vague. | Define JSONL export schema and include 3 aggregation examples. |
| "plugin protocol stable" | Undefined. | Protocol has version negotiation, permission model, fixture tests, and backwards compatibility policy. |

## Recommended Performance SLAs

| Diff Class | Definition | Target | Hard Cap | Required Behavior |
| --- | --- | --- | --- | --- |
| XS | <= 50 changed LOC, one language | P50 <= 2 min | 6 min | All P0 gates run, mutation/fuzz may be sampled. |
| S | <= 200 changed LOC, <= 5 files | P50 <= 5 min | 12 min | Static, oracle, bundle required; mutation/fuzz budgeted. |
| M | <= 800 changed LOC, <= 20 files | P50 <= 8 min | 20 min | Parallel stages; mutation/fuzz partial receipts allowed. |
| L | > 800 LOC or multi-package | P50 <= 15 min | 35 min | Split by package/file; report partial coverage explicitly. |
| Security-sensitive | Any size touching auth/crypto/SQL/secrets/permissions | P50 <= 12 min | 35 min | Security gates become hard or require override. |

## Hard Gates vs Warning-Only

### Hard Gates by Default

- Bundle tamper or unverifiable manifest.
- Missing base/head checkout evidence.
- Static compile/type failure in touched language when tool is configured.
- Test skip added in touched oracle without explicit scope note.
- Assertion removed or weakened in test covering claimed behavior.
- Original failing test does not pass unchanged when an original failing test exists.
- Security-sensitive diff with removed validation/authz/sanitization and no override.
- Policy config changed in the PR without maintainer-owned approval.
- Required stage unavailable due to missing tool and repo policy marks it required.

### Warning-Only by Default

- Surviving mutants below severe threshold.
- Fuzz/property not applicable due to impure or complex changed functions.
- Optional critic disagreement.
- Low-confidence claim scope.
- Missing linked issue.
- Performance benchmark missing unless hot path/security policy requires it.
- Formal verification not applicable.

## Store vs Redact

### Store in Full Internal Bundle

- Receipt JSON.
- Tool versions and command summaries.
- Artifact hashes.
- Exit codes and durations.
- Seeds, replay paths, minimized counterexamples.
- Base/head commit IDs.
- Policy decision and override record.
- Agent attribution metadata with confidence/source.

### Redact or Hash Before Export

- Environment variables and secrets.
- Internal hostnames, IP ranges, service URLs.
- Absolute private paths and usernames.
- Raw logs containing tokens, customer data, or proprietary snippets.
- Full prompt content unless explicitly allowed; store hash and source instead.
- Network endpoint details unless policy permits.
- Dependency mirror URLs with credentials or internal topology.

## New Eval/Corpus Categories

1. Weakened assertion.
2. Removed test.
3. Added skip/xfail.
4. Snapshot approval drift.
5. Fixture meaning drift.
6. Missing original regression.
7. Invented import/API.
8. Invalid parameter.
9. Undefined symbol.
10. FIM hallucination.
11. Policy config weakening.
12. Lockfile dependency drift.
13. Cache poisoning.
14. Malicious fuzzer/test-runner exploit attempt.
15. Parser bomb or huge fixture timeout.
16. Security validation removed.
17. Authorization boundary weakened.
18. Injection sanitization removed.
19. Claim/implementation mismatch.
20. Multi-agent handoff confusion.
21. Critic position bias.
22. Agent self-preference bias.
23. GitHub artifact retention/loss.
24. GitLab artifact/identity mismatch.
25. Redaction removes necessary evidence.
26. Plugin receipt forgery.
27. Bundle manifest tamper.
28. Baseline-noise alert fatigue.
29. Trend drift without per-PR threshold breach.
30. Benchmark overfitting/mutation escape.

## Resulting GSD Change

Add Phase 17: Policy, CI Hardening, and Evaluation Intelligence.

Phase 17 should consolidate the research-driven upgrades that cross-cut existing phases:

- policy-as-code gates;
- CI security hardening;
- performance SLA enforcement;
- SLSA VSA output;
- benchmark integrity mutation;
- secure-code corpus;
- redaction profiles;
- non-GitHub artifact/identity abstraction.

