# Phase 11 Workstreams

## Workstream A: Sandbox Evidence

- Added richer environment evidence for OS, architecture, shell, locale, timezone, and toolchain versions.
- Added explicit image name, image digest, and network policy fields.
- Added base/head lockfile drift detection with Git blob digests.
- Marked dependency drift as residual risk when a lockfile changes.

## Workstream B: Claim Scope

- Added GitHub event and environment-variable PR metadata ingestion.
- Captured linked issue references from PR text.
- Added deterministic changed-public-API extraction for Python, TypeScript/TSX, and Rust files.
- Updated claim receipt summaries so PR-grounded evidence is not mislabeled as synthetic-only.

## Workstream C: Static Checks

- Added configured Python `pyright` stage.
- Added Rust `cargo clippy --all-targets --no-deps` stage.
- Expanded hallucination taxonomy and updated smoke tests to require the more specific category output.

## Workstream D: Bundle Path Hygiene

- Fixed `pramaan verify` artifact references to be bundle-root relative, preventing ambiguous manifest resolution in nested output directories.
