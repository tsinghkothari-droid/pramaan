# Pramaan Tasks to Serious v1

This file tracks the concrete work needed to make Pramaan a serious production
verification layer for AI-authored pull requests.

## P0: Killer Demo

- [ ] Build a standalone demo repository where normal CI passes but Pramaan fails because a test assertion was weakened.
- [ ] Add a second demo where a snapshot or fixture change silently approves wrong behavior.
- [ ] Add a third demo where a fake import/API passes superficial review but fails static/hallucination checks.
- [ ] Create a short reviewer walkthrough that shows the proof bundle can be understood in under 30 seconds.
- [ ] Add generated example bundles for each demo scenario.

## P0: Receipt and Bundle Trust

- [ ] Freeze receipt schema version `0.1`.
- [ ] Add schema compatibility tests for all checked-in fixture receipts.
- [ ] Add golden tests that diff generated receipts against approved fixtures.
- [ ] Add artifact graph support so every receipt can point to hashed logs, corpora, and tool outputs.
- [ ] Add bundle-level verification summary with mitigated, residual, skipped, and not-applicable risk families.
- [ ] Add tamper tests for missing artifacts, modified receipts, modified manifests, and changed signing metadata.

## P0: GitHub Action Readiness

- [ ] Make the action install or download the Pramaan CLI deterministically.
- [ ] Add `base-ref`, `head-ref`, `out-dir`, `fail-on`, and `upload-bundle` inputs.
- [ ] Upload the proof bundle as a GitHub Actions artifact.
- [ ] Render a concise PR summary focused on failed stages and residual risks.
- [ ] Add permissions documentation for pull requests from forks.
- [ ] Add a minimal example workflow for Python, TypeScript, and Rust repositories.

## P1: Sandbox and Environment Evidence

- [ ] Capture OS, architecture, shell, timezone, locale, and toolchain versions.
- [ ] Record base/head commit IDs and dirty/untracked file state.
- [ ] Hash dependency manifests and lockfiles.
- [ ] Detect lockfile changes and mark dependency-drift risks.
- [ ] Capture container image names and digests when running inside OCI containers.
- [ ] Add network policy evidence: disabled, allowed, observed, or unknown.
- [ ] Detect source changes after a stage runs and mark dirty-after-run risk.

## P1: Claim Scope

- [ ] Parse PR title and body from GitHub Actions context.
- [ ] Ingest linked issue text when available.
- [ ] Detect changed public APIs for Python, TypeScript, and Rust.
- [ ] Add low-confidence claim-scope warnings for vague or missing PR descriptions.
- [ ] Allow maintainers to provide a scope note file for expected and out-of-scope behavior.
- [ ] Map claim-scope warnings to stable risk IDs.

## P1: Static and Hallucination Checks

- [ ] Python: integrate `compileall`, `ruff`, `mypy`, and `pyright` when configured.
- [ ] TypeScript: integrate package-manager detection, `tsc --noEmit`, and ESLint when configured.
- [ ] Rust: integrate `cargo check`, `cargo test --no-run`, and `cargo clippy` when configured.
- [ ] Classify failures as `invented_api`, `invalid_parameter`, `undefined_symbol`, `nonexistent_import`, `resource_mismatch`, `logic_mismatch`, or `unknown`.
- [ ] Detect relaxed static-check configuration in the PR.
- [ ] Emit skipped receipts when tools are unavailable instead of hiding missing checks.

## P1: Oracle Integrity

- [ ] Python: implement AST diff for `pytest` assertions, skips, xfails, raises, and parametrized cases.
- [ ] TypeScript: implement AST diff for Jest, Vitest, and common `expect` patterns.
- [ ] Rust: detect weakened `assert!`, `assert_eq!`, panic tests, and snapshot changes.
- [ ] Detect deleted tests and renamed tests through stable body fingerprints.
- [ ] Classify fixture and snapshot diffs as oracle-sensitive.
- [ ] Detect removed boundary cases, error cases, and parameter values.
- [ ] Add reviewer-facing summaries that explain exactly which assertion changed.

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

## P2: Adversarial Corpus and Evals

- [ ] Expand the adversarial corpus to 25 scenarios.
- [ ] Expand the adversarial corpus to 75 scenarios.
- [ ] Expand the adversarial corpus to 100+ scenarios mapped to risk IDs.
- [ ] Add real-world replay cases from open-source bug-fix PRs.
- [ ] Add flaky-case quarantine rules.
- [ ] Track false positives, false negatives, runtime, and reviewer time-to-understand.
- [ ] Create a benchmark report template.

## P2: Documentation and Adoption

- [ ] Write an operator guide for running Pramaan in CI.
- [ ] Write a plugin-author guide.
- [ ] Write a security model.
- [ ] Write a threat model for malicious PR authors and compromised tools.
- [ ] Write an enterprise deployment guide.
- [ ] Add troubleshooting docs for slow mutation, missing tools, flaky tests, and forked PR permissions.
- [ ] Add screenshots or rendered examples of PR summaries and bundle inspection.

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

### Real MVP

- [ ] Python, TypeScript, and Rust paths have real tool integrations.
- [ ] Oracle integrity catches weakened assertions, skipped tests, and snapshot/fixture drift.
- [ ] Mutation and property/fuzz stages run within practical CI budgets.
- [ ] At least 75 adversarial scenarios exist.
- [ ] Documentation is good enough for an external maintainer to install and inspect a bundle.

### Serious v1

- [ ] Production-grade orchestrator with parallel scheduling and stage budgets.
- [ ] Hardened sandbox and environment evidence.
- [ ] Sigstore/GitHub attestation support.
- [ ] 100+ adversarial scenarios mapped to risk IDs.
- [ ] Cross-platform CI.
- [ ] Security model and threat model complete.
- [ ] Public demo proves "GitHub green, Pramaan red" in under 30 seconds.
