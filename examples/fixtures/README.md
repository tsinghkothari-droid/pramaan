# Pramaan Synthetic Fixtures

These fixtures exist to validate the Phase 1 evidence contracts before the Rust harness is available.

They intentionally describe what Pramaan checked and what remains risky. They do not claim that any code change is correct, and they do not collapse risk into a single score.

## Files

- `claim_scope.synthetic.json` models the behavior a PR claims to change, out-of-scope behavior, touched public APIs, source refs, extraction method, and confidence.
- `receipt.synthetic.json` models a stage receipt with normalized status, timing, tool identity, artifacts, limitations, and mitigated/residual/not-applicable risk IDs.
- `risk_taxonomy.synthetic.json` is a small schema-validation subset drawn from the top-100 flaw register in `.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md`.
- `bundle.synthetic.json` models a proof-bundle manifest that references receipts and artifacts by digest and reserves space for later signing or attestation metadata.
