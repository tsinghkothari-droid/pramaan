# Phase 36.5 Summary

Status: PASS_WITH_RISKS

What landed:

- Added `clap` `about` text across the CLI command tree.
- Added `pramaan export redacted` as a top-level alias for redacted bundle
  export while keeping `bundle export-redacted`.
- Suppressed stage-runner summary chatter when stages are invoked by
  `pramaan verify`; standalone stage commands still print their own summaries.
- Split `pramaan doctor` evidence into `blockers`, `warnings`, and
  `informational`.
- Added shared cargo target-cache evidence for Rust static checks when
  `CARGO_TARGET_DIR` is not already set.
- Added structured static diagnostic-code parsing before substring fallback.
- Expanded named risk constants for oracle, fuzz, bundle, and release-adjacent
  IDs.
- Added `SECURITY.md`, `CHANGELOG.md`, release workflow scaffold, cargo publish
  metadata, quickstart install notes, and a README evidence visual.

Deferred:

- Crates.io publication.
- Real tag release binaries.
- A fresh live PR Action proof after this branch lands.
- Full compiler-AST oracle extraction.

Machine verification:

- See `MACHINE_VERIFICATION.md`.

Human sign-off:

- See `HUMAN_SIGNOFF.md`.
