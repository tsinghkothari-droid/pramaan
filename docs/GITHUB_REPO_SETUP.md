# GitHub Repository Setup

## Intended Repository Description

Pramaan: hash-linked proof bundles and risk receipts for AI-generated code changes.

## Suggested Topics

- ai-code-review
- ai-agents
- software-testing
- mutation-testing
- fuzzing
- supply-chain-security
- sigstore
- slsa
- github-actions
- mcp

## Suggested GitHub About Text

Evidence, not vibes, for AI-generated code. Pramaan verifies PRs with hash-linked receipts, oracle-integrity checks, visible skipped stages, and residual-risk summaries. Signing, real mutation, and real fuzz/property execution are roadmap gates.

## Initial Repository Goals

- Publish the product intent clearly.
- Keep the broad product-family ideas saved but secondary.
- Make Pramaan's first build path obvious.
- Avoid claiming correctness.
- Show the top-100 risk register as a serious differentiator.

## First Public Milestone

The first public milestone should be:

> GitHub CI passes, but Pramaan catches an AI agent weakening the test.

Deliverables:

- Rust CLI skeleton.
- Receipt/claim/risk/bundle schemas.
- Oracle-integrity detector for Python assertions.
- Demo repository or fixture.
- Hash-linked proof bundle, with signing kept explicit as a follow-on gate.
- GitHub Action summary.
