# Claude Improvement Plan

**Created:** 2026-05-19  
**Status:** approved for GSD planning; implementation should proceed by the P0/P1 completion track  
**Scope:** close the gap between Pramaan's public README claims and the early-stage Rust workspace that currently ships.

This plan was written before implementation. On 2026-05-21, the user asked to
complete P0/P1 through GSD. The actionable follow-on is now the supplemental
Phase 18-25 track in `.planning/ROADMAP.md` and `.planning/phases/`.

Update on 2026-05-21: Phase 22.5 was inserted before Phase 23 as a blocking
P0 assertion truth audit gate. The rule is that Pramaan must test its own public
claims before adding more feature depth: every README/status/task/planning
assertion needs executable evidence, a checked fixture, a manual proof command,
or an honest partial/planned/experimental label.

## Exploration Completed

I inspected the requested project surface:

- `README.md`
- `TASKS.md`
- `.planning/ROADMAP.md`
- `docs/*.md`
- `schemas/*`
- `action.yml`
- every workspace `Cargo.toml`
- crate entry points:
  - `crates/pramaan-core/src/lib.rs`
  - `crates/pramaan-bundle/src/lib.rs`
  - `crates/pramaan-sandbox/src/lib.rs`
  - `crates/pramaan-cli/src/main.rs`
  - CLI stage modules including static checks, oracle, mutation, and fuzz support

## Build and Test State

Commands run from repository root:

```powershell
cargo build --workspace
cargo test --workspace
```

Result:

- `cargo build --workspace`: pass
- `cargo test --workspace`: pass
- Workspace tests observed: 34 Rust tests passing across bundle, CLI, core, sandbox, smoke tests, and doc-tests

## What Exists Today

Pramaan is not empty. It already has a credible receipt-first foundation:

| Area | Current state |
| --- | --- |
| Rust workspace | Four crates: `pramaan-core`, `pramaan-bundle`, `pramaan-sandbox`, `pramaan-cli`. |
| Receipt model | Runtime receipt structs with schema versions, stage status, risk IDs, artifact references, policy hooks, reviewer override hooks, agent attribution hooks, and redaction metadata. |
| Bundle model | Local manifest creation, SHA-256 artifact references, manifest verification, tamper tests, and local-dev/GitHub attestation metadata fields. |
| Sandbox | Git worktree isolation for base/head, commit evidence, dirty-state evidence, lockfile/config hashing, dependency drift evidence, network policy metadata, and toolchain/environment capture. |
| Static checks | Python, TypeScript, and Rust subprocess adapters for common compile/type/lint checks when tools exist, with skipped receipts when absent. |
| Oracle integrity | Heuristic diff scanner for Python/TS/Rust tests: deleted tests, skip/ignore markers, weakened assertions, fixture/snapshot drift, removed boundary/error cases. |
| Mutation | Subprocess adapter skeletons for mutmut, StrykerJS, and cargo-mutants with timeout and receipt normalization, but not yet production-grade. |
| Fuzz/differential | Deterministic simulated differential checks for narrow pure-function patterns with replay artifacts, but not real Hypothesis/fast-check execution. |
| GitHub Action | Composite action builds CLI, runs `pramaan verify`, uploads bundle, renders summary, and can call GitHub artifact attestation. |
| Examples/docs | Demo fixtures, risk taxonomy, adversarial corpus docs, proof bundle docs, and checked-in fixtures exist. |

## README Promises vs. Shipping Reality

| Public claim / implied capability | Reality today | Gap |
| --- | --- | --- |
| `pramaan verify` creates a full staged verification bundle | `verify` emits claim scope, sandbox setup, and a synthetic verification receipt; it does not yet orchestrate static/oracle/mutation/fuzz into one real pipeline. | Largest product gap. |
| Signed proof bundles | Bundle integrity hashing exists; real Sigstore/cosign signing and in-toto statements do not. | Security/trust gap. |
| Mutation testing | Adapters and normalizers exist, but tool-backed behavior is not yet proven end-to-end and missing tools produce skipped receipts. | Needs real execution path, fixtures, and policy. |
| Property/fuzz checks | Narrow deterministic simulated differential runner exists; real Hypothesis and fast-check adapters do not. | Needs real plugin/tool integration and replay. |
| Hermetic sandbox | Git worktree isolation and environment evidence exist; no actual container boundary, network enforcement, plugin isolation, or malicious-runner hardening yet. | Threat-model and safety gap. |
| Schema-backed receipts/bundles | Schemas and structs exist, but generated artifacts are not fully validated against canonical schemas, and canonical hashing is not JCS-stable. | Determinism gap. |
| GitHub Action production use | Action works structurally, but because `verify` does not orchestrate all stages, the PR summary can over-imply coverage. | Honesty and orchestration gap. |
| Release-ready project | No v0.1.0 release workflow, binaries, Marketplace publish flow, or published trust model. | Release gap. |

## Biggest Gaps to Close First

1. **Honest public surface:** README must clearly distinguish implemented, stubbed, planned, and experimental capabilities.
2. **Canonical data model:** schema versions and deterministic hashing must be stable before receipts become long-lived evidence.
3. **Real signing:** local hashes are useful but not the same as Sigstore/in-toto attestations.
4. **Pipeline orchestration:** `pramaan verify` must eventually run the real stages, not only emit synthetic evidence.
5. **Oracle killer use case:** test weakening detection should be the first crisp production-grade demo.
6. **Real plugin/tool execution:** one full Python path should work before expanding language breadth.
7. **Replayability:** fuzz/property failures need replay commands that reviewers can run.
8. **Threat model:** Pramaan executes untrusted PR code and tools; the verifier itself is an attack surface.

## GSD Phase Mapping

The repository already has GSD phases 1 through 17. The approved follow-on GSD
track maps the remaining P0/P1 work into focused execution phases:

| Proposed GSD phase | User phase | Name |
| --- | --- | --- |
| Phase 18 | P0 | Product honesty and direction |
| Phase 19 | P0 | Receipt golden tests and canonical evidence |
| Phase 20 | P0 | SLA and policy gates |
| Phase 21 | P1 | Sandbox, threat model, and redaction |
| Phase 22 | P1 | Claim scope and static security signals |
| Phase 23 | P1 | AST oracle extractors |
| Phase 24 | P1 | Real mutation and property/fuzz adapters |
| Phase 25 | P0/P1 gate | Pilot gate and Alpha decision |

Roadmap entries and phase directories have been created. Phase implementation
should start at Phase 18.

## Phase 1 - Honesty and Ground Truth

**Goal:** Make every public claim auditable against what the code actually implements.

Concrete file-level changes:

- Add `STATUS.md`.
  - Matrix fields: feature, README claim, current status, evidence files, command to verify, status category.
  - Status categories: `implemented`, `partial`, `stub`, `planned`, `experimental`.
- Update `README.md`.
  - Keep strong marketing positioning.
  - Add a visible "Current implementation status" section linking to `STATUS.md`.
  - Move non-shipping claims into a clearly marked roadmap section.
  - Preserve the ethical claim: Pramaan produces evidence, not correctness proof.
- Update `TASKS.md` only where the claim audit finds missing work that is not already tracked.
- Add tests only if there is a lightweight doc/status consistency check already supported by the repo; otherwise document this as manual review for the phase.

Acceptance criteria:

- A reader can tell in under 60 seconds what Pramaan currently ships and what is planned.
- No README section implies Sigstore, in-toto, real Hypothesis, real fast-check, or real mutation integrations are fully shipped unless the code proves it.
- `cargo fmt --check`, `cargo test --workspace`, and `cargo clippy --workspace -- -D warnings` pass.

Risks:

- Overcorrecting the README could weaken the product story. The copy should stay ambitious but precise.

## Phase 2 - Schema and Receipt Foundations

**Goal:** Make receipt and bundle data stable enough to become durable evidence.

Concrete file-level changes:

- Update canonical schemas in `schemas/`.
  - `schemas/receipt.schema.json`
  - `schemas/bundle.schema.json`
  - any linked schemas whose runtime shape has drifted from Rust structs
- Add or refactor schema-version fields:
  - `pramaan.receipt/v1`
  - `pramaan.bundle/v1`
  - keep compatibility notes for existing fixtures if needed
- Implement deterministic canonical serialization in `pramaan-core`.
  - Likely file: `crates/pramaan-core/src/lib.rs` initially, or split into `canonical.rs` if the module grows.
  - JCS-style ordered object keys.
  - Stable number/string/bool/null handling.
  - Hashes use canonical bytes, not pretty JSON.
- Separate mutable timestamps from content identity.
  - `event_time`: records when a stage happened.
  - `signing_time`: records when an artifact was signed.
  - Content hash excludes fields that must change across otherwise-identical runs.
- Update bundle hashing in `pramaan-bundle`.
  - Ensure manifest digest and receipt digests use canonical serialization.
- Add tests:
  - serialize -> hash -> deserialize -> re-hash matches
  - field ordering differences do not affect hash
  - timestamp/signing metadata changes do not corrupt content identity when they are intentionally excluded

Acceptance criteria:

- Canonical hash tests pass across receipt and bundle types.
- Checked-in fixtures either validate against schema or are explicitly migrated.
- No hash depends on map insertion order or pretty-printer behavior.

Risks:

- Changing hash semantics can invalidate existing example bundle fixtures. Migration should be explicit and documented.

## Phase 3 - Real Bundle Signing

**Goal:** Turn local integrity bundles into verifiable signed evidence.

Concrete file-level changes:

- Prefer a subprocess wrapper around `cosign` first to avoid heavy Rust dependency risk.
  - Add a small signing adapter in `pramaan-bundle` or `pramaan-cli`.
  - Record command, version, identity, certificate metadata, and signature artifact paths.
- Add in-toto statement output beside the bundle.
  - Map Pramaan manifest to a bounded predicate.
  - Do not claim SLSA level beyond what evidence supports.
- Extend CLI verification paths:
  - hash-only integrity verification
  - signature verification
  - attestation policy check
- Add `docs/threat-model.md`.
  - Who can forge a bundle.
  - Trust anchors.
  - What keyless identity proves and does not prove.
  - What an attacker controlling the CI runner can tamper with.
  - How private repos and forked PRs differ.
- Update `docs/attestation.md` and `docs/bundle-verification.md`.
- Add tests with fixtures:
  - hash-only verification succeeds/fails
  - missing signature is a warning or failure depending on policy
  - malformed attestation fails cleanly

Acceptance criteria:

- `pramaan bundle verify` can clearly report hash-only, signature, and policy outcomes separately.
- A bundle can carry an in-toto statement even in dev/local signing mode.
- Any new dependency or subprocess requirement is justified in the commit message.

Risks:

- Sigstore/cosign behavior can be environment-dependent. The first implementation should support skipped/unsupported receipts honestly instead of faking success.

## Phase 4 - Oracle Integrity Rules

**Goal:** Make the killer use case production-grade: catch AI-authored test weakening even when normal CI is green.

Concrete file-level changes:

- Harden existing oracle logic rather than replacing it wholesale at first.
  - Likely files: `crates/pramaan-cli/src/oracle.rs`, `crates/pramaan-core/src/lib.rs`.
- Add stable risk IDs per rule:
  - deleted tests
  - added skip/ignore markers
  - weakened assertions
  - snapshot/fixture churn
  - missing original failing test / missing regression coverage
- Add golden fixtures under `examples/oracle-integrity/`.
  - synthetic base/head pair per rule
  - expected receipt per rule
- Cross-check claim scope.
  - If a PR claims a bug fix but no failing/regression test exists, emit a residual risk.
  - Keep this as evidence, not a correctness claim.
- Add tests:
  - one positive and one negative case per rule
  - stable receipt/risk ID assertions

Acceptance criteria:

- A weakened-test demo produces a clear failed receipt with file/rule/risk ID.
- Fixture and snapshot changes are flagged as oracle-sensitive, not silently accepted.
- Golden-file tests prevent accidental wording or risk-ID drift.

Risks:

- Heuristic scanners can false-positive. Receipts should explain evidence and allow reviewer override capture.

## Phase 5 - One Real Language Plugin End-to-End

**Goal:** Prove the plugin model with one useful language path before chasing breadth.

Concrete file-level changes:

- Add `docs/plugins.md`.
  - Subprocess protocol.
  - JSON-on-stdout contract.
  - input schema, output schema, exit-code meaning.
  - receipt permissions and artifact boundaries.
- Implement one full plugin: Python.
  - likely under `plugins/python/`
  - use `mutmut` for mutation
  - use `Hypothesis` for property/differential checks
  - preserve subprocess-orchestrator role; do not reimplement tools in Rust
- Wire plugin execution into CLI stage commands first, then into `pramaan verify` only after receipts are stable.
- Diff-scoped behavior:
  - changed Python files/functions only where safely detectable
  - strict time budget
  - skipped receipt when tools are absent
  - timeout receipt when budget is exhausted
- Evidence fields:
  - seeds
  - corpus hashes
  - command lines
  - tool versions
  - changed functions/files
  - raw output artifact hashes
  - minimized counterexample or replay metadata where available
- Add example repo:
  - `examples/python-weakened-test/`
  - include a recorded bundle showing Pramaan catching the weakened test

Acceptance criteria:

- Python plugin can run in a repo with mutmut/Hypothesis installed.
- Missing tools do not pass silently.
- Recorded example bundle demonstrates the result.
- `cargo fmt`, `cargo test`, `cargo clippy` pass.

Risks:

- Tool availability on Windows/CI can be fragile. Tests should include protocol-level fixtures that do not require global installs.

## Phase 6 - Replay and Reproducibility

**Goal:** Make failures rerunnable, not just reportable.

Concrete file-level changes:

- Add CLI command:
  - `pramaan replay <bundle> --case <id>`
- Add replay lookup logic.
  - Find receipt by case ID.
  - Resolve artifact paths safely inside the bundle.
  - Re-run only the selected failing case when supported.
- Extend fuzz/property receipt metadata.
  - Hypothesis seed/settings/example database path.
  - fast-check seed/replayPath when TypeScript lands.
  - simulated fallback explicitly labeled as simulated.
- Extend sandbox receipt.
  - rustc/cargo/python/node versions where applicable
  - OS and architecture
  - container digest if present
  - network policy
  - dependency lockfile hashes
- Add tests:
  - replay command rejects path traversal
  - replay command reports unsupported cases honestly
  - replay metadata round-trips through bundle verification

Acceptance criteria:

- A recorded failing property/fuzz case can be selected by ID.
- Unsupported replay is explicit and does not pretend to validate anything.
- Toolchain fingerprint appears in the bundle summary.

Risks:

- Replay cannot be fully deterministic without dependencies and environment. The docs should call this out.

## Phase 7 - Dogfooding and Release

**Goal:** Make Pramaan usable on Pramaan itself and stage a credible v0.1.0 release.

Concrete file-level changes:

- Add `.github/workflows/pramaan.yml`.
  - Runs Pramaan on pull requests to Pramaan.
  - Uploads proof bundle artifact.
  - Uses least-privilege permissions.
  - Documents fork behavior.
- Add release workflow or release documentation.
  - linux-x86_64
  - linux-aarch64
  - macos-arm64
  - Windows can be planned separately unless needed for v0.1.0
- Stage `action.yml` for GitHub Marketplace.
  - branding
  - accurate description
  - inputs/outputs documented
- Add release checklist.
  - tag `v0.1.0`
  - changelog
  - binary checksum generation
  - artifact signing/attestation path
- Update README install/use sections to match the release path.

Acceptance criteria:

- Pramaan can run on its own PRs.
- Release workflow or manual release doc is enough for the owner to cut `v0.1.0`.
- Marketplace metadata is staged without claiming unpublished status as published.

Risks:

- Public release before signing and threat-model docs are credible could damage trust. Release notes should label v0.1.0 as an evidence-first alpha if implementation is still partial.

## Phase 13 Notes Folded Into This Plan

Two read-only agents reviewed the current mutation/fuzz area before this planning gate. Their guidance should inform Phase 5 and later hardening:

- Missing mutation/fuzz tools must emit `skipped` or `not_applicable`, never pass.
- A mutation pass should require `mutants_total > 0`, `survived == 0`, no timeout, threshold met, and parsed evidence bound to the raw artifact digest.
- Deterministic simulated fuzz should be labeled advisory unless a real adapter ran.
- `needs_review > 0` should prevent a clean pass or force warning policy.
- Add stress fixtures for stale Stryker reports, child processes surviving timeouts, function rename/move misses, unsupported pure-function downgrade, broad claim-scope masking, and edge corpus gaps.

These are not implemented yet.

## Execution Rules After Approval

For each approved phase:

1. Implement only that phase's scope.
2. Run:
   ```powershell
   cargo fmt --check
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   ```
3. Add docs and tests in the same commit.
4. Append a short completion paragraph to this file with:
   - what landed
   - what was deferred
   - risks discovered
5. Commit once with a clear message.
6. Push only when the phase is complete and the user wants the branch updated.

## Approval Gate

Implementation should start with Phase 18: P0 Product Honesty and Direction.

## Phase 22.5 Completion - 2026-05-21

Created `docs/claim-audit.md` with 27 audited claims covering README, STATUS,
schemas, docs, Action behavior, oracle, mutation, fuzz, sandbox, policy, and
release gates. The audit found 0 false-or-stale claims left in public copy and
kept 3 public Alpha blockers explicit: external pilots, safe Hypothesis/
fast-check harnesses, and full compiler-AST/signing claims. Verification used
`cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`,
`node --test action/render-summary.test.mjs`, and targeted `rg` claim searches.

## Phase 23 Completion - 2026-05-21

Added structured oracle extractor evidence to `oracle-diff.json`: extractor
profile, evidence label, assertion-signal kind, assertion strength, signal hash,
and skip markers. The implementation improves auditability without pretending
to be a full compiler AST parser; full parser-backed integrations remain a
hardening task. The oracle fixture command produced 17 findings across Python,
TypeScript, Rust, fixtures, and snapshots.

## Phase 24 Completion - 2026-05-21

Mutation receipts now distinguish `tool_executed`, `missing_tool`, and
`not_applicable`; skipped/missing tools keep mutation risks out of
`mitigated_risks` and put them in `not_applicable_risks`. Executed mutation runs
record raw-output path and digest. Differential fuzz receipts now record
Hypothesis/fast-check availability and `tool_backed=false` when deterministic
replay evidence is selected.

## Phase 25 Completion - 2026-05-21

Recorded `.planning/research/P0_P1_ALPHA_PILOT_2026-05-21.md`. Internal pilots
ran oracle, mutation, Python fuzz, and TypeScript fuzz fixtures in under one
second each on this machine, producing useful local evidence. Decision:
private technical preview is reasonable; public Alpha is a no-go until three
external real repositories are measured and the remaining release blockers are
closed.

## Research-Driven GSD Continuation - 2026-05-21

Added `.planning/research/GSD_PHASE_RESEARCH_REFRESH_2026-05-21.md` and mapped
the remaining `TASKS.md` work into Phases 26-40. The revised order starts with
external pilots and live Action proof, then parser-backed oracle evidence,
tool-backed property/fuzz replay, attestation/VSA/offline verification,
redaction, plugin trust, SARIF/policy/agentic-workflow security, corpus growth,
calibration, adoption docs, language depth, forge support, multi-agent
provenance, bounded adapter certification, and a Serious v1 release gate. This
keeps research attached to files, fixtures, policies, schemas, reports, or
release decisions.

## Phase 28.5 Confidence-Vote Insert - 2026-05-21

Inserted Phase 28.5 between tool-backed property/fuzz replay and
attestation/VSA work. This phase owns the auditable confidence-vote algorithm:
hard gates that cannot be averaged away, weak-signal aggregation, dependency
discounts for correlated checks, Wilson lower bounds for mutation confidence,
rule-of-three residual-risk bounds for zero-failure fuzz/property campaigns,
`schemas/confidence.schema.json`, and signed `confidence.json` / `confidence.md`
artifacts. Phase 34 remains responsible for later calibration against pilot
outcomes using Brier score, log loss, and reliability diagrams / expected
calibration error.
