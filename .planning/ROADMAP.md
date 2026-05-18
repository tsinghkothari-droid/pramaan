# Roadmap: Pramaan

**Created:** 2026-05-18
**Granularity:** Coarse
**v1 Requirements:** 48 mapped

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

## Coverage

| Phase | Requirements | Count |
|-------|--------------|-------|
| Phase 1 | CLI-01, CLI-02, CLI-03, RCPT-01, RCPT-02, RCPT-03, RCPT-05, RISK-01, RISK-02, SCOP-01, SCOP-02 | 11 |
| Phase 2 | SNDB-01, SNDB-02, SNDB-03, STAT-01, STAT-02, STAT-03, STAT-04, STAT-05 | 8 |
| Phase 3 | SCOP-03, ORCL-01, ORCL-02, ORCL-03, ORCL-04, ORCL-05 | 6 |
| Phase 4 | MUTN-01, MUTN-02, MUTN-03, MUTN-04, MUTN-05, FUZZ-01, FUZZ-02, FUZZ-03, FUZZ-04 | 9 |
| Phase 5 | RCPT-04, RISK-03, BNDL-01, BNDL-02, BNDL-03, BNDL-04 | 6 |
| Phase 6 | GHAC-01, GHAC-02, GHAC-03, GHAC-04, RISK-04, DEMO-01, DEMO-02, DEMO-03 | 8 |

**Total mapped:** 48 / 48

---
*Roadmap updated: 2026-05-18 after top-100 flaw research*
