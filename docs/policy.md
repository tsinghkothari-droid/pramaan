# Policy and SARIF Export

Pramaan keeps its default policy in Rust for v0.1, but Phase 32 adds portable
exports so security teams can inspect the same evidence in familiar systems.
Phase 32.5 adds named built-in policy packs; see
[Policy Packs](policy-packs.md) for adoption guidance.

## SARIF

Export bundle findings to SARIF:

```powershell
cargo run -p pramaan-cli -- export sarif target/pramaan --out target/pramaan/pramaan.sarif.json
```

The SARIF file maps residual and skipped/not-applicable risk IDs to rules and
results. Locations point to the stage receipt that produced the evidence. This
is designed for GitHub code scanning ingestion, but it remains an evidence
surface: a SARIF warning is not a proof that the code is wrong, and a clean
SARIF file is not a proof that the code is correct.

## Rego

Export a starter Rego policy:

```powershell
cargo run -p pramaan-cli -- export rego --out target/pramaan/pramaan-default.rego
```

The Rego policy mirrors the conservative shape of the Rust default policy:
hard-fail failed, errored, timed-out, or missing required stages; warn when
residual risk remains. It is meant for review and parity tests before teams
wire Pramaan into OPA or Conftest.

## Agentic Workflow Injection

Pramaan also scans untrusted PR title/body and issue text for agentic workflow
injection patterns, including:

- requests to ignore governing instructions;
- shell/network execution snippets such as `curl ... | sh`;
- references to CI tokens or GitHub environment mutation;
- privileged workflow terms such as `pull_request_target`.

Those signals map to risk `R-093` and are recorded as claim-scope limitations.
They are warnings for human review, not an LLM judge.
