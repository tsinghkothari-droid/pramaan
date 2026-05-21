# Operator Guide

This guide is for maintainers who want to run Pramaan on pull requests and
inspect the resulting evidence bundle without private project knowledge.

## Install From Source

Pramaan is currently source-installed:

```powershell
git clone https://github.com/tsinghkothari-droid/pramaan.git
cd pramaan
cargo build --locked -p pramaan-cli
target\debug\pramaan --help
```

On Linux/macOS, use `target/debug/pramaan` instead of `target\debug\pramaan`.

## Run Locally

From a repository with a base and head ref:

```powershell
pramaan verify --base main --head HEAD --out target/pramaan
```

Mutation testing is opt-in because it can be slow and requires language tools:

```powershell
pramaan verify --base main --head HEAD --out target/pramaan --with-mutation
```

The output directory should contain `bundle.manifest.json`, stage receipts,
artifacts, logs, and any generated confidence or attestation files.

## Inspect A Bundle

Start with the manifest:

```powershell
pramaan bundle verify target/pramaan
pramaan policy explain target/pramaan --profile private-preview
```

If the GitHub Action produced local attestation files, verify them offline:

```powershell
pramaan bundle verify-offline target/pramaan
```

For replayable differential cases:

```powershell
pramaan replay target/pramaan/fuzz --case "<stable-case-id>"
```

## Run In GitHub Actions

The minimal workflow is documented in [GitHub Action](github-action.md). Use
`fail-on: failed` to block only failed manifests, or `fail-on: never` for a
shadow rollout that uploads bundles without failing CI.

Recommended private-preview posture:

- upload the bundle artifact;
- keep `pull_request`, not `pull_request_target`, for untrusted forks;
- use `policy-profile: private-preview` until repo-specific baselines exist;
- review skipped, timed-out, and missing-tool stages as open risk.

## Rollout Sequence

1. Run Pramaan locally against one known AI-authored PR.
2. Add the Action with `fail-on: never`.
3. Review five to ten bundles and tune repository baselines.
4. Enable `fail-on: failed`.
5. Enable stricter policy packs only after reviewers understand the receipts.

Pramaan gives evidence, not automatic correctness. A green Pramaan run should
mean "no configured blocker was found," not "merge without review."
