# Plan 02 Summary - Signing Metadata and Risk Summary

## Completed

- Added local dev signing/signable metadata to the bundle model with `mode:
  local_dev`, `status: signable`, `dev_mode: true`, a signable digest, and a
  clear non-CI provenance note.
- Added optional GitHub artifact attestation metadata fields for issuer,
  subject, workflow, repository, commit SHA, and transparency mode.
- Updated CLI risk-family summaries to keep `mitigated`, `residual`, `skipped`,
  and `not_applicable` buckets separate.
- Updated the bundle schema and synthetic fixture to include the signing and
  GitHub attestation metadata fields without adding any opaque trust score.
- Added `docs/attestation.md` to document local dev signing, GitHub artifact
  attestation metadata, and the four-bucket risk summary policy.

## Verification

- `cargo test -p pramaan-bundle`
- `cargo test -p pramaan-cli verify_writes_receipts_and_prints_a_claim_disciplined_summary`
- Python `jsonschema` validation of `examples/fixtures/bundle.synthetic.json`
  against `schemas/bundle.schema.json`

## Notes

- Bundle verification code was already changing in parallel; this work preserved
  the stage-aware manifest and verification path while layering Plan 02 signing
  and summary metadata on top.
- No Git commands that mutate history, staging, commits, branches, or remotes
  were run.
