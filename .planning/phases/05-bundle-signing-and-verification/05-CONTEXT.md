# Phase 5: Bundle Signing and Verification - Context

**Status:** Ready for execution after Phase 4

Phase 5 makes Pramaan durable: bundle manifests, content hashes, local dev signing/signable output, bundle verification, and risk-family summaries. This phase is where Pramaan becomes audit evidence rather than CI logs.

Locked decisions:

- Verification path is mandatory; signing without verify is theater.
- Local signing is dev-mode unless backed by CI attestation.
- Bundle summary must show residual risk, not an opaque trust score.
