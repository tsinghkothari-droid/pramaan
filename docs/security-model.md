# Security Model

Pramaan protects the integrity of evidence bundles. It does not make untrusted
pull request code safe to execute by itself.

## Trust Boundaries

| Boundary | Current posture | Residual risk |
| --- | --- | --- |
| Pull request code | Treated as untrusted input. | Strong container isolation is not enforced yet. |
| Verifier runner | Trusted to execute Pramaan honestly. | A compromised runner can forge local evidence before signing. |
| Plugin subprocesses | Restricted by receipt permissions and bundle validation. | Strong OS sandboxing for risky tools remains future work. |
| Bundle artifacts | Hash-linked and locally verifiable. | Production Sigstore identity is still planned. |
| Human reviewer | Final authority for merge decisions. | Overrides must be recorded with reason and accepted risk IDs. |

## Protected Properties

Pramaan aims to make these events visible:

- tests deleted, skipped, or weakened;
- fixtures and snapshots changed in oracle-sensitive paths;
- static or import-binding failures;
- skipped, missing, timed-out, or not-applicable verification stages;
- tampered bundle files or missing artifacts;
- dangerous plugin permissions or untrusted plugin provenance;
- unsafe GitHub workflow patterns for untrusted PR code.

## Not Protected Yet

- kernel-level containment for malicious test execution;
- production keyless Sigstore identity;
- full compiler-AST oracle extraction for every language;
- real Hypothesis/fast-check campaigns for all eligible deltas;
- hosted analytics for long-term drift and override correlation.

## Runner Guidance

- Prefer ephemeral hosted runners for untrusted PRs.
- Avoid `pull_request_target` for workflows that execute PR code.
- Do not expose deployment secrets to Pramaan jobs on forked PRs.
- Treat self-hosted runners as a separate security review.
- Keep uploaded bundles redacted before sharing outside the organization.

See [Threat Model](threat-model.md) for attacker scenarios and
[Redaction](redaction.md) for bundle sharing profiles.
