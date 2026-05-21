# Quickstart: Minimum Lovable Verifier Loop

This is the shortest public-review path for Pramaan today.

It demonstrates the product wedge:

```text
ordinary CI green -> Pramaan oracle receipt red -> bundle verifies -> reviewer can explain the blocker
```

Pramaan still produces evidence, not correctness proof.

## Install

Until the crate is published, use the repository build:

```powershell
cargo build -p pramaan-cli --release
target\release\pramaan.exe --help
```

After the first crates.io publish, the intended one-command install path is:

```powershell
cargo install pramaan-cli
```

Tagged releases are also staged to publish platform archives from
`.github/workflows/release.yml`; release binaries are not yet a production
claim until that workflow has run on a real tag.

## One Command

From the repository root:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/run-minimum-lovable-loop.ps1
```

The script writes:

```text
target/pramaan-minimum-lovable/
  bundle.manifest.json
  oracle-diff.json
  confidence.json
  confidence.md
  policy-explain.txt
  minimum-lovable-report.md
  receipts/
```

## What You Should See

The script first runs ordinary Python unit tests against the AI-style branch of
the weakened-test demo. Those tests pass.

Then Pramaan compares the base and head test oracles. The expected result is a
failed `oracle_integrity` receipt because the test stopped checking the exact
discounted total.

The reviewer-facing file is:

```text
target/pramaan-minimum-lovable/minimum-lovable-report.md
```

Read the `Blockers First` section. A reviewer should be able to say:

> CI is green because the test was weakened. Pramaan failed the oracle receipt,
> so this PR still needs human repair or explicit override.

## Inspect The Bundle

```powershell
cargo run -p pramaan-cli -- bundle verify target/pramaan-minimum-lovable
cargo run -p pramaan-cli -- policy explain target/pramaan-minimum-lovable --profile private-preview
cargo run -p pramaan-cli -- confidence explain target/pramaan-minimum-lovable
```

## Manual UAT

Ask a fresh reviewer to open only:

```text
target/pramaan-minimum-lovable/minimum-lovable-report.md
```

Pass condition:

- They can identify the blocker in under 30 seconds.
- They do not describe Pramaan as proving correctness.
- They notice that skipped or missing evidence is residual risk, not success.

## What This Does Not Claim

- It is not a production v1 release.
- It is not calibrated confidence.
- It is not production Sigstore/cosign signing.
- It is not full compiler-AST oracle extraction.
- It is not proof that every review assistant misses this case.
