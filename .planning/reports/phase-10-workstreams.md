# Phase 10 Workstreams

| Workstream | Scope | Verification |
| --- | --- | --- |
| Action inputs | Add `out-dir`, `upload-bundle`, and `fail-on`; keep aliases | Node action test checks YAML content |
| Deterministic CLI build | Use `cargo build --locked -p pramaan-cli` and run `target/debug/pramaan` | Node action test checks command text |
| Evidence upload ordering | Upload bundle before `fail-on` can exit non-zero | Node action test checks step ordering |
| Workflow examples | Add Python, TypeScript, Rust examples | YAML read check |
| Documentation | Update GitHub Action docs with permissions and fork behavior | Markdown link check |
