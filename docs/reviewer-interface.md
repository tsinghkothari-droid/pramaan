# Reviewer Interface

Pramaan's reviewer interface is intentionally evidence-first. It should make a
pull request easier to inspect without becoming a chatty review assistant or a
merge authority.

## Entry Points

Current supported entry points:

```bash
pramaan verify --base <base-ref> --head <head-ref> --out target/pramaan
pramaan report markdown --bundle target/pramaan
pramaan report html --bundle target/pramaan --out target/pramaan/report.html
pramaan policy explain target/pramaan --profile private-preview
pramaan doctor --config .pramaan.toml --out target/pramaan/doctor.json
```

Planned pull-request URL entry point:

```bash
pramaan verify-pr --url <pull-request-url> --out target/pramaan
```

`verify-pr` is a staged interface contract, not an implemented command yet. It
must resolve the base/head refs from a forge URL, run the same evidence stages
as `verify`, and emit the same bundle artifacts.

## Reviewer Commands

These command names are Pramaan-owned interface contracts for future PR comment
or chat integrations:

- `/pramaan verify`
- `/pramaan explain`
- `/pramaan replay`
- `/pramaan policy <profile>`

The commands must summarize existing receipts and reports. They must not ask an
LLM to decide whether a pull request is safe to merge.

## Configuration Contract

`.pramaan.toml` now has a narrow private-preview runtime slice for local
defaults:

```toml
[policy]
profile = "private-preview"

[redaction]
profile = "reviewer-redacted"

[mutation]
enabled = false

[fuzz]
seed = 12345

[stages]
skip = ["static_checks"]

[reports]
markdown = "target/pramaan/reviewer-report.md"
html = "target/pramaan/reviewer-report.html"
```

Runtime loading supports the fields documented in `docs/configuration.md`.
External policy files, custom risk weights, stage budgets, forge credentials,
and persistent PR summary updates remain future work.

## Summary Behavior

Pramaan should prefer one durable summary surface per run or pull request:

- GitHub Actions step summary for CI runs;
- uploaded proof bundle artifact for the audit trail;
- local Markdown/HTML reports for offline review;
- future persistent PR summary update where the forge supports it.

It should avoid posting repeated noisy comments for the same evidence unless a
human explicitly asks for that workflow.

## Out Of Scope

- Auto-fixing code.
- Rewriting pull-request descriptions.
- Conversational code review loops.
- Merge recommendations based only on generated commentary.
- Copying command names, prompts, docs, screenshots, config keys, or source
  code from adjacent projects.
