# Top 100 Flaws and Mitigations: Pramaan

**Date:** 2026-05-18
**Purpose:** Turn current software-testing, AI-code, fuzzing, mutation, and supply-chain research into a practical risk register that Pramaan can design against.

## Design Upgrade

Pramaan should become a **risk-intelligence proof bundle**, not only a sequence of checks. Each stage receipt should be able to reference risk IDs from this register:

```text
mitigated_risks: ["R-001", "R-022"]
residual_risks: ["R-041"]
not_applicable_risks: ["R-073"]
```

This lets a human reviewer audit in 30 seconds: what risks were reduced, what remains, and why.

## Top 100 Flaws

| ID | Flaw | Why It Breaks Trust | Mitigation To Encode |
|----|------|---------------------|----------------------|
| R-001 | No explicit PR claim | Checks cannot know intended behavior | Add claim-scope receipt before execution stages |
| R-002 | PR title/body underspecified | Multiple correct fixes may exist | Record claim confidence and request human scope note when low |
| R-003 | Linked issue missing | Oracle alignment cannot be judged | Receipt marks `missing_issue_context` residual risk |
| R-004 | Tests enforce implementation detail | Correct code rejected or agents overfit | Flag `narrow_oracle_risk` against claim scope |
| R-005 | Tests cover unmentioned behavior | Correct scoped fix fails unrelated expectations | Flag `wide_oracle_risk` against claim scope |
| R-006 | Original failing test absent | No proof the bug was reproduced | Require reproduction receipt or mark missing regression |
| R-007 | Test added after fix only | Test may simply mirror new behavior | Compare pre-patch failure and post-patch pass |
| R-008 | Fixture silently changes oracle | Behavior meaning changes outside assertions | Classify fixture/snapshot diffs as oracle-sensitive |
| R-009 | Snapshot update hides regression | Visual/text golden changed without review | Require snapshot receipt with before/after digest and scope reason |
| R-010 | Test skip added | CI goes green by removing coverage | AST and textual skip/xfail/todo detection |
| R-011 | Assertion weakened | Test still exists but no longer proves fix | Assertion AST diff with strength heuristic |
| R-012 | Assertion removed | Test body executes without checking result | Detect deleted assert/expect calls |
| R-013 | Test expectation inverted incorrectly | Wrong behavior becomes approved | Classify semantic assertion inversions as high risk |
| R-014 | Test only checks non-null/truthy | Weak oracle misses wrong values | Mutation receipt should flag surviving mutants in touched behavior |
| R-015 | Test relies on broad mock | Real integration path untested | Receipt records mocked dependencies and integration gap |
| R-016 | Test over-mocks changed function | The function under test is not exercised | Static call graph checks test-to-source reachability |
| R-017 | Test names changed to evade detection | Oracle diff misses renamed tests | Stable test fingerprint based on body/location |
| R-018 | Parametrized cases reduced | Edge coverage shrinks silently | Detect removed parameter values and case counts |
| R-019 | Boundary case removed | Off-by-one regressions survive | Boundary-case diff heuristic in oracle receipt |
| R-020 | Negative/error case removed | Exception behavior regresses | Detect removed `raises`, `rejects`, error-status assertions |
| R-021 | Hidden dependency on clock/timezone | Tests pass locally, fail in CI | Environment receipt records timezone/time mocks |
| R-022 | Hidden dependency on locale | String/date behavior differs | Environment receipt records locale and language settings |
| R-023 | Hidden dependency on OS | Windows/Linux path/case behavior differs | Sandbox receipt records OS and path assumptions |
| R-024 | Hidden dependency on Python/Node/Rust version | Toolchain drift changes result | Tool identity includes exact runtime/compiler versions |
| R-025 | Lockfile changed without notice | Dependency behavior changes with code | Sandbox receipt hashes lockfiles and dependency manifests |
| R-026 | Transitive dependency drift | Re-run is not reproducible | Record resolved dependency tree where feasible |
| R-027 | Missing container digest | Build environment cannot be trusted | Capture image digest or mark local/non-hermetic |
| R-028 | Non-hermetic network access | Tests may depend on live external state | Network policy receipt: blocked/allowed/observed endpoints |
| R-029 | Secret-dependent tests | Results cannot be reproduced by reviewer | Redacted secret presence receipt and non-reproducible flag |
| R-030 | Generated files stale | Static/test run uses old artifacts | Detect generated-file inputs and require regen receipt |
| R-031 | Build script side effects | Build mutates source or environment | Sandbox captures dirty tree after build |
| R-032 | Untracked files affect result | CI/local mismatch | Receipt records untracked/ignored files used or dirty state |
| R-033 | Tool not installed | Stage skipped but looks green | `skipped` status must include missing tool and risk IDs |
| R-034 | Tool version unsupported | False confidence from old tool | Version policy and minimum supported versions in receipt |
| R-035 | Linter disabled in config | Static stage sees less than expected | Config diff receipt flags relaxed rules |
| R-036 | Typechecker made permissive | Broken code hidden by config | Detect `strict=false`, ignore comments, exclusions |
| R-037 | Import path hack added | Code works only in test layout | Static receipt flags sys.path/module-resolution changes |
| R-038 | Invented API | LLM calls method that does not exist | Hallucination category `invented_api` |
| R-039 | Non-existent import | Import compiles nowhere | Hallucination category `nonexistent_import` |
| R-040 | Undefined symbol | Name binding fails only on path | Hallucination category `undefined_symbol` |
| R-041 | Invalid parameter | API exists but call signature wrong | Hallucination category `invalid_parameter` |
| R-042 | Wrong resource name/path | File/env/table/API endpoint made up | Hallucination category `resource_mismatch` |
| R-043 | Plausible but wrong logic | Static passes but behavior wrong | Require execution stages; critic never sole gate |
| R-044 | Dead code added | Tests pass because code path unused | Diff call graph and coverage reachability receipt |
| R-045 | Changed public API undocumented | Downstream break not tested | Claim scope records touched public APIs |
| R-046 | Backward compatibility broken | Existing callers fail outside changed tests | Differential tests and public API compatibility scan |
| R-047 | Serialization format drift | Data compatibility breaks | Fixture-based differential serialization checks |
| R-048 | Database migration unsafe | Runtime deploy breaks data | Migration dry-run receipt and rollback note |
| R-049 | Concurrency race introduced | Normal tests rarely fail | Fuzz/stress stage marks concurrency coverage gap |
| R-050 | Async await missing | Promise/task failure hidden | Static async lint and runtime unhandled rejection capture |
| R-051 | Error swallowed | Tests see success while operation failed | Detect broad catch/except and missing assertion on error path |
| R-052 | Logging replaces behavior | Agent adds logs but no fix | Diff classifier flags non-behavioral fix risk |
| R-053 | Security check weakened | Authz/authn bypass | Security-sensitive diff tags and mandatory review risk |
| R-054 | Input validation narrowed | Malformed input reaches sink | Property/fuzz strategies for invalid inputs |
| R-055 | Sanitization removed | Injection risk | Static taint/security plugin future hook |
| R-056 | Permission boundary changed | Privilege escalation | Claim scope public/security API classification |
| R-057 | Performance regression | Functional tests pass but latency breaks | Optional benchmark receipt and changed hot-path flag |
| R-058 | Memory leak | Tests pass but long-run fails | Sanitizer/formal/fuzz optional stage when language supports |
| R-059 | Panic/crash path added | Rare input crashes | Fuzz receipt with crash artifacts and replay input |
| R-060 | Numeric precision drift | Edge results wrong | Property tests over numeric boundaries |
| R-061 | Unicode/path normalization bug | Real inputs differ from tests | Strategy libraries include unicode/path cases |
| R-062 | Time-ordering bug | Sort/order nondeterminism | Metamorphic relation checks for permutation/order invariants |
| R-063 | Non-deterministic output | Golden tests flaky | Rerun sampling and deterministic seed receipts |
| R-064 | Flaky test passes once | Confidence overstated | Flakiness probe/rerun budget for new/changed tests |
| R-065 | Flaky failure ignored | Real bug hidden as flake | Receipt distinguishes flaky from deterministic pass/fail |
| R-066 | Test timeout too short | False red from environment | Timeout policy recorded and baseline-relative thresholds |
| R-067 | Test timeout too long | CI budget unusable | Per-stage budget receipt and hard cap |
| R-068 | Mutation equivalent mutant | False test gap | Survivor classification supports likely equivalent/review/test gap |
| R-069 | Mutation too broad | Stage too slow, disabled | Diff-scoped mutate patterns and budget receipts |
| R-070 | Mutation cache stale | Reused result invalid | Cache key includes code, tests, tool version, config, env |
| R-071 | Mutation ignores changed tests | Surviving mutant status stale | Record runner support for test-file/test-location reporting |
| R-072 | Mutant times out | Could be bug or artifact | Timeout/unviable categories preserved separately |
| R-073 | Property strategy too weak | Generated cases miss risk | Strategy coverage metrics and corpus/hash receipts |
| R-074 | Property generated from implementation | Tautological property | Mark LLM/generated properties as bonus unless human/claim anchored |
| R-075 | Differential test overflags intended change | Correct fix marked regression | Divergence classified against claim scope |
| R-076 | Differential test underflags changed side effect | Side effect ignored | Capture observable output channels explicitly |
| R-077 | Fuzz harness not reaching changed code | Clean fuzz pass meaningless | Coverage receipt maps harness to changed lines/functions |
| R-078 | Fuzz corpus not persisted | Bug cannot be reproduced | Corpus hash and crash/replay artifact required |
| R-079 | Fuzz seed missing | Failure unreplayable | Seed/replay path mandatory in fuzz/property receipt |
| R-080 | Fuzz only low-severity paths | Signal overvalued | Severity and coverage separated from pass/fail |
| R-081 | Formal spec absent | Stage skipped but users assume proof | `not_applicable` formal receipt with risk residuals |
| R-082 | LLM-generated spec wrong | Proof proves wrong property | Generated spec is bonus signal, never sole gate |
| R-083 | Path explosion | Formal stage times out | Bounded scope and timeout receipt |
| R-084 | Critic position bias | LLM judge favors first/last option | Position-swap and critic as non-gating signal |
| R-085 | Critic self-preference | Model likes its own style | Use independent critic identity and execution-first gates |
| R-086 | Critic fooled by idiomatic code | Pretty wrong code passes review | Critic cannot override failed execution stages |
| R-087 | Agent edits tests and code together | Hard to separate fix from oracle tampering | Oracle stage runs before accepting test changes |
| R-088 | Agent deletes failing scenario | Regression disappears | Test fingerprint and removed-case detection |
| R-089 | Agent changes benchmark/eval harness | Evaluation gamed | Eval harness files protected and diff-sensitive |
| R-090 | Bundle missing failed receipts | Audit trail incomplete | Every planned stage emits receipt even on failure |
| R-091 | Bundle summary hides skipped stages | Humans overtrust green summary | Summary lists residual/skipped risk IDs |
| R-092 | Artifact hash missing | Evidence can be swapped | Content hash mandatory for all artifacts |
| R-093 | Signature without verification path | Signing theater | `bundle verify` is required product path |
| R-094 | OIDC identity ambiguous | Attestation signer unclear | Record issuer, subject, workflow, repo, commit SHA |
| R-095 | Private repo no public transparency log | Audit assumptions wrong | Attestation metadata records public/private transparency mode |
| R-096 | Long-lived signing key compromised | Bundle trust broken | Prefer keyless/OIDC where possible; local signing marked dev-only |
| R-097 | CI workflow mutable by PR | Attacker alters verifier | Require protected/reusable workflow guidance |
| R-098 | Action permissions overbroad | Verifier becomes attack surface | Minimal GitHub token permissions documented |
| R-099 | Dashboard hides raw evidence | Product becomes another opaque score | Raw receipts and schemas always downloadable |
| R-100 | No adversarial regression corpus | Same failures return repeatedly | Maintain corpus mapped to risk IDs and demo scenarios |

## Phase Ownership

| Phase | Risk Families Owned |
|-------|---------------------|
| Phase 1 | R-001 to R-006, R-090 to R-093, R-099 to R-100 through schemas, risk taxonomy, fixtures, and CLI summary contract |
| Phase 2 | R-021 to R-046 through sandbox, tool identity, dependency evidence, and hallucination/static categorization |
| Phase 3 | R-004 to R-020, R-087 to R-089 through oracle integrity and claim/scope mismatch detection |
| Phase 4 | R-049 to R-080 through mutation, property, fuzz, replay, coverage, and budget receipts |
| Phase 5 | R-081 to R-086, R-090 to R-096 through formal/critic policy, bundle verification, and signing/attestation metadata |
| Phase 6 | R-097 to R-100 through GitHub Action hardening, public demo, and adversarial corpus packaging |

## Research Anchors

- Test oracle problem and automated oracle limits: Barr et al., "The Oracle Problem in Software Testing: A Survey."
- Flaky test prevalence and rerun cost: empirical studies of flaky tests in Python, JavaScript, UI tests, and developer perception.
- SWE-bench Verified audit: hidden tests can be too narrow, too wide, underspecified, or environment-sensitive.
- Code hallucination taxonomy: CodeHalu and Delulu.
- Mutation testing practicality and limitations: Just et al., Papadakis et al., StrykerJS, mutmut, PIT, cargo-mutants.
- Property/fuzz replay and corpus evidence: Hypothesis, fast-check, libFuzzer, OSS-Fuzz.
- Attestation and provenance: SLSA, in-toto, Sigstore, GitHub artifact attestations.

## Source URLs

- https://discovery.ucl.ac.uk/1471263/
- https://arxiv.org/abs/2101.09077
- https://arxiv.org/abs/2207.01047
- https://openai.com/index/why-we-no-longer-evaluate-swe-bench-verified/
- https://arxiv.org/abs/2405.00253
- https://arxiv.org/abs/2605.07024
- https://homes.cs.washington.edu/~mernst/pubs/mutation-effectiveness-fse2014-abstract.html
- https://stryker-mutator.io/docs/stryker-js/incremental/
- https://mutmut.readthedocs.io/en/latest/
- https://mutants.rs/timeouts.html
- https://hypothesis.readthedocs.io/en/latest/reference/api.html
- https://fast-check.dev/docs/introduction/why-property-based/
- https://releases.llvm.org/9.0.0/docs/LibFuzzer.html
- https://google.github.io/oss-fuzz/advanced-topics/code-coverage/
- https://slsa.dev/spec/
- https://github.com/in-toto/attestation
- https://docs.sigstore.dev/cosign/signing/overview/
- https://docs.github.com/en/actions/concepts/security/artifact-attestations

## GSD Planning Implications

1. Add risk taxonomy support in Phase 1 schemas and fixtures.
2. Require every receipt type to include `mitigated_risks`, `residual_risks`, and `limitations`.
3. Add risk coverage to bundle summary and GitHub Action output.
4. Treat the top-100 register as a living corpus plan: every future demo/eval should map to at least one risk ID.
5. Avoid any single "Pramaan score" in v1; use risk families and residuals instead.
