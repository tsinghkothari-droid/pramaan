# Signing and Offline Attestation

Pramaan's v0 signing path is intentionally modest. It makes a bundle
downloadable and locally verifiable, but it does not claim that local output is
CI-backed provenance or a proof of code correctness.

## Commands

Create local/offline attestation material for an existing bundle:

```powershell
cargo run -p pramaan-cli -- bundle attest target/pramaan
```

This writes:

- `attestations/bundle.vsa.json`
- `attestations/bundle.in-toto.json`

Verify the bundle plus local attestation material:

```powershell
cargo run -p pramaan-cli -- bundle verify-offline target/pramaan
```

`bundle verify-offline` first runs normal bundle hash verification. It then
checks that the in-toto statement wraps the same VSA predicate, that the VSA
subject digest matches the current `bundle.manifest.json` digest, that the
reported VSA result matches Pramaan's deterministic local policy mapping, and
that the referenced `confidence.json` digest matches the manifest when a
confidence artifact is present.

## Trust Model

Local/offline attestation provides tamper evidence for downloaded bundle
artifacts. It can catch accidental edits or obvious attempts to change the VSA
decision after the bundle was emitted.

It does not prove who ran Pramaan. A developer or compromised runner that can
rewrite both the bundle and the local attestation can still create a new
self-consistent local bundle. GitHub artifact attestations, Sigstore keyless
signing, certificate identity checks, and transparency-log verification remain
future production trust anchors.

## VSA Result Mapping

The local VSA result is deterministic:

- `FAILED` when the bundle final status is `failed` or `error`.
- `PASSED` when all stages passed and no residual or skipped risks are present.
- `WARNING` when the bundle is internally consistent but still carries residual
  or skipped risk.

That mapping is review evidence. It is not a merge authorization.

## GitHub Artifact Attestation

GitHub artifact attestation integration remains planned. For public
repositories, GitHub can provide an OIDC-backed provenance trail. For private
repositories, teams need to account for private visibility, retention policy,
and whether their artifact store preserves attestation material beside the
bundle.

Pramaan's manifest already has fields for artifact-attestation metadata. The
local/offline Phase 29 path marks the provider as `slsa`, status as `present`,
and transparency mode as `none` so reviewers do not confuse it with public
Sigstore/Rekor-backed provenance.
