# Pramaan Rust Mutation Adapter

Phase 4 mutation skeleton implemented by `pramaan mutation`.

Discovery:

- Rust source is selected from `--changed-file` values ending in `.rs`.
- `Cargo.toml` must be present for the adapter to be applicable.

Execution:

- The adapter uses `cargo mutants --timeout <seconds>` when `cargo-mutants` is
  installed.
- Changed Rust files are passed as `--file <path>` filters.

Receipt behavior:

- no Rust source or manifest => `not_applicable`;
- missing Cargo or `cargo mutants` => `skipped`;
- cargo-mutants output is normalized into killed, survived, timed-out,
  unviable, and skipped mutant counts;
- timeout and unviable categories are kept separate so hangs do not look like
  clean survivors or clean kills.
