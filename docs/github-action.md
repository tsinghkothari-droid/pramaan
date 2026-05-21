# GitHub Action

Pramaan ships a composite GitHub Action that runs the CLI against pull request
base/head refs, uploads the proof bundle, and writes a risk-aware job summary.

## Live Proof Status

Local action-summary tests pass, and Phase 26 recorded three external local
pilot runs. A live GitHub Actions run on a real pull request is still required
before public Alpha. That remaining proof is tracked as Phase 26.1 and must
record the run URL, uploaded artifact, manifest digest, and rendered job
summary.

```yaml
name: Pramaan

on:
  pull_request:

permissions:
  contents: read
  actions: read
  checks: read

jobs:
  pramaan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - uses: dtolnay/rust-toolchain@stable

      - uses: ./
        with:
          base-ref: ${{ github.event.pull_request.base.sha }}
          head-ref: ${{ github.event.pull_request.head.sha }}
```

The wrapper runs:

```bash
cargo build --locked -p pramaan-cli
target/debug/pramaan verify --base "$BASE_REF" --head "$HEAD_REF" --out target/pramaan
target/debug/pramaan policy explain target/pramaan
```

It then uploads `target/pramaan` as the `pramaan-proof-bundle` artifact and
appends a summary to `GITHUB_STEP_SUMMARY`. The policy explanation is appended
to the run log before summary rendering so reviewers can see the default gate
reasoning without opening raw JSON.

## Inputs

| Input | Default | Purpose |
| --- | --- | --- |
| `base-ref` | pull request base SHA, then `HEAD~1` | Base ref passed to `pramaan verify`. |
| `head-ref` | pull request head SHA, then `GITHUB_SHA` | Head ref passed to `pramaan verify`. |
| `out-dir` | `target/pramaan` | Proof bundle output directory. |
| `bundle-path` | empty | Deprecated alias for `out-dir`. |
| `artifact-name` | `pramaan-proof-bundle` | Uploaded artifact name. |
| `upload-bundle` | `true` | Set to `false` to skip `actions/upload-artifact`. |
| `upload-artifact` | empty | Deprecated alias for `upload-bundle`. |
| `fail-on` | `failed` | `failed`, `actionable`, or `never`. |
| `attest` | `false` | Set to `true` to invoke GitHub artifact attestation. |
| `pramaan-args` | empty | Extra arguments appended to `pramaan verify`. |

`fail-on: failed` fails the job when the manifest final status is `failed`.
`fail-on: actionable` also fails on `error` or `inconclusive`. `fail-on: never`
always leaves the action green while still uploading the bundle and writing the
summary.

## Outputs

| Output | Meaning |
| --- | --- |
| `bundle-path` | Proof bundle directory. |
| `manifest-path` | Path to `bundle.manifest.json`. |
| `summary-path` | Rendered summary markdown path. |
| `final-status` | Manifest `final_status`, such as `passed`, `failed`, or `inconclusive`. |

## Minimal Permissions

For normal pull request runs with artifact upload:

```yaml
permissions:
  contents: read
  actions: read
  checks: read
```

`contents: read` lets `actions/checkout` fetch the repository. `actions: read`
and `checks: read` are enough for a read-only proof run and summary. The action
does not require `pull-requests: write` because it writes to the job summary, not
to PR comments.

For forked pull requests, keep the workflow on `pull_request` rather than
`pull_request_target` unless you have reviewed the security implications.
Pramaan runs repository code and should not receive write tokens or secrets from
untrusted forks.

## CI Hardening Checks

Phase 21 adds text-level GitHub workflow hardening checks in core so future
receipts can flag unsafe verifier environments. The current detector looks for:

- `pull_request_target` on untrusted code paths;
- `permissions: write-all`;
- `self-hosted` runners;
- `actions/cache` cache-poisoning review needs;
- actions without an `@ref`;
- actions pinned to mutable `@main` or `@master` refs.

These checks are intentionally conservative. They are not a full GitHub Actions
policy engine yet, but they make common verifier hazards visible.

## Optional Artifact Attestation

To request GitHub artifact attestation, enable the input and grant the additional
permissions required by `actions/attest-build-provenance`:

```yaml
permissions:
  contents: read
  actions: read
  checks: read
  id-token: write
  attestations: write

steps:
  - uses: ./
    with:
      attest: "true"
```

The wrapper attests `target/pramaan/bundle.manifest.json`. The uploaded proof
bundle remains the review artifact; attestation adds CI provenance for the
manifest and should be read beside the residual risk families in the summary.

## Summary Shape

The job summary highlights failed, skipped, timed-out, or errored stages first.
It also groups risk IDs by family across `mitigated`, `residual`, `skipped`, and
`not_applicable` buckets. This keeps open risk visible without turning Pramaan
into a single trust score.

The summary also shows the manifest policy decision when present. Phase 20's
default policy profile uses:

- required stages: `claim_scope`, `sandbox_setup`;
- hard statuses: `failed`, `error`, `timed_out`;
- warning statuses: `skipped`, `not_applicable`;
- SLA classes: small PRs target 4 minutes, medium PRs target 8 minutes, large
  PRs target 15 minutes, with stricter per-stage budgets recorded in receipts.

Skipped, missing-tool, not-applicable, and timed-out stages must remain visible;
they are never rewritten as successful mitigation.

## Minimal Workflow Examples

Python repositories can run their normal tests before Pramaan:

```yaml
steps:
  - uses: actions/checkout@v4
    with:
      fetch-depth: 0
  - uses: actions/setup-python@v5
    with:
      python-version: "3.12"
  - run: python -m pytest
  - uses: pramaan/pramaan@v0
    with:
      out-dir: target/pramaan
      fail-on: failed
```

TypeScript repositories should install dependencies before the action if
Pramaan stages need local toolchains:

```yaml
steps:
  - uses: actions/checkout@v4
    with:
      fetch-depth: 0
  - uses: actions/setup-node@v4
    with:
      node-version: "22"
      cache: npm
  - run: npm ci
  - run: npm test
  - uses: pramaan/pramaan@v0
    with:
      out-dir: target/pramaan
      fail-on: failed
```

Rust repositories can reuse the same toolchain for project tests and Pramaan:

```yaml
steps:
  - uses: actions/checkout@v4
    with:
      fetch-depth: 0
  - uses: dtolnay/rust-toolchain@stable
  - run: cargo test --locked
  - uses: pramaan/pramaan@v0
    with:
      out-dir: target/pramaan
      fail-on: failed
```
