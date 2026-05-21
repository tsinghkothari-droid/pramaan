# P0/P1 Alpha Pilot Gate - 2026-05-21

## Decision

**Private technical preview: yes. Public Alpha MVP: not yet.**

Pramaan now has enough internal fixture evidence to show the core loop:
oracle integrity, mutation receipts, and differential replay evidence. It does
not yet satisfy the public Alpha gate because three external real-repository
pilots were not run and Phase 22.5 claim-audit work is still open.

## Pilot Runs

Commands were run from the repository root on 2026-05-21.

| Run | Command | Runtime | Output |
| --- | --- | ---: | --- |
| Oracle fixture | `cargo run -p pramaan-cli -- oracle --base-repo examples\fixtures\oracle\base --head-repo examples\fixtures\oracle\head --out target\pramaan-pilot\oracle` | 831 ms | `target/pramaan-pilot/oracle` |
| Mutation fixture | `cargo run -p pramaan-cli -- mutation --repo examples\fixtures\mutation --changed-file python/checkout.py --changed-file typescript/src/checkout.ts --changed-file rust/src/lib.rs --timeout-ms 1000 --out target\pramaan-pilot\mutation` | 616 ms | `target/pramaan-pilot/mutation` |
| Python fuzz fixture | `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base --head-repo examples\fixtures\fuzz\head --claim-scope examples\fixtures\fuzz\claim_scope.json --seed 4242 --out target\pramaan-pilot\fuzz-python` | 680 ms | `target/pramaan-pilot/fuzz-python` |
| TypeScript fuzz fixture | `cargo run -p pramaan-cli -- fuzz --base-repo examples\fixtures\fuzz\base-ts --head-repo examples\fixtures\fuzz\head-ts --claim-scope examples\fixtures\fuzz\claim_scope_ts.json --seed 1337 --out target\pramaan-pilot\fuzz-typescript` | 545 ms | `target/pramaan-pilot/fuzz-typescript` |

The generated `target/` outputs are local verification artifacts and are not
committed.

## What The Pilot Proved

- Oracle integrity emits failed receipts for weakened assertions, deleted tests,
  renamed tests, added skips, removed boundary/error paths, and
  fixture/snapshot drift.
- Oracle diff artifacts now include extractor engines, evidence labels,
  assertion-signal kinds, assertion strength, signal hashes, and skip markers.
- Mutation adapters emit receipts for Python, TypeScript, and Rust scopes.
  Missing tools are visible as skipped/not-applicable risk, not as mitigation.
- Differential replay evidence records seeds, corpus hashes, generated input
  counts, replay paths, divergence classification, and adapter availability.

## What The Pilot Did Not Prove

- It did not run on three external real repositories.
- It did not prove full compiler AST coverage for Python, TypeScript, or Rust.
  Current oracle extractors are structured block parsers with honest evidence
  labels.
- It did not run real Hypothesis or fast-check campaigns. The fuzz stage records
  `tool_backed=false` for deterministic replay evidence.
- It did not prove Sigstore/in-toto production signing.
- It did not prove enterprise-safe redaction for arbitrary private CI logs.

## Top Residual Risks

| Risk | Status |
| --- | --- |
| External-repo signal/noise unknown | Must run 3 real repositories before public Alpha. |
| Full AST parser coverage incomplete | Add parser-backed integrations only with dependency justification and golden fixtures. |
| Mutation tools often missing locally | Action docs should install tools or mark skipped stages as warning/fail by policy. |
| Property/fuzz stage not tool-backed yet | Hypothesis/fast-check generated harnesses need a safe sandbox and replay contract. |
| Claim-audit gate open | Phase 22.5 should run before any Alpha release copy. |

## Next Decision

Proceed to **private technical preview** and use the first external pilots to
decide the public Alpha date. Do not start P2 dashboard or broad adapter work
until the external pilot report exists.
