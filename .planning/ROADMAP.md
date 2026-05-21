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

## Phase 18: P0 Product Honesty and Direction

**Goal:** Lock the narrow product thesis and make the public surface honest without weakening the story.

**Priority:** P0

**Success Criteria:**

1. `STATUS.md` exists and maps shipped, partial, stub, planned, and experimental capabilities.
2. `README.md` links to status and avoids implying unimplemented Sigstore, in-toto, real mutation, real fuzz, or full pipeline orchestration.
3. `.planning/STATE.md` names Pramaan as a PR evidence-bundle verifier, not a correctness oracle or generic CI replacement.
4. First ICP, first killer workflow, non-goals, research sufficiency checklist, and pivot criteria are documented.

## Phase 19: P0 Receipt Golden Tests and Canonical Evidence

**Goal:** Make receipt and bundle evidence stable enough that fixture drift and hash drift are caught early.

**Priority:** P0

**Success Criteria:**

1. Generated receipts can be compared to approved golden fixtures.
2. Receipt and bundle serialization rules are deterministic and documented.
3. Schema/runtime drift is visible through tests.
4. Existing checked-in examples are migrated or explicitly labeled legacy.

## Phase 20: P0 SLA and Policy Gates

**Goal:** Make Pramaan usable in PR CI by giving reviewers clear runtime budgets and explainable policy outcomes.

**Priority:** P0

**Success Criteria:**

1. Performance SLA classes and per-stage budgets are documented and represented in receipts.
2. Default policy-as-code profile separates hard gates, warning gates, waivers, and required stages.
3. `pramaan policy explain` explains why a bundle passes, warns, or fails.
4. GitHub Action summaries reflect policy decisions without hiding skipped or timed-out stages.

## Phase 21: P1 Sandbox, Threat Model, and Redaction

**Goal:** Treat the verifier itself as an attack surface and harden the evidence boundary around untrusted PR code.

**Priority:** P1

**Success Criteria:**

1. Sandbox evidence captures OCI/container identity where available.
2. Source changes after stage execution are detected and surfaced.
3. `docs/threat-model.md` covers malicious PR code, plugins, mutation tools, fuzzers, parsers, CI runners, cache poisoning, and artifact tampering.
4. Bundle redaction policies protect secrets, internal endpoints, private paths, and sensitive CI metadata.
5. CI hardening checks warn on unsafe workflow patterns for untrusted PRs.

## Phase 22: P1 Claim Scope and Static Security Signals

**Goal:** Improve the bridge between what the PR claims, what changed, and which static/security risks should raise gate severity.

**Priority:** P1

**Success Criteria:**

1. Linked issue text can be ingested when available.
2. Maintainer scope notes can define expected and out-of-scope behavior.
3. Vague or missing claims produce stable risk IDs.
4. Relaxed static-check configuration is detected.
5. Security-sensitive diffs are classified for auth, authorization, crypto, SQL/query construction, subprocess, filesystem, deserialization, secrets, network, and permissions.
6. Semantic claim-implementation mismatch remains bounded evidence, not a sole merge gate.

## Phase 22.5: P0 Assertion Truth Audit Gate

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS.

**Goal:** Audit every public and planning assertion so the repo cannot overstate
what is implemented.

**Priority:** P0

**Success Criteria:**

1. `docs/claim-audit.md` inventories claims from `README.md`, `STATUS.md`,
   `TASKS.md`, `.planning/ROADMAP.md`, docs, schemas, examples, and
   `action.yml`.
2. Every claim is labeled as `executable-test`, `checked-fixture`,
   `manual-proof`, `implemented-untested`, `partial`, `planned`,
   `experimental`, or `false-or-stale`.
3. Every implemented claim points to a test, fixture, repeatable command, or
   accepted-risk follow-up.
4. Every false, stale, or overstated claim is rewritten, downgraded, or removed.
5. Alpha release gates fail if public implemented claims lack evidence.
6. The completion summary records claim counts and unresolved risk.

## Phase 23: P1 AST Oracle Extractors

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS. Structured extractor
evidence landed; full compiler AST integrations remain explicit follow-up.

**Goal:** Replace the highest-risk heuristic oracle checks with AST-backed extractors and golden fixtures.

**Priority:** P1

**Success Criteria:**

1. Python pytest assertions, skips, xfails, raises, parametrized cases, and deleted/renamed tests are extracted through an AST-aware path.
2. TypeScript/Jest/Vitest expectations and skip markers are extracted through an AST-aware path.
3. Rust assertions, panic tests, `#[ignore]`, and snapshot-sensitive patterns are extracted through a parser-backed path.
4. Golden fixtures cover positive and negative cases for each language.

## Phase 24: P1 Real Mutation and Property/Fuzz Adapters

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS. Tool-backed mutation
execution is available when adapters are installed; fuzz/property remains
deterministic replay evidence with adapter availability labels until safe
Hypothesis/fast-check harness generation lands.

**Goal:** Replace simulated/advisory execution with real tool-backed mutation and property/fuzz checks where tools are installed.

**Priority:** P1

**Success Criteria:**

1. Python `mutmut` and Hypothesis run diff-scoped with budgets and replay metadata.
2. TypeScript StrykerJS and fast-check run diff-scoped with budgets and replay metadata.
3. Rust cargo-mutants runs on changed crates/modules with timeout and survivor evidence.
4. Receipts record mutants created, killed, survived, timed out, skipped, unviable, thresholds, cache state, seeds, corpus hashes, and counterexamples.
5. Missing tools, unsupported code, and timeouts are visible residual risk, never silent passes.

## Phase 25: P0/P1 Pilot Gate and Alpha Decision

**Status:** Completed 2026-05-21 as NO_GO_PUBLIC_ALPHA. Internal pilot fixtures
passed, but external real-repository pilots and remaining public-claim gaps
remain public Alpha blockers.

**Goal:** Prove whether the P0/P1 loop is actually useful before expanding into P2/P3 features.

**Priority:** P0/P1 gate

**Success Criteria:**

1. Pramaan runs on at least three selected real repositories.
2. Pilot report records runtime, skipped-stage profile, false positives, false negatives, reviewer time-to-understand, and top residual risks.
3. Alpha MVP release gates are evaluated honestly.
4. Unfinished P0/P1 items are either closed, split, or explicitly accepted as alpha residual risk.
5. The next roadmap decision is documented: continue to P2 signing/attestation, repeat P0/P1 hardening, or pivot.

## Phase 26: External Alpha Pilots and Live Action Proof

**Status:** External local pilots completed 2026-05-21 as NO_GO_PUBLIC_ALPHA.
Three public repositories were measured locally; live GitHub Action proof is
split to Phase 26.1 and remains the public Alpha gate.

**Goal:** Run Pramaan on three external real repositories and prove the GitHub Action on a live PR before public Alpha.

**Priority:** Alpha gate

**Success Criteria:**

1. Pilot report covers one Python, one TypeScript, and one Rust repository where possible.
2. Runtime, skipped-stage profile, false-positive/false-negative notes, reviewer time-to-understand, and residual risks are recorded.
3. At least one live GitHub Action run uploads a bundle and renders a useful PR summary.
4. Public Alpha decision is updated from evidence, not internal fixture confidence.

## Phase 26.1: Live GitHub Action Proof

**Goal:** Run Pramaan's composite GitHub Action on a real PR or PR-like branch
and capture the uploaded proof bundle plus rendered job summary.

**Priority:** Alpha gate

**Status:** Completed 2026-05-21 via live `workflow_dispatch` run
`26229890652`. The run built Pramaan, executed verification, emitted local VSA
and in-toto attestations, rendered `github-step-summary.md`, and uploaded the
`pramaan-proof-bundle` artifact. The result was `inconclusive` with policy
`warning`, which is honest residual-risk evidence rather than a production v1
claim.

**Success Criteria:**

1. A live GitHub Actions run URL is recorded.
2. The proof bundle artifact is downloadable from the run.
3. The job summary shows failed/actionable stages and residual risk families.
4. `TASKS.md`, `.planning/STATE.md`, and `.planning/ROADMAP.md` distinguish
   live Action evidence from local action-summary tests.

## Phase 26.2: Competitive Benchmark and Prior-Art Matrix

**Goal:** Make Pramaan's public positioning evidence-grounded by comparing it
against adjacent GitHub tools and primitives before claiming superiority.

**Priority:** Positioning

**Status:** Completed 2026-05-21. `docs/competitive-benchmark.md` now maps
Pramaan against AI PR reviewers, reviewdog-style aggregators, test-change
monitors, test-generation tools, mutation/property engines, and
SLSA/Sigstore/in-toto/GitHub attestation primitives. It narrows the public
claim to evidence-bundle verification and leaves "catches what X misses" proof
to Phase 26.3 executable fixtures.

**Success Criteria:**

1. `docs/competitive-benchmark.md` compares Pramaan against PR-Agent,
   OpenReview, inspect, Testomatio, quality-monitor, actions/attest,
   SLSA verifier, and in-toto primitives.
2. The benchmark separates competitors from reusable primitives.
3. Each comparison records overlap, Pramaan's differentiated evidence layer,
   and what Pramaan should not duplicate.
4. README or marketing claims that imply "more comprehensive" are tied to the
   benchmark or softened.

## Phase 26.3: Competitor-Gap Fixtures

**Goal:** Convert "Pramaan catches what other tools miss" into executable
fixtures rather than marketing copy.

**Priority:** Demo credibility

**Status:** Completed 2026-05-21 for the v0.1 category-level fixture corpus.
`corpus/competitor-gap-fixtures.v0.1.json`, `examples/competitor-gaps/`, and
`scripts/check-competitor-gap-fixtures.mjs` now validate seven gap scenarios.
Some entries are metadata fixtures rather than full executable demos; named-tool
benchmarking remains a future empirical study.

**Success Criteria:**

1. Fixtures cover weakened assertions, added skips, fixture/snapshot drift,
   hallucinated API usage, false-green CI, unsigned quality reports, and hidden
   skipped verification stages.
2. Each fixture maps to stable risk IDs, expected receipts, and expected policy
   decision.
3. A gap report explains which adjacent tool category would miss or under-report
   the scenario.
4. Stale, duplicate, or unmapped gap fixtures fail validation.

## Phase 26.4: Minimum Lovable Verifier Loop

**Goal:** Prove the first product wedge feels complete: one command, one
report, one proof bundle, one killer demo, and a 30-second reviewer answer.

**Priority:** Real MVP wedge

**Status:** Completed 2026-05-21 for the local oracle-focused quickstart loop.
`scripts/run-minimum-lovable-loop.ps1` runs the weakened-test demo, emits a
verifiable bundle manifest, adds confidence/policy evidence, and writes
`minimum-lovable-report.md`. Phase 35.5 still owns Rust-native HTML/Markdown
report commands.

**Success Criteria:**

1. A fresh reviewer can run one command and get a bundle plus report.
2. The canonical weakened-test demo is the default quickstart path.
3. The report explains blockers, skipped stages, confidence explanation, and
   replay/inspection commands without requiring source-code archaeology.
4. Any skipped required stage that looks like a pass is treated as a blocker.

## Phase 26.5: Agent Harness Interface for Coding Agents

**Status:** Completed 2026-05-21. The deterministic agent done gate, explain
command, schema, docs, command template, and pass/warn/block tests are in place.

**Goal:** Make Pramaan callable by Claude Code, Codex, Cursor-style agents, and
custom harnesses as a completion gate before an agent claims done.

**Priority:** Agent adoption

**Success Criteria:**

1. `pramaan agent done-gate` emits pass, warn, or block JSON with required
   actions.
2. `agent_decision.schema.json` defines the machine-readable contract.
3. `AGENTS.md` and Claude Code command/hook templates instruct agents not to
   claim completion while the gate blocks.
4. A blocked oracle fixture returns actionable repair instructions.

## Phase 27: Parser-Backed Oracle Extractors

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS. Parser-backed subset
extractors, negative fixtures, and docs landed; full compiler AST integrations
are split to Phase 27.1.

**Goal:** Close the current structured-extractor risk with parser-backed Python, TypeScript, and Rust oracle evidence.

**Priority:** P1 hardening

**Success Criteria:**

1. Parser-backed extractors handle assertions, skips, ignores, xfails, snapshots, fixtures, and renamed/deleted tests.
2. Golden negative fixtures cover comments, strings, macros, multiline assertions, generated tests, and renamed test bodies.
3. Receipts preserve stable risk IDs and make unsupported syntax explicit.

## Phase 27.1: Full Compiler AST Oracle Extractors

**Goal:** Add full compiler/parser AST integrations with pinned dependency or
subprocess choices, parser-version evidence, and disagreement reporting.

**Priority:** Parser hardening split

**Success Criteria:**

1. Python, TypeScript, and Rust parser choices have dependency/runtime
   justifications.
2. Golden fixtures cover comments, strings, generated tests, multiline
   assertions, macros, renamed bodies, decorators/attributes, and skipped tests.
3. Oracle evidence records parser version, fallback reason, unsupported syntax,
   and disagreement counts.
4. Claim audit stops marking full compiler AST extraction as planned only after
   executable fixtures pass.

## Phase 28: Tool-Backed Property, Fuzz, and Replay

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS. Recorded-case replay CLI
landed; safe Hypothesis/fast-check generated harness execution is split to
Phase 28.1.

**Goal:** Execute real Hypothesis and fast-check campaigns safely where eligible code exists, with replayable evidence.

**Priority:** P1 hardening

**Success Criteria:**

1. Safe generated harnesses run within strict budgets.
2. Seeds, generated counts, corpus hashes, counterexamples, shrink data, and tool versions are recorded.
3. `pramaan replay` can reproduce recorded failing generated cases.
4. Missing tools remain skipped evidence, not mitigated evidence.

## Phase 28.1: Safe Hypothesis and fast-check Harness Execution

**Goal:** Execute real Hypothesis and fast-check campaigns from generated
harnesses where eligible pure functions and project dependencies make that safe.

**Priority:** Property/fuzz hardening split

**Success Criteria:**

1. Generated harnesses are limited to safe pure-function candidates.
2. Python Hypothesis records bounded examples, deadlines, seeds, tool versions,
   example databases, and counterexamples.
3. TypeScript fast-check records bounded runs, seeds, timeouts, tool versions,
   shrink data, and counterexamples.
4. Missing tools or unsafe candidates remain visible residual/skipped evidence.

## Phase 28.25: AI Evidence-Seeking Probe Generator

**Status:** Completed 2026-05-21 as PASS_WITH_RISKS. Provider-neutral probe
planning landed; sandbox execution of generated probes is split to Phase 28.26.

**Goal:** Use AI to generate candidate tests, properties, differential inputs,
and security probes while counting only sandbox-executed evidence.

**Priority:** 10x evidence depth

**Success Criteria:**

1. `pramaan probe plan` creates a probe plan from claim, diff, risk IDs, and
   residual/skipped evidence.
2. Probe schema records prompt hash, risk IDs, candidate code, sandbox result,
   kept/rejected status, and rejection reason.
3. Generated probes compile/run in isolation before contributing mitigation.
4. Weak, non-compiling, or irrelevant probes are preserved as rejected evidence.

## Phase 28.26: Sandbox Execution for Generated Probes

**Goal:** Execute provider- or agent-generated probe candidates in isolated temp
test locations and preserve accepted/rejected execution evidence.

**Priority:** AI probe hardening split

**Success Criteria:**

1. Probe candidates compile/run in a temporary isolated test location.
2. Non-compiling, non-running, or irrelevant probes are preserved with rejection
   reasons.
3. Accepted probes record sandbox output, changed-behavior binding evidence,
   and mutation/differential validation where practical.
4. AI/provider output still never mitigates risk without execution evidence.

## Phase 28.5: Auditable Confidence Vote and Calibration Schema

**Goal:** Add a deterministic, decomposable confidence-vote artifact that
aggregates receipts without pretending to prove correctness.

**Priority:** P1/P2 trust insert

**Status:** Completed 2026-05-21 for the v0.1 uncalibrated confidence bridge:
core artifact builder, CLI `pramaan confidence explain`,
`schemas/confidence.schema.json`, reviewer Markdown output, confidence receipt,
manifest-linked artifacts, schema/fixture checks, critical-path hard gates,
invalid-attestation metadata hard gates, and docs. Phase 34 owns real
calibration.

**Success Criteria:**

1. `schemas/confidence.schema.json` defines algorithm version, hard gates,
   weak-signal votes, stage reliability inputs, dependency clusters,
   statistical intervals, top drivers, calibration metadata, and residual risk.
2. Pramaan emits `confidence.json` and `confidence.md` from bundle receipts.
3. Hard gates such as weakened tests, bundle tamper, invalid attestation, and
   untrusted plugins cannot be averaged away by positive evidence elsewhere.
4. Mutation confidence uses Wilson lower bounds; zero-failure fuzz/property
   evidence records a rule-of-three residual-risk upper bound.
5. Initial weights are deterministic and documented; Phase 34 owns later
   calibration using pilot outcomes, Brier score, log loss, and reliability
   diagrams / expected calibration error.

## Phase 29: Attestation, VSA, and Offline Verification

**Goal:** Make bundles verifiable outside the original CI job with signing, artifact attestations, and VSA-style summaries.

**Priority:** P2 trust

**Status:** Completed 2026-05-21 for local/offline VSA and in-toto
attestation consistency checks. Production Sigstore/cosign identity and live
GitHub attestation evidence remain later hardening, not current guarantees.

**Success Criteria:**

1. Bundle manifests can emit local/offline in-toto and SLSA VSA-style
   attestation material.
2. GitHub artifact attestation integration is documented and wired where
   permissions allow.
3. Pramaan emits an in-toto/SLSA-compatible VSA-style verification summary
   that can reference the confidence artifact.
4. Offline verification rejects tampered bundle, manifest, confidence, and
   attestation fixtures.
5. Production Sigstore/cosign signer identity remains explicitly planned until
   certificate and transparency-log verification are implemented.

## Phase 30: Redaction Profiles and Public Bundle Export

**Goal:** Make shared bundles safe for reviewers, customers, and public demos.

**Priority:** P2 trust

**Status:** Completed 2026-05-21 for profile validation, pattern-based text/JSON
redaction, `pramaan bundle export-redacted`, manifest rebuilds, and public-demo
scrub tests. `summary-only` is accepted as a profile but full artifact
minimization remains later hardening.

**Success Criteria:**

1. `internal-full`, `reviewer-redacted`, `public-demo`, and `summary-only`
   profiles are implemented or explicitly scoped.
2. Secret, private path, internal hostname, endpoint, and CI metadata fixtures
   are scrubbed.
3. Redacted bundles remain inspectable and verifiable under allowed
   transformations.
4. Stale offline attestations are removed during export because redaction
   changes manifest hashes.

## Phase 31: Plugin Protocol Trust and Isolation

**Goal:** Prevent plugins from becoming an accidental trusted root for receipt evidence.

**Priority:** P2 security

**Status:** Completed 2026-05-21 for the v0.1 subprocess JSON protocol shape,
plugin trust validator, bundle-time rejection of high/critical plugin findings,
and a malicious-plugin fixture. Stronger runtime isolation remains future
hardening.

**Success Criteria:**

1. Plugin protocol defines identity, version, provenance, allowed receipt capabilities, and optional signatures.
2. Plugins cannot edit prior receipts or bundle manifests directly.
3. Malicious-plugin fixtures cover false pass emission, artifact path escape, receipt tampering, and environment leakage.
4. Isolation boundary and residual risks are documented.

## Phase 32: SARIF, Policy, and Agentic Workflow Security

**Goal:** Export Pramaan evidence into existing review systems and harden untrusted agent workflow inputs.

**Priority:** P2 security

**Status:** Completed 2026-05-21 for SARIF export, starter Rego export,
claim-scope workflow-injection detection, GitHub code-scanning guidance, and
the minimal SARIF export schema contract. CodeQL/security scanner ingestion
remains warning-only future correlation.

**Success Criteria:**

1. Pramaan findings can export as SARIF and import into GitHub code scanning.
2. OPA/Rego policy export or parity tests match default Rust policy decisions.
3. Agentic workflow-injection fixtures map untrusted PR/issue/comment text to stable risk IDs.
4. Optional CodeQL/security scanner evidence is correlation, not sole gate.

## Phase 32.5: Policy Pack Library and Enterprise Profiles

**Goal:** Provide selectable policy packs for different adoption contexts
without changing Pramaan code.

**Priority:** Enterprise adoption

**Status:** Completed 2026-05-21 for built-in profiles, checked-in policy
fixtures, `policy list`, `policy explain --profile`, schema coverage, and
GitHub Action `policy-profile` input. External policy-file loading remains
future hardening.

**Success Criteria:**

1. Built-in policy profiles exist for `startup-fast`, `open-source-maintainer`,
   `security-sensitive`, `fintech-strict`, and `private-preview`.
2. `pramaan policy list` and `policy explain --profile <id>` work.
3. GitHub Action accepts a `policy-profile` input.
4. Policy fixtures cover pass, warn, fail, waiver, and security-sensitive path
   escalation.

## Phase 32.75: Anti-Gaming and Verifier-Abuse Hardening

**Goal:** Make Pramaan resistant to PRs or plugins that try to game the
verification surface itself.

**Priority:** Trust hardening

**Success Criteria:**

1. Malicious PR fixtures cover relaxed config, removed hooks, skipped tests,
   altered fixtures, poisoned snapshots, and changed verification scripts.
2. Verifier-abuse fixtures cover artifact path escape, receipt tampering,
   hidden skipped stages, fake tool output, timeout laundering, and benchmark
   overfitting.
3. Policy rules escalate verifier-surface changes according to the selected
   policy profile.
4. Skipped required stages cannot improve confidence, and plugin/PR code cannot
   overwrite existing receipts or manifests.

## Phase 33: Adversarial Corpus 25 and Secure-Code Scenarios

**Goal:** Grow the adversarial corpus to 25 high-signal scenarios before larger eval claims.

**Priority:** P2 evals

**Status:** Completed 2026-05-21. `corpus/adversarial-scenarios-v0.1.json`,
`schemas/adversarial_corpus.schema.json`, and
`scripts/check-adversarial-corpus.mjs` now validate 25 scenarios. Most entries
are scenario specs, not executable demos; Phase 40 still owns the 100-scenario
corpus and broader runnable fixture set.

**Success Criteria:**

1. Corpus has at least 25 scenarios mapped to stable risk IDs.
2. Secure-code scenarios cover validation removal, authorization weakening, unsafe deserialization, injection sanitization removal, crypto misuse, and secret exposure.
3. Malicious verifier or CI-abuse scenarios are included.
4. Corpus runner or report catches duplicate or stale scenarios.

## Phase 34: Calibration, Drift, and Reviewer Feedback Loop

**Goal:** Reduce alert fatigue by comparing findings to repo baselines, capturing human override outcomes, and calibrating the confidence-vote model against real outcomes.

**Priority:** P2 feedback

**Status:** Completed 2026-05-21. `pramaan feedback override` now writes
first-class override evidence into bundles, and `pramaan feedback analyze`
compares one or more bundles against local repo baselines, calibration labels,
and dashboard-ready JSON/CSV exports. This is local-file calibration and drift
evidence, not a hosted analytics backend; long-lived outcome correlation with
reverts or defects remains future work.

**Success Criteria:**

1. Reviewer overrides persist with accepted risk IDs, reason, reviewer identity source, and merge outcome when available.
2. Repo baselines track mutation survival, oracle warnings, skipped stages, runtime, and static findings.
3. Confidence predictions are evaluated with Brier score, log loss, and reliability diagrams / expected calibration error when labels exist.
4. Trend exports show drift by repo, risk family, agent author, confidence bucket, and time window.
5. Dashboard data exists without making a dashboard required for CLI adoption.

## Phase 35: Operator Docs, Release Packaging, and Adoption

**Goal:** Make Pramaan installable, runnable, and explainable by an external maintainer.

**Priority:** P2 adoption

**Status:** Completed 2026-05-21. Operator, plugin-author, security-model,
enterprise-deployment, troubleshooting, release-packaging, and rendered-example
docs now exist. Release publication and Marketplace listing remain intentionally
unclaimed until Phase 26.1 live Action proof and a tagged release happen.

**Success Criteria:**

1. Operator, plugin-author, security model, enterprise deployment, and troubleshooting docs exist.
2. PR summary screenshots or rendered examples cover pass, warning, and fail cases.
3. Release packaging workflow or manual release checklist is staged.
4. Marketplace status is documented honestly.

## Phase 35.5: Reviewer UX and Local HTML Report

**Goal:** Make proof bundles inspectable in under 30 seconds without requiring
a hosted dashboard.

**Priority:** Adoption UX

**Success Criteria:**

1. `pramaan report html --bundle <path>` emits a local static report.
2. `pramaan report markdown --bundle <path>` emits PR-comment-ready markdown.
3. Reports group blockers, warnings, ran/skipped stages, oracle changes,
   replay commands, and override fields.
4. Weakened-test demo report is manually understandable in under 30 seconds.

## Phase 36: Language Plugin Depth for Python, TypeScript, and Rust

**Goal:** Deepen the first three supported language paths before expanding language count.

**Priority:** P2 language depth

**Success Criteria:**

1. Python, TypeScript, and Rust support matrices are accurate and claim-audited.
2. Each language has static, oracle, mutation, property/fuzz, and fixture coverage.
3. Changed-function detection and diff scoping are language-aware.
4. Go and Java remain blocked until plugin protocol and first-language depth are credible.

## Phase 37: Provider-Neutral Forge and GitLab Support

**Goal:** Design non-GitHub support before GitHub assumptions harden into the bundle model.

**Priority:** P3 enterprise

**Success Criteria:**

1. Provider-neutral PR, commit, identity, artifact, and attestation interfaces are defined.
2. GitLab OIDC and attestation differences are documented with fixtures.
3. GitHub behavior remains stable.
4. Gitea and Bitbucket are documented later targets unless a pilot forces earlier support.

## Phase 38: Multi-Agent Provenance and Handoff Tracking

**Goal:** Track modern workflows where multiple agents and humans contribute to one PR.

**Priority:** P3 provenance

**Success Criteria:**

1. Provenance model supports author agent, reviewer agent, test-writing agent, unknown agent, and final human reviewer.
2. Intermediate commit attribution and handoff metadata are recorded when available.
3. Unknown provenance is explicit and policy-visible.

## Phase 39: Adapter Certification as a Bounded Adjacent Track

**Goal:** Preserve adapter-certification ideas without distracting from PR verification.

**Priority:** P3 adjacent

**Success Criteria:**

1. Adapter certification remains optional and separate from public Alpha gates.
2. Checks cover tool names, descriptions, schemas, auth scopes, idempotency, retry behavior, rate limits, and auditability.
3. Adapter proof-bundle examples exist without implying an adapter registry exists.

## Phase 40: Serious v1 Release Gate and Corpus 100

**Goal:** Decide Serious v1 only after real pilots, verifiable bundles, redaction, plugin trust, calibration, and 100 adversarial scenarios.

**Priority:** Serious v1 gate

**Success Criteria:**

1. Corpus has 100+ non-duplicate scenarios mapped to risk IDs.
2. Benchmark-integrity checks detect stale fixtures and overfit eval assumptions.
3. Cross-platform CI is green or residual failures are explicitly accepted.
4. Final claim audit has zero false-or-stale public claims.
5. Serious v1 report names go, no-go, or conditional release.

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

**Supplemental P0/P1 completion track:** Phases 18-25 are a delivery overlay for
the remaining P0/P1 tasks in `TASKS.md`; they do not change the original 111
research-mapped requirement count.

**Research-driven continuation track:** Phases 26-40 plus decimal inserts
26.5, 28.25, 28.5, 32.5, and 35.5 map the remaining
unchecked P1/P2/P3 and Serious v1 task families to executable GSD plans after
the 2026-05-21 internet research refresh. They are delivery phases, not a new
requirement-counting scheme.

---
*Roadmap updated: 2026-05-21 after research-driven GSD continuation planning*
