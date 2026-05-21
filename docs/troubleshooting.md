# Troubleshooting

## Mutation Is Slow

Use mutation as an explicit stage while the project is tuning baselines:

```powershell
pramaan verify --base main --head HEAD --out target/pramaan --with-mutation
```

If runtime exceeds the documented SLA, inspect `bundle.manifest.json`
`stage_budgets` and compare with the repo baseline through
`pramaan feedback analyze`.

## Tool Is Missing

Missing mutation or fuzz tools should produce skipped receipts. That is
expected during early rollout and should remain visible to reviewers.

Install the language tool, rerun Pramaan, and compare the receipt status:

- Python mutation: `mutmut`
- TypeScript mutation: StrykerJS
- Rust mutation: `cargo-mutants`
- Python property checks: Hypothesis
- JavaScript property checks: fast-check

Do not edit receipts by hand to hide skipped stages.

## Flaky Tests

Pramaan is not a flake quarantine tool. If normal project tests are flaky,
record the flake in the bundle and keep the risk visible. A future eval phase
will add explicit flaky-case quarantine rules.

## Forked Pull Requests

For forked PRs:

- prefer `pull_request`, not `pull_request_target`;
- do not expose secrets;
- keep permissions read-only unless artifact attestation is intentionally
  enabled;
- treat self-hosted runners as high risk.

If the Action cannot upload artifacts from forks, run with `upload-bundle:
"false"` and inspect the logs until repository policy is clear.

## Bundle Verification Fails

Run:

```powershell
pramaan bundle verify target/pramaan
```

Common causes:

- an artifact listed in the manifest was deleted;
- a receipt was edited after bundle creation;
- a path escaped the bundle root;
- local attestation files no longer match the manifest digest.

For GitHub Action bundles, download the whole artifact directory before
verifying; verifying a single manifest file is not enough.

## Reviewer Sees Too Many Warnings

Start with local calibration:

```powershell
pramaan feedback analyze --bundle target/pramaan --baseline examples/fixtures/repo-baseline.synthetic.json --out target/pramaan-feedback
```

Replace the synthetic baseline with a repo-specific baseline once enough
shadow-mode bundles exist. Do not lower policy gates just to make warnings
disappear; record reviewer overrides with accepted risk IDs when a human
intentionally accepts the risk.
