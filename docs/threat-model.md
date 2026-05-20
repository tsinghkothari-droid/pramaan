# Threat Model

Pramaan verifies AI-authored pull requests by running tools against untrusted
repository content and then emitting evidence bundles. That makes the verifier
itself part of the attack surface.

## Assets

- Receipt JSON and bundle manifests.
- Raw tool logs, fuzz corpora, replay files, and mutation reports.
- CI identity, artifact attestations, and future signing material.
- Repository source, dependency lockfiles, fixtures, snapshots, and tests.
- Reviewer trust in the final bundle summary.

## Adversaries

- A malicious pull request author.
- A compromised or hallucinating coding agent.
- A malicious test, fixture, build script, mutation runner, fuzzer, or parser.
- A malicious or compromised Pramaan plugin.
- A compromised CI runner or poisoned dependency/cache layer.

## Current Controls

- Base and head refs are checked out into isolated Git worktrees.
- Receipts and artifacts are hash-linked in the bundle manifest.
- Bundle verification rejects missing files, path traversal, changed sizes, and
  digest mismatches.
- Sandbox evidence records dirty source state, lockfile drift, environment/tool
  versions, network policy metadata, and best-effort container identity.
- Phase 21 records whether the source checkout changed during sandbox setup.
- Redaction helpers mask common secret assignments and private user paths.
- CI hardening analysis flags `pull_request_target`, broad write permissions,
  self-hosted runners, mutable action refs, unpinned actions, and cache use.

## What These Controls Do Not Prove

- They do not make untrusted PR code safe to execute.
- They do not prove a CI runner was uncompromised.
- They do not prove a mutation engine, fuzzer, parser, or plugin is bug-free.
- They do not prove no secret was printed before redaction.
- They do not prove code correctness.

## Runner Control Boundary

If an attacker controls the CI runner, they can tamper with files before Pramaan
observes them, suppress commands, leak secrets, or forge local-only artifacts.
Sigstore, GitHub artifact attestations, and in-toto statements can improve
provenance, but they cannot rescue evidence produced on a fully compromised
runner.

## Malicious PR Code Boundary

Pramaan must assume tests, build scripts, fixtures, mutation targets, and fuzz
targets may be hostile. Risky tools should run with:

- least-privilege tokens;
- no repository write token for untrusted forks;
- no secrets for untrusted PR code;
- short timeouts;
- isolated working directories;
- cache isolation;
- future container or VM isolation for high-risk stages.

## Plugin Boundary

Plugins must not be allowed to edit prior receipts or bundle manifests directly.
Every plugin-emitted receipt should identify plugin name, version, provenance,
permissions, and sandbox boundary. A plugin can contribute evidence, but the
bundle builder remains responsible for final manifest construction and hashing.

## Redaction Boundary

Redaction is a last-mile sharing control, not a secret-protection strategy.
Secrets should not be available to untrusted PR jobs in the first place. Bundle
redaction should still remove or hash:

- tokens, passwords, API keys, and authorization headers;
- private user paths;
- internal endpoints and hostnames;
- sensitive CI metadata;
- secret-derived logs or tool outputs.

## Default CI Guidance

Use `pull_request`, not `pull_request_target`, for untrusted PRs. Keep
permissions read-only unless a later signed release workflow explicitly needs
more. Prefer pinned actions, avoid mutable branch refs, quarantine caches, and
treat self-hosted runners as privileged infrastructure requiring separate
isolation.
