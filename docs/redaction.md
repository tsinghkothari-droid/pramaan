# Redaction Profiles

Pramaan bundles are useful only if teams can share them without leaking local
paths, secrets, private hosts, or sensitive CI metadata. Redaction is a sharing
control, not a substitute for keeping secrets out of untrusted CI jobs.

## Profiles

| Profile | Use | Current behavior |
| --- | --- | --- |
| `internal-full` | Internal debugging inside the producing team. | Copies the bundle without text redaction and records the profile when used by receipts. |
| `reviewer-redacted` | Normal reviewer sharing. | Redacts common secret assignments, token prefixes, user paths, emails, private IPs, internal hosts, cache keys, and artifact URLs in text and JSON string values. |
| `public-demo` | Public issue, README, or sales-demo bundle. | Uses the same redaction engine as reviewer export, and should be the default for examples that leave the repo. |
| `summary-only` | Later minimal disclosure. | Accepted as a profile name, but currently uses the same text/JSON redaction engine; artifact minimization remains a future hardening step. |

## Export Command

```powershell
cargo run -p pramaan-cli -- bundle export-redacted target/pramaan `
  --profile public-demo `
  --out target/pramaan-public
```

The export command copies the bundle, redacts supported text/JSON files,
removes stale offline attestations, adds a `bundle_redaction` receipt, and
rebuilds `bundle.manifest.json` so `pramaan bundle verify` still works on the
redacted copy.

Offline VSA files are removed because redaction changes the manifest digest.
Run `pramaan bundle attest target/pramaan-public` again if the exported bundle
also needs local/offline VSA material.

## Redacted Patterns

The Phase 30 redaction engine covers:

- `password`, `token`, `secret`, `api_key`, `authorization`, GitHub token, CI
  job token, cache-key, and artifact-url assignments;
- common token prefixes such as `ghp_`, `ghs_`, `xoxb-`, `sk-`, and AWS access
  key prefixes;
- Windows, macOS, and Linux user paths;
- email-like values;
- `.internal`, `.corp`, `.local`, and localhost hostnames;
- private IPv4 ranges.

Any redacted file path is recorded in the export manifest and the redaction
receipt. That gives reviewers visibility into what changed without exposing the
raw value.

## Limits

Redaction is pattern-based. It can miss domain-specific secrets, binary blobs,
screenshots, compressed artifacts, and secrets that were transformed before
Pramaan saw them. Public demo bundles should still be reviewed before
publication.
