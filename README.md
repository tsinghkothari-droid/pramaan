# Pramaan

Pramaan is a receipt-first verification system for AI-generated code changes.

It does not claim that code is correct. It creates an auditable proof bundle
showing what was checked, what evidence was produced, which risk families were
mitigated, and which risks remain.

The product thesis is simple:

> AI coding agents can make pull requests that look green while hiding real
> regressions. Pramaan gives reviewers a compact, execution-grounded evidence
> bundle instead of another vague "looks good" signal.

## Current Status

Pramaan is currently a working prototype foundation:

- Rust CLI and workspace skeleton.
- JSON receipt, bundle, claim-scope, risk-taxonomy, and adapter-certification schemas.
- Sandbox/worktree evidence model.
- Static/hallucination receipt paths for Python, TypeScript, and Rust fixtures.
- Oracle-integrity demo for weakened tests, fixtures, and snapshots.
- Diff-scoped mutation and differential fuzz receipt models.
- Bundle verification with artifact hashes and local signing/signable metadata.
- GitHub Action wrapper and PR-summary renderer.
- Starter adversarial corpus and top-100 risk register.

Approximate current size:

- Total tracked repo: about 12.8k lines.
- Product code plus schemas: about 7.6k lines.
- Rust source: about 6.0k lines.

This is not yet Serious v1. The remaining gap is depth, determinism, hostile
fixture coverage, real tool integrations, and reliability on real pull requests.

## Quickstart

From the repository root:

```powershell
cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke
```

The CLI writes a bundle directory:

```text
target/pramaan-smoke/
  claim_scope.synthetic.json
  receipts/
    claim-scope.receipt.json
    synthetic-verification.receipt.json
```

Validate the workspace:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke
```

## Why This Exists

AI coding agents can make changes that pass CI while still being unsafe to
merge:

- tests were weakened, skipped, or deleted;
- the original bug was never reproduced;
- snapshots or fixtures changed the oracle silently;
- a fake API, import, symbol, or parameter was invented;
- a shallow test was added that does not actually test the fix;
- a refactor works for one path but breaks another;
- CI passed, but the evidence cannot be audited later.

Pramaan is built for this review gap.

## What Pramaan Does

For a pull request, Pramaan should:

1. Capture what the PR claims to change.
2. Create isolated base/head worktrees.
3. Record dependency, environment, tool, seed, corpus, and artifact hashes.
4. Run static and hallucination checks.
5. Detect test-oracle tampering.
6. Run diff-scoped mutation checks.
7. Run replayable property/fuzz/differential checks.
8. Emit signed or signable receipts for every stage.
9. Summarize mitigated, residual, skipped, and not-applicable risks using stable risk IDs.

Example summary:

```text
Claim: Fix invoice rounding for mixed tax rates.

Evidence:
- Original failing test now passes unchanged.
- No assertions were weakened.
- Static checks found no invented imports or undefined symbols.
- Mutation killed 91% of changed-line mutants.
- Differential property checks found no unexpected divergence.

Residual risks:
- R-049 concurrency not exercised.
- R-057 performance not benchmarked.
- R-081 formal verification not applicable.
```

## Product Boundary

Pramaan says:

> Here is the evidence for this code change.

Pramaan does not say:

> This code is definitely correct.

That distinction is the product.

The stages are intentionally diverse, so their failure modes should be less
correlated than a single test pass or critic review. They are not statistically
independent, and Pramaan should never claim that the probability of failure is
the product of stage failure rates.

## Planned v1 Pipeline

```text
PR diff
  -> Sandbox and environment evidence
  -> Claim scope
  -> Static/hallucination checks
  -> Oracle integrity
  -> Delta mutation
  -> Property/fuzz/differential checks
  -> Bundle signing and verification
  -> GitHub Action summary
```

## What Is Missing for Serious v1

Serious v1 is not "more stages." Serious v1 means the receipts are trusted,
fast enough, deterministic enough, and hard to game.

| Area | Current State | Missing for Serious v1 |
| --- | --- | --- |
| CLI/orchestrator | Prototype commands and stage receipts exist | Stable plugin protocol, resumable runs, parallel scheduling, stage budgets, robust error taxonomy |
| Receipts/schemas | Core schemas exist | Schema versioning policy, compatibility tests, richer artifact graph, full in-toto/SLSA predicate mapping |
| Sandbox | Worktree/environment evidence exists | Pinned OCI image support, network policy capture, dependency tree hashing, dirty-tree after-run detection across real repos |
| Claim scope | Schema exists | PR/issue ingestion, changed public API extraction, low-confidence scope warnings, reviewer override notes |
| Static/hallucination | Fixture-based paths exist | Real pyright/mypy/ruff, tsc/eslint, cargo/clippy integration; hallucination taxonomy coverage |
| Oracle integrity | Weakened-test demo exists | Strong AST diff for pytest/Jest/Vitest/Rust tests, snapshot/fixture review UX, test fingerprinting, parametrized-case diffs |
| Mutation | Receipt model exists | Real mutmut/StrykerJS/cargo-mutants adapters, changed-file targeting, equivalent-mutant handling, incremental cache receipts |
| Property/fuzz | Receipt model exists | Real Hypothesis and fast-check discovery, replay corpus storage, differential base/head execution, minimized counterexamples |
| GitHub Action | Wrapper exists | Marketplace-ready action, PR comments, artifact upload/attestation, permissions hardening, failure-mode docs |
| Bundle signing | Local signable metadata exists | Sigstore keyless path, GitHub artifact attestation path, verification summary, tamper-evident artifact tree |
| Corpus/evals | Starter fixtures exist | 100+ adversarial PR scenarios, real-world replay set, flaky-case quarantine, benchmark dashboard |
| Tests | Unit and smoke tests exist | Full golden-receipt suite, cross-platform CI, property tests for schemas, integration tests against toy repos |
| Documentation | Product docs exist | Operator guide, security model, threat model, contributor plugin guide, demo walkthrough with screenshots |

## Roadmap

### Prototype: 8k-15k LOC

Goal: Prove the core thesis.

- CLI can run locally.
- Receipts are emitted for every stage, including failures and skips.
- Weakened-test demo passes normal CI but fails Pramaan.
- Claim scope, receipt, bundle, and risk-taxonomy schemas exist.
- Basic bundle verification catches tampering.

Status: mostly present in this repository, but still needs more real-world polish.

### Alpha MVP: 15k-30k LOC

Goal: Work on selected real repositories.

- Real Python, TypeScript, and Rust static checks.
- Real oracle-integrity checks for pytest and Jest/Vitest.
- Basic GitHub Action usable on pull requests.
- Local bundle signing/signable output.
- First 25 adversarial PR fixtures.
- Clear reviewer summary: failed stages, residual risks, replay commands.

### Real MVP: 30k-60k LOC

Goal: Be useful in serious engineering teams.

- Diff-scoped mutmut, StrykerJS, and cargo-mutants adapters.
- Hypothesis and fast-check differential testing for eligible pure functions.
- Deterministic seeds, replay data, corpus hashes, minimized counterexamples.
- Pinned container image and lockfile/dependency evidence.
- GitHub artifact upload and optional attestation.
- Stable schema versioning and compatibility tests.
- 75+ adversarial fixtures and at least 10 real-repo case studies.

### Serious v1: 80k-140k LOC

Goal: Become a credible trust layer for AI-authored pull requests.

- Production-grade orchestrator with parallel scheduling and stage budgets.
- Hardened sandboxing with OCI digest capture, network policy evidence, and dependency provenance.
- Deep Python, TypeScript, and Rust plugin coverage; Go/Java only after protocol stability.
- Full oracle integrity engine across assertions, skips, snapshots, fixtures, mocks, and parametrized cases.
- Mutation and fuzz stages that are fast, replayable, and honest about timeouts/skips.
- in-toto/SLSA-compatible proof bundle with Sigstore/GitHub attestation support.
- Large adversarial corpus mapped to stable risk IDs.
- Public demo repository showing "GitHub green, Pramaan red" in under 30 seconds.
- Security model, threat model, operator guide, plugin authoring guide, and enterprise deployment notes.

## Research Basis

Pramaan is intentionally built from existing research and production tooling
rather than a new claim of correctness.

### AI-code reliability and benchmark evidence

- [tau2-bench](https://arxiv.org/abs/2506.07982): motivates repeated, process-aware evaluation rather than one lucky pass.
- [SWE-Lancer](https://arxiv.org/abs/2502.12115): highlights how frontier models can silently fail real software tasks.
- [SWE-bench Verified](https://openai.com/index/introducing-swe-bench-verified/): motivates stronger task and oracle curation.
- [SWE-bench Verified retirement analysis](https://openai.com/index/why-we-no-longer-evaluate-swe-bench-verified/): motivates claim-scope and oracle-alignment receipts.

### Process supervision and auditability

- [Let's Verify Step by Step](https://arxiv.org/abs/2305.20050): supports step-level evidence over only final outcome labels.
- [Lost in the Middle](https://arxiv.org/abs/2307.03172): motivates chunked, per-file/per-stage receipts instead of giant context reviews.

### LLM judge and critic limitations

- [Self-preference bias](https://arxiv.org/abs/2410.21819): warns against trusting a model's own style preferences.
- [Position bias in LLM judges](https://arxiv.org/html/2406.07791v9): motivates position-swap and non-critic execution stages.
- [Don't Judge by Its Cover](https://arxiv.org/abs/2505.16222): reinforces that critic agreement is only a signal, not a gate.
- [CodeJudge](https://arxiv.org/abs/2410.02184): useful as a specialized review signal, but not as the sole pass/fail mechanism.

### Hallucination and static failure detection

- [CodeHalu](https://arxiv.org/abs/2405.00253): supports classifying invented APIs, resource mismatches, naming errors, and logic issues.
- [Collu-Bench](https://arxiv.org/html/2410.09997v1): motivates detecting hallucinated code behavior beyond simple syntax failures.
- [Delulu](https://arxiv.org/abs/2605.07024): motivates fill-in-the-middle hallucination categories such as invented APIs, invalid parameters, undefined variables, and non-existent imports.

### Mutation testing

- [Just et al., FSE 2014](https://homes.cs.washington.edu/~mernst/pubs/mutation-effectiveness-fse2014.pdf): mutation score correlates with real fault detection.
- [Papadakis et al., ICSE 2018](https://dl.acm.org/doi/pdf/10.1145/3180155.3180183): confirms mutation testing is useful but imperfect.
- [LLMorpheus](https://arxiv.org/abs/2404.09952): connects mutation-style testing with LLM-generated-code defect discovery.
- [mutmut](https://mutmut.readthedocs.io/en/latest/), [StrykerJS](https://stryker-mutator.io/docs/stryker-js/incremental/), and [cargo-mutants](https://mutants.rs/timeouts.html): production tools Pramaan can wrap with budget and timeout receipts.

### Property, fuzz, and differential testing

- [Fuzz4All](https://arxiv.org/abs/2308.04748): motivates broad fuzzing for compiler/interpreter-style systems.
- [Agentic property-based testing](https://arxiv.org/html/2510.09907v1): motivates generated properties with replayable evidence.
- [CodaMosa](https://dl.acm.org/doi/10.1109/ICSE48619.2023.00085): supports search-based test amplification for coverage gaps.
- [Metamorphic Prompt Testing](https://arxiv.org/abs/2406.06864): motivates metamorphic relations where exact assertions are hard.
- [Hypothesis](https://hypothesis.readthedocs.io/en/latest/reference/api.html) and [fast-check](https://fast-check.dev/docs/introduction/why-property-based/): production property-testing engines for replayable Python and TypeScript checks.

### Formal and semi-formal verification

- [Kani](https://github.com/model-checking/kani): Rust model checking where harnesses and bounds are available.
- [CBMC](https://www.cprover.org/cbmc/): C/C++ bounded model checking for selected safety properties.
- [Dafny](https://dafny.org/): specification-oriented verification where projects already have specs.
- [SpecGen](https://arxiv.org/abs/2401.08807): useful as a bonus spec-generation signal, not a v1 merge gate.

### Supply-chain attestations and reproducibility

- [SLSA](https://slsa.dev/spec/): provenance and build-integrity framework.
- [Sigstore](https://docs.sigstore.dev/cosign/signing/overview/): keyless signing and transparency-log-backed identity.
- [in-toto](https://in-toto.io/): supply-chain layout and attestation framework.
- [GitHub artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations): practical Sigstore-backed artifact provenance path for GitHub Actions.
- [Nix reproducibility research](https://arxiv.org/pdf/2501.15919): informs the honest boundary around bit-for-bit reproducibility.

### Agent-safe coding and adjacent future work

- [Quasar](https://arxiv.org/abs/2506.12202): motivates constrained generation for agent-friendly code.
- [Type-Constrained Code Generation](https://arxiv.org/abs/2504.09246): supports strict typing and grammar/type-constrained generation instead of inventing a new general-purpose language.
- [MCP](https://modelcontextprotocol.io/): relevant to Pramaan Adapter Certification, where agent tools need typed, auditable, replayable behavior.

## Risk Register

Pramaan tracks risks with stable IDs rather than a single opaque score.

Examples:

- `R-001`: no explicit PR claim.
- `R-006`: original failing test absent.
- `R-010`: test skip added.
- `R-011`: assertion weakened.
- `R-025`: lockfile changed without notice.
- `R-038`: invented API.
- `R-049`: concurrency race introduced.
- `R-057`: performance regression.

See [.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md](.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md) and [docs/risk-taxonomy.md](docs/risk-taxonomy.md).

## Repository Map

- [crates/pramaan-cli](crates/pramaan-cli): CLI entry point and stage commands.
- [crates/pramaan-core](crates/pramaan-core): receipt, claim-scope, risk, and shared models.
- [crates/pramaan-sandbox](crates/pramaan-sandbox): worktree and environment evidence.
- [crates/pramaan-bundle](crates/pramaan-bundle): bundle manifest, hashing, signing metadata, and verification.
- [schemas](schemas): public JSON Schemas.
- [docs](docs): public product and operator documentation.
- [examples](examples): fixtures, demos, and synthetic receipts.
- [plugins](plugins): language plugin plans and adapters.
- [.planning](.planning): GSD planning, requirements, roadmap, research, and phase validation.

## Documentation

- [Receipt model](docs/receipt-model.md)
- [Risk taxonomy](docs/risk-taxonomy.md)
- [Bundle verification](docs/bundle-verification.md)
- [Attestation](docs/attestation.md)
- [GitHub Action](docs/github-action.md)
- [Killer demo](docs/demo.md)
- [Research index](docs/RESEARCH_INDEX.md)
- [Roadmap](.planning/ROADMAP.md)

## Autonomous Build

The planned autonomous build sequence is documented here:

[.planning/AUTONOMOUS_BUILD_COMMAND.md](.planning/AUTONOMOUS_BUILD_COMMAND.md)

## License

Pramaan is licensed under the MIT License. See [LICENSE](LICENSE).
