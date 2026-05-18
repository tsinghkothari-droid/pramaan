# Mutation Fixtures

These small fixture projects let `pramaan mutation` exercise adapter discovery
and receipt emission without requiring mutmut, StrykerJS, or cargo-mutants to be
installed in every developer environment.

Expected fixture command:

```powershell
cargo run -p pramaan-cli -- mutation --repo examples/fixtures/mutation --changed-file python/checkout.py --changed-file typescript/src/checkout.ts --changed-file rust/src/lib.rs --out target/pramaan-mutation-fixture
```

The adapters should emit receipts for Python, TypeScript, and Rust. In a minimal
environment they may be `skipped`, but the receipts must still include mutant
count fields, timeout/filter/cache metadata, and risk IDs R-068 through R-072.
