# Pramaan Product Intent

## One-Sentence Product

Pramaan is a signed evidence layer for AI-generated code changes.

## The User Problem

Teams are starting to merge code written by Codex, Claude Code, Cursor, Copilot, and other agents. Existing CI tells them whether tests passed, but not whether the agent cheated the test, weakened the oracle, invented APIs, broke behavior outside the issue, or left important risk untested.

Human reviewers need a fast way to answer:

- What did the agent claim to fix?
- What evidence supports that claim?
- Were tests changed in a suspicious way?
- What was actually executed?
- What risks remain?
- Can this evidence be audited later?

## The Product Answer

Pramaan creates a proof bundle for every PR.

The bundle contains:

- claim-scope receipt;
- sandbox/environment receipt;
- static/hallucination receipt;
- oracle-integrity receipt;
- mutation receipt;
- property/fuzz/differential receipt;
- bundle manifest;
- risk summary;
- signing or attestation metadata where available.

## The Core Bet

No single check is enough. The strength comes from stacking diverse, execution-grounded checks and making their evidence inspectable.

The point is not to create a magic score. The point is to give a reviewer a compact, honest risk ledger.

## Killer Demo

The first demo should show:

1. A bug exists.
2. An AI agent "fixes" it by weakening a test.
3. Normal CI passes.
4. Pramaan fails the PR and names the weakened assertion.

This demo explains the product in under one minute.

## What Makes Pramaan Different

Most AI review tools generate comments. Pramaan generates evidence.

Most scanners report findings. Pramaan also reports what was checked, what was skipped, and what remains risky.

Most CI logs are transient. Pramaan produces a signed or signable bundle intended for audit.

## Non-Goals

- Proving arbitrary code correctness.
- Becoming a generic AI reviewer.
- Hiding uncertainty behind one score.
- Relying on an LLM critic as the primary gate.
- Shipping a dashboard before the evidence model works.
