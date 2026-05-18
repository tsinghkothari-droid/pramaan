# Autonomous Recursive GSD Command

Use this from `C:\Users\Tushar\Pictures\pramaan` when you want Codex to keep executing Pramaan GSD phases recursively until the project is complete.

This command is intentionally strict. It lets Codex move autonomously, spawn subagents, stress-test each phase, write an unbiased review file, aggregate phase status, commit, push, and continue. It also defines stop conditions so the loop does not blindly commit broken work.

## Paste-Ready Command

```text
You are Codex working in C:\Users\Tushar\Pictures\pramaan.

Run the Pramaan autonomous recursive GSD loop from START_PHASE={{START_PHASE}} to END_PHASE={{END_PHASE}}.

Default values if not supplied:
- START_PHASE: read .planning/STATE.md and choose the current incomplete phase; if unclear, start at Phase 8.
- END_PHASE: highest numbered directory under .planning/phases.

Important ordering rule:
- Do not freeze receipt schema v0.1 in Phase 9 until the schema-impact subset of Phase 16 is complete.
- If running the Serious v1 loop, use this safe order:
  `8 -> 16a schema-impact subset -> 9 -> 10 -> 11 -> 12 -> 13 -> 14 -> 15 -> 16 remainder -> 17`.
- Phase 16a means only the receipt/bundle schema hooks for agent attribution, reviewer override, multi-agent provenance, plugin identity, redaction manifest, policy decision, and stage budgets.

Hard rules:
- Do not skip verification.
- Do not collapse multiple phases into one implementation batch.
- Execute Phase n fully before starting Phase n+1.
- Use subagents wherever work can be parallelized safely.
- Keep commits phase-scoped.
- If a phase cannot be completed, stop, write a blocker report, and do not fake completion.

For n from START_PHASE to END_PHASE:

0. Preflight
   - Run `git status --short --branch`; the worktree must be clean before starting a phase.
   - Confirm phase dependencies are implemented, not merely planned.
   - Record intended files before editing.
   - If this is Phase 9 and Phase 16a has not been completed, stop and execute Phase 16a first.

1. Phase intake
   - Read TASKS.md, .planning/PROJECT.md, .planning/REQUIREMENTS.md, .planning/ROADMAP.md, .planning/STATE.md, and .planning/phases/<n>-*/01-PLAN.md.
   - Extract the exact phase goal, requirements, success criteria, likely file changes, and known risks.
   - Write .planning/reports/phase-<n>-execution-brief.md.

2. Workstream decomposition
   - Break the phase into implementation workstreams based on success criteria.
   - Identify dependencies between workstreams.
   - Decide which workstreams can run in parallel through subagents.
   - Write .planning/reports/phase-<n>-workstreams.md.

3. Spawn subagents before implementation
   - Spawn one implementation-planning subagent to inspect the phase plan and identify concrete file changes.
   - Spawn one verification/stress-test subagent to identify phase-specific tests, adversarial checks, and failure modes.
   - If the phase touches security, CI, attestations, plugins, or sandboxing, spawn one security-review subagent.
   - For broad phases, use focused subagents such as schema-contract, cli-orchestrator, plugin-stage, tests-fixtures, and docs-reviewer-output.
   - Give each subagent a bounded, non-overlapping task.

4. Execute the phase
   - Run `$gsd-execute-phase n`.
   - If the GSD workflow has multiple plans or waves, execute them in dependency order.
   - Integrate subagent results carefully.
   - Resolve conflicts manually and coherently.
   - Make conservative implementation choices based on existing repo patterns.
   - Do not ask for clarification unless blocked by missing secrets, destructive external action, or impossible ambiguity.

5. Stress-test the phase
   Always run:
   - `cargo fmt --check`
   - `cargo test --workspace`
   - README/TASKS/planning link check
   Also run when applicable:
   - Node/action tests for files under action/
   - JSON schema validation for schemas/ and examples/fixtures/
   - Pramaan CLI smoke runs for affected commands
   - Demo verification for examples/
   - Bundle tamper tests when bundle code changes
   - Redaction/security fixtures when sandbox, CI, plugin, or artifact code changes
   - Performance/SLA smoke test when orchestration or stage budgets change
   - Bundle gate: generate a bundle, verify the bundle, tamper one artifact/receipt/manifest, and confirm verification fails
   - Honesty gate: missing tools, skipped checks, timeouts, unavailable external services, and partial evidence must produce explicit receipts, never silent green
   - Replay gate: every fuzz/mutation/demo failure must include seed, fixture path, corpus hash, or replay command
   - Any repo-specific scripts found in package files, Makefiles, justfiles, CI configs, or docs
   - Write commands and outcomes to .planning/reports/phase-<n>-verification.md.

6. Write unbiased review
   Create .planning/reviews/phase-<n>-unbiased-review.md.
   The review must include:
   - pass/fail per roadmap success criterion;
   - evidence file paths;
   - missing tests;
   - behavioral risks;
   - security or trust-model concerns;
   - strongest evidence the phase is complete;
   - strongest evidence the phase is not complete;
   - possible false confidence;
   - verdict: PASS, PASS_WITH_RISKS, or FAIL.
   The review must be skeptical. Do not write marketing language.

7. Aggregate phase state
   Create or update .planning/reports/phase-<n>-aggregate-report.md.
   Update .planning/PHASE_AGGREGATE.md with:
   - phase number;
   - status;
   - commit hash;
   - tests run;
   - review recommendation;
   - residual risks;
   - next action.

8. Fix loop if needed
   - If the unbiased review says FAIL or a required stress test fails, do not advance.
   - Diagnose the failure.
   - Create or update a gap/fix plan under the same phase directory.
   - Run `$gsd-execute-phase n --gaps-only` if a gap plan exists.
   - Repeat stress-test and unbiased review.
   - Stop after two failed fix cycles and write .planning/reports/phase-<n>-BLOCKED.md.

9. Commit and push
   If the phase is PASS or PASS_WITH_RISKS:
   - Run `git status`.
   - Stage only relevant files.
   - Commit with message: `Phase <n>: <short roadmap phase name>`.
   - Record the commit hash in the aggregate report and .planning/PHASE_AGGREGATE.md.
   - Push to `origin main`.
   - Continue to phase n+1.

10. Stop conditions
   Stop immediately if:
   - a secret, token, or external credential is required;
   - a destructive action outside the repository would be needed;
   - tests fail after two fix cycles;
   - the repo has unrelated user changes that conflict with the phase;
   - GitHub push fails due to auth or branch protection;
   - a security issue suggests the verifier could leak secrets or execute unsafe code.
   - schema incompatibility appears after schema freeze;
   - bundle verification or tamper tests fail;
   - weakened oracle is not detected in the killer demo;
   - redaction leaks secrets, private paths, or internal endpoints;
   - replay metadata is missing for a generated failing case;
   - CI permission ambiguity could expose secrets to untrusted PR code.

11. Final output
   When END_PHASE is complete, write .planning/AUTONOMOUS_COMPLETION_REPORT.md with:
   - phases completed;
   - commits pushed;
   - tests run;
   - unresolved risks;
   - remaining manual decisions;
   - whether Pramaan is Alpha MVP, Real MVP, or Serious v1 ready.

Begin now.
```

## Quick Commands

Start from Phase 8 and run through the highest planned phase:

```text
Run .planning/AUTONOMOUS_RECURSIVE_GSD_COMMAND.md with START_PHASE=8 and END_PHASE=17. Use the safe order: 8 -> 16a -> 9 -> 10 -> 11 -> 12 -> 13 -> 14 -> 15 -> 16 remainder -> 17.
```

Run only two phases, the safer batch size:

```text
Run .planning/AUTONOMOUS_RECURSIVE_GSD_COMMAND.md with START_PHASE=<n> and END_PHASE=<n+1>.
```

Run only one phase with the same stress/review/commit loop:

```text
Run .planning/AUTONOMOUS_RECURSIVE_GSD_COMMAND.md with START_PHASE=12 and END_PHASE=12.
```

Resume after a blocked phase:

```text
Run .planning/AUTONOMOUS_RECURSIVE_GSD_COMMAND.md with START_PHASE=<blocked phase> and END_PHASE=17. First read .planning/reports/phase-<blocked phase>-BLOCKED.md.
```

## Required Unbiased Review Template

```md
# Phase <n> Unbiased Review

## Verdict

PASS | PASS_WITH_RISKS | FAIL

## Roadmap Criteria

## What Was Built

## Tests and Stress Tests

## Evidence For Completion

## Evidence Against Completion

## Missing Tests

## Security and Trust Risks

## False Confidence Risks

## Files Changed

## Commit

## Next Action
```

## Required Aggregate File Format

`.planning/PHASE_AGGREGATE.md` should use this table:

```md
| Phase | Status | Commit | Tests | Review | Residual Risks | Next Action |
| --- | --- | --- | --- | --- | --- | --- |
```
