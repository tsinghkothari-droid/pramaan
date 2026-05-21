# Release Packaging

Pramaan has no published binary release yet. This checklist stages what a
manual `v0.1.0` release should do without claiming that the release exists.

## Release Gates

Run from a clean checkout:

```powershell
cargo fmt --check
cargo test --workspace
cargo clippy --workspace -- -D warnings
node scripts/check-claim-audit.mjs
node --test action/render-summary.test.mjs
node scripts/check-phase35-docs.mjs
```

The release should not proceed if `STATUS.md` or README claims are ahead of the
code.

## Artifacts

Target artifacts for the first binary release:

| Artifact | Build environment | Command |
| --- | --- | --- |
| `pramaan-linux-x86_64.tar.gz` | Linux x86_64 | `cargo build --locked --release -p pramaan-cli --target x86_64-unknown-linux-gnu` |
| `pramaan-linux-aarch64.tar.gz` | Linux aarch64 runner or reviewed cross build | `cargo build --locked --release -p pramaan-cli --target aarch64-unknown-linux-gnu` |
| `pramaan-macos-arm64.tar.gz` | macOS arm64 | `cargo build --locked --release -p pramaan-cli --target aarch64-apple-darwin` |

Package each artifact with:

- `pramaan` binary;
- `LICENSE`;
- `README.md`;
- `STATUS.md`;
- `schemas/`;
- `docs/operator-guide.md`;
- `docs/security-model.md`.

Generate checksums:

```bash
shasum -a 256 pramaan-*.tar.gz > SHA256SUMS
```

## Tagging

```bash
git tag -s v0.1.0 -m "Pramaan v0.1.0"
git push origin v0.1.0
```

If a signing key is not available, use an unsigned annotated tag and say so in
the release notes.

## Marketplace Status

The GitHub Action is staged in `action.yml`, but it is not yet claimed as
published in the GitHub Marketplace. Before Marketplace publication:

1. complete Phase 26.1 live GitHub Action proof;
2. attach a passing real workflow URL;
3. publish a tag that the Action can reference;
4. verify the uploaded proof bundle artifact;
5. update README install instructions from local path usage to the tagged
   action reference.
