# Phase 12 Workstreams

## Workstream A: Language Coverage

- Python pytest checks now cover weakened assertions, xfail/skip, raises, parametrized reductions, and renamed/deleted tests.
- TypeScript checks cover common Jest/Vitest `test`/`it`, `.todo`, `.skip`, `expect`, `toThrow`, truthy weakening, and snapshot/fixture artifacts.
- Rust checks cover `#[test]`, `#[ignore]`, `assert!`, `assert_eq!`, `panic!`, and `#[should_panic]` signals.

## Workstream B: Finding Quality

- Added `renamed_test` so pure renames are not reported only as deleted tests.
- Added `removed_error_path` and `removed_boundary_case` so high-value oracle losses are visible as first-class findings.
- Added before/after artifact fingerprints for changed fixtures and snapshots.

## Workstream C: Reviewer Output

- CLI output now prints each finding's details, not only kind/test/path.
- Smoke tests assert exact categories and reviewer-facing detail strings are emitted.

## Workstream D: Fixtures

- Extended the oracle fixture pair with Rust regression, ignore, panic, and assertion-weakening examples.
- Added Python rename fixtures to protect stable body fingerprint behavior.
