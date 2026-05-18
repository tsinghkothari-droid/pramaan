# Autonomous Build Command

Use this in Codex from `C:\Users\Tushar\Pictures\pramaan` to build Pramaan end to end without further clarification prompts:

```text
$gsd-execute-phase 1 --auto --no-transition
$gsd-execute-phase 2 --auto --no-transition
$gsd-execute-phase 3 --auto --no-transition
$gsd-execute-phase 4 --auto --no-transition
$gsd-execute-phase 5 --auto --no-transition
$gsd-execute-phase 6 --auto --no-transition
```

Operational instruction:

```text
Run all six commands sequentially. Do not ask for clarification. Make conservative implementation choices that follow `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and each phase plan. Pause only for missing secrets, unavailable external services, or destructive actions outside the repository. After each phase, run that phase's validation commands and continue to the next phase if green.
```

Expected result:

- Rust CLI and workspace implemented.
- Receipts, risk taxonomy, claim scope, bundle schemas, and fixtures exist.
- Sandbox/static/oracle/mutation/fuzz/bundle/GitHub Action paths exist.
- Public demo and adversarial corpus exist.
- Final `cargo fmt --check`, `cargo test`, and demo verification pass.
