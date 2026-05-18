# Pramaan

## What This Is

Pramaan is an execution-grounded verification system for AI-generated code changes. It runs a stack of independent-ish checks against a pull request, emits signed receipts for every stage, and produces an auditable proof bundle that tells humans what was actually verified.

The product is for teams using Codex, Claude Code, GitHub Actions, or other agentic coding tools who need a stronger answer than "tests passed" before reviewing or merging AI-written code.

## Core Value

Pramaan must make it obvious when an AI agent weakened the oracle, faked confidence, or passed shallow tests while still breaking the intended behavior.

## Requirements

### Validated

(None yet - ship to validate)

### Active

- [ ] CLI can verify a PR diff against a base branch and emit a bundle.
- [ ] Each stage emits a structured receipt, including failures and skipped checks.
- [ ] Sandbox stage captures reproducible checkout and dependency/environment evidence.
- [ ] Static stage detects broken imports, undeclared symbols, typecheck failures, and likely invented APIs.
- [ ] Oracle integrity stage detects weakened, skipped, deleted, or suspiciously changed tests.
- [ ] Mutation stage runs diff-scoped mutation checks for at least Python, TypeScript, and Rust.
- [ ] Differential property/fuzz stage compares pre-patch and post-patch behavior on generated/shared inputs.
- [ ] Bundle stage emits a signed or signable manifest with tool versions, seeds, corpus hashes, and stage receipts.
- [ ] GitHub Action wraps the CLI and publishes a concise PR summary.
- [ ] First demo proves the product by catching an AI fix that makes GitHub green by weakening a test.

### Out of Scope

- Full proof of code correctness - the ethical claim is auditable confidence, not correctness.
- Formal verification as a required v1 gate - useful where available, but too narrow for most PRs.
- LLM critic as a merge gate - critic output is signal only and cannot override failed execution checks.
- All-language support in v1 - initial serious support is Python, TypeScript/JavaScript, and Rust.
- Dashboard-heavy product work before the CLI and bundle are trusted.

## Context

The founding thesis is that no single technique catches AI-generated code regressions reliably. Pramaan stacks diverse execution-grounded checks into a signed proof bundle. The safer statistical framing is not that false-negative probabilities multiply cleanly, but that diverse stages reduce correlated blind spots compared with one test suite or one LLM critic.

The seed pipeline is:

```text
PR diff -> Sandbox -> Static/Hallucination -> Oracle Integrity ->
Delta Mutation -> Property+Fuzz -> Adversarial Tests ->
Optional Formal -> Dual Critic -> Signed Bundle
```

For v1, the build should deliberately narrow this to the stages that can become reliable first:

```text
CLI + GitHub Action -> Receipts -> Sandbox -> Static -> Oracle Integrity ->
Diff Mutation -> Differential Property/Fuzz -> Signed Bundle
```

The first demo should be brutally simple: an AI agent fixes a bug by weakening a test, ordinary CI goes green, and Pramaan fails the PR with a clean receipt naming the weakened assertion.

## Constraints

- **Claim discipline**: Never market Pramaan as proving code correct - it proves specific checks ran and what they found.
- **Stage independence**: Treat stages as diverse but not fully independent; shared bad oracles and bad scope definitions can correlate failures.
- **v1 stack**: Rust core orchestrator, Python plugins for language verifiers, TypeScript GitHub Action wrapper.
- **Initial languages**: Python, TypeScript/JavaScript, and Rust first; Go and Java after the core receipts and oracle checks are stable.
- **Latency target**: 15-25 minutes single-machine and 5-8 minutes parallel for typical PRs.
- **Artifact-first architecture**: Every stage must write a receipt even when it fails, times out, or is not applicable.
- **Trust boundary**: Execution stages are mandatory gates; LLM critic and generated specs are bonus/corroborating signal.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Build receipts before breadth | Trust depends on inspectable evidence, not on having many flashy stages. | - Pending |
| Start with Python, TypeScript, and Rust | They cover high-value AI coding workflows and have workable tooling for typecheck, mutation, and property/fuzz testing. | - Pending |
| Delay adversarial amplification, formal, and dual critic | Stages 5-7 are valuable but can distract from making core execution gates deterministic and hard to game. | - Pending |
| Use signed bundles as the product boundary | The durable deliverable is an auditable proof bundle, not a transient CI log. | - Pending |
| Lead demo focuses on test weakening | Test tampering is legible, common, and sells the problem instantly. | - Pending |

---
*Last updated: 2026-05-18 after initialization*
