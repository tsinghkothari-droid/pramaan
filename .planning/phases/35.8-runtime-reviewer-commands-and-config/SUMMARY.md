# Phase 35.8 Summary

Status: PASS_WITH_RISKS

What landed:

- `pramaan doctor --config .pramaan.toml --out doctor.json` emits a structured
  report with config parsing evidence, tool availability, and residual-risk
  warnings.
- `pramaan verify --config .pramaan.toml` now loads private-preview defaults
  for skipped stages, mutation enablement, fuzz seed, and configured Markdown /
  HTML reviewer reports.
- `docs/configuration.md`, `STATUS.md`, `TASKS.md`, and the claim audit now
  describe the runtime slice honestly.
- Smoke tests cover config loading, configured report generation, and doctor
  output.

Deferred:

- Runtime `pramaan verify-pr --url`.
- Persistent forge summary/comment updates.
- External policy files, custom risk weights, and production-grade config
  validation.

Machine verification:

- See `MACHINE_VERIFICATION.md`.

Human sign-off:

- See `HUMAN_SIGNOFF.md`.
