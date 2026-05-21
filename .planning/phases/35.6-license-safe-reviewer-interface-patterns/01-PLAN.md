# Phase 35.6: License-Safe Reviewer Interface Patterns

## Goal

Make Pramaan easier to adopt from a pull-request review surface while keeping
the interface, docs, commands, prompts, configuration, and implementation
original to this MIT-licensed repository.

## License-Safety Rule

Do not copy source code, prompts, configuration keys, documentation text,
screenshots, command names, or branded terminology from adjacent projects.
Borrow only category-level product lessons:

- reviewers want one PR URL entrypoint;
- summaries should update in place instead of spamming comments;
- command names should be memorable and stable;
- configuration should live in one documented project file;
- generated commentary is weaker than executed evidence.

## Interface Patterns To Build

1. Add or stage a PR URL entrypoint:
   `pramaan verify-pr --url <pull-request-url>`.
2. Define original reviewer commands:
   `/pramaan verify`, `/pramaan explain`, `/pramaan replay`, and
   `/pramaan policy <profile>`.
3. Add `.pramaan.toml` documentation for policy profile, stage budgets,
   redaction, mutation opt-in, report behavior, and persistent summary mode.
4. Make GitHub Action summaries persistent where the platform allows it, so
   repeated runs update one summary surface instead of creating noisy repeated
   comments.
5. Add `pramaan doctor` to validate config, tool availability, redaction
   profile, policy profile, and bundle output permissions.
6. Add `pramaan explain --pr <url>` or an equivalent report mode that turns a
   bundle into a concise human explanation without asking an LLM to decide
   merge safety.
7. Keep LLM review, auto-fix, and conversational suggestion loops out of scope.
   Pramaan's wedge remains executed, auditable evidence.

## Files To Change

- `TASKS.md`: phase entry and task checklist.
- `.planning/ROADMAP.md`: phase entry and acceptance criteria.
- `.planning/STATE.md`: decision record.
- `docs/competitive-benchmark.md`: category-level benchmark language only.
- `docs/action.md` or a new `docs/reviewer-interface.md`: command and config
  docs.
- `README.md`: no named adjacent-project comparisons in public positioning.

## Exit Criteria

- A new user can start from either one Action step or a PR URL command.
- Reviewer command names are clearly Pramaan-owned.
- `.pramaan.toml` is documented with a minimal example.
- Public docs use category-level prior-art language.
- A docs scan for risky adjacent-project names returns no matches.
