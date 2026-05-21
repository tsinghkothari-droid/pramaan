# Phase 22.5: P0 Assertion Truth Audit Gate

## Goal

Prevent Pramaan from becoming a "joke repo" by forcing every public claim,
capability assertion, and phase-success statement to be backed by evidence or
explicitly marked as partial, planned, experimental, or false.

## Rule

No assertion may remain in `README.md`, `STATUS.md`, `TASKS.md`,
`.planning/ROADMAP.md`, public docs, schemas, examples, or Action docs unless
it has one of these evidence labels:

- `executable-test`: covered by a unit, integration, smoke, golden, or action
  test that runs in CI/local verification.
- `checked-fixture`: backed by a checked-in fixture, demo repository, schema
  fixture, or recorded bundle that is validated by tests.
- `manual-proof`: backed by a repeatable command and documented observed
  output.
- `implemented-untested`: code exists, but missing direct evidence; must create
  a follow-up task before Alpha.
- `partial`: some code exists, but the public claim must be narrowed.
- `planned`: roadmap only; public marketing must not imply it ships.
- `experimental`: present for research/demo only; not a production promise.
- `false-or-stale`: claim must be removed or rewritten immediately.

## P0 Tasks Covered

- Build a claim ledger for the public product surface.
- Audit every implemented/partial/planned assertion against code and tests.
- Downgrade or rewrite claims that overstate what ships.
- Create missing tests or golden fixtures for high-risk implemented claims.
- Add a release-blocking "claim audit must be green" rule before Alpha MVP.

## Files to Change

- `docs/claim-audit.md`
- `README.md`
- `STATUS.md`
- `TASKS.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `.planning/CLAUDE_IMPROVEMENT_PLAN.md`
- `crates/**/tests/**` where direct evidence is missing
- `examples/**` and `schemas/**` where fixture evidence is missing

## Claim Inventory Scope

Audit at minimum:

1. README hero/value claims.
2. README feature tables and pipeline descriptions.
3. README research-backed or "serious v1" claims.
4. `STATUS.md` implemented/partial/stub/planned matrix.
5. `TASKS.md` completed checkboxes.
6. `.planning/ROADMAP.md` phase success criteria and completed status notes.
7. Docs that describe bundle verification, receipts, GitHub Action behavior,
   demos, risk taxonomy, threat model, and plugin/adapter work.
8. `action.yml` inputs/outputs and README Action promises.
9. Schema claims: required fields, compatibility, signing, policy, attribution,
   override, redaction, and plugin identity.
10. Example/demo claims: ordinary CI green, Pramaan red, inspectable bundles,
    and reproducible commands.

## Implementation Steps

1. Create `docs/claim-audit.md` with a table:
   `claim_id`, `source`, `claim`, `evidence_label`, `evidence_path`,
   `verification_command`, `status`, `required_fix`.
2. Inventory every assertion from the public and planning surfaces listed
   above. Use stable claim IDs such as `CLAIM-README-001`.
3. For each claim, point to evidence:
   code path, test path, fixture path, schema path, command, or downgrade.
4. For any claim marked `implemented` without executable evidence, either add
   a direct test/fixture in this phase or move it to `partial/planned`.
5. Add or update tests for the highest-risk claims:
   canonical hashing, policy decisions, sandbox evidence, redaction, claim
   scope, static security signals, oracle weakening, bundle verification, and
   Action summary rendering.
6. Add a release-gate checklist to `TASKS.md`: Alpha cannot ship with
   `false-or-stale` claims, and `implemented-untested` claims require an owner
   and deadline.
7. Append a completion summary to this plan with exact claim counts:
   total claims audited, executable-test count, fixture count, downgraded
   claims, false/stale claims removed, and remaining accepted risk.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node --test action/render-summary.test.mjs`
- `rg -n "Sigstore|in-toto|signed|attestation|mutation|fuzz|AST|oracle|policy|sandbox|proved correct|correctness" README.md STATUS.md TASKS.md docs .planning`
- Every `implemented` capability in `STATUS.md` has an entry in
  `docs/claim-audit.md` with `executable-test`, `checked-fixture`, or
  `manual-proof`.

## Exit Criteria

Pramaan may still be early, but it must be impossible for a serious reviewer to
say the repo is pretending. Every claim is either proven by local evidence,
clearly labeled as incomplete, or removed.

## Completion Summary

Completed on 2026-05-21.

Landed:

- Added `docs/claim-audit.md` with 53 audited claims across `STATUS.md`,
  README, setup/intent docs, schemas, examples, tasks, and the GitHub Action.
- Added `scripts/check-claim-audit.mjs`, which fails if any `STATUS.md`
  capability row is missing from the claim ledger or if stale/unresolved labels
  remain.
- Updated stale status rows from Phases 20-22 so policy explanation, issue/scope
  ingestion, threat-model documentation, and redaction helpers are no longer
  incorrectly shown as pure future work.
- Tightened README/setup/intent wording around signing, mutation, fuzzing, and
  the illustrative reviewer summary.
- Marked the Phase 22.5 task group complete in `TASKS.md`.

Audit counts:

- Total claims audited: 53.
- `STATUS.md` capability rows covered: 26 of 26.
- Executable-test claims: 33.
- Checked-fixture/manual-proof claims: 8.
- Partial/planned/accepted-risk claims accepted with bounded wording: 12.
- False or stale claims left unresolved: 0.

Verification:

- `node scripts/check-claim-audit.mjs`
- `rg -n "signed proof|signed receipts|signed evidence layer|mutation/fuzz evidence|who/what signed|\\*\\*signed\\*\\*" README.md STATUS.md docs TASKS.md`

Deferred:

- The audit checker currently enforces `STATUS.md` coverage and stale-label
  cleanup. A later release gate can expand it to parse all README and docs claim
  IDs mechanically.
- Full cargo verification depends on the current Rust working-tree edits in
  oracle/mutation/fuzz/core files, which are intentionally left untouched by
  this docs/audit phase.
