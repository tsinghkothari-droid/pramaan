# Pramaan Status

Pramaan is an early-stage evidence-bundle verifier for AI-authored pull
requests. This page is the ground-truth matrix for what the repository currently
ships versus what remains planned.

Status labels:

- **Implemented:** working code, tests, and docs exist in this repository.
- **Partial:** useful code exists, but the capability is not complete enough to
  market as production-ready.
- **Stub:** command, schema, fixture, or placeholder exists, but it is not real
  verification yet.
- **Planned:** documented roadmap item with no meaningful implementation yet.
- **Experimental:** useful for demos or research, not a stable contract.

## Capability Matrix

| Capability | Status | Evidence | How to verify |
| --- | --- | --- | --- |
| Rust workspace and CLI skeleton | Implemented | `Cargo.toml`, `crates/pramaan-cli/src/main.rs` | `cargo build --workspace` |
| Receipt model with risk IDs and artifact refs | Implemented | `crates/pramaan-core/src/lib.rs`, `schemas/receipt.schema.json` | `cargo test --workspace` |
| Bundle manifest and hash-integrity verification | Implemented | `crates/pramaan-bundle/src/lib.rs` | `cargo test -p pramaan-bundle` |
| Real Sigstore/cosign keyless signing | Planned | `docs/attestation.md`, roadmap Phase 14/Phase 20+ | Not implemented yet |
| in-toto/SLSA-compatible statement output | Planned | `docs/attestation.md`, roadmap Phase 14 | Not implemented yet |
| Sandbox base/head git worktrees | Implemented | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox` |
| Container/OCI sandbox enforcement | Planned | `TASKS.md`, Phase 21 | Not implemented yet |
| Environment/toolchain evidence | Partial | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox` |
| Claim-scope receipt | Partial | `crates/pramaan-cli/src/main.rs`, `schemas/claim_scope.schema.json` | `pramaan verify --base <ref> --head <ref>` |
| Linked issue ingestion and maintainer scope notes | Partial | `crates/pramaan-cli/src/main.rs`, `docs/receipt-model.md` | `cargo test -p pramaan-cli --test smoke` |
| Static checks for Python, TypeScript, and Rust | Partial | `crates/pramaan-cli/src/static_checks.rs` | `pramaan static-checks --base <ref> --head <ref>` |
| Hallucination classification | Partial | `crates/pramaan-cli/src/static_checks.rs`, `docs/risk-taxonomy.md` | Static-check fixture tests |
| Oracle integrity parser-backed subset extractors | Partial | `crates/pramaan-cli/src/oracle.rs`, `crates/pramaan-core/src/lib.rs`, `docs/oracle-integrity.md` | `pramaan oracle --base-repo <base> --head-repo <head>` |
| Oracle parser metadata and full-AST residual reporting | Partial | `crates/pramaan-core/src/lib.rs`, `docs/oracle-parser-decision.md`, `scripts/check-oracle-parser-metadata.mjs` | `node scripts/check-oracle-parser-metadata.mjs <oracle-diff.json>` |
| Full compiler AST-backed oracle extractors | Planned | `docs/claim-audit.md`, Phase 23 residual risk | Not implemented yet |
| Demo weakened-test / fixture / hallucination scenarios | Implemented | `examples/`, `docs/demo.md` | Demo commands in docs |
| Diff-scoped mutation wrappers | Partial | `crates/pramaan-cli/src/mutation.rs` | Mutation tests and skipped receipts |
| Production-grade mutmut/StrykerJS/cargo-mutants integration | Partial | `crates/pramaan-cli/src/mutation.rs`, `docs/plugins.md` | Runs when tools are installed; missing tools emit skipped receipts |
| Differential fuzz/property simulated mode | Experimental | `crates/pramaan-cli/src/fuzz.rs` | Fuzz tests and replay fixtures |
| Real Hypothesis/fast-check adapters | Planned | Phase 24 | Not implemented yet |
| Replay command for recorded generated cases | Partial | `crates/pramaan-cli/src/main.rs`, `docs/replay.md` | `pramaan replay <bundle> --case <id>` |
| AI evidence-seeking probe plan | Partial | `crates/pramaan-cli/src/main.rs`, `schemas/probe.schema.json`, `docs/ai-probe-generator.md` | `pramaan probe plan --bundle <bundle>` |
| GitHub Action wrapper | Partial | `action.yml`, `action/render-summary.mjs` | Node tests and example workflows |
| Policy-as-code and `pramaan policy explain` | Partial | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs`, `docs/github-action.md` | `cargo test --workspace` |
| Auditable confidence vote | Partial | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs`, `schemas/confidence.schema.json`, `docs/confidence.md` | `pramaan confidence explain <bundle>` |
| Agent completion gate | Partial | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs`, `schemas/agent_decision.schema.json`, `docs/agent-harness.md` | `pramaan agent explain --bundle <bundle>` |
| Threat model for malicious PRs/verifier plugins | Implemented | `docs/threat-model.md` | Manual doc review |
| Redaction helpers for shareable evidence | Partial | `crates/pramaan-core/src/lib.rs`, `docs/receipt-model.md` | `cargo test -p pramaan-core` |
| Adapter certification mode | Partial | `docs/adapter-certification.md`, schemas | Docs/schema only |
| Public claim audit gate | Implemented | `docs/claim-audit.md` | Manual ledger plus `cargo test --workspace` and Node action tests |

## Current Honest Product Claim

Pramaan currently provides a receipt-first Rust CLI foundation with working
bundle hash verification, sandbox/environment evidence, static-check adapters
that record the real underlying tool versions, parser-backed subset oracle
integrity checks, demo fixtures, a default policy explanation path,
recorded-case replay for differential fuzz evidence, an AI evidence-seeking
probe plan that requires sandbox execution before mitigation, an uncalibrated
auditable confidence vote, a deterministic agent completion gate, redaction
helpers, threat-model documentation, a claim-audit ledger, and a GitHub Action
wrapper. Operator, security, enterprise, troubleshooting, rendered-example, and
release-packaging docs exist for private technical preview adoption.

`pramaan verify` orchestrates real stages end-to-end: claim scope, sandbox
setup, static checks, oracle integrity, and differential fuzz run by default,
each producing real receipts under the bundle. Mutation testing is opt-in via
`--with-mutation`. Stages can be excluded with repeated `--skip-stage <name>`
flags. The synthetic-verification placeholder is now a fallback only, emitted
exclusively when every real stage was skipped.

For the first local reviewer loop, `scripts/run-minimum-lovable-loop.ps1` runs
the weakened-test demo, writes a verifiable oracle bundle manifest, adds
confidence and policy evidence, and emits a blockers-first Markdown report. This
is a quickstart demo path, not production v1 readiness.

It does **not** yet provide production-grade signed attestations, enforced
container isolation, real Hypothesis/fast-check property execution, full
compiler-AST oracle parsing, or sandbox execution of AI-generated probes. The
confidence vote is implemented as decomposed residual-risk evidence, not as a
calibrated probability or merge authority.

## First Target User

The first target user is a team reviewing AI-authored pull requests in an
existing Python, TypeScript, or Rust repository with CI already in place. They
need a reviewer-facing evidence bundle that explains why ordinary green CI may
not be enough.

## First Killer Workflow

The first workflow to make undeniable is:

> A coding agent weakens a failing test, ordinary CI goes green, and Pramaan
> fails the PR with a clear oracle-integrity receipt that a reviewer can
> understand in under 30 seconds.

## Non-Goals for v0.1

- Pramaan is not a correctness oracle.
- Pramaan will not automatically merge pull requests.
- Pramaan is not a generic CI replacement.
- Pramaan is not an agent registry or orchestration platform.
- A dashboard must not block the CLI and GitHub Action from becoming trustworthy.

## Research Sufficiency Checklist

Broad research should pause once these are true and remaining questions should
be converted into fixtures, policies, schemas, or experiments:

- 40 source-backed research notes.
- 30 mapped failure modes.
- 10 competing or prior-art tools.
- 25 adversarial fixtures.
- 3 pilot repositories.
- Measured runtime and skipped-stage baselines.

## Pivot / Pause Criteria

Pause feature expansion and fix trust/UX first if any of these hold:

- reviewers cannot understand a bundle in under 30 seconds;
- PR verification regularly exceeds the documented SLA;
- skipped, missing-tool, or timed-out stages look like passes;
- the README claims more than the code can demonstrate;
- bundles expose secrets, private paths, internal endpoints, or sensitive CI
  metadata.
