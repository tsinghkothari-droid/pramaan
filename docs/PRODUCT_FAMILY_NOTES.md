# Product Family Notes

These notes preserve related product ideas while keeping Pramaan as the current build focus.

## Family Thesis

Everything an agent does should be discoverable, typed, verifiable, and replayable.

The broader family could eventually look like:

```text
Sutra                 orchestration DSL
Pramaan               verification and proof bundles
Certified Registry    trusted adapters and MCP servers
MCP / Skills          protocol and capability layer
Agent-Safe TS/Python  coding constraint set
```

## Pramaan's Role

Pramaan is the trust layer. It should answer:

> What evidence proves this agent-authored change or adapter behavior, and what risks remain?

## Ideas To Save For Later

### Certified Agent Registry

An open registry for MCP servers and agent tools where adapters are ranked and published with Pramaan certification.

Potential checks:

- tool count and naming quality;
- typed inputs and outputs;
- permission-scope templates;
- rate-limit and retry semantics;
- idempotency metadata;
- OAuth boundary tests;
- replayable contract tests;
- per-call audit receipts.

### Sutra

A small declarative orchestration DSL for typed, replayable agent workflows.

Do not build this before Pramaan works. Sutra becomes valuable only after certified tools and proof bundles exist.

### Agent-Safe Python/TypeScript

A linter and constraint set for code that agents can safely modify:

- strict types;
- no dynamic imports;
- no uncontrolled monkey-patching;
- effect annotations;
- small files/functions;
- machine-readable signatures;
- pre/postcondition contracts.

## Current Decision

Keep these ideas in this repo as strategic context, but build Pramaan first.

The first adjacent expansion worth adding later is **Pramaan Adapter Certification**, because it naturally extends the proof-bundle model from code PRs to MCP servers and agent tools.
