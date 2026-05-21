# Phase 30 Summary: Redaction Profiles and Public Bundle Export

## Status

Completed the first public-safe export slice on 2026-05-21.

## Landed

- Added redaction profile validation for:
  - `internal-full`
  - `reviewer-redacted`
  - `public-demo`
  - `summary-only`
- Expanded redaction coverage for:
  - secret assignments;
  - GitHub/Slack/OpenAI/AWS-style token prefixes;
  - private user paths;
  - email-like values;
  - internal hostnames;
  - private IPv4 addresses;
  - cache keys and artifact URLs.
- Added `pramaan bundle export-redacted <bundle> --profile <profile> --out <dir>`.
- Exported bundles are copied, scrubbed, given a `bundle_redaction` receipt,
  and rebuilt with a fresh manifest so `pramaan bundle verify` still passes.
- Stale `attestations/` output is removed during export because redaction
  changes manifest hashes.
- Added docs in `docs/redaction.md` and updated receipt-model docs.
- Added CLI smoke coverage for a deliberately leaky bundle export.

## Deferred Honestly

- `summary-only` currently uses the same text/JSON redaction engine; artifact
  minimization is still future hardening.
- Binary, compressed, screenshot, and domain-specific secret redaction remains
  out of scope for this phase.
- Redacted exports should be re-attested with `pramaan bundle attest` if local
  offline VSA material is needed after export.

## Verification

- Targeted redaction tests passed during implementation.
- Full required phase verification is recorded in the phase commit workflow.
