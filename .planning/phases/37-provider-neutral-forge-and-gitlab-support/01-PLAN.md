# Phase 37: Provider-Neutral Forge and GitLab Support

## Goal

Design the forge abstraction before GitHub assumptions become too deeply baked
into schemas, Action logic, and reports.

## Research Drivers

- GitLab has different attestation and OIDC surfaces from GitHub.
- Enterprise buyers often use self-hosted GitLab, Gitea, Bitbucket, or private
  artifact stores.

## Tasks Covered

- Provider-neutral VCS/CI interfaces.
- GitLab CI support design.
- GitLab attestation and OIDC differences.
- Gitea and Bitbucket notes.

## Files to Change

- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `schemas/`
- `docs/forge-support.md`
- `docs/gitlab.md`
- `TASKS.md`

## Implementation Steps

1. Identify all GitHub-specific assumptions in schemas, CLI, Action, docs, and
   receipts.
2. Define provider-neutral PR, commit, artifact, identity, and attestation
   interfaces.
3. Document GitLab OIDC and attestation differences before code expansion.
4. Add tests that parse provider metadata from GitHub and GitLab-style inputs.
5. Keep Gitea and Bitbucket as documented later targets unless a pilot demands
   implementation.

## Verification

- GitHub behavior remains stable.
- GitLab metadata fixtures can produce provider-neutral receipt fields.
- Docs state exactly which forge support is implemented versus planned.

## Exit Criteria

Pramaan can grow beyond GitHub without rewriting its bundle schema or trust
model.
