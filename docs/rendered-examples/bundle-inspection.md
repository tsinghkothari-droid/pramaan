# Bundle Inspection Example

```text
target/pramaan/
  bundle.manifest.json
  receipts/
    claim_scope.json
    sandbox_setup.json
    static_checks.json
    oracle_integrity.json
    differential_fuzz.json
  artifacts/
    oracle-diff.json
    fuzz-cases.json
  confidence.json
  confidence.md
  attestations/
    bundle.vsa.json
    bundle.in-toto.json
  github-step-summary.md
```

Recommended inspection order:

1. `bundle.manifest.json`
2. `github-step-summary.md`
3. failed or skipped stage receipts
4. `confidence.md`
5. replay or override evidence when present
