# Phase 12 Verification

## Commands

```powershell
cargo fmt --check
cargo test --workspace
node --test action\render-summary.test.mjs
```

Result: PASS.

```powershell
cargo run -q -- oracle --base-repo examples/fixtures/oracle/base --head-repo examples/fixtures/oracle/head --out target/pramaan/phase12-oracle
```

Result: PASS. The command emitted a failed oracle receipt, as expected for the weakened fixture pair.

## Fixture Findings Observed

- `sensitive_artifact_changed` for `fixtures/prices.json`.
- `sensitive_artifact_changed` for `tests/__snapshots__/order.snap`.
- `added_skip` for Python xfail, TypeScript todo, and Rust ignore.
- `deleted_test` for Python and Rust removed regression tests.
- `parametrized_case_reduction` for Python `pytest.mark.parametrize` row reduction.
- `renamed_test` for a pure Python test rename.
- `removed_error_path` for TypeScript throw and Rust panic-path weakening.
- `removed_boundary_case` for a TypeScript boundary signal removal.
- `weakened_assertion` for Python, TypeScript, and Rust assertion weakening.

## Residual Verification Risks

- Line-oriented extraction can miss generated tests, custom wrappers, macro-generated Rust tests, and framework-specific variants.
- Token-strength comparison can miss some semantic weakenings, such as changed constants or relaxed ranges that retain a strong token.
- Artifact detection does not yet cover every naming convention such as `goldens/`, `expected/`, or approval-test directories.
