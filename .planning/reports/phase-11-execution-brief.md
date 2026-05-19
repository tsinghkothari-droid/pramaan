# Phase 11 Execution Brief

## Phase

Phase 11: Sandbox, Claim, and Static Depth

## Implementation Commit

`76b73ac` - `Phase 11: deepen sandbox claim and static evidence`

## What Changed

- Sandbox evidence now records shell, timezone, locale, Rust/Cargo/Node/npm/Python versions, image name, image digest, network policy, and base/head lockfile drift.
- Lockfile drift uses Git blob digests and maps changed lockfiles to dependency-drift residual risk.
- `pramaan verify` can derive claim scope from GitHub event JSON or `PRAMAAN_PR_TITLE` / `PRAMAAN_PR_BODY`, including linked issue references.
- Claim scope now runs deterministic public API scans for changed Python, TypeScript/TSX, and Rust files.
- Static checks now include configured Python `pyright` and Rust `cargo clippy` receipts.
- Static hallucination categories expanded to `invented_api`, `invalid_parameter`, `undefined_symbol`, `nonexistent_import`, `resource_mismatch`, `logic_mismatch`, and `unknown`.
- Bundle-internal artifact paths are now emitted relative to the bundle root to avoid ambiguous manifest resolution.

## Why It Matters

Phase 11 turns Pramaan's early sandbox and static receipts from a shape demo into evidence reviewers can actually interrogate: what environment ran, whether dependencies drifted, what the PR claimed, what public APIs changed, and which static tools were skipped or failed.

## Remaining Scope

- Network policy is recorded, not enforced or observed.
- Linked issue text is not fetched yet; only references are captured.
- Public API detection is deterministic and shallow, not AST-complete.
- Static config relaxation detection remains open.
