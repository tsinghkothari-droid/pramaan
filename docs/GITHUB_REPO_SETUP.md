# GitHub Repository Setup

## Intended Repository Description

Pramaan: signed proof bundles and risk receipts for AI-generated code changes.

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

Evidence, not vibes, for AI-generated code. Pramaan verifies PRs with signed receipts, oracle-integrity checks, mutation/fuzz evidence, and residual-risk summaries.

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
- Signed or signable proof bundle.
- GitHub Action summary.
