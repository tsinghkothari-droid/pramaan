# Bundle Verification

Pramaan writes a proof-bundle manifest at the bundle root:

```powershell
cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan
```

The manifest is `target/pramaan/bundle.manifest.json`. It records:

- stage receipts, one entry per `*.receipt.json` under `receipts/`;
- receipt and artifact content hashes using SHA-256;
- stage status, tool versions, risk IDs, fuzz seeds, and corpus hashes when a receipt exposes them;
- final bundle status derived from included stage statuses;
- local-dev signable digest metadata and no-attestation placeholders for later CI-backed signing work.

Verify a bundle with:

```powershell
cargo run -p pramaan-cli -- bundle verify target/pramaan
```

The verifier loads `bundle.manifest.json`, validates the v1 manifest shape,
recomputes the manifest digest, then recomputes every referenced receipt and
artifact hash. Verification fails if the manifest is malformed, a referenced
receipt no longer parses as a Pramaan receipt, a referenced file is missing, a
file size changes, or a SHA-256 digest differs.

You can also point directly at the manifest file:

```powershell
cargo run -p pramaan-cli -- bundle verify target/pramaan/bundle.manifest.json
```

Tamper checks are intentionally local and deterministic. If a receipt, artifact,
or signing metadata field is edited after manifest emission,
`pramaan bundle verify` exits non-zero and reports the mismatched path or digest.

Explain the default policy decision with:

```powershell
cargo run -p pramaan-cli -- policy explain target/pramaan
```

`policy explain` reads the bundle manifest and reports:

- the default policy ID;
- required stages;
- hard gate statuses;
- warning statuses;
- SLA classes;
- hard failures, warnings, and waivers.

The default policy is intentionally conservative. Failed, errored, or timed-out
stages are hard failures. Missing or skipped required stages are hard failures.
Skipped and not-applicable non-required stages are warnings. Residual risk,
not-applicable risk, and partial evidence are warnings unless a later policy
profile elevates them.

## Path Policy

Bundle manifest paths must stay inside the bundle root. The verifier rejects
absolute paths and parent traversal such as `../outside.json` in manifest
references. During manifest construction, receipt-declared file artifacts must
exist; missing artifacts are not silently dropped. If a receipt names only a
basename and multiple files in the bundle share that basename, manifest
construction fails as ambiguous.

Directory artifacts such as `inode/directory` placeholders can remain in the
stage receipt, but they are not added as file artifacts to the manifest.

## What Verification Does Not Prove

Local bundle verification proves local self-consistency: manifest digest,
receipt parsing, file size, and SHA-256 hashes. It does not prove signer identity
or CI provenance by itself. Sigstore, GitHub artifact attestations, and stronger
signing verification remain separate hardening paths.
