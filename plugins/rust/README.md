# Pramaan Rust Static Adapter

Phase 2 static skeleton implemented by `pramaan static-checks`.

Discovery:

- `cargo check`: applicable when Rust files and `Cargo.toml` are present.
- `cargo test --no-run`: applicable when Rust files and `Cargo.toml` are
  present.

Receipt behavior:

- no Rust files or missing manifest => `not_applicable`;
- missing Cargo executable => `skipped`;
- command failures are normalized into Pramaan receipts and classified as
  broken imports or undefined symbols when diagnostics support it.
