# Competitive Benchmark and Prior-Art Matrix

Last refreshed: 2026-05-21

Pramaan should not pretend the market is empty. AI pull-request reviewers,
test-generation tools, quality-report aggregators, and software supply-chain
attestation systems all solve adjacent problems. The defensible claim is
narrower:

> Pramaan is an evidence-bundle verifier for AI-authored pull requests. It
> combines execution receipts, oracle-integrity evidence, policy decisions, and
> hash-linked bundle verification so reviewers can inspect what ran, what was
> skipped, what was weakened, and what residual risk remains.

It is not a replacement for CI, code review, SAST, artifact attestations, or
LLM critique. It is the connective verification layer around those signals.

## Positioning Summary

| Category | Representative tools | Primary job | Evidence style | Pramaan stance |
| --- | --- | --- | --- | --- |
| AI PR reviewer | [PR-Agent](https://github.com/The-PR-Agent/pr-agent), Qodo-style reviewers | Explain a diff, suggest improvements, comment on PRs | LLM analysis and review comments | Complement. Pramaan should consume or link reviewer comments only as weak signal, never as the sole gate. |
| Structural quality reporter | [reviewdog](https://github.com/reviewdog/reviewdog), lint/check aggregators | Put analyzer findings into PR review surfaces | Tool output converted into review comments/checks | Reuse the UX pattern. Pramaan emits receipts and SARIF/policy outputs rather than becoming another generic reporter. |
| Test-change monitor | Test-change detection and check-tests style systems | Flag suspicious edits to tests, fixtures, or snapshots | Diff heuristics over tests and artifacts | Overlap. Pramaan's oracle-integrity stage should be deeper and risk-ID backed, but it should not replace specialized test-management workflows. |
| Test generation / amplification | [Pynguin](https://www.pynguin.eu/), [EvoSuite](https://www.evosuite.org/evosuite/), Pynguin/EvoSuite-style generators | Generate tests or candidate oracles | Generated tests, coverage, search-based assertions | Reuse as optional engines later. Generated tests are not proof until sandbox-executed and recorded as accepted/rejected evidence. |
| Mutation testing | mutmut, StrykerJS, cargo-mutants | Measure whether tests kill behavior perturbations | Executed mutants, killed/survived/timeouts | Use as an execution stage, scoped to the diff with explicit skipped-tool receipts. |
| Property/fuzz testing | Hypothesis, fast-check, libFuzzer family | Explore input spaces and find counterexamples | Seeds, generated inputs, shrink/corpus data | Use as execution evidence with replay data. Missing tools and timeouts remain residual risk. |
| Supply-chain attestations | [GitHub artifact attestations](https://docs.github.com/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations), [SLSA VSA](https://slsa.dev/spec/v1.2/verification_summary), [Sigstore](https://docs.sigstore.dev/), [in-toto](https://in-toto.io/) | Prove provenance, build identity, and verifier decisions | Signed attestations over artifacts and predicates | Reuse primitives. Pramaan should emit attestable verification summaries, not invent new crypto. |

## Feature Matrix

| Capability | AI PR reviewer | Quality reporter | Test-change monitor | Test generator | Attestation primitive | Pramaan target |
| --- | --- | --- | --- | --- | --- | --- |
| Diff explanation | Strong | Weak | Weak | None | None | Concise summary only |
| LLM critique | Strong | None | None | Sometimes | None | Optional weak signal only |
| Real command execution | Usually limited | Delegated to tools | Usually no | Yes, for generated tests | No code validation by itself | Core requirement |
| Oracle weakening detection | Usually heuristic | Only if analyzer exists | Core overlap | Not the primary goal | None | Core requirement with stable risk IDs |
| Mutation/fuzz receipts | Usually no | Can display output | No | Adjacent | None | Core execution evidence |
| Skipped-tool visibility | Varies | Varies | Varies | Varies | Not applicable | Required residual risk |
| Bundle hash verification | Usually no | Usually no | Usually no | No | Core primitive | Core bundle contract |
| Signed provenance | Usually no | Usually no | Usually no | No | Core primitive | Reuse, emit, and verify |
| Reviewer 30-second artifact | Comments | Comments/checks | Alerts | Generated tests | Attestation metadata | One bundle/report with blockers, warnings, and replay links |
| Agent-harness support | Sometimes | No | No | No | No | First-class done gate |

## Tool Notes

### PR-Agent and AI PR Reviewers

PR-Agent is a serious adjacent project: it automates PR review, description,
Q&A, and improvement suggestions. Pramaan should not compete on being a better
chatty reviewer. Its stronger wedge is evidence that survives the review
thread: receipts, hashes, policy outcomes, skipped-stage risk, replay metadata,
and bundle verification.

What Pramaan should reuse:

- PR-comment ergonomics and concise summaries.
- Optional reviewer hints as a weak signal.
- Forge integrations as compatibility targets.

What Pramaan should not duplicate:

- General-purpose LLM review comments.
- Code-writing or auto-fix loops.
- Broad conversational review UX before the proof bundle is trusted.

Evidence gap Pramaan targets:

- A reviewer can see whether tests were weakened, whether mutation/fuzz stages
  actually ran, and whether the bundle was tampered with. An LLM comment alone
  cannot provide that audit trail.

### reviewdog and Quality Aggregators

reviewdog is a useful model for taking arbitrary analyzer output and presenting
it in a PR. Pramaan should preserve that integration lesson but keep a stricter
evidence model. A generic aggregator can display lint findings; Pramaan must
also say which evidence was missing, which risks are residual, and whether the
receipt graph verifies.

What Pramaan should reuse:

- Diff-scoped reporting.
- SARIF/review-surface interoperability.
- Non-ownership of every analyzer.

What Pramaan should not duplicate:

- A generic "run any linter and comment" platform.

Evidence gap Pramaan targets:

- The bundle can be verified offline, policy-explained, and tied back to risk
  IDs rather than existing only as PR comments.

### Test-Change Monitors

Test-change tools are close to Pramaan's killer demo. The difference is scope:
Pramaan treats tests, fixtures, snapshots, generated cases, mutation evidence,
and claim scope as one trust boundary. A changed snapshot is not merely "a file
changed"; it is oracle-sensitive evidence that can affect merge risk.

What Pramaan should reuse:

- Simple test/fixture/snapshot diff patterns.
- Clear reviewer wording for suspicious test changes.

What Pramaan should not duplicate:

- Full test-management platforms, coverage dashboards, or flaky-test suites.

Evidence gap Pramaan targets:

- Oracle weakening is linked to policy, confidence, receipts, and bundle
  verification instead of being a standalone alert.

### Pynguin, EvoSuite, and Test Generation

Pynguin and EvoSuite show why generated tests can improve search and coverage,
but they also reinforce Pramaan's core rule: a generated test is not evidence
until it is executed, recorded, and classified. Generated tests can preserve
current behavior even when current behavior is wrong, so Pramaan must record
accepted and rejected probes separately.

What Pramaan should reuse:

- Search-based generation as optional future engines.
- Generated assertion/corpus artifacts as replayable evidence.

What Pramaan should not duplicate:

- Full test-generation engines inside the core verifier.

Evidence gap Pramaan targets:

- The proof bundle distinguishes "candidate generated" from "sandbox-executed
  and relevant." Phase 28.26 exists because this distinction matters.

### GitHub Attestations, SLSA, Sigstore, and in-toto

These are primitives, not competitors. GitHub artifact attestations establish
build provenance for artifacts. SLSA VSA defines a way for a trusted verifier
to summarize verification of artifacts and attestations against policy. Sigstore
and in-toto provide signing and attestation building blocks.

What Pramaan should reuse:

- GitHub artifact attestation where repository permissions support it.
- SLSA VSA predicate shape for verification summaries.
- Sigstore/cosign keyless identity once production signing is ready.
- in-toto statements for digest-linked evidence.

What Pramaan should not duplicate:

- Cryptographic signing infrastructure.
- A new attestation format where established predicates are sufficient.

Evidence gap Pramaan targets:

- Existing attestations can say where an artifact came from. Pramaan's job is
  to produce the verification predicate: which PR checks ran, which risks
  remain, which receipts were included, and whether the bundle graph verifies.

## Adoption Positioning

| Buyer question | Existing tool answer | Pramaan answer |
| --- | --- | --- |
| "Can an AI reviewer spot issues?" | AI PR reviewer comments on likely issues. | Useful weak signal, but not enough for merge confidence. |
| "Can findings appear in the PR?" | reviewdog/SARIF/checks can display analyzer output. | Yes, plus a durable bundle that survives the PR UI. |
| "Were tests weakened?" | Some tools flag test diffs. | Oracle integrity emits stable risk IDs and policy-visible receipts. |
| "Did generated tests actually run?" | Test generators produce candidate tests. | Only sandbox-executed accepted probes count as evidence. |
| "Can I trust the artifact?" | SLSA/Sigstore/in-toto prove provenance/signature properties. | Pramaan reuses those primitives for its evidence bundle and verifier decision. |
| "Should I merge?" | Some tools provide recommendations. | Pramaan provides evidence and policy status, not merge authority. |

## Public Claim Rules

Pramaan can honestly say:

- it is evidence-bundle infrastructure for AI-authored PRs;
- it combines execution receipts, oracle integrity, policy explanation, and
  bundle verification;
- it reuses supply-chain attestation primitives instead of inventing crypto;
- it treats AI review and generated probes as weak or pending signal until
  execution evidence exists.

Pramaan must not say yet:

- it is the most comprehensive PR verifier;
- it proves code correct;
- it replaces AI reviewers, CI, SAST, or attestations;
- generated tests or probes count before sandbox execution;
- production Sigstore identity is shipped.

## Maintenance Trigger

Refresh this benchmark before:

- public Alpha announcement;
- Serious v1 decision;
- adding or changing claims that compare Pramaan to AI PR reviewers, test
  generation systems, quality aggregators, or supply-chain attestation tools;
- publishing a "Pramaan catches what X misses" demo.

Phase 26.3 must turn the strongest comparison claims into executable fixtures.
Until then, this document is positioning evidence, not proof of superiority.
