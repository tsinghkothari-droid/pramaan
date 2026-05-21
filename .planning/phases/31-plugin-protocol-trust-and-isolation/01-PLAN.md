# Phase 31: Plugin Protocol Trust and Isolation

## Goal

Define and enforce a plugin trust model before third-party plugins can emit
receipts into Pramaan bundles.

## Research Drivers

- Plugins are a verifier supply-chain risk: a malicious plugin can poison all
  future evidence.
- Wasmtime security guidance supports treating WASM as one possible isolation
  boundary, not as magic safety.

## Tasks Covered

- Plugin protocol and least-privilege receipt-writing permissions.
- Plugin identity, version, provenance, and optional signature in receipts.
- Isolation boundary for risky parsers, test runners, mutation engines, and
  fuzzers.
- Malicious-plugin fixtures.

## Files to Change

- `docs/plugins.md`
- `docs/threat-model.md`
- `schemas/`
- `crates/pramaan-core/`
- `crates/pramaan-cli/`
- `plugins/`
- `corpus/`
- `TASKS.md`

## Implementation Steps

1. Define subprocess JSON protocol boundaries and receipt capabilities.
2. Require plugin identity metadata on every plugin receipt.
3. Prevent plugins from editing previous receipts or bundle manifests.
4. Add plugin allowlist or trust policy hooks.
5. Evaluate process, container, or WASM isolation for high-risk plugins.
6. Add malicious plugin fixtures for receipt tampering, artifact path escape,
   false pass emission, and environment leakage.

## Verification

- Plugin receipts without identity or allowed capability fail policy.
- Malicious plugin fixtures cannot tamper with prior evidence.
- Threat model documents what plugin isolation does and does not protect.

## Exit Criteria

Pramaan can accept plugin output without making plugins trusted root by
accident.
