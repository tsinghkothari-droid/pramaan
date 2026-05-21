# Phase 30: Redaction Profiles and Public Bundle Export

## Goal

Make Pramaan bundles safe to share with reviewers, customers, or public demos
without leaking secrets, private paths, hostnames, or sensitive CI metadata.

## Research Drivers

- Artifact and provenance systems can accidentally expose build arguments,
  internal paths, and environment metadata.
- External pilots require exportable bundles before public Alpha claims are
  credible.

## Tasks Covered

- Redaction profiles: `internal-full`, `reviewer-redacted`, `public-demo`, and
  `summary-only`.
- Secret, endpoint, path, and CI metadata scrub tests.
- Public-safe bundle export command or policy.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-bundle/`
- `crates/pramaan-cli/`
- `schemas/`
- `docs/redaction.md`
- `examples/`
- `TASKS.md`

## Implementation Steps

1. Define redaction profile semantics and what evidence each profile can keep.
2. Add redaction tests for tokens, private paths, internal hostnames, email-like
   values, IPs, cache keys, artifact URLs, and CI variables.
3. Ensure hashes remain useful after redaction without leaking raw values.
4. Add CLI support for exporting a redacted bundle copy.
5. Record the active redaction profile in bundle metadata.

## Verification

- Golden redaction fixtures prove sensitive values are removed.
- Bundle verification still works after allowed redaction transformations.
- Public-demo export contains enough evidence for review but no known secrets.

## Exit Criteria

The project can publish demo bundles without hand-editing JSON or exposing
private CI information.
