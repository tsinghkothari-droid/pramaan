---
phase: 1
plan: 2
title: Rust Workspace, CLI, and Orchestrator Skeleton
wave: 1
depends_on:
  - 01-PLAN
files_modified:
  - Cargo.toml
  - crates/pramaan-cli/Cargo.toml
  - crates/pramaan-cli/src/main.rs
  - crates/pramaan-core/Cargo.toml
  - crates/pramaan-core/src/lib.rs
  - crates/pramaan-bundle/Cargo.toml
  - crates/pramaan-bundle/src/lib.rs
  - crates/pramaan-sandbox/Cargo.toml
  - crates/pramaan-sandbox/src/lib.rs
  - plugins/python/README.md
  - plugins/typescript/README.md
  - plugins/rust/README.md
autonomous: true
requirements:
  - CLI-01
  - CLI-02
  - RCPT-01
  - RCPT-02
---

# Plan 02 - Rust Workspace, CLI, and Orchestrator Skeleton

## Objective

Create the executable skeleton that can run a synthetic verification, write receipts to an output directory, and leave stable crate boundaries for future stages.

## Must Haves

- `cargo test` runs from repo root.
- `pramaan verify --base <ref> --head <ref> --out <dir>` parses arguments.
- CLI writes a synthetic claim-scope receipt and synthetic stage receipt to the output directory.
- Core crate owns shared receipt/claim types.
- Bundle crate owns manifest/hash helpers, even if signing is placeholder-only.
- Sandbox crate exists as a boundary for Phase 2.
- Plugin directories exist but do not pretend to implement real checks.

## Tasks

<task id="1-02-01">
Create the Rust workspace and crate skeletons for `pramaan-cli`, `pramaan-core`, `pramaan-bundle`, and `pramaan-sandbox`.
</task>

<task id="1-02-02">
Implement core Rust data types matching the committed receipt and claim-scope schema fields.
</task>

<task id="1-02-03">
Implement `pramaan verify --base --head --out` argument parsing and output directory creation.
</task>

<task id="1-02-04">
Implement a synthetic orchestrator path that writes claim-scope and receipt JSON files without running real verification tools.
</task>

<task id="1-02-05">
Add placeholder plugin README files explaining the future plugin contract boundaries.
</task>

## Verification

<automated>
Run `cargo fmt --check` and `cargo test`.
</automated>

<automated>
Run `cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke` and confirm receipt files are created.
</automated>
