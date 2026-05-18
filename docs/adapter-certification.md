# Adapter Certification

Pramaan Adapter Certification is a bounded Pramaan mode for evaluating MCP servers, agent tools, and other adapters through typed, auditable evidence.

The goal is not to publish a registry, rank vendors, or create a workflow DSL. The goal is to produce a proof bundle that helps a reviewer answer: what adapter was tested, what contracts were exercised, which permissions and side effects were observed, and which risks remain.

## Scope

Adapter certification extends Pramaan's evidence model from code changes to tool behavior. A certification run should produce receipts for the adapter surface and for representative calls against that surface.

In scope:

- adapter identity, version, protocol, and declared capabilities;
- typed input and output contract review;
- permission, auth, and secret-handling boundaries;
- replayable contract fixtures for safe calls;
- idempotency, retry, timeout, and rate-limit semantics;
- auditability of side effects and returned evidence;
- residual risk mapping with stable `A-###` adapter risk IDs.

Out of scope for this phase:

- a public certified adapter registry;
- Sutra or any orchestration DSL;
- vendor scoring or marketplace ranking;
- guarantees that an adapter is safe in every deployment;
- live credentialed testing against third-party services without explicit authorization.

## Certification Checks

The starter mode should separate declarations from observations.

### Surface Declaration

Record the adapter name, package/source, protocol version, exposed tools, input schemas, output schemas, required environment variables, declared scopes, and side-effect classes.

Evidence should include paths or digests for manifests, schemas, README excerpts, and generated tool listings.

### Type And Contract Checks

Validate that every callable tool has a machine-readable input contract and a reviewable output contract. Optional fields, defaults, enums, unions, and error shapes should be explicit enough for an agent to call the tool without guessing.

### Permission Boundary Checks

Map each tool to its least required permission. A read-only tool should not require write scopes. A local-filesystem tool should declare which roots it can touch. A networked tool should declare which hosts, auth tokens, and rate limits apply.

### Replay And Fixture Checks

Run safe synthetic calls where possible. The receipt should record inputs, normalized outputs, artifacts, timing, and whether the call was hermetic, mocked, or live. Live calls must identify authorization and redact secrets.

### Side-Effect And Idempotency Checks

Classify each tool as read-only, write-once, update, delete, external-send, or privileged. For tools with side effects, the adapter should document dry-run support, idempotency keys, rollback behavior, or human confirmation requirements.

### Error And Degradation Checks

Record how the adapter behaves for invalid input, expired auth, rate limits, timeouts, missing resources, and upstream 5xx responses. A certified adapter should fail closed and return structured errors that an agent can interpret.

## Receipt Discipline

Adapter certification receipts should use precise claims:

- Say "this adapter exposes typed contracts for these tools" instead of "this adapter is safe".
- Say "this fixture replayed without side effects" instead of "this tool is harmless".
- Say "authorization was not tested" when credentialed behavior was skipped.
- Keep failed, skipped, and timed-out checks in the bundle.
- Map every covered or residual adapter issue to `A-###` IDs from the adapter risk register.

## Relationship To Pramaan

Pramaan remains the current product focus. Adapter certification is the first adjacent expansion because it reuses the same trust primitives: claim scope, typed receipts, evidence artifacts, risk IDs, bundle manifests, and audit-friendly summaries.

Registry and Sutra ideas remain deferred product-family notes. They may depend on adapter certification later, but this phase does not build them.
