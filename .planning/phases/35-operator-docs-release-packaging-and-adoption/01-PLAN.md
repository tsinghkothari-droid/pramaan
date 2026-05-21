# Phase 35: Operator Docs, Release Packaging, and Adoption

## Goal

Make Pramaan installable and understandable by an external maintainer without
requiring help from the repo author.

## Research Drivers

- A trust product fails if operators cannot reproduce, inspect, or explain the
  evidence.
- Marketplace and CI adoption require precise permissions, troubleshooting,
  and example outputs.

## Tasks Covered

- Operator guide.
- Plugin-author guide.
- Security model.
- Enterprise deployment guide.
- Troubleshooting for slow mutation, missing tools, flaky tests, and forked PRs.
- PR summary screenshots and release packaging.

## Files to Change

- `docs/`
- `README.md`
- `STATUS.md`
- `.github/workflows/`
- `action.yml`
- `TASKS.md`
- `.planning/STATE.md`

## Implementation Steps

1. Write install, run, inspect, verify, and troubleshoot guides.
2. Add screenshots or rendered summaries for successful, warning, and failed
   bundles.
3. Stage release packaging for Linux x86_64, Linux aarch64, and macOS arm64.
4. Document marketplace readiness without claiming publication until it happens.
5. Add docs for private technical preview onboarding.

## Verification

- Fresh checkout instructions work on a clean machine or CI.
- Docs links are checked.
- Release workflow can build artifacts or has a manual release checklist with
  exact commands.

## Exit Criteria

An external maintainer can install Pramaan, run it on a PR, inspect the bundle,
and understand common failure modes without private guidance.
