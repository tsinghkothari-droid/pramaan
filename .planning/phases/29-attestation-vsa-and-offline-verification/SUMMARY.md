# Phase 29 Summary: Attestation, VSA, and Offline Verification

## Status

Completed local/offline trust slice on 2026-05-21.

## Landed

- Added `pramaan bundle attest <bundle>` to emit:
  - `attestations/bundle.vsa.json`
  - `attestations/bundle.in-toto.json`
- Added `pramaan bundle verify-offline <bundle>` to verify:
  - normal bundle manifest/hash integrity;
  - in-toto statement type and predicate type;
  - in-toto predicate equality with the VSA artifact;
  - VSA subject digest against the current manifest digest;
  - deterministic VSA result against current bundle risk/status;
  - `confidence.json` artifact digest when present.
- Added `schemas/vsa.schema.json` and
  `schemas/in_toto_statement.schema.json`.
- Updated the composite GitHub Action to emit local/offline attestation files
  before uploading the proof bundle.
- Documented the trust model in `docs/signing.md`, `docs/attestation.md`,
  `docs/bundle-verification.md`, `docs/github-action.md`, and
  `docs/threat-model.md`.
- Added smoke coverage that creates confidence evidence, emits local
  attestations, verifies them offline, and rejects a tampered VSA result.

## Deferred Honestly

- Real Sigstore/cosign keyless signing is not implemented.
- Certificate identity and transparency-log verification are not implemented.
- Live GitHub artifact-attestation proof still depends on a workflow run with
  `id-token: write`, `attestations: write`, and `attest: "true"`.

## Verification

- `cargo test -p pramaan-bundle -p pramaan-cli --tests` passed during
  implementation.
- Full required phase verification is recorded in the phase commit workflow.
