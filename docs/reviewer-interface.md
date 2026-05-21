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

A future `.pramaan.toml` should keep project policy in one place:

```toml
[policy]
profile = "private-preview"

[stages]
mutation = "opt-in"
fuzz_timeout_ms = 10000
mutation_timeout_ms = 60000

[redaction]
profile = "reviewer-redacted"

[reports]
format = ["markdown", "html"]
persistent_summary = true

[artifacts]
out_dir = "target/pramaan"
```

This file is documented as a contract only. Runtime loading remains future work.

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
