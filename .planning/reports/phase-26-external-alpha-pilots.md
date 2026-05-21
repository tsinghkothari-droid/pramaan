# Phase 26 External Alpha Pilot Report

Date: 2026-05-21

## Decision

Public Alpha remains **no-go**. Pramaan now has three external local pilot runs,
but the required live GitHub Action proof on a real PR has not been executed in
this environment. That live proof is split into Phase 26.1 so later hardening
work can continue without claiming public Alpha readiness.

## Pilot Command

```powershell
cargo build -p pramaan-cli
powershell -ExecutionPolicy Bypass -File scripts/run-phase26-pilots.ps1
```

Generated local artifacts:

```text
target/pramaan-phase26-pilots/pilot-results.json
target/pramaan-phase26-pilots/bundles/
```

`target/` outputs are not committed. This report records the durable evidence
needed to rerun the pilot.

## External Repositories

| Repo | Language | Base | Head | Changed files | Runtime profile |
| --- | --- | --- | --- | --- | --- |
| `https://github.com/pypa/packaging.git` | Python | `ca5d8fcef371c203eb68e098ed314f1d516ae69b` | `28e1791b5ece2e998329a78a300b9cbe8549c3ca` | `.github/workflows/test.yml`, `noxfile.py`, `pyproject.toml` | verify 1761 ms, static 8634 ms, oracle 1240 ms, fuzz 872 ms, mutation 46 ms |
| `https://github.com/sindresorhus/is.git` | TypeScript | `a4393055546c2d90aa386dd6f6f9cde80b9e6338` | `7821031c66cdeb7256a0feb2d506535f9e84fcaf` | `source/index.ts`, `source/utilities.ts` | verify 2830 ms, static 1463 ms, oracle 36 ms, fuzz 521 ms, mutation 64 ms |
| `https://github.com/dtolnay/itoa.git` | Rust | `73a7c03e23852fd51f9eb1ff6caa44bdb956dbed` | `af77385d0daf4d0e949e81f2588be2e44f69f086` | `Cargo.toml`, `src/lib.rs` | verify 1723 ms, static 1518 ms, oracle 24 ms, fuzz 121 ms, mutation 321 ms |

## Observed Signal

| Repo | Useful evidence | Residual risk / noise |
| --- | --- | --- |
| Python packaging | Static Python receipts ran, oracle found test-oracle-sensitive changes, mutation/fuzz missing-tool or not-applicable states were visible. | Oracle failed on config/workflow/test-harness churn; this is useful review signal but likely noisy without claim-scope and parser depth. `mutmut`, `mypy`, and pyright-style evidence were missing/skipped. |
| TypeScript is | Oracle passed on source-only changes; TypeScript changed files were detected; skipped StrykerJS and tsc evidence remained visible instead of hidden. | Dependency installation was not performed, so TypeScript static/mutation evidence is mostly skipped/not-applicable. |
| Rust itoa | Oracle passed; Rust static receipts ran and surfaced failed cargo checks under the pilot environment. | Cargo failures need normal project setup context before being treated as product findings; `cargo-mutants` was unavailable and correctly emitted skipped mutation evidence. |

## False Positive / False Negative Notes

- Likely false positive / noisy signal: Python packaging oracle failure from
  workflow and test-harness configuration changes. This should become
  claim-scope-aware rather than automatically marketed as a bug.
- Likely false negative risk: TypeScript and Rust pilots skipped mutation tools,
  so mutation confidence remains unknown.
- Likely false negative risk: deterministic fuzz/property evidence found no
  eligible safe functions in these external diffs, so Phase 28 real harness work
  remains important.
- Environment sensitivity: running without each repo's full dependency install
  makes static outcomes less comparable to ordinary CI.

## Reviewer Time-To-Understand

Manual inspection time was estimated at 45-90 seconds per repo because the
current evidence is spread across separate stage output folders. Phase 35.5
should reduce this with a local HTML/Markdown report.

## Live GitHub Action Proof

Not executed. Required evidence for public Alpha remains:

- a PR or PR-like branch in a public repository;
- Pramaan composite Action run from GitHub Actions;
- uploaded proof bundle artifact;
- rendered job summary;
- captured log or screenshot;
- final public Alpha decision update.

## Alpha Decision

Measured external local pilots are now present. Public Alpha still remains
blocked by live Action proof, parser-backed oracle hardening, real
Hypothesis/fast-check harnesses, production signing/attestation, and redaction
profile tests.
