# Pramaan Tasks to Serious v1

This file tracks the concrete work needed to make Pramaan a serious production
verification layer for AI-authored pull requests.

## Strategic Re-Think: Are We on the Right Track?

Current answer: **yes, if Pramaan stays narrow and evidence-first.**

The strong product is not "AI proves code correct." The strong product is:

> Pramaan creates an auditable evidence bundle for AI-authored pull requests so
> reviewers can see what was checked, what was skipped, what was weakened, what
> risks remain, and whether the bundle itself can be trusted.

That direction is worth continuing because it targets a real adoption blocker:
teams do not merely need more code generation; they need reviewable evidence
when generated code claims to be safe. The risk is scope creep. Pramaan should
not become a generic agent platform, a full CI replacement, a new programming
language, or a universal security scanner before the core PR-verification loop
is trusted.

### Research Discipline

For a project like this, the appropriate amount of research is **four focused
research phases before build commitment**, then short research refreshes only
when a phase has a concrete unknown.

The four research phases are:

1. **Problem and buyer validation:** who has this pain, what they currently do,
   what would make them trust or reject Pramaan.
2. **Technical feasibility and prior art:** papers, tools, benchmarks, CI
   systems, mutation/fuzz/property tools, Sigstore/SLSA/in-toto, and competing
   products.
3. **Failure-mode and threat research:** adversarial PRs, weak oracles, poisoned
   fixtures, hallucinated APIs, malicious CI execution, plugin poisoning,
   redaction failures, and reviewer alert fatigue.
4. **Pilot/eval research:** 3-5 real repositories, 25-100 adversarial fixtures,
   runtime baselines, false-positive/false-negative measurements, and reviewer
   time-to-understand.

After those four research phases, new research must be tied to a deliverable:
a schema field, risk ID, fixture, policy rule, plugin contract, demo, or
benchmark. Broad "keep researching" work should pause until the alpha loop runs
on real repositories.

### P0 Product Direction Tasks

- [x] Declare the narrow product thesis in `README.md`, `STATUS.md`, and
  `.planning/STATE.md`: Pramaan is a PR evidence-bundle verifier, not a
  correctness oracle or generic CI replacement.
- [x] Add explicit non-goals: no automatic merge authority, no "proved correct"
  claims, no generic agent registry work inside the core v0.1 path, and no
  dashboard-first roadmap before CLI/Action trust is real.
- [x] Define the first ideal customer profile: teams reviewing AI-authored PRs
  in Python, TypeScript, or Rust repos with CI already in place.
- [x] Define the first killer workflow: "GitHub green, Pramaan red" for a
  weakened-test PR, with a bundle understood in under 30 seconds.
- [x] Create a research sufficiency checklist: at least 40 source-backed notes,
  30 mapped failure modes, 10 competing/prior-art tools, 25 adversarial
  fixtures, 3 pilot repositories, and measured runtime/noise baselines.
- [x] Stop broad research once the sufficiency checklist is met; convert every
  remaining research question into an experiment, fixture, policy rule, or doc.
- [x] Add pivot/kill criteria: if reviewers cannot understand a bundle in under
  30 seconds, if runtime regularly exceeds the SLA, or if skipped stages look
  like passes, pause feature expansion and fix trust/UX first.
- [x] Review the phase plan every 4 build phases and remove or merge phases that
  do not directly improve evidence quality, reviewer trust, runtime, or adoption.

### P0/P1 GSD Completion Track

The remaining P0/P1 work is now mapped to a focused GSD track. These items are
**planned, not complete**. A checkbox below should only be marked done after the
phase lands with tests/docs and verification evidence.

| GSD phase | Priority | What it completes |
| --- | --- | --- |
| Phase 18 | P0 | Product thesis, non-goals, `STATUS.md`, current-vs-planned README honesty, ICP, killer workflow, research sufficiency, pivot criteria. |
| Phase 19 | P0 | Receipt golden tests, canonical serialization/hashing plan, fixture drift controls, schema/runtime consistency guardrails. |
| Phase 20 | P0 | Performance SLA, stage budgets, default policy profile, `pramaan policy explain`, hard-gate vs warning-gate behavior. |
| Phase 21 | P1 | Sandbox hardening, OCI/container identity, dirty-after-run detection, verifier threat model, redaction, CI hardening checks. |
| Phase 22 | P1 | Claim-scope issue ingestion, scope-note support, vague-claim warnings, semantic mismatch signal, relaxed static-config and security-sensitive diff classification. |
| Phase 23 | P1 | AST-backed oracle integrity extractors for Python, TypeScript, and Rust with golden fixtures. |
| Phase 24 | P1 | Real mutation and property/fuzz adapters for Python, TypeScript, and Rust with budgets, thresholds, replay metadata, and honest skipped/timeout receipts. |
| Phase 25 | P0/P1 gate | Pilot validation across 3 repositories, P0/P1 acceptance report, unresolved-risk register, and go/no-go decision for Alpha MVP. |

Execution rule: do not start P2 signing/attestation expansion or dashboard work
until the Phase 25 P0/P1 gate says the core PR-verification loop is trustworthy
enough for external users.

## P0: Killer Demo

- [x] Build a standalone demo repository where normal CI passes but Pramaan fails because a test assertion was weakened.
- [x] Add a second demo where a snapshot or fixture change silently approves wrong behavior.
- [x] Add a third demo where a fake import/API passes superficial review but fails static/hallucination checks.
- [x] Create a short reviewer walkthrough that shows the proof bundle can be understood in under 30 seconds.
- [x] Add generated example bundles for each demo scenario.

## P0: Receipt and Bundle Trust

- [x] Freeze receipt schema version `0.1`.
- [x] Add agent-author attribution before schema freeze: agent product, model family/version when available, execution mode, prompt/context hash, and commit provenance.
- [x] Add reviewer override capture before schema freeze: override decision, reason, reviewer identity source, timestamp, risk IDs accepted, and linked merge outcome.
- [x] Add schema compatibility tests for all checked-in fixture receipts.
- [x] Add golden tests that diff generated receipts against approved fixtures.
- [x] Add artifact graph support so every receipt can point to hashed logs, corpora, and tool outputs.
- [x] Add bundle-level verification summary with mitigated, residual, skipped, and not-applicable risk families.
- [x] Add tamper tests for missing artifacts, modified receipts, modified manifests, and changed signing metadata.

## P0: GitHub Action Readiness

- [x] Make the action install or download the Pramaan CLI deterministically.
- [x] Add `base-ref`, `head-ref`, `out-dir`, `fail-on`, and `upload-bundle` inputs.
- [x] Define hard performance SLA targets for PR use: target runtime, max runtime, per-stage budget, and behavior when a budget is exhausted.
- [x] Add default policy-as-code profile for hard gates, warning gates, waiver rules, stage requirements, and security-sensitive paths.
- [x] Add `pramaan policy explain` so reviewers can see why a bundle failed, warned, or passed under a policy.
- [x] Upload the proof bundle as a GitHub Actions artifact.
- [x] Render a concise PR summary focused on failed stages and residual risks.
- [x] Add permissions documentation for pull requests from forks.
- [x] Add a minimal example workflow for Python, TypeScript, and Rust repositories.

## P1: Sandbox and Environment Evidence

- [x] Capture OS, architecture, shell, timezone, locale, and toolchain versions.
- [x] Record base/head commit IDs and dirty/untracked file state.
- [x] Hash dependency manifests and lockfiles.
- [x] Detect lockfile changes and mark dependency-drift risks.
- [x] Capture container image names and digests when supplied by CI/container environment metadata.
- [x] Auto-detect OCI/container identity when CI does not provide image metadata explicitly.
- [x] Add network policy evidence: disabled, allowed, observed, or unknown.
- [x] Detect source changes after a stage runs and mark dirty-after-run risk.
- [x] Threat-model the verifier as an attack target, including malicious PR code exploiting mutation engines, fuzzers, parsers, test runners, or plugin hooks.
- [x] Add PII/secrets scrubbing rules for environment evidence, logs, network endpoints, internal hostnames, paths, and artifact metadata before bundle emission.
- [x] Add CI hardening checks for untrusted PR execution: least-privilege token permissions, `pull_request_target` hazards, cache poisoning, unpinned actions, artifact retention, and self-hosted runner warnings.

## P1: Claim Scope

- [x] Parse PR title and body from GitHub Actions context.
- [ ] Ingest linked issue text when available.
- [x] Detect changed public APIs for Python, TypeScript, and Rust.
- [ ] Add low-confidence claim-scope warnings for vague or missing PR descriptions.
- [ ] Allow maintainers to provide a scope note file for expected and out-of-scope behavior.
- [ ] Map claim-scope warnings to stable risk IDs.
- [ ] Add semantic claim-implementation mismatch detection as a bounded signal: compare stated intent, touched APIs, tests, and changed behavior without making it a sole merge gate.

## P1: Static and Hallucination Checks

- [x] Python: integrate `compileall`, `ruff`, `mypy`, and `pyright` when configured.
- [x] TypeScript: integrate package-manager detection, `tsc --noEmit`, and ESLint when configured.
- [x] Rust: integrate `cargo check`, `cargo test --no-run`, and `cargo clippy` when configured.
- [x] Classify failures as `invented_api`, `invalid_parameter`, `undefined_symbol`, `nonexistent_import`, `resource_mismatch`, `logic_mismatch`, or `unknown`.
- [ ] Detect relaxed static-check configuration in the PR.
- [x] Emit skipped receipts when tools are unavailable instead of hiding missing checks.
- [ ] Add security-sensitive diff classification for auth, authorization, cryptography, SQL/query construction, subprocess, filesystem, deserialization, secrets, network, and permissions.

## P1: Oracle Integrity

- [x] Python: implement deterministic diff for `pytest` assertions, skips, xfails, raises, and parametrized cases.
- [x] TypeScript: implement deterministic diff for Jest, Vitest, and common `expect` patterns.
- [x] Rust: detect weakened `assert!`, `assert_eq!`, panic tests, `#[ignore]`, and snapshot/fixture changes.
- [x] Detect deleted tests and renamed tests through stable body fingerprints.
- [x] Classify fixture and snapshot diffs as oracle-sensitive.
- [x] Detect removed boundary cases, error cases, and parameter values.
- [x] Add reviewer-facing summaries that explain exactly which assertion or oracle artifact changed.
- [ ] Replace heuristic oracle scanning with AST-backed Python, TypeScript, and Rust extractors.

## P1: Mutation Adapters

- [ ] Python: run `mutmut` on changed files and directly affected tests.
- [ ] TypeScript: run StrykerJS in diff-scoped mode where possible.
- [ ] Rust: run `cargo-mutants` on changed crates/modules.
- [ ] Record mutants created, killed, survived, timed out, skipped, and unviable.
- [ ] Record mutation threshold, timeout policy, incremental-cache state, and filtering mode.
- [ ] Add equivalent-mutant and requires-review classifications where tool output supports it.
- [ ] Keep stage budgets strict enough for pull-request CI.

## P1: Property, Fuzz, and Differential Checks

- [ ] Python: auto-discover eligible pure functions and run Hypothesis differential checks.
- [ ] TypeScript: auto-discover eligible pure functions and run fast-check differential checks.
- [ ] Record seeds, replay data, minimized counterexamples, corpus hashes, and generated input counts.
- [ ] Compare base/head outputs on identical generated inputs.
- [ ] Classify divergences as in-scope, out-of-scope, suspicious, or unknown.
- [ ] Add replay commands for every failing generated case.

## P2: Attestation and Signing

- [ ] Add Sigstore keyless signing path for local and CI runs.
- [ ] Add GitHub artifact attestation integration.
- [ ] Map bundle manifest fields to in-toto/SLSA-compatible predicates.
- [ ] Add offline verification mode for downloaded bundles.
- [ ] Document public-repo and private-repo attestation differences.
- [ ] Add signing identity and certificate metadata to bundle summaries.
- [ ] Define plugin trust model: plugin identity, version, signature, sandbox boundary, receipt permissions, and tamper resistance.
- [ ] Add SLSA Verification Summary Attestation output mode for Pramaan's final verifier decision.
- [ ] Add redaction profiles: internal-full, reviewer-redacted, public-demo, and summary-only.

## P2: Adversarial Corpus and Evals

- [ ] Expand the adversarial corpus to 25 scenarios.
- [ ] Expand the adversarial corpus to 75 scenarios.
- [ ] Expand the adversarial corpus to 100+ scenarios mapped to risk IDs.
- [ ] Add real-world replay cases from open-source bug-fix PRs.
- [ ] Add flaky-case quarantine rules.
- [ ] Track false positives, false negatives, runtime, and reviewer time-to-understand.
- [ ] Create a benchmark report template.
- [ ] Add repo-level baseline calibration: expected mutation survival range, expected skipped-stage profile, runtime baseline, and noise-floor warnings.
- [ ] Add trend/drift metrics across PRs: agent failure rate, mutation survival drift, oracle-risk drift, skipped-stage drift, and runtime drift.
- [ ] Add benchmark-integrity mutation harness to detect agents overfitting eval tasks or hidden-test assumptions.
- [ ] Add secure-code corpus categories for removed validation, weakened authorization, unsafe deserialization, injection sanitization removal, crypto misuse, and secret exposure.

## P2: Documentation and Adoption

- [ ] Write an operator guide for running Pramaan in CI.
- [ ] Write a plugin-author guide.
- [ ] Write a security model.
- [ ] Write a threat model for malicious PR authors and compromised tools.
- [ ] Write an enterprise deployment guide.
- [ ] Add troubleshooting docs for slow mutation, missing tools, flaky tests, and forked PR permissions.
- [ ] Add screenshots or rendered examples of PR summaries and bundle inspection.
- [ ] Document non-GitHub roadmap and minimum abstraction layer for GitLab, Gitea, and Bitbucket support.
- [ ] Document GitLab artifact, identity, and OIDC differences before implementing GitLab support.

## P2: Feedback, Calibration, and Drift

- [ ] Persist reviewer override decisions as first-class evidence, not comments that disappear in PR history.
- [ ] Correlate override outcomes with later defects or revert signals when available.
- [ ] Store per-repo baselines for mutation survival, oracle warnings, skipped stages, runtime, and static/hallucination findings.
- [ ] Expose a trend API or export format for weekly/monthly agent-code quality drift.
- [ ] Add dashboard-ready metrics without making the dashboard a blocker for CLI adoption.
- [ ] Track agent-author attribution over time to compare failure modes by agent, model, workflow, and repository.

## P2: Verifier and Plugin Security

- [ ] Define a plugin protocol with least-privilege receipt-writing permissions.
- [ ] Require plugin identity, version, provenance, and optional signature in every plugin-emitted receipt.
- [ ] Prevent plugins from editing prior receipts or bundle manifests directly.
- [ ] Run risky parsers, test runners, mutation engines, and fuzzers behind stronger sandbox boundaries.
- [ ] Add malicious-plugin and malicious-PR fixtures to the adversarial corpus.
- [ ] Add bundle redaction policy tests for secrets, internal endpoints, private paths, and CI metadata.

## P3: Multi-Agent and Multi-Forge Support

- [ ] Model multi-agent provenance chains: code author agent, review agent, test-writing agent, and final human reviewer.
- [ ] Record intermediate commit attribution and handoff metadata where available.
- [ ] Add provider-neutral VCS interfaces before adding GitLab support.
- [ ] Add GitLab CI support after GitHub Action readiness stabilizes.
- [ ] Add Gitea and Bitbucket notes as later adoption targets, not MVP blockers.

## P2: Language Expansion

- [ ] Deepen Python plugin quality before adding more languages.
- [ ] Deepen TypeScript plugin quality before adding more languages.
- [ ] Deepen Rust plugin quality before adding more languages.
- [ ] Add Go support after plugin protocol stability.
- [ ] Add Java support after plugin protocol stability.
- [ ] Keep each language plugin accountable for static checks, oracle integrity, mutation, fuzz/property, and fixture coverage.

## P3: Adapter Certification

- [ ] Keep adapter certification as an adjacent mode, not a distraction from PR verification.
- [ ] Add certification checks for MCP tool names, descriptions, schemas, auth scopes, idempotency, retry behavior, rate limits, and auditability.
- [ ] Add adapter proof-bundle examples.
- [ ] Add failure-mode taxonomy for agent tool adapters.
- [ ] Integrate adapter certification only after the core PR-verification bundle is trusted.

## Release Gates

### Alpha MVP

- [ ] The weakened-test demo is undeniable.
- [ ] Pramaan runs successfully on at least three selected real repositories.
- [ ] GitHub Action posts a useful PR summary.
- [ ] Bundle verification catches tampering.
- [ ] Missing tools and skipped checks are visible.
- [ ] Receipt schema includes agent attribution and reviewer override fields before v0.1 freeze.
- [ ] PR runtime SLA is documented and enforced through stage budgets.
- [ ] Default policy profile can explain hard-fail vs warning-only decisions.

### Real MVP

- [ ] Python, TypeScript, and Rust paths have real tool integrations.
- [ ] Oracle integrity catches weakened assertions, skipped tests, and snapshot/fixture drift.
- [ ] Mutation and property/fuzz stages run within practical CI budgets.
- [ ] At least 75 adversarial scenarios exist.
- [ ] Documentation is good enough for an external maintainer to install and inspect a bundle.
- [ ] Repo-level calibration prevents obvious alert fatigue.
- [ ] Plugin trust model prevents untrusted plugins from poisoning receipts.
- [ ] CI hardening checks catch unsafe workflow patterns for untrusted PR code.
- [ ] Redaction profiles are tested before any bundle is safe to export.

### Serious v1

- [ ] Production-grade orchestrator with parallel scheduling and stage budgets.
- [ ] Hardened sandbox and environment evidence.
- [ ] Sigstore/GitHub attestation support.
- [ ] 100+ adversarial scenarios mapped to risk IDs.
- [ ] Cross-platform CI.
- [ ] Security model and threat model complete.
- [ ] Public demo proves "GitHub green, Pramaan red" in under 30 seconds.
- [ ] Reviewer overrides, agent attribution, baseline calibration, and drift reporting are part of the proof-bundle lifecycle.
- [ ] PII/secrets scrubbing is tested before enterprise bundle export.
- [ ] Pramaan can emit a VSA-style verification summary attestation.
