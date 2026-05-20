# Phase 18: P0 Product Honesty and Direction

## Goal

Lock Pramaan's narrow product thesis and make the public surface honest without
weakening the ambition.

## P0 Tasks Covered

- Declare the narrow thesis in `README.md`, `STATUS.md`, and `.planning/STATE.md`.
- Add explicit non-goals.
- Define the first ICP.
- Define the first killer workflow.
- Create the research sufficiency checklist.
- Add pivot/kill criteria.
- Add four-phase review discipline.

## Files to Change

- `README.md`
- `STATUS.md`
- `TASKS.md`
- `.planning/STATE.md`
- `.planning/CLAUDE_IMPROVEMENT_PLAN.md`

## Implementation Steps

1. Add `STATUS.md` with feature status: implemented, partial, stub, planned, experimental.
2. Update README to link to status and stop implying unimplemented capabilities are complete.
3. Add a short non-goals section: no correctness proof, no auto-merge authority, no generic CI replacement, no dashboard-first roadmap.
4. Add ICP and first workflow: AI-authored PR review in Python/TS/Rust repos, "GitHub green, Pramaan red" weakened-test demo.
5. Add research sufficiency and pivot criteria to planning docs.

## Verification

- Manual README/status consistency check.
- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`

## Exit Criteria

A new visitor can understand what Pramaan currently ships, what is planned, and
why the project exists in under 60 seconds.

## Completion Summary

Completed on 2026-05-21.

Landed:

- Added `STATUS.md` as the ground-truth feature matrix.
- Updated `README.md` to link status, clarify current implementation limits,
  and name non-goals.
- Marked the P0 product-direction tasks complete in `TASKS.md`.
- Updated `.planning/STATE.md` to advance the current focus to Phase 19.

Deferred:

- No code behavior changed in this phase.
- Full signing, real mutation/fuzz adapters, and policy gates remain planned in
  later phases.

Risks discovered:

- The README was stronger than the shipped implementation. The new status link
  reduces that risk without weakening the product thesis.
