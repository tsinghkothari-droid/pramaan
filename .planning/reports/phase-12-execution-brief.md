# Phase 12 Execution Brief

## Phase

Phase 12: Oracle Integrity Engine

## Implementation Commit

`44ff416` - `Phase 12: deepen oracle integrity detection`

## What Changed

- Added Rust oracle discovery for `#[test]`, `#[ignore]`, `assert!`, `assert_eq!`, `panic!`, and `#[should_panic]` style signals.
- Added renamed-test detection using stable body fingerprints that exclude test names.
- Added finding kinds for `renamed_test`, `removed_error_path`, and `removed_boundary_case`.
- Added richer fixture/snapshot change details with before/after fingerprints.
- Added Rust oracle fixtures and expanded Python fixtures for rename coverage.
- Updated CLI oracle summaries to print reviewer-facing finding details under each finding row.

## Why It Matters

Phase 12 makes the weakened-test demo generalize: Pramaan can now catch common ways an AI agent makes CI green by weakening the oracle instead of fixing the behavior.

## Remaining Scope

- Extraction is still heuristic, not full AST parsing.
- Some framework wrappers and generated tests remain out of scope.
- Artifact sensitivity is still mostly path and extension based.
