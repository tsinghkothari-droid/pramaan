# Phase 26.4: Minimum Lovable Verifier Loop

## Goal

Make the first product wedge feel complete: one command, one report, one proof
bundle, one killer demo, and a reviewer who can understand the result in under
30 seconds.

## Why This Phase Exists

The roadmap is broad enough. The risk is that Pramaan becomes impressive on
paper but clumsy in the first five minutes. This phase forces a compact loop
that an external maintainer can run without knowing the internals.

## Minimum Lovable Loop

```text
checkout repo -> pramaan verify -> proof bundle -> local/markdown report ->
reviewer understands blocker -> optional replay/inspection command
```

## Files To Change

- `README.md`
- `docs/quickstart.md`
- `docs/demo.md`
- `docs/github-action.md`
- `examples/`
- `crates/pramaan-cli/src/main.rs`
- `crates/pramaan-core/src/lib.rs`
- `.planning/reports/`

## Implementation Steps

1. Pick the canonical first-run demo and make every quickstart point to it.
2. Make the one-command local path obvious and repeatable.
3. Ensure generated bundles include receipts, manifest, policy summary,
   confidence explanation when available, and honest skipped-stage evidence.
4. Ensure the reviewer report shows blockers first, then warnings, skipped
   stages, and replay/inspection commands.
5. Add a manual UAT script for a fresh reviewer to explain the blocker in
   under 30 seconds.
6. Treat skipped required stages that look like passes as release blockers.

## Verification

- Fresh local run produces the expected bundle and report.
- Reviewer UAT transcript or checklist is captured in `.planning/reports/`.
- Quickstart avoids unimplemented claims.

## Exit Criteria

Pramaan has one tight, lovable product loop before adding more engines,
languages, dashboards, or enterprise options.
