# Changelog

All notable changes to Pramaan are recorded here. The project follows an
evidence-first release style: entries should describe what is implemented,
what remains residual risk, and whether receipt or bundle schemas changed.

## Unreleased

### Added

- Phase 36.5 repo-health pass with clearer CLI help text, a top-level
  `pramaan export redacted` alias, quieter `verify` orchestration, `doctor`
  warning categories, release hygiene docs, and a real evidence-style README
  visual.
- Release workflow scaffold for future tagged binaries with build provenance
  attestation.

### Changed

- Static hallucination classification now prefers structured diagnostic codes
  from ruff/mypy/cargo/tsc-shaped output and keeps text matching as fallback.
- Rust static checks use a shared Pramaan cargo target cache when
  `CARGO_TARGET_DIR` is not already set.

### Still Residual Risk

- Production Sigstore/cosign identity verification remains future work.
- Full compiler-AST oracle extraction remains a planned follow-up.
- Release binaries are scaffolded but not published from this local run.

## v0.1.1 - 2026-05-21

### Added

- Runtime `pramaan doctor` and `.pramaan.toml` loading.
- `pramaan bundle cosign-plan` readiness evidence.
- Public-review readiness docs and local reviewer reports.

### Still Residual Risk

- Cosign readiness is not production OIDC identity proof.
- Config loading is a private-preview subset.

## v0.1.0 - 2026-05-18

### Added

- Receipt-first Rust CLI foundation.
- Bundle manifest and hash-integrity verification.
- Sandbox/environment evidence.
- Static checks, oracle integrity fixtures, mutation wrappers, differential
  fuzz/property evidence, and GitHub Action wrapper.

### Still Residual Risk

- Early release focused on evidence contracts and demos, not production-grade
  sandboxing, signing, or calibrated confidence.
