# Attestation Metadata

Pramaan v1 treats signing and attestation as evidence metadata, not as a magic
verdict. A bundle can be signable locally for development, or it can carry
GitHub artifact attestation metadata when produced by CI.

## Local Dev Signing

Local dev signing metadata uses:

```json
{
  "mode": "local_dev",
  "status": "signable",
  "dev_mode": true,
  "signable_digest": {
    "algorithm": "sha256",
    "value": "..."
  },
  "note": "Local dev signing metadata only; this is not CI-backed provenance."
}
```

`dev_mode: true` is required for local signing/signable output because a local
developer machine does not provide OIDC-backed CI identity, workflow context, or
an external transparency log by itself.

## GitHub Artifact Attestation

When a bundle is produced in GitHub Actions, Pramaan can carry optional
artifact-attestation metadata:

- `issuer`: OIDC issuer, typically `https://token.actions.githubusercontent.com`.
- `subject`: attestation subject such as the repo/ref identity.
- `workflow`: workflow file or workflow identity that produced the bundle.
- `repository`: owner/repo that produced the bundle.
- `commit_sha`: commit SHA associated with the attested bundle.
- `transparency_mode`: public Rekor, GitHub private transparency, none, unknown,
  or other.

These fields identify where provenance came from. They do not replace receipt
hash checks, bundle verification, or review of residual risks.

## Risk Summary

Bundle and CLI summaries must keep risk families grouped by:

- `mitigated`
- `residual`
- `skipped`
- `not_applicable`

Pramaan v1 must not emit a single opaque trust score. The point of the summary
is to make skipped and residual risk visible beside the evidence that ran.
