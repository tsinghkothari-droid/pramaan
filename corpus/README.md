# Pramaan Adversarial Corpus

This directory contains public demo and evaluation scenarios for AI-code trust
failures that ordinary CI can miss.

The Phase 33 manifest is:

```text
corpus/adversarial-scenarios-v0.1.json
```

The Phase 26.3 competitor-gap fixture manifest is:

```text
corpus/competitor-gap-fixtures.v0.1.json
```

The original starter manifest remains for compatibility:

```text
corpus/starter-adversarial-scenarios.json
```

## Validate

```powershell
node scripts/check-adversarial-corpus.mjs
```

Validate competitor-gap fixtures:

```powershell
node scripts/check-competitor-gap-fixtures.mjs
```

The validator checks:

- at least 25 scenarios;
- unique `ADV-...` IDs;
- non-empty base/head/ordinary-CI/Pramaan/reviewer fields;
- known `R-...` risk ID shape;
- replay commands for every scenario;
- secure-code coverage for validation, authorization, deserialization,
  injection, crypto, and secrets;
- malicious verifier, malicious CI, overfitted AI, compromised plugin, and
  careless-AI adversary models.

The competitor-gap validator checks category-level examples for weakened
assertions, skipped tests, fixture drift, hallucinated APIs, false-green CI,
unsigned aggregate reports, and hidden skipped stages. Those fixtures compare
Pramaan's evidence model against adjacent tool categories, not against a named
product's latest hosted behavior.

## Scenario Status

| Status | Meaning |
| --- | --- |
| `implemented_demo` | Runnable public example exists under `examples/`. |
| `implemented_fixture` | Checked fixture exists but may not be a full repo demo. |
| `scenario_spec` | Risk-mapped scenario is defined; executable fixture remains future work. |

## Public Demo Path

Run normal CI on the implemented vulnerable PR:

```powershell
python -m unittest discover -s examples/vulnerable-python-pr/weakened-test -p "test_*.py"
```

Then run Pramaan oracle integrity:

```powershell
cargo run -p pramaan-cli -- oracle --base-repo examples/vulnerable-python-pr/base --head-repo examples/vulnerable-python-pr/weakened-test --out target/pramaan-demo/oracle
```

The inspection path for reviewers is:

```text
target/pramaan-demo/oracle/receipts/oracle-integrity.receipt.json
target/pramaan-demo/oracle/oracle-diff.json
examples/vulnerable-python-pr/risk-map.json
corpus/adversarial-scenarios-v0.1.json
```
