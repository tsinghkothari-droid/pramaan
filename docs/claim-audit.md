# Claim Audit

Last updated: 2026-05-21

Purpose: keep Pramaan's public surface as evidence-first as the product itself.
Every important claim below is either backed by executable evidence, backed by a
fixture/manual proof, narrowed to partial/planned, or marked as an accepted
risk before Alpha.

## Labels

| Label | Meaning |
| --- | --- |
| `executable-test` | Covered by unit, smoke, golden, or action tests. |
| `checked-fixture` | Backed by a checked-in fixture or proof bundle. |
| `manual-proof` | Backed by a repeatable command and observed output. |
| `partial` | Implemented in part; public language must stay narrow. |
| `planned` | Roadmap only; not a shipped promise. |
| `experimental` | Present for demo/research, not production trust. |
| `accepted-risk` | Known gap accepted for private preview only. |

## Ledger

| Claim ID | Source | Claim | Evidence label | Evidence path | Verification command | Status | Required fix |
| --- | --- | --- | --- | --- | --- | --- | --- |
| CLAIM-README-001 | `README.md` | Pramaan produces evidence, not correctness proof. | manual-proof | `README.md`, `docs/receipt-model.md` | `rg -n "correct" README.md docs` | pass | Keep wording honest. |
| CLAIM-README-002 | `README.md` | Receipt-first verification bundle exists. | executable-test | `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-README-003 | `README.md` | Bundle verification catches tampering. | executable-test | `crates/pramaan-cli/tests/smoke.rs`, `crates/pramaan-bundle/src/lib.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-README-004 | `README.md` | Signing/Sigstore/in-toto are roadmap, not shipped production signing. | manual-proof | `STATUS.md`, `docs/threat-model.md` | `rg -n "Sigstore|in-toto|sign" README.md STATUS.md docs` | pass | Keep public copy as planned/partial. |
| CLAIM-STATUS-001 | `STATUS.md` | Implemented/partial/planned matrix exists. | manual-proof | `STATUS.md` | `Get-Content STATUS.md` | pass | Keep updated after every phase. |
| CLAIM-SCHEMA-001 | `schemas/` | Receipt schema version is `pramaan.receipt.v1`. | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-CORE-001 | `docs/receipt-model.md` | Canonical JSON hashing is deterministic. | executable-test | `crates/pramaan-core/src/lib.rs` | `cargo test -p pramaan-core canonical` | pass | None. |
| CLAIM-POLICY-001 | `docs/github-action.md` | Default policy can explain pass/warn/fail. | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-ACTION-001 | `action.yml` | Action renders reviewer summary. | executable-test | `action/render-summary.test.mjs` | `node --test action/render-summary.test.mjs` | pass | None. |
| CLAIM-ACTION-002 | `docs/github-action.md` | Artifact attestation is optional. | manual-proof | `action.yml`, `docs/github-action.md`, `.planning/reports/phase-26.1-live-action-proof.md` | `rg -n "attest" action.yml docs/github-action.md .planning/reports/phase-26.1-live-action-proof.md` | pass-with-risk | Live workflow-dispatch proof exists; production Sigstore/cosign identity remains planned. |
| CLAIM-SANDBOX-001 | `docs/threat-model.md` | Sandbox records environment/toolchain/dirty evidence. | executable-test | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox` | pass | None. |
| CLAIM-SANDBOX-002 | `docs/threat-model.md` | Container identity capture is best effort. | executable-test | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox container_identity` | pass | Keep "best effort" wording. |
| CLAIM-CLAIM-001 | `TASKS.md` | Claim scope supports issue text and maintainer notes. | executable-test | `crates/pramaan-cli/src/main.rs`, `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-STATIC-001 | `TASKS.md` | Static checks classify hallucination-style failures. | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-STATIC-002 | `TASKS.md` | Relaxed static config and security-sensitive categories are surfaced. | executable-test | `crates/pramaan-cli/src/static_checks.rs` | `cargo test -p pramaan-cli static_checks` | pass | None. |
| CLAIM-ORACLE-001 | `docs/demo.md` | Weakened-test demo produces a failed oracle receipt. | executable-test | `crates/pramaan-cli/tests/smoke.rs`, `examples/vulnerable-python-pr/` | `cargo test --workspace` | pass | None. |
| CLAIM-ORACLE-002 | `TASKS.md` | Python/TS/Rust oracle weakening patterns are detected. | executable-test | `examples/fixtures/oracle/`, `crates/pramaan-core/src/lib.rs` | `cargo test -p pramaan-core oracle_fixture` | pass | None. |
| CLAIM-ORACLE-003 | `.planning/ROADMAP.md` | Full compiler AST extraction exists. | accepted-risk | `docs/receipt-model.md`, `docs/risk-taxonomy.md`, `docs/oracle-integrity.md` | `rg -n "full compiler AST|not yet|parser-backed subset" docs` | narrowed | Public language must say parser-backed subset evidence, not full AST proof. |
| CLAIM-ORACLE-004 | `docs/oracle-integrity.md` | Oracle evidence records parser metadata and full-AST residual risk. | executable-test | `crates/pramaan-core/src/lib.rs`, `scripts/check-oracle-parser-metadata.mjs`, `docs/oracle-parser-decision.md` | `cargo test -p pramaan-core oracle_fixture && node scripts/check-oracle-parser-metadata.mjs target/pramaan-minimum-lovable/oracle-diff.json` | pass-with-risk | Metadata is not full compiler AST extraction. |
| CLAIM-MUTATION-001 | `TASKS.md` | Mutation adapters run real tools when installed. | manual-proof | `crates/pramaan-cli/src/mutation.rs` | `cargo run -p pramaan-cli -- mutation --repo examples/fixtures/mutation --changed-file python/checkout.py --timeout-ms 1000 --out target/pramaan-pilot/mutation` | pass-with-risk | Needs CI image with tools installed for positive execution proof. |
| CLAIM-MUTATION-002 | `docs/plugins.md` | Missing mutation tools do not mitigate risk. | executable-test | `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-FUZZ-001 | `TASKS.md` | Differential replay records seed, corpus hash, counterexamples, and divergence classes. | executable-test | `crates/pramaan-cli/tests/smoke.rs` | `cargo test --workspace` | pass | None. |
| CLAIM-FUZZ-002 | `TASKS.md` | Hypothesis/fast-check campaigns run today. | executable-test | `crates/pramaan-cli/src/fuzz.rs`, `scripts/check-fuzz-harness-evidence.mjs`, `docs/plugins.md` | `cargo test -p pramaan-cli --test smoke fuzz && node scripts/check-fuzz-harness-evidence.mjs <differential-fuzz.json>` | pass-with-risk | Tool-backed mode runs only when tools are installed; missing tools remain deterministic fallback evidence. |
| CLAIM-CONFIDENCE-001 | `docs/confidence.md` | Confidence vote is decomposed, uncalibrated, and explicitly not a correctness proof. | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/tests/smoke.rs`, `schemas/confidence.schema.json` | `cargo test --workspace` | pass | Keep calibration marked uncalibrated until Phase 34. |
| CLAIM-BUNDLE-001 | `docs/bundle-verification.md` | Manifest references artifacts by digest and rejects tampering. | executable-test | `crates/pramaan-bundle/src/lib.rs` | `cargo test -p pramaan-bundle` | pass | None. |
| CLAIM-REDACTION-001 | `docs/receipt-model.md` | Common secrets and private paths are redacted by helper. | executable-test | `crates/pramaan-core/src/lib.rs` | `cargo test -p pramaan-core redaction` | pass | Enterprise logs need broader fixtures. |
| CLAIM-DEMO-001 | `docs/demo.md` | Public demos are inspectable in under 30 seconds. | manual-proof | `docs/demo.md`, `examples/proof-bundles/` | `cargo run -p pramaan-cli -- oracle --base-repo examples/fixtures/oracle/base --head-repo examples/fixtures/oracle/head --out target/pramaan-pilot/oracle` | pass-with-risk | Needs user study/external reviewer timing before marketing as measured. |
| CLAIM-PILOT-001 | `.planning/ROADMAP.md` | Three external real repositories were validated. | manual-proof | `.planning/reports/phase-26-external-alpha-pilots.md` | `powershell -ExecutionPolicy Bypass -File scripts/run-phase26-pilots.ps1 -SkipClone` | pass | Live GitHub Action proof remains separate. |
| CLAIM-POSITIONING-001 | `README.md` | Pramaan is complementary to AI PR reviewers, quality aggregators, test generators, and attestation primitives. | manual-proof | `docs/competitive-benchmark.md`, `README.md` | `rg -n "competitive benchmark|complement|not a replacement" README.md docs/competitive-benchmark.md` | pass | Refresh before public Alpha and Serious v1. |
| CLAIM-GAP-001 | `docs/competitive-benchmark.md` | Category-level competitor-gap fixtures exist for evidence gaps Pramaan targets. | manual-proof | `corpus/competitor-gap-fixtures.v0.1.json`, `scripts/check-competitor-gap-fixtures.mjs` | `node scripts/check-competitor-gap-fixtures.mjs` | pass-with-risk | Metadata fixtures are not named-tool benchmark results. |
| CLAIM-QUICKSTART-001 | `docs/quickstart.md`, `STATUS.md` | One-command local quickstart produces a bundle, confidence evidence, policy explanation, and blockers-first report. | manual-proof | `scripts/run-minimum-lovable-loop.ps1`, `.planning/reports/phase-26.4-minimum-lovable-loop-uat.md` | `powershell -ExecutionPolicy Bypass -File scripts/run-minimum-lovable-loop.ps1` | pass-with-risk | This is an oracle-focused demo loop; Phase 35.5 owns Rust-native report commands. |
| CLAIM-RELEASE-001 | `TASKS.md` | Alpha MVP gates are satisfied. | accepted-risk | `TASKS.md`, `.planning/research/P0_P1_ALPHA_PILOT_2026-05-21.md` | `rg -n "Alpha MVP|not yet|NO_GO" TASKS.md .planning` | blocked | Private preview only until external repository pilots close. |
| CLAIM-STATUS-CAP-001 | `STATUS.md` | STATUS: Rust workspace and CLI skeleton | executable-test | `Cargo.toml`, `crates/pramaan-cli/src/main.rs` | `cargo build --workspace` | pass | Keep smoke tests green. |
| CLAIM-STATUS-CAP-002 | `STATUS.md` | STATUS: Receipt model with risk IDs and artifact refs | executable-test | `crates/pramaan-core/src/lib.rs`, `schemas/receipt.schema.json` | `cargo test -p pramaan-core` | pass | Keep schema/runtime fixture tests aligned. |
| CLAIM-STATUS-CAP-003 | `STATUS.md` | STATUS: Bundle manifest and hash-integrity verification | executable-test | `crates/pramaan-bundle/src/lib.rs` | `cargo test -p pramaan-bundle` | pass | Do not market as external signing. |
| CLAIM-STATUS-CAP-004 | `STATUS.md` | STATUS: Real Sigstore/cosign keyless signing | planned | `docs/attestation.md`, `TASKS.md` | Not implemented yet | pass | Keep roadmap-only. |
| CLAIM-STATUS-CAP-005 | `STATUS.md` | STATUS: in-toto/SLSA-compatible statement output | planned | `docs/attestation.md`, `TASKS.md` | Not implemented yet | pass | Keep roadmap-only. |
| CLAIM-STATUS-CAP-006 | `STATUS.md` | STATUS: Sandbox base/head git worktrees | executable-test | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox` | pass | Container enforcement remains separate. |
| CLAIM-STATUS-CAP-007 | `STATUS.md` | STATUS: Container/OCI sandbox enforcement | planned | `docs/threat-model.md`, `TASKS.md` | Not implemented yet | pass | Keep distinct from OCI identity evidence. |
| CLAIM-STATUS-CAP-008 | `STATUS.md` | STATUS: Environment/toolchain evidence | executable-test | `crates/pramaan-sandbox/src/lib.rs` | `cargo test -p pramaan-sandbox` | pass | Keep bounded to captured fields. |
| CLAIM-STATUS-CAP-009 | `STATUS.md` | STATUS: Claim-scope receipt | executable-test | `crates/pramaan-cli/src/main.rs`, `schemas/claim_scope.schema.json` | `cargo test -p pramaan-cli --test smoke` | pass | Keep status Partial until matching is deeper. |
| CLAIM-STATUS-CAP-010 | `STATUS.md` | STATUS: Linked issue ingestion and maintainer scope notes | executable-test | `crates/pramaan-cli/src/main.rs`, `docs/receipt-model.md` | `cargo test -p pramaan-cli --test smoke` | pass | No automatic forge fetch yet. |
| CLAIM-STATUS-CAP-011 | `STATUS.md` | STATUS: Static checks for Python, TypeScript, and Rust | executable-test | `crates/pramaan-cli/src/static_checks.rs`, `examples/fixtures/static/` | `cargo test -p pramaan-cli --test smoke` | pass | Tool availability remains explicit. |
| CLAIM-STATUS-CAP-012 | `STATUS.md` | STATUS: Hallucination classification | executable-test | `crates/pramaan-core/src/lib.rs`, `docs/risk-taxonomy.md` | `cargo test --workspace` | pass | Keep categories evidence-backed. |
| CLAIM-STATUS-CAP-013 | `STATUS.md` | STATUS: Oracle integrity parser-backed subset extractors | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/oracle.rs`, `docs/oracle-integrity.md` | `cargo test --workspace` | pass | Full AST remains residual risk. |
| CLAIM-STATUS-CAP-013A | `STATUS.md` | STATUS: Oracle parser metadata and full-AST residual reporting | executable-test | `crates/pramaan-core/src/lib.rs`, `docs/oracle-parser-decision.md`, `scripts/check-oracle-parser-metadata.mjs` | `cargo test -p pramaan-core oracle_fixture` | pass-with-risk | Metadata keeps residual risk visible; it does not implement full AST extraction. |
| CLAIM-STATUS-CAP-014 | `STATUS.md` | STATUS: Full compiler AST-backed oracle extractors | accepted-risk | `docs/receipt-model.md`, `docs/risk-taxonomy.md`, `docs/oracle-integrity.md` | `rg -n "full compiler AST|not yet|parser-backed subset" docs` | narrowed | Current wording must stay as parser-backed subset evidence, not full AST proof. |
| CLAIM-STATUS-CAP-015 | `STATUS.md` | STATUS: Demo weakened-test / fixture / hallucination scenarios | checked-fixture | `examples/`, `docs/demo.md` | Demo commands in `docs/demo.md` | pass | Demos are not CI-attested proof bundles. |
| CLAIM-STATUS-CAP-016 | `STATUS.md` | STATUS: Diff-scoped mutation wrappers | executable-test | `crates/pramaan-cli/src/mutation.rs` | `cargo test -p pramaan-cli --test smoke` | pass-with-risk | Positive tool-backed CI image still needed. |
| CLAIM-STATUS-CAP-017 | `STATUS.md` | STATUS: Production-grade mutmut/StrykerJS/cargo-mutants integration | accepted-risk | `.planning/research/P0_P1_ALPHA_PILOT_2026-05-21.md` | `rg -n "mutation" .planning/research docs` | narrowed | Keep as private-preview residual risk. |
| CLAIM-STATUS-CAP-018 | `STATUS.md` | STATUS: Differential fuzz/property simulated mode | executable-test | `crates/pramaan-cli/src/fuzz.rs`, `examples/fixtures/fuzz/` | `cargo test -p pramaan-cli --test smoke` | pass | Keep `tool_backed=false` visible where simulated. |
| CLAIM-STATUS-CAP-019 | `STATUS.md` | STATUS: Real Hypothesis/fast-check adapters | executable-test | `crates/pramaan-cli/src/fuzz.rs`, `scripts/check-fuzz-harness-evidence.mjs` | `cargo test -p pramaan-cli --test smoke fuzz` | pass-with-risk | Tool-backed mode depends on installed Hypothesis/fast-check; missing tools must remain visible. |
| CLAIM-STATUS-CAP-020 | `STATUS.md` | STATUS: Replay command for recorded generated cases | executable-test | `crates/pramaan-cli/src/main.rs`, `crates/pramaan-cli/tests/smoke.rs`, `docs/replay.md` | `cargo test --workspace` | pass | Keep status Partial until replay can re-execute generated harnesses. |
| CLAIM-STATUS-CAP-020A | `STATUS.md` | STATUS: AI evidence-seeking probe plan | executable-test | `crates/pramaan-cli/src/main.rs`, `crates/pramaan-cli/tests/smoke.rs`, `schemas/probe.schema.json`, `docs/ai-probe-generator.md` | `cargo test --workspace` | pass | Keep status Partial until generated probes execute in a sandbox and rejected probes preserve real failure reasons. |
| CLAIM-STATUS-CAP-021 | `STATUS.md` | STATUS: GitHub Action wrapper | executable-test | `action.yml`, `action/render-summary.test.mjs`, `.planning/reports/phase-26.1-live-action-proof.md` | `node --test action/render-summary.test.mjs` | pass | PR-event demo remains useful, but workflow-dispatch proof exists. |
| CLAIM-STATUS-CAP-022 | `STATUS.md` | STATUS: Policy-as-code and `pramaan policy explain` | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs` | `cargo test --workspace` | pass | External policy-file loading remains future work. |
| CLAIM-STATUS-CAP-023 | `STATUS.md` | STATUS: Auditable confidence vote | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs`, `schemas/confidence.schema.json`, `docs/confidence.md` | `cargo test --workspace` | pass | Keep status Partial until calibration and broader fixtures land. |
| CLAIM-STATUS-CAP-024 | `STATUS.md` | STATUS: Agent completion gate | executable-test | `crates/pramaan-core/src/lib.rs`, `crates/pramaan-cli/src/main.rs`, `schemas/agent_decision.schema.json`, `docs/agent-harness.md` | `cargo test --workspace` | pass | Keep status Partial until richer IDE/MCP integrations exist. |
| CLAIM-STATUS-CAP-025 | `STATUS.md` | STATUS: Threat model for malicious PRs/verifier plugins | manual-proof | `docs/threat-model.md` | `Get-Content docs/threat-model.md` | pass | Documentation is not enforcement. |
| CLAIM-STATUS-CAP-026 | `STATUS.md` | STATUS: Redaction helpers for shareable evidence | executable-test | `crates/pramaan-core/src/lib.rs`, `docs/receipt-model.md` | `cargo test -p pramaan-core` | pass | Redaction profiles remain broader future work. |
| CLAIM-STATUS-CAP-027 | `STATUS.md` | STATUS: Adapter certification mode | checked-fixture | `docs/adapter-certification.md`, `examples/fixtures/adapter_certification.synthetic.json` | Fixture/manual inspection | pass | Keep adjacent, not core v0.1 path. |
| CLAIM-STATUS-CAP-028 | `STATUS.md` | STATUS: Public claim audit gate | executable-test | `docs/claim-audit.md`, `scripts/check-claim-audit.mjs` | `node scripts/check-claim-audit.mjs` | pass | Keep the ledger updated after every phase. |

## Counts

| Bucket | Count |
| --- | ---: |
| Total claims audited | 62 |
| `STATUS.md` capability rows covered | 30 |
| Executable-test claims | 42 |
| Checked-fixture/manual-proof claims | 12 |
| Partial/planned/accepted-risk claims | 8 |
| False-or-stale claims left in public copy | 0 |
| Public Alpha blockers | 1 |

## Public Alpha Blockers

1. Keep every full-AST or production-signing statement narrowed until parser and
   Sigstore/in-toto integrations are executable and fixture-backed.
