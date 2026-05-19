# Roadmap: Pramaan

**Created:** 2026-05-18
**Granularity:** Coarse
**v1 Requirements:** 111 mapped

## Phase 1: Receipt-First CLI Skeleton

**Goal:** Create the project skeleton, CLI entry point, schemas, risk taxonomy, claim-scope model, and orchestrator contract so every future stage can emit auditable receipts tied to known risk IDs.

**Requirements:** CLI-01, CLI-02, CLI-03, RCPT-01, RCPT-02, RCPT-03, RCPT-05, RISK-01, RISK-02, SCOP-01, SCOP-02

**Success Criteria:**

1. `pramaan verify --base <ref> --head <ref>` runs in a Git repo and creates an output directory.
2. CLI emits at least one synthetic stage receipt using the real schema.
3. Schema validation passes for generated receipts.
4. Terminal summary distinguishes pass, fail, skipped, and not-applicable statuses.
5. Rust workspace and plugin directory skeleton match the intended architecture.
6. Claim-scope schema can represent expected behavior, out-of-scope behavior, touched public APIs, and confidence.
7. Risk taxonomy schema and top-100 flaw register exist, and synthetic receipts can reference risk IDs.

## Phase 2: Sandbox and Static Checks

**Goal:** Produce real environment evidence and language static-check receipts for Python, TypeScript, and Rust repositories.

**Requirements:** SNDB-01, SNDB-02, SNDB-03, STAT-01, STAT-02, STAT-03, STAT-04, STAT-05

**Success Criteria:**

1. Sandbox creates isolated base/head worktrees and records commit/environment evidence.
2. Python plugin emits receipts for compile/type/lint checks when tools are present.
3. TypeScript plugin emits receipts for type/lint checks when tools are present.
4. Rust plugin emits receipts for cargo check/test-build checks when tools are present.
5. Missing or unavailable tools are represented as skipped/not-applicable receipts, not hidden failures.
6. Static failures include hallucination categories when the evidence supports classification.

## Phase 3: Oracle Integrity and Killer Demo

**Goal:** Detect test weakening and oracle/scope mismatch, then prove Pramaan's value through a demo where ordinary CI passes but Pramaan fails.

**Requirements:** SCOP-03, ORCL-01, ORCL-02, ORCL-03, ORCL-04, ORCL-05

**Success Criteria:**

1. Oracle stage detects deleted/skipped tests in changed test files.
2. Oracle stage detects weakened Python assertions.
3. Oracle stage detects weakened TypeScript/JavaScript assertions.
4. Snapshot and fixture diffs are classified as oracle-sensitive.
5. Demo PR passes normal tests and fails Pramaan with a clear receipt naming the weakened oracle.
6. Oracle stage can flag narrow, wide, changed, and missing-regression risks relative to the claim scope.

## Phase 4: Diff Mutation and Differential Fuzz

**Goal:** Add execution-grounded test-quality and regression checks scoped to changed files and eligible pure functions.

**Requirements:** MUTN-01, MUTN-02, MUTN-03, MUTN-04, MUTN-05, FUZZ-01, FUZZ-02, FUZZ-03, FUZZ-04

**Success Criteria:**

1. Python diff-scoped mutation stage reports created/killed/survived/timed-out mutants.
2. TypeScript diff-scoped mutation stage reports kill-rate against threshold.
3. Rust diff-scoped mutation stage produces usable receipts.
4. Mutation receipts record timeout policy, filtering mode, cache/reuse state, and skipped/unviable mutant rationale.
5. Python Hypothesis differential checks run for eligible changed pure functions.
6. TypeScript fast-check differential checks run for eligible changed pure functions.
7. Differential receipts include seeds, generated input counts, corpus hashes, replay data, and divergence summaries.

## Phase 5: Bundle Signing and Verification

**Goal:** Turn receipts into a durable, verifiable proof bundle with manifest, hashes, and local signing/signable output.

**Requirements:** RCPT-04, RISK-03, BNDL-01, BNDL-02, BNDL-03, BNDL-04

**Success Criteria:**

1. Bundle manifest references every receipt and artifact by content hash.
2. Bundle includes tool versions, seeds, corpus hashes, stage statuses, and final status.
3. Local dev signing or signable output is supported.
4. `pramaan bundle verify <path>` validates manifest, hashes, and signature/signable metadata.
5. Bundle summary uses accurate language and avoids correctness-proof claims.
6. Bundle manifest can carry GitHub artifact attestation metadata when available.
7. Bundle summary shows mitigated, residual, skipped, and not-applicable risk families without one opaque score.

## Phase 6: GitHub Action and Public Demo Loop

**Goal:** Make Pramaan usable on pull requests and package the killer demo as a repeatable proof of value.

**Requirements:** GHAC-01, GHAC-02, GHAC-03, GHAC-04, RISK-04, DEMO-01, DEMO-02, DEMO-03

**Success Criteria:**

1. GitHub Action runs Pramaan on pull requests.
2. Action uploads the proof bundle as an artifact.
3. Action publishes a concise PR summary focused on failed and risky stages.
4. Action can optionally request artifact attestation for the proof bundle.
5. Demo repository includes the weakened-test AI-fix scenario.
6. Demo documentation shows normal CI green and Pramaan red.
7. Demo proof bundle can be inspected in under 30 seconds.
8. Demo/eval scenarios map to risk IDs from the top-100 flaw register.

## Phase 7: Adapter Certification Expansion

**Goal:** Extend Pramaan from AI-authored PR verification into MCP/agent-tool adapter certification without distracting from the core trust layer.

**Requirements:** ADPT-01, ADPT-02, ADPT-03, ADPT-04

**Success Criteria:**

1. Repository documents Pramaan Adapter Certification as an adjacent certification mode, not a separate registry product.
2. Adapter certification schema models tool name quality, input/output typing, auth scopes, idempotency, retry/rate-limit metadata, and audit receipts.
3. Starter adapter risk register exists for MCP/agent tool failure modes.
4. Example adapter certification receipt fixture maps findings to stable adapter risk IDs.

## Phase 8: Killer Demo and Proof Bundles

**Goal:** Create undeniable public demos where normal CI passes but Pramaan catches weakened tests, oracle drift, or hallucinated code evidence.

**Requirements:** DEMO-04, DEMO-05, DEMO-06, RISK-05

**Success Criteria:**

1. Weakened-test demo passes ordinary CI and fails Pramaan with a precise oracle receipt.
2. Snapshot/fixture drift demo produces a clear oracle-sensitive receipt.
3. Static/hallucination demo maps invented code to a stable risk ID.
4. Generated proof bundles can be inspected and verified.
5. Demo documentation explains the value in under 30 seconds.

## Phase 9: Receipt and Bundle Trust Hardening

**Goal:** Freeze the first receipt/bundle contract and add compatibility, golden, and tamper tests.

**Requirements:** RCPT-06, RCPT-07, BNDL-05, BNDL-06

**Success Criteria:**

1. Receipt schema version `0.1` has documented compatibility rules.
2. Checked-in fixtures are validated by schema compatibility tests.
3. Golden tests detect accidental receipt shape changes.
4. Bundle verification catches missing artifacts, changed receipts, changed manifests, and changed signing metadata.

## Phase 10: GitHub Action Production Readiness

**Goal:** Make Pramaan usable as a serious pull-request GitHub Action.

**Requirements:** GHAC-05, GHAC-06, GHAC-07, GHAC-08

**Success Criteria:**

1. Action installs or downloads the CLI deterministically.
2. Action exposes stable inputs for refs, output path, failure policy, and bundle upload.
3. Action uploads bundles and renders reviewer-focused PR summaries.
4. Forked-PR permissions and failure modes are documented.

## Phase 11: Sandbox, Claim Scope, and Static Depth

**Status:** Completed 2026-05-19 as PASS_WITH_RISKS. See `.planning/reports/phase-11-aggregate-report.md`.

**Goal:** Produce practical environment, claim-scope, and static evidence for real Python, TypeScript, and Rust repositories.

**Requirements:** SNDB-04, SNDB-05, SCOP-04, SCOP-05, STAT-06, STAT-07

**Success Criteria:**

1. Environment receipts include OS, architecture, timezone, locale, toolchains, commits, dirty state, and lockfile hashes.
2. Container image and network-policy evidence is captured when available.
3. PR title/body, linked issue context, and changed public APIs feed claim-scope receipts.
4. Python, TypeScript, and Rust static tools run when configured and skip honestly when absent.
5. Hallucination failures are classified with stable categories.

## Phase 12: Oracle Integrity Engine

**Status:** Completed 2026-05-19 as PASS_WITH_RISKS. See `.planning/reports/phase-12-aggregate-report.md`.

**Goal:** Detect realistic test-oracle tampering across Python, TypeScript, and Rust.

**Requirements:** ORCL-06, ORCL-07, ORCL-08, ORCL-09

**Success Criteria:**

1. Python pytest assertion, skip, xfail, raises, and parametrized-case changes are detected.
2. Jest/Vitest expectation weakening is detected.
3. Rust assertion and snapshot weakening is detected.
4. Deleted/renamed tests are tracked through stable fingerprints.
5. Fixture, snapshot, boundary-case, and error-case removals produce clear reviewer summaries.

## Phase 13: Mutation and Differential Fuzz Adapters

**Goal:** Wrap real mutation and property-testing tools with budgets, replay metadata, and honest skipped/timeout receipts.

**Requirements:** MUTN-06, MUTN-07, FUZZ-05, FUZZ-06

**Success Criteria:**

1. mutmut, StrykerJS, and cargo-mutants adapters run on changed scopes.
2. Mutation receipts record mutant counts, thresholds, timeouts, cache state, and filtering mode.
3. Hypothesis and fast-check differential checks run for eligible pure functions.
4. Fuzz receipts include seeds, replay data, minimized counterexamples, corpus hashes, and divergence classification.

## Phase 14: Attestation, Corpus, and Evals

**Goal:** Make proof bundles durable and measurable through signing/attestation plus adversarial evaluation.

**Requirements:** BNDL-07, BNDL-08, EVAL-01, EVAL-02

**Success Criteria:**

1. Sigstore and GitHub artifact attestation paths are available.
2. Bundle manifests map cleanly to in-toto/SLSA-compatible predicates.
3. Offline verification works for downloaded bundles.
4. Adversarial corpus reaches 100+ risk-mapped scenarios.
5. Benchmark reports track false positives, false negatives, runtime, and reviewer time-to-understand.

## Phase 15: Documentation, Language Expansion, and Adapter Gates

**Goal:** Prepare Pramaan for external adoption while keeping PR verification as the primary product.

**Requirements:** DOCS-01, DOCS-02, LANG-01, LANG-02, ADPT-05

**Success Criteria:**

1. Operator, plugin-author, security-model, threat-model, enterprise-deployment, and troubleshooting guides exist.
2. PR-summary and bundle-inspection examples are rendered for reviewers.
3. Python, TypeScript, and Rust plugin readiness gates are defined.
4. Go and Java support wait for protocol stability and first-language depth.
5. Adapter certification remains bounded and does not distract from core PR verification.

## Phase 16: Attribution, Feedback, Calibration, and Verifier Security

**Goal:** Add the missing production trust layer around Pramaan itself: who authored the code, who overrode the evidence, what baseline is normal for this repo, whether agent quality is drifting, whether plugins can be trusted, and whether bundles safely redact sensitive data.

**Requirements:** ATTR-01, FBCK-01, PERF-01, CAL-01, DRFT-01, PLGN-01, SCRB-01, VCS-01

**Success Criteria:**

1. Receipt/bundle schemas can represent agent-author attribution and multi-agent provenance before v0.1 freezes.
2. Reviewer overrides are captured as auditable evidence with accepted risk IDs and reasons.
3. Performance SLA targets and per-stage budgets are documented and reflected in receipts.
4. Repo-level calibration distinguishes normal noise from newly risky PR behavior.
5. Trend/drift exports aggregate multiple bundles by agent, repo, risk family, and runtime.
6. Plugin trust model prevents untrusted plugins from poisoning receipts or manifests.
7. Bundle redaction policy prevents secrets, private paths, internal endpoints, and sensitive CI metadata from leaking.
8. Provider-neutral VCS/CI abstraction prepares GitLab support without destabilizing GitHub Action work.

## Phase 17: Policy, CI Hardening, and Evaluation Intelligence

**Goal:** Add policy-as-code gates, CI attack resistance, performance SLA enforcement, VSA output, redaction profiles, and benchmark/evaluation intelligence based on current research.

**Requirements:** POL-01, POL-02, CI-01, CI-02, VSA-01, REDACT-01, EVAL-03, SEC-01, SLA-01, FORGE-01, CORP-01, SUMM-01

**Success Criteria:**

1. A default policy profile classifies hard gates, warning gates, waivers, and stage requirements.
2. `pramaan policy explain` can explain final decisions from a bundle and policy profile.
3. CI hardening checks detect risky untrusted-PR workflow patterns.
4. Performance SLA classes are reflected in receipts and budget-exhausted summaries.
5. Pramaan can produce a VSA-style verification summary fixture.
6. Redaction profiles preserve verification-critical hashes while removing sensitive details.
7. Security-sensitive diff classification changes gate severity for auth, crypto, SQL, secrets, filesystem, subprocess, network, deserialization, and permissions.
8. Adversarial corpus taxonomy includes security-code, malicious-CI, policy-weakening, benchmark-overfitting, redaction-loss, critic-bias, and trend-drift cases.
9. Non-GitHub CI abstraction captures artifacts, identity, comments, refs, merge requests, and OIDC signing.

## Coverage

| Phase | Requirements | Count |
|-------|--------------|-------|
| Phase 1 | CLI-01, CLI-02, CLI-03, RCPT-01, RCPT-02, RCPT-03, RCPT-05, RISK-01, RISK-02, SCOP-01, SCOP-02 | 11 |
| Phase 2 | SNDB-01, SNDB-02, SNDB-03, STAT-01, STAT-02, STAT-03, STAT-04, STAT-05 | 8 |
| Phase 3 | SCOP-03, ORCL-01, ORCL-02, ORCL-03, ORCL-04, ORCL-05 | 6 |
| Phase 4 | MUTN-01, MUTN-02, MUTN-03, MUTN-04, MUTN-05, FUZZ-01, FUZZ-02, FUZZ-03, FUZZ-04 | 9 |
| Phase 5 | RCPT-04, RISK-03, BNDL-01, BNDL-02, BNDL-03, BNDL-04 | 6 |
| Phase 6 | GHAC-01, GHAC-02, GHAC-03, GHAC-04, RISK-04, DEMO-01, DEMO-02, DEMO-03 | 8 |
| Phase 7 | ADPT-01, ADPT-02, ADPT-03, ADPT-04 | 4 |
| Phase 8 | DEMO-04, DEMO-05, DEMO-06, RISK-05 | 4 |
| Phase 9 | RCPT-06, RCPT-07, BNDL-05, BNDL-06 | 4 |
| Phase 10 | GHAC-05, GHAC-06, GHAC-07, GHAC-08 | 4 |
| Phase 11 | SNDB-04, SNDB-05, SCOP-04, SCOP-05, STAT-06, STAT-07 | 6 |
| Phase 12 | ORCL-06, ORCL-07, ORCL-08, ORCL-09 | 4 |
| Phase 13 | MUTN-06, MUTN-07, FUZZ-05, FUZZ-06 | 4 |
| Phase 14 | BNDL-07, BNDL-08, EVAL-01, EVAL-02 | 4 |
| Phase 15 | DOCS-01, DOCS-02, LANG-01, LANG-02, ADPT-05 | 5 |
| Phase 16 | ATTR-01, FBCK-01, PERF-01, CAL-01, DRFT-01, PLGN-01, SCRB-01, VCS-01 | 8 |
| Phase 17 | POL-01, POL-02, CI-01, CI-02, VSA-01, REDACT-01, EVAL-03, SEC-01, SLA-01, FORGE-01, CORP-01, SUMM-01 | 12 |

**Total mapped:** 111 / 111

---
*Roadmap updated: 2026-05-19 after next-level internet research pass*
