# Phase 25 Summary

## Status

Completed 2026-05-21 as `NO_GO_PUBLIC_ALPHA`.

## Landed

- Added `.planning/research/P0_P1_ALPHA_PILOT_2026-05-21.md`.
- Ran internal oracle, mutation, Python fuzz, and TypeScript fuzz fixture pilots.
- Recorded runtimes, skipped-stage behavior, residual risks, and Alpha decision.
- Kept public Alpha blocked until external pilots and remaining claim/product
  evidence work are complete.

## Verification

- `cargo run -p pramaan-cli -- oracle --base-repo examples\fixtures\oracle\base --head-repo examples\fixtures\oracle\head --out target\pramaan-pilot\oracle`
- `cargo run -p pramaan-cli -- mutation --repo examples\fixtures\mutation --changed-file python/checkout.py --changed-file typescript/src/checkout.ts --changed-file rust/src/lib.rs --timeout-ms 1000 --out target\pramaan-pilot\mutation`
- `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base --head-repo examples\fixtures\fuzz\head --claim-scope examples\fixtures\fuzz\claim_scope.json --seed 4242 --out target\pramaan-pilot\fuzz-python`
- `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base-ts --head-repo examples\fixtures\fuzz\head-ts --claim-scope examples\fixtures\fuzz\claim_scope_ts.json --seed 1337 --out target\pramaan-pilot\fuzz-typescript`

## Residual Risk

Internal fixtures are not a substitute for real repository pilots. Public Alpha
requires three external repos with measured runtime, noise, skipped stages,
false positives, false negatives, and reviewer time-to-understand.
