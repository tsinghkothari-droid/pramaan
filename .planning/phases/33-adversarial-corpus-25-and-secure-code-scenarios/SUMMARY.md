# Phase 33 Summary: Adversarial Corpus 25 and Secure-Code Scenarios

## Status

Completed the Phase 33 corpus expansion on 2026-05-21.

## Landed

- Added `corpus/adversarial-scenarios-v0.1.json` with `ADV-001` through
  `ADV-025`.
- Added secure-code scenario categories for validation removal, authorization
  weakening, unsafe deserialization, injection sanitization removal, crypto
  misuse, and secret exposure.
- Added malicious verifier, malicious CI, compromised plugin, overfitted AI,
  and reviewer-feedback scenarios.
- Added `schemas/adversarial_corpus.schema.json`.
- Added `scripts/check-adversarial-corpus.mjs` for duplicate/stale coverage
  validation and per-scenario inspection.
- Added a Rust regression test that checks scenario count, risk-ID validity,
  secure-code coverage, adversary models, and verifier/CI-abuse coverage.
- Updated corpus docs and task status.

## Deferred Honestly

- Most new Phase 33 entries are scenario specifications, not full executable
  demo repositories.
- Phase 40 still owns 100+ scenarios, benchmark-integrity hardening, and wider
  real-world replay cases.
- Stronger sandboxing for risky parsers/test runners remains a separate
  verifier-security task.

## Verification

- `node scripts/check-adversarial-corpus.mjs` validates the corpus.
- Required Rust/Node verification was run before commit.
