# Receipt Model

Pramaan's Phase 1 model is deliberately modest: it records evidence about a
verification run. It does not certify that the changed code is correct.

## Terms

`ClaimScope` is the bounded statement of what the change appears to claim. In
Phase 1 synthetic runs it is generated from `--base` and `--head`; later stages
can replace that with PR title, PR body, issue links, and reviewer notes.

`Receipt` is a stage-level record. A receipt says which stage ran, what status it
reported, which inputs and outputs it referenced, which artifacts it produced,
and which risk IDs are mitigated, residual, or not applicable for that stage.

`ArtifactRef` and `OutputRef` are pointers to evidence. They should be reviewable
paths plus digests where the producing stage can calculate them. A path without a
digest is still useful during Phase 1, but it is weaker evidence.

`BundleManifest` is the run-level index. It should reference receipts and
artifacts so a reviewer or GitHub Action can inspect the run without reading
every file first.

## How They Relate

The claim scope sets the target for later checks. Stage receipts then report
evidence against that target. The bundle manifest gathers those receipts into
one auditable directory.

```text
claim scope
  -> stage receipts
  -> evidence artifacts
  -> bundle manifest
  -> reviewer summary
```

The direction matters. Pramaan should not start with a verdict and then search
for supporting evidence. It should record what each stage actually observed and
leave residual risk visible.

## Phase 1 Synthetic Receipts

The current CLI command is:

```powershell
cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke
```

This writes a synthetic claim scope and two receipts. Those receipts exercise the
contract for status, artifact paths, and risk references. They do not run static
analysis, mutation testing, differential fuzzing, or sandbox replay.

## Claim Discipline

Receipts should use precise language:

- Say "this stage produced evidence" rather than "the change is safe".
- Say "residual risk remains" when a check did not run or could not cover a
  behavior.
- Say "not applicable" only when a risk family genuinely does not apply to the
  stage.
- Keep skipped and failed stages in the bundle so reviewers can audit absence as
  well as success.
