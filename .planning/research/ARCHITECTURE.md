# Architecture Research: Pramaan

## Major Components

### CLI

Entry point for local and CI usage.

```text
pramaan verify --base main --head HEAD
```

Responsibilities:

- Parse repo/base/head options.
- Discover changed files.
- Load config.
- Invoke orchestrator.
- Print concise result.
- Write bundle path.

### Core Orchestrator

Coordinates stages and enforces stage contracts.

Responsibilities:

- Build execution plan from repo/language detection.
- Run stages with timeouts.
- Normalize pass/fail/skip/not-applicable states.
- Ensure every stage writes a receipt.
- Aggregate receipts into bundle manifest.

### Sandbox

Creates isolated worktrees and records environment evidence.

Responsibilities:

- Check out base/head.
- Capture commit SHAs, lockfile hashes, dependency files, container digest.
- Record tool versions.
- Prepare pre-patch and post-patch runners for differential checks.

### Plugin Host

Runs language plugins through a stable protocol.

Responsibilities:

- Pass changed files and stage context to plugins.
- Collect plugin receipts.
- Keep plugin failures isolated.
- Support Python plugins initially, with future Rust/native plugins if needed.

### Bundle Signer

Creates manifest and signature.

Responsibilities:

- Hash receipts/artifacts.
- Emit manifest.
- Support local dev signature first.
- Leave room for Sigstore keyless/OIDC in CI.

### GitHub Action

CI wrapper and user-facing PR integration.

Responsibilities:

- Install/cache CLI.
- Run verification.
- Upload artifacts.
- Render summary comment/check annotation.

## Data Flow

```text
Repository + PR refs
  -> sandbox checkout
  -> diff/language detection
  -> stage execution
  -> receipts
  -> bundle manifest
  -> signature
  -> CI artifact + PR summary
```

## Suggested Build Order

1. Schemas and receipt writer.
2. CLI skeleton and orchestrator.
3. Sandbox/worktree evidence.
4. Static plugin slice.
5. Oracle integrity plugin slice.
6. Mutation plugin slice.
7. Differential property/fuzz slice.
8. Bundle signing and verification.
9. GitHub Action.
10. Demo repos and adversarial corpus.
