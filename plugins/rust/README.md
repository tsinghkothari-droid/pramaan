# Pramaan Rust Static Adapter

Phase 2 static skeleton implemented by `pramaan static-checks`.

Discovery:

- `cargo check`: applicable when Rust files and `Cargo.toml` are present.
- `cargo test --no-run`: applicable when Rust files and `Cargo.toml` are
  present.
- `cargo clippy`: applicable when configured and available; Pramaan records
  failures as static evidence rather than hiding them behind ordinary CI.

Receipt behavior:

- no Rust files or missing manifest => `not_applicable`;
- missing Cargo executable => `skipped`;
- command failures are normalized into Pramaan receipts and classified as
  broken imports or undefined symbols when diagnostics support it.

Depth status:

- oracle and mutation details live in the sibling plugin README files;
- Rust property/fuzz parity is not complete yet and is documented under
  `plugins/rust/fuzz/README.md`;
- missing Cargo or cargo-mutants must stay visible as skipped evidence.
