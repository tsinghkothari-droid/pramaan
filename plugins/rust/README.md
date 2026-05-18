# Pramaan Rust Plugin

This directory is reserved for the future Rust verification plugin.

Phase 1 does not implement real Rust checks. The intended boundary is:

- run configured `cargo check`, test-build, mutation, and property checks;
- convert tool output into Pramaan stage receipts;
- report skipped or not-applicable checks explicitly when tooling is absent;
- preserve Pramaan's receipt-first contract for every stage outcome.
