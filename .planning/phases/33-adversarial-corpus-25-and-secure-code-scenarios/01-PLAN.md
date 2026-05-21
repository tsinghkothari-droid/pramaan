# Phase 33: Adversarial Corpus 25 and Secure-Code Scenarios

## Goal

Expand the adversarial corpus to 25 high-signal scenarios before making larger
eval claims.

## Research Drivers

- AI-generated code vulnerability studies show functionally plausible code can
  still introduce authorization, injection, deserialization, crypto, and secret
  exposure risks.
- Benchmark contamination work makes eval integrity a product concern.

## Tasks Covered

- Corpus expansion to 25 scenarios.
- Secure-code scenarios for validation, authorization, deserialization,
  injection, crypto misuse, and secret exposure.
- Malicious verifier and malicious CI scenarios.

## Files to Change

- `corpus/`
- `examples/`
- `docs/adversarial-corpus.md`
- `docs/risk-taxonomy.md`
- `TASKS.md`
- `.planning/reports/`

## Implementation Steps

1. Inventory existing corpus scenarios and risk IDs.
2. Add scenario templates with base, head, expected ordinary-CI result, expected
   Pramaan finding, and reviewer explanation.
3. Prioritize high-signal scenarios over count padding.
4. Add secure-code and verifier-abuse fixtures.
5. Add a corpus runner or report script that summarizes pass/fail coverage.

## Verification

- Corpus report lists at least 25 scenarios mapped to risk IDs.
- Each scenario has expected finding metadata and a replay command.
- Broken or duplicate scenarios fail corpus validation.

## Exit Criteria

Pramaan has enough adversarial evidence to guide Real MVP work without claiming
the full 100-scenario Serious v1 corpus.
