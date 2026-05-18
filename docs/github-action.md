# GitHub Action

Pramaan ships a composite GitHub Action that runs the CLI against pull request
base/head refs, uploads the proof bundle, and writes a risk-aware job summary.

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
cargo run -p pramaan-cli -- verify --base "$BASE_REF" --head "$HEAD_REF" --out target/pramaan
```

It then uploads `target/pramaan` as the `pramaan-proof-bundle` artifact and
appends a summary to `GITHUB_STEP_SUMMARY`.

## Inputs

| Input | Default | Purpose |
| --- | --- | --- |
| `base-ref` | pull request base SHA, then `HEAD~1` | Base ref passed to `pramaan verify`. |
| `head-ref` | pull request head SHA, then `GITHUB_SHA` | Head ref passed to `pramaan verify`. |
| `bundle-path` | `target/pramaan` | Proof bundle output directory. |
| `artifact-name` | `pramaan-proof-bundle` | Uploaded artifact name. |
| `upload-artifact` | `true` | Set to `false` to skip `actions/upload-artifact`. |
| `attest` | `false` | Set to `true` to invoke GitHub artifact attestation. |
| `pramaan-args` | empty | Extra arguments appended to `pramaan verify`. |

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
