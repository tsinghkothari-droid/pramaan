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
| Linked issue ingestion and maintainer scope notes | Planned | `TASKS.md`, Phase 22 | Not implemented yet |
| Static checks for Python, TypeScript, and Rust | Partial | `crates/pramaan-cli/src/static_checks.rs` | `pramaan static-checks --base <ref> --head <ref>` |
| Hallucination classification | Partial | `crates/pramaan-cli/src/static_checks.rs`, `docs/risk-taxonomy.md` | Static-check fixture tests |
| Oracle integrity heuristic checks | Partial | `crates/pramaan-cli/src/oracle.rs` | `pramaan oracle --base <ref> --head <ref>` |
| AST-backed oracle extractors | Planned | Phase 23 | Not implemented yet |
| Demo weakened-test / fixture / hallucination scenarios | Implemented | `examples/`, `docs/demo.md` | Demo commands in docs |
| Diff-scoped mutation wrappers | Partial | `crates/pramaan-cli/src/mutation.rs` | Mutation tests and skipped receipts |
| Production-grade mutmut/StrykerJS/cargo-mutants integration | Planned | Phase 24 | Not implemented yet |
| Differential fuzz/property simulated mode | Experimental | `crates/pramaan-cli/src/fuzz.rs` | Fuzz tests and replay fixtures |
| Real Hypothesis/fast-check adapters | Planned | Phase 24 | Not implemented yet |
| Replay command for failing generated cases | Planned | Phase 25 or later | Not implemented yet |
| GitHub Action wrapper | Partial | `action.yml`, `action/render-summary.mjs` | Node tests and example workflows |
| Policy-as-code and `pramaan policy explain` | Planned | Phase 20 | Not implemented yet |
| Threat model for malicious PRs/verifier plugins | Planned | Phase 21 | Not implemented yet |
| Redaction profiles for shareable bundles | Planned | Phase 21 | Not implemented yet |
| Adapter certification mode | Partial | `docs/adapter-certification.md`, schemas | Docs/schema only |

## Current Honest Product Claim

Pramaan currently provides a receipt-first Rust CLI foundation with working
bundle hash verification, sandbox/environment evidence, static-check adapters,
heuristic oracle integrity checks, demo fixtures, and a GitHub Action wrapper.

It does **not** yet provide production-grade signed attestations, enforced
container isolation, real property/fuzz tool execution, fully integrated
mutation testing, or a complete end-to-end `verify` pipeline that runs every
stage automatically.

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
