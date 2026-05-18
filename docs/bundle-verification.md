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

The verifier loads `bundle.manifest.json`, validates the v1 manifest shape, recomputes the manifest digest, then recomputes every referenced receipt and artifact hash. Verification fails if the manifest is malformed, a referenced receipt no longer parses as a Pramaan receipt, a referenced file is missing, a file size changes, or a SHA-256 digest differs.

You can also point directly at the manifest file:

```powershell
cargo run -p pramaan-cli -- bundle verify target/pramaan/bundle.manifest.json
```

Tamper checks are intentionally local and deterministic. If a receipt or artifact is edited after manifest emission, `pramaan bundle verify` exits non-zero and reports the mismatched path.
