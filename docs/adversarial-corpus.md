# Adversarial Corpus

Pramaan keeps public adversarial scenarios in `corpus/` so demos and future
evaluations reuse the same failure-mode IDs, risk mappings, and reviewer
expectations.

Phase 33 promotes the main corpus manifest to:

```text
corpus/adversarial-scenarios-v0.1.json
```

The older `corpus/starter-adversarial-scenarios.json` remains as the initial
public-demo seed, but new eval work should target the v0.1 manifest.

## Validation

Run the corpus validator:

```powershell
node scripts/check-adversarial-corpus.mjs
```

Inspect one scenario:

```powershell
node scripts/check-adversarial-corpus.mjs --scenario ADV-010
```

The validator fails duplicate IDs, missing reviewer explanations, missing replay
commands, malformed risk IDs, fewer than 25 scenarios, missing secure-code
categories, or missing verifier/CI-abuse coverage.

## Phase 33 Coverage

| Requirement | Status |
| --- | --- |
| 25+ scenarios | Met: `ADV-001` through `ADV-025` |
| Secure-code validation removal | `ADV-007` |
| Authorization weakening | `ADV-008` |
| Unsafe deserialization | `ADV-009` |
| Injection sanitization removal | `ADV-010` |
| Crypto misuse | `ADV-011` |
| Secret exposure | `ADV-012` |
| Malicious verifier or CI abuse | `ADV-013`, `ADV-014`, `ADV-022` |
| Plugin poisoning | `ADV-015` |
| Benchmark overfitting | `ADV-023` |
| Reviewer override / calibration gap | `ADV-025` |

## Implemented Demo Scenarios

`ADV-001` is implemented in `examples/vulnerable-python-pr/`.

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

`ADV-003` is implemented in `examples/hallucinated-rust-pr/`.

```powershell
cargo run -p pramaan-cli -- static-checks --repo examples/hallucinated-rust-pr --out target/pramaan-demo/hallucinated-rust
```

`ADV-006` is implemented in `examples/snapshot-fixture-drift-pr/`.

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/snapshot-fixture-drift-pr/base --head-repo examples/snapshot-fixture-drift-pr/head --out target/pramaan-demo/snapshot-fixture-drift
```

`ADV-015` is an implemented malicious-plugin receipt fixture under
`corpus/plugin-security/`.

## Corpus Rules

- Keep scenario IDs stable after publication.
- Add executable fixture paths when a scenario spec becomes runnable.
- Keep ordinary-CI expectations separate from Pramaan expected findings.
- Keep risk mappings conservative and grounded in the risk register.
- Do not remove old scenarios when detection improves; mark them as
  implemented, superseded, or retained for regression coverage.
- Metadata-only replay commands are not proof of detection. They exist so a
  reviewer can inspect the scenario until an executable fixture lands.
