# Phase 35 Summary: Operator Docs, Release Packaging, and Adoption

## Landed

- Added an operator guide for source install, local verification, bundle
  inspection, GitHub Action rollout, and private-preview rollout sequence.
- Added plugin-author guidance tied to the v0.1 subprocess protocol and plugin
  trust rules.
- Added a current security model that separates protected evidence properties
  from unimplemented sandbox/signing hardening.
- Added an enterprise deployment guide covering GitHub Enterprise, self-hosted
  runners, redaction, and the non-GitHub abstraction needed before GitLab,
  Gitea, or Bitbucket support.
- Added troubleshooting docs for slow mutation, missing tools, flaky tests,
  forked PRs, bundle verification failures, and warning fatigue.
- Added rendered pass, warning, fail, and bundle-inspection examples.
- Added a manual release checklist for `v0.1.0` artifacts and Marketplace
  readiness without claiming publication.
- Added `scripts/check-phase35-docs.mjs` to verify required docs and relative
  links.

## Deferred

- Actual tagged binary release and Marketplace publication remain future work.
- A hosted reviewer dashboard remains Phase 35.5+ work; Phase 35 only provides
  static rendered examples and docs.
- Cross-forge implementation remains future work after the provider-neutral
  interface is designed.

## Verification

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `node scripts/check-claim-audit.mjs`
- `node --test action/render-summary.test.mjs`
- `node scripts/check-phase35-docs.mjs`

## Residual Risk

An external maintainer now has enough documentation for a private technical
preview. The docs do not replace the missing live GitHub Action proof,
production Sigstore release path, or tagged binary release.
