# Stack Research: Pramaan

## Recommended v1 Stack

### Core

- Rust workspace for `pramaan-cli`, `pramaan-core`, `pramaan-sandbox`, and `pramaan-bundle`.
- `serde`, `serde_json`, and `schemars` for receipt/bundle serialization and schema generation.
- `clap` for CLI parsing.
- `tracing` and `tracing-subscriber` for structured logs.
- `camino` for UTF-8 path handling across Windows/Linux CI.
- `sha2` or `blake3` for deterministic artifact hashing.

### Sandbox and Execution

- Start with hermetic worktrees and container image digest capture.
- Support pinned Docker/OCI images for GitHub Action use.
- Keep Nix as an advanced future path, not a hard v1 dependency.

### Python Plugin

- Static checks: `python -m compileall`, `ruff`, `mypy` when configured.
- Oracle integrity: AST-based test comparison using Python `ast`, plus pytest marker/assertion heuristics.
- Mutation: `mutmut` or `cosmic-ray`, narrowed to changed files.
- Property testing: Hypothesis for generated inputs and regression corpora.

### TypeScript Plugin

- Static checks: `tsc --noEmit`, package-manager-aware script detection, ESLint when configured.
- Oracle integrity: AST parsing via TypeScript compiler API or Babel parser.
- Mutation: StrykerJS scoped to changed files.
- Property testing: `fast-check`.

### Rust Plugin

- Static checks: `cargo check`, `cargo test --no-run`, `clippy` when configured.
- Oracle integrity: compare changed Rust test modules and snapshot fixtures.
- Mutation: `cargo-mutants`.
- Property testing: `proptest` or existing test harness detection.

### GitHub Action

- TypeScript action wrapping CLI installation/execution.
- Upload bundle artifact.
- Post concise PR summary with links to receipts and failed stages.

## What Not To Use First

- Do not make the dashboard the product before the CLI/bundle are trusted.
- Do not make formal verification required in v1.
- Do not rely on an LLM critic for pass/fail.
- Do not overfit the architecture to one language's test framework.

## Confidence

- Rust core plus plugin model: high.
- Python/TS/Rust initial tooling: high.
- Fully hermetic reproducibility across arbitrary repos: medium.
- Fast mutation testing on all real PRs: medium.
