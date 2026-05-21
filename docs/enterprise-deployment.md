# Enterprise Deployment Guide

Phase 35 supports private technical preview deployment. It is not yet a
turnkey enterprise service.

## Recommended Private Preview

1. Select one repository with active AI-authored pull requests.
2. Run Pramaan in GitHub Actions with `fail-on: never`.
3. Upload proof bundles as short-retention workflow artifacts.
4. Export redacted bundles for cross-team review when needed.
5. Capture reviewer overrides with `pramaan feedback override`.
6. Compare bundles against local baselines with `pramaan feedback analyze`.
7. Move to `fail-on: failed` only after reviewers trust the output.

## GitHub Enterprise

Use the composite action from this repository. For artifact attestation, the
workflow needs:

```yaml
permissions:
  contents: read
  actions: read
  checks: read
  id-token: write
  attestations: write
```

Keep `attest: "false"` until the organization is ready to publish GitHub
artifact attestations. Local/offline VSA files are emitted by the Action even
without GitHub OIDC.

## Self-Hosted Runners

Self-hosted runners are allowed only after an explicit runner-security review.
They should be isolated per repository or trust zone, run without deployment
secrets for untrusted PRs, and clean workspaces after every job.

## GitLab, Gitea, And Bitbucket

Pramaan's core model is forge-neutral, but the current wrapper is GitHub-native.
Before implementing GitLab support, Pramaan needs a provider-neutral interface
for:

- base/head refs;
- artifact upload and retention;
- job-summary rendering;
- OIDC or workload identity;
- attestation publication;
- merge-request metadata and reviewer override capture.

GitLab CI differs from GitHub Actions in artifact retention, job-token scope,
OIDC claims, and merge-request pipelines from forks. Those differences should be
documented in a dedicated GitLab phase before any production claim.

## Data Handling

Use redaction profiles before sharing bundles externally:

```powershell
pramaan bundle export-redacted target/pramaan --profile reviewer-redacted --out target/pramaan-redacted
```

Do not publish internal-full bundles unless the repository owner has approved
the artifact contents.
