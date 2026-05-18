# Research Summary: Pramaan

## Stack

Build a Rust core with a plugin protocol, Python language plugins for v1 verification logic, and a TypeScript GitHub Action wrapper. Use JSON Schema-backed receipts and a bundle manifest from day one. Start with worktree/container evidence before attempting full Nix-style reproducibility.

## Table Stakes

The v1 must have a local CLI, GitHub Action, structured receipts, sandbox evidence, static checks, oracle integrity, diff-scoped mutation, differential property/fuzz checks, and signed/signable bundles.

## Watch Out For

- Do not claim code correctness.
- Do not claim fully independent stage probabilities.
- Do not let critic output override execution failures.
- Do not let mutation testing become too slow to run.
- Do not expand language support before receipt and plugin contracts stabilize.

## Build Bias

The fastest credible route is a demo-first CLI that catches weakened tests in a small Python or TypeScript repo, then hardens receipts and expands into mutation/property checks.
