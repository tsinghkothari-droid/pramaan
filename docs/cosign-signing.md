# Cosign Signing Readiness

Pramaan's current production-signing boundary is deliberately conservative.
`pramaan bundle attest` emits local/offline VSA and in-toto-style artifacts.
`pramaan bundle cosign-plan <bundle>` adds a readiness artifact for the next
production signing step, but it does not itself prove CI identity.

Example:

```powershell
pramaan verify --base main --head HEAD --out target/pramaan-bundle
pramaan bundle attest target/pramaan-bundle
pramaan bundle cosign-plan target/pramaan-bundle --out target/pramaan-bundle/attestations/cosign-plan.json
```

The cosign plan records:

- the bundle path and manifest path;
- the manifest digest;
- whether `cosign` is available on the runner;
- detected cosign version output when available;
- a suggested `cosign sign-blob` command;
- residual risks that a live CI identity and transparency-log check still need
  to close.

This is useful because reviewers can see whether the bundle is ready to be
signed without mistaking a local readiness file for a real Sigstore identity
claim.

Production work still required:

- run signing in a trusted CI context with OIDC;
- verify certificate identity and issuer;
- verify transparency-log material;
- record the verified identity in a receipt or attestation;
- fail closed when an expected production signature is absent.
