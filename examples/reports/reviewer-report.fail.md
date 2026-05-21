# Pramaan Reviewer Report

Final status: **failed**

Compared refs: `main` -> `agent/weakened-test`

This report is reviewer evidence, not a proof that the code is correct.

## Blockers

- **oracle_integrity** `failed` risks: `R-014`

## Warnings

- **differential_fuzz** residual risks: `R-077`

## What Ran

| Stage | Status | Receipt |
| --- | --- | --- |
| `claim_scope` | `passed` | `receipts/claim-scope.receipt.json` |
| `oracle_integrity` | `failed` | `receipts/oracle-integrity.receipt.json` |

## What Skipped

- **mutation_python_mutmut** `skipped` open risks: `R-068`

## What Changed In Tests

- **oracle_integrity** `failed`: Weakened assertion detected.

## Replay Commands

- `pramaan replay target/pramaan --case <case-id>`

## Human Override

| Field | Value |
| --- | --- |
| Accepted risk IDs |  |
| Reason |  |
| Reviewer identity source |  |
| Timestamp |  |
