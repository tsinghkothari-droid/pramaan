# Phase 1 Research: Receipt-First CLI Skeleton

**Phase:** 1
**Goal:** Create the project skeleton, CLI entry point, schemas, claim-scope model, and orchestrator contract so every future stage can emit auditable receipts.

## What Matters Technically

Phase 1 is a contract phase. The product can be empty internally, but its evidence model cannot be mushy. Later stages will only be trustworthy if they all emit the same normalized receipt shape and if bundle verification can reason over those receipts.

## Architecture Guidance

### Workspace

Use a Rust workspace:

```text
crates/
  pramaan-cli/
  pramaan-core/
  pramaan-sandbox/
  pramaan-bundle/
plugins/
  python/
  typescript/
  rust/
schemas/
  receipt.schema.json
  claim_scope.schema.json
  bundle.schema.json
examples/
  fixtures/
```

### Core Types

`pramaan-core` should own:

- `StageStatus`
- `StageReceipt`
- `ClaimScope`
- `ToolIdentity`
- `InputDigest`
- `ArtifactRef`
- `StageSummary`
- `Limitation`

`pramaan-cli` should own:

- argument parsing;
- output directory selection;
- command dispatch;
- terminal summary rendering.

`pramaan-bundle` should own:

- manifest model;
- content hashing;
- bundle verification API skeleton.

`pramaan-sandbox` should only be a placeholder in Phase 1 unless needed for type boundaries.

## Validation Architecture

Phase 1 validation should be quick and local:

- `cargo fmt --check`
- `cargo test`
- CLI smoke test that runs `pramaan verify --base HEAD --head HEAD --out target/pramaan-smoke`
- JSON fixtures validate against committed schemas
- Generated synthetic receipt contains a claim-scope reference

## Risks

- If schema files are too vague, every later phase will interpret receipts differently.
- If claim scope is deferred, Phase 3 will bolt it on awkwardly and oracle mismatch risk will be under-modeled.
- If the CLI tries to run real tools too soon, Phase 1 will sprawl into Phase 2/3.

## Planning Recommendation

Split Phase 1 into three plans:

1. Schema and contract files.
2. Rust workspace/CLI/orchestrator skeleton.
3. Test fixtures, smoke tests, and documentation.
