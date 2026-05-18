# Requirements: Pramaan

**Defined:** 2026-05-18
**Core Value:** Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

## v1 Requirements

### CLI

- [ ] **CLI-01**: Developer can run `pramaan verify --base <ref> --head <ref>` against a Git repository.
- [ ] **CLI-02**: Developer can choose an output directory for receipts and the final bundle.
- [ ] **CLI-03**: Developer can see a concise terminal summary with passed, failed, skipped, and not-applicable stages.

### Receipts

- [ ] **RCPT-01**: Every stage emits a JSON receipt whether it passes, fails, skips, times out, or is not applicable.
- [ ] **RCPT-02**: Receipt includes stage name, tool name/version, input hashes, start/end time, exit code/status, summary, and artifact paths.
- [ ] **RCPT-03**: Receipt and bundle schemas are versioned and committed under `schemas/`.
- [ ] **RCPT-04**: Bundle manifest references every receipt and artifact by content hash.

### Claim Scope

- [ ] **SCOP-01**: Developer can provide or auto-derive a PR claim scope from PR title, PR body, linked issue text, changed public APIs, and optional notes.
- [ ] **SCOP-02**: Claim scope receipt records expected behavior, explicitly out-of-scope behavior, touched public APIs, and confidence level.
- [ ] **SCOP-03**: Pramaan flags oracle/scope mismatch risks, including narrow oracle, wide oracle, changed oracle, and missing regression risks.

### Sandbox

- [ ] **SNDB-01**: Sandbox creates isolated base and head worktrees for a PR verification run.
- [ ] **SNDB-02**: Sandbox records commit SHAs, dependency lockfile hashes, relevant config hashes, and container/image digest when available.
- [ ] **SNDB-03**: Sandbox records enough environment evidence to explain why a run is or is not reproducible.

### Static

- [ ] **STAT-01**: Python plugin can run configured compile/type/lint checks and convert results to receipts.
- [ ] **STAT-02**: TypeScript plugin can run configured type/lint checks and convert results to receipts.
- [ ] **STAT-03**: Rust plugin can run configured cargo check/test-build checks and convert results to receipts.
- [ ] **STAT-04**: Static stage flags broken imports, undeclared symbols, and missing referenced APIs where language tooling exposes them.
- [ ] **STAT-05**: Static stage classifies likely code hallucinations when evidence allows, such as invented API, invalid parameter, undefined symbol, nonexistent import, resource mismatch, or logic mismatch.

### Oracle Integrity

- [ ] **ORCL-01**: Oracle stage detects deleted or newly skipped tests in changed test files.
- [ ] **ORCL-02**: Oracle stage detects weakened assertions in changed Python tests.
- [ ] **ORCL-03**: Oracle stage detects weakened assertions in changed TypeScript/JavaScript tests.
- [ ] **ORCL-04**: Oracle stage classifies snapshot and fixture changes as oracle-sensitive artifacts.
- [ ] **ORCL-05**: Oracle stage can fail the demo PR where CI passes only because an assertion was weakened.

### Mutation

- [ ] **MUTN-01**: Python mutation stage runs diff-scoped mutation testing on changed source files.
- [ ] **MUTN-02**: TypeScript mutation stage runs diff-scoped mutation testing on changed source files.
- [ ] **MUTN-03**: Rust mutation stage runs diff-scoped mutation testing on changed source files.
- [ ] **MUTN-04**: Mutation receipt reports mutants created, killed, survived, timed out, and kill-rate threshold result.
- [ ] **MUTN-05**: Mutation receipt records timeout policy, coverage/filter mode, incremental cache/reuse status, and skipped/unviable mutant rationale where available.

### Differential Fuzz

- [ ] **FUZZ-01**: Python plugin can run Hypothesis-based differential checks for eligible changed pure functions.
- [ ] **FUZZ-02**: TypeScript plugin can run fast-check-based differential checks for eligible changed pure functions.
- [ ] **FUZZ-03**: Differential receipt records seeds, generated input count, corpus hashes, and observed divergences.
- [ ] **FUZZ-04**: Differential stage can mark cases as not applicable when safe function discovery is not possible.

### Bundle

- [ ] **BNDL-01**: Bundle stage emits a manifest containing tool versions, stage receipts, artifact hashes, seeds, corpus hashes, and final status.
- [ ] **BNDL-02**: Bundle stage supports local dev signing or signable output in v1.
- [ ] **BNDL-03**: User can verify a bundle manifest against included receipts and artifacts.
- [ ] **BNDL-04**: Bundle manifest can carry GitHub artifact attestation metadata when generated in GitHub Actions.

### GitHub Action

- [ ] **GHAC-01**: GitHub Action runs Pramaan on pull requests.
- [ ] **GHAC-02**: GitHub Action uploads the proof bundle as a CI artifact.
- [ ] **GHAC-03**: GitHub Action publishes a concise PR summary focused on failed or risky stages.
- [ ] **GHAC-04**: GitHub Action can optionally request artifact attestation for the uploaded proof bundle.

### Demo

- [ ] **DEMO-01**: Repository includes a vulnerable Python or TypeScript demo PR where a test is weakened to make CI pass.
- [ ] **DEMO-02**: Demo instructions show ordinary CI passing while Pramaan fails oracle integrity.
- [ ] **DEMO-03**: Demo bundle includes a clear receipt naming the weakened assertion or skipped oracle.

## v2 Requirements

### Advanced Stages

- **ADV-01**: Adversarial test amplification via Pynguin/EvoSuite/CodaMosa-style tools.
- **ADV-02**: Optional formal verification receipts through Kani, CBMC, or Dafny where specs exist.
- **ADV-03**: Dual critic output with position-swap mitigation, marked as non-gating signal.
- **ADV-04**: Go and Java language plugins.
- **ADV-05**: Dashboard for 30-second human audit of bundles.
- **ADV-06**: Sigstore keyless OIDC signing in hosted CI.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Correctness proof | Ethically and technically false for the target problem. |
| Critic-only approval | LLM judges are biased and must not override execution evidence. |
| Required formal verification | Applies to too few real PRs and creates path-explosion risk. |
| Whole-repo mutation as default | Too slow for PR feedback; v1 should be diff-scoped. |
| Production dashboard before CLI | Trust comes from receipts and bundles first. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLI-01 | Phase 1 | Pending |
| CLI-02 | Phase 1 | Pending |
| CLI-03 | Phase 1 | Pending |
| RCPT-01 | Phase 1 | Pending |
| RCPT-02 | Phase 1 | Pending |
| RCPT-03 | Phase 1 | Pending |
| RCPT-04 | Phase 5 | Pending |
| SCOP-01 | Phase 1 | Pending |
| SCOP-02 | Phase 1 | Pending |
| SCOP-03 | Phase 3 | Pending |
| SNDB-01 | Phase 2 | Pending |
| SNDB-02 | Phase 2 | Pending |
| SNDB-03 | Phase 2 | Pending |
| STAT-01 | Phase 2 | Pending |
| STAT-02 | Phase 2 | Pending |
| STAT-03 | Phase 2 | Pending |
| STAT-04 | Phase 2 | Pending |
| STAT-05 | Phase 2 | Pending |
| ORCL-01 | Phase 3 | Pending |
| ORCL-02 | Phase 3 | Pending |
| ORCL-03 | Phase 3 | Pending |
| ORCL-04 | Phase 3 | Pending |
| ORCL-05 | Phase 3 | Pending |
| MUTN-01 | Phase 4 | Pending |
| MUTN-02 | Phase 4 | Pending |
| MUTN-03 | Phase 4 | Pending |
| MUTN-04 | Phase 4 | Pending |
| MUTN-05 | Phase 4 | Pending |
| FUZZ-01 | Phase 4 | Pending |
| FUZZ-02 | Phase 4 | Pending |
| FUZZ-03 | Phase 4 | Pending |
| FUZZ-04 | Phase 4 | Pending |
| BNDL-01 | Phase 5 | Pending |
| BNDL-02 | Phase 5 | Pending |
| BNDL-03 | Phase 5 | Pending |
| BNDL-04 | Phase 5 | Pending |
| GHAC-01 | Phase 6 | Pending |
| GHAC-02 | Phase 6 | Pending |
| GHAC-03 | Phase 6 | Pending |
| GHAC-04 | Phase 6 | Pending |
| DEMO-01 | Phase 6 | Pending |
| DEMO-02 | Phase 6 | Pending |
| DEMO-03 | Phase 6 | Pending |

**Coverage:**
- v1 requirements: 43 total
- Mapped to phases: 43
- Unmapped: 0

---
*Requirements defined: 2026-05-18*
*Last updated: 2026-05-18 after improvement research*
