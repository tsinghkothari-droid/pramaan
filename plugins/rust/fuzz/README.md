# Pramaan Rust Property/Fuzz Adapter

Rust property/fuzz support is not at parity with Python Hypothesis or
TypeScript fast-check yet.

Current behavior:

- Rust mutation evidence can use `cargo-mutants` when installed.
- Rust oracle evidence covers parser-backed subset test changes.
- Differential/property evidence for Rust is recorded as deterministic or
  skipped residual risk today.

Promotion gate:

- add a bounded subprocess adapter for a Rust property/fuzz tool;
- record seed, corpus hash, timeout, raw-output digest, and replay command;
- keep missing tools visible as skipped evidence;
- add fixtures proving failures affect the final fuzz/property verdict.
