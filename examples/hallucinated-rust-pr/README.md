# Static Hallucination Demo

This demo models an AI-generated patch that imports a plausible but nonexistent
helper crate. The code looks small and reasonable, but the static stage catches
the invented dependency before reviewers treat it as a real fix.

## Pramaan

```powershell
cargo run -p pramaan-cli -- static-checks --repo examples/hallucinated-rust-pr --out target/pramaan-demo/hallucinated-rust
```

Expected result: the Rust static check emits a failed receipt and classifies the
diagnostic as static hallucination evidence.

