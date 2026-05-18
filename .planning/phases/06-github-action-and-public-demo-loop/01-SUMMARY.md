# Plan 01 Summary - GitHub Action Wrapper

## Scope

Implemented the Phase 6 Plan 01 wrapper lane only:

- `action.yml`
- `action/**`
- `.github/workflows/pramaan.yml`
- `docs/github-action.md`

## Deliverables

- Added a composite GitHub Action that resolves pull request base/head refs,
  runs `cargo run -p pramaan-cli -- verify`, writes the proof bundle, uploads it
  with `actions/upload-artifact`, and optionally runs GitHub artifact attestation.
- Added a deterministic Node summary renderer for `bundle.manifest.json` that
  foregrounds failed/skipped/incomplete stages and groups residual risk IDs by
  Pramaan risk family.
- Added wrapper unit tests for ref resolution and summary rendering.
- Added a ready-to-use PR workflow with minimal default permissions.
- Documented inputs, outputs, minimal permissions, and optional attestation in
  `docs/github-action.md`.

## Verification

- `node --test action/render-summary.test.mjs`
- `node action/render-summary.mjs --manifest examples/fixtures/bundle.synthetic.json --out target/pramaan-action-summary-test.md --base main --head feature`
- `cargo test -p pramaan-cli verify_writes_receipts_and_prints_a_claim_disciplined_summary`

## Notes

- The default workflow keeps attestation disabled because GitHub requires
  `id-token: write` and `attestations: write`; the docs show the opt-in block.
- The wrapper writes to `GITHUB_STEP_SUMMARY` rather than PR comments, so it does
  not need write permission on pull requests.
