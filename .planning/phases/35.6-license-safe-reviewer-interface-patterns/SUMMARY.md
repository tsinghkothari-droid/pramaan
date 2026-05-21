# Phase 35.6 Summary: License-Safe Reviewer Interface Patterns

Date: 2026-05-21

Status: PASS_WITH_RISKS

## What Landed

- Added `docs/reviewer-interface.md` with Pramaan-owned reviewer commands,
  planned PR URL entrypoint, `.pramaan.toml` contract, persistent-summary
  posture, and explicit out-of-scope boundaries.
- Reworked public competitive-benchmark language to describe adjacent tool
  categories rather than naming or borrowing from specific projects.
- Updated public docs that described "AI reviewers" as generic "review
  assistants" where the public copy did not need named comparisons.
- Added `scripts/check-license-safe-public-docs.mjs` to fail if risky adjacent
  project names appear in public-facing docs.
- Updated roadmap, task, and state docs to keep the phase honest.

## Verification

- `node scripts/check-license-safe-public-docs.mjs`

Full workspace verification is recorded in the commit closeout.

## Deferred

- `pramaan verify-pr --url <pull-request-url>` is documented as a staged
  interface contract, not implemented.
- Runtime `.pramaan.toml` loading is documented as a contract, not implemented.
- `pramaan doctor` remains future CLI work.
- Forge-specific persistent PR comment updates remain future integration work.

## Residual Risks

- Category-level docs avoid license and naming risk, but public comparisons
  should still be reviewed before any marketing launch.
- The check script covers selected risky names in public docs, not every
  possible adjacent project or all research/planning files.
