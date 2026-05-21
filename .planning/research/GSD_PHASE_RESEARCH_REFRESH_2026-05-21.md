# GSD Phase Research Refresh - 2026-05-21

Purpose: revise Pramaan's remaining GSD phases after reviewing current
research, benchmarks, and production docs. The product remains evidence-first:
Pramaan produces auditable verification evidence, not proof of arbitrary code
correctness.

## Sources Reviewed

| Source | Link | What it changes for Pramaan |
| --- | --- | --- |
| SWE-Lancer | https://openai.com/index/swe-lancer/ | Keep real-world task evidence and economic failure framing in pilot reports. |
| SWE-Bench Pro | https://arxiv.org/abs/2509.16941 | Add external long-horizon pilots and failure-mode clustering before public Alpha. |
| SWE-Bench contamination analysis | https://www.microsoft.com/en-us/research/publication/the-swe-bench-illusion-when-state-of-the-art-llms-remember-instead-of-reason/ | Add benchmark-integrity and contamination controls to eval phases. |
| Property-Generated Solver | https://arxiv.org/abs/2506.18315 | Prioritize property/fuzz harness execution and invariant capture, not only generated examples. |
| Mutation-guided LLM test generation at Meta | https://arxiv.org/abs/2501.12862 | Use mutation survivors to generate or require better tests; record survivor classes. |
| GitHub artifact attestations | https://docs.github.com/en/actions/concepts/security/artifact-attestations | Build GitHub-native provenance and offline verification into signing phases. |
| GitHub offline attestation verification | https://docs.github.com/en/actions/how-tos/security-for-github-actions/using-artifact-attestations/verifying-attestations-offline | Add downloaded-bundle/offline verification as a release gate. |
| SLSA Verification Summary Attestation | https://slsa.dev/spec/v1.2/verification_summary | Emit Pramaan's final policy decision as a VSA-style artifact. |
| Sigstore cosign blob signing | https://docs.sigstore.dev/cosign/signing/signing_with_blobs/ | Start with signed blobs/bundles before deeper custom signing code. |
| GitHub Actions hardening | https://docs.github.com/en/enterprise-cloud@latest/actions/how-tos/security-for-github-actions/security-guides/security-hardening-for-github-actions | Add untrusted PR and `pull_request_target` gates to CI hardening. |
| Agentic Workflow Injection | https://arxiv.org/abs/2605.07135 | Add taint-style analysis from issue/PR text to agent prompts, tools, and workflow sinks. |
| GitHub SARIF/code scanning | https://docs.github.com/en/code-security/code-scanning/integrating-with-code-scanning/using-code-scanning-with-your-existing-ci-system | Export Pramaan findings as SARIF so they show up in existing security review surfaces. |
| Open Policy Agent CI/CD docs | https://www.openpolicyagent.org/docs/cicd | Make policy profiles executable and portable, not hard-coded only. |
| OpenSSF Scorecard | https://openssf.org/scorecard/ | Use repository security posture as pilot metadata and adoption signal. |
| OpenSSF Allstar | https://github.com/ossf/allstar | Consider org-level policy enforcement and workflow hygiene checks. |
| GitLab attestations API | https://docs.gitlab.com/api/attestations/ | Design attestation and forge abstraction for non-GitHub enterprises. |
| GitLab OIDC ID tokens | https://docs.gitlab.com/ci/secrets/id_token_authentication/ | Add GitLab identity/OIDC differences before implementation. |
| Wasmtime security docs | https://docs.wasmtime.dev/security.html | Treat WASM as one possible plugin boundary, with explicit threat model. |
| LLM judge position bias | https://arxiv.org/abs/2406.07791 | Keep critics warning-only and require position-swap/human review if added. |
| Self-preference bias in LLM judgments | https://huggingface.co/papers/2506.02592 | Do not make LLM judge agreement a merge gate. |
| AI-generated code vulnerability study | https://arxiv.org/abs/2510.26103 | Add secure-code adversarial scenarios and CodeQL/SARIF integration. |

## Research-Driven Process Changes

1. **External pilots before public Alpha.** Internal fixtures are useful but do
   not prove signal/noise on real codebases.
2. **VSA and SARIF are first-class outputs.** Reviewers already use provenance
   and code scanning surfaces; Pramaan should plug into those instead of only
   inventing new dashboards.
3. **Policy should become portable.** Keep the Rust default policy, but add
   OPA/Conftest export or parity tests so enterprise users can encode their own
   gates.
4. **Agentic workflow injection is now in scope.** PR bodies, issue text, and
   comments can become untrusted agent inputs; CI hardening must track those
   flows.
5. **Benchmark integrity needs its own phase.** SWE-Bench contamination research
   makes overfit evals a real product risk.
6. **Mutation survivors should drive test generation.** The Meta mutation-guided
   work suggests survivors are not just a score; they are prompts for better
   tests.
7. **Plugin isolation must precede plugin ecosystem expansion.** Third-party
   plugins are a verifier supply-chain risk.
8. **Calibration is not a dashboard nice-to-have.** Per-repo baselines are how
   Pramaan avoids alert fatigue.
9. **Non-GitHub support should be designed before hard-coded assumptions deepen.**
   GitLab attestation and OIDC differ enough to affect schemas and trust model.
10. **Critic/LLM review stays warning-only.** Bias research supports the current
    execution-first product posture.

## Revised Build Order

| New phase | Reason |
| --- | --- |
| 26 External Alpha Pilots | Proves value on real repos before more feature expansion. |
| 27 Parser-Backed Oracle | Closes the strongest current "structured but not AST" gap. |
| 28 Tool-Backed Fuzz/Replay | Turns deterministic replay into real Hypothesis/fast-check evidence. |
| 29 Attestation/VSA | Makes bundles portable and verifiable. |
| 30 Redaction Profiles | Makes bundle export safe enough for pilots. |
| 31 Plugin Trust/Isolation | Prevents verifier plugins from poisoning receipts. |
| 32 SARIF/Policy/AWI | Integrates with existing security review and agentic workflow threat models. |
| 33 Corpus 25 | Builds targeted adversarial evidence before large corpus claims. |
| 34 Calibration/Drift/Overrides | Prevents alert fatigue and captures human feedback. |
| 35 Docs/Release Packaging | Makes external adoption possible after trust basics exist. |
| 36 Language Depth | Deepens Python/TS/Rust before adding Go/Java. |
| 37 Forge Abstraction | Prepares GitLab/Gitea/Bitbucket without destabilizing GitHub. |
| 38 Multi-Agent Provenance | Tracks real agent handoffs and commit chains. |
| 39 Adapter Certification | Keeps adjacent adapter work bounded. |
| 40 Serious v1 Gate | Forces an evidence-based release decision. |

## Acceptance Rule

Every new phase must produce at least one of:

- executable code and tests;
- a checked fixture or pilot report;
- a schema or policy artifact;
- a security/threat model update;
- a measurable release-gate decision.

No new broad research phase should be added unless it names a file, fixture,
schema, policy, or gate it will change.
