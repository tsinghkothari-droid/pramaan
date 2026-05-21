# Phase 29: Attestation, VSA, and Offline Verification

## Goal

Make proof bundles verifiable outside the original CI run with signed artifacts,
GitHub attestations, and SLSA Verification Summary Attestation-style output.

## Research Drivers

- GitHub artifact attestations provide a production-native provenance path.
- SLSA VSA defines a standard shape for summarizing verification decisions.
- Sigstore/cosign blob signing is a practical v0.1 signing path.

## Tasks Covered

- Sigstore keyless or cosign-backed signing.
- GitHub artifact attestation integration.
- in-toto/SLSA-compatible predicate mapping.
- Offline verification for downloaded bundles.
- Public/private repository trust-model documentation.

## Files to Change

- `crates/pramaan-bundle/`
- `crates/pramaan-cli/`
- `schemas/`
- `.github/workflows/`
- `docs/signing.md`
- `docs/threat-model.md`
- `TASKS.md`

## Implementation Steps

1. Start with cosign blob signing unless the Rust Sigstore path is clearly
   mature enough for this repo.
2. Emit an in-toto statement for the bundle manifest.
3. Emit a SLSA VSA-style summary for the final policy decision.
4. Add GitHub Action support for artifact attestation when permissions allow.
5. Implement offline verification for local bundle, signature, certificate, and
   attestation files.
6. Document trust anchors, failure modes, private-repo caveats, and runner
   compromise limits.

## Verification

- Tampered bundle, signature, manifest, and attestation fixtures fail
  verification.
- Offline verification works without calling GitHub APIs when local attestation
  material is present.
- Public docs do not imply stronger guarantees than the implemented trust
  model.

## Exit Criteria

Reviewers can download a bundle and verify its integrity, signer identity, and
final verification summary without trusting a screenshot of CI.
