# Phase 26.1: Live GitHub Action Proof

## Goal

Run Pramaan's composite GitHub Action on a real pull request or PR-like branch
and capture the proof bundle artifact plus rendered job summary.

## Why This Split Exists

Phase 26 produced three external local pilot runs, but the live GitHub Action
proof requires repository/CI control that was not available in the local
execution environment. It cannot be marked complete from local tests alone.

## Tasks

1. Create or select a public repository branch where Pramaan can run safely.
2. Add the Pramaan composite Action workflow with `fetch-depth: 0`.
3. Open a pull request or equivalent PR event.
4. Confirm the workflow uploads `pramaan-proof-bundle`.
5. Capture the rendered `GITHUB_STEP_SUMMARY`.
6. Store the run URL, artifact name, manifest digest, and screenshots/logs in a
   phase report.
7. Update the public Alpha decision.

## Exit Criteria

- A live GitHub Actions run URL is recorded.
- The proof bundle artifact is downloadable from the run.
- The job summary shows failed/actionable stages and residual risk families.
- `TASKS.md`, `.planning/STATE.md`, and `.planning/ROADMAP.md` distinguish
  live Action evidence from local action-summary tests.
