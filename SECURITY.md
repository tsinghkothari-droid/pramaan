# Security Policy

Pramaan is security-sensitive verification infrastructure. Please report issues
privately first when they could let an attacker forge, suppress, or tamper with
evidence.

## Supported Versions

| Version | Supported |
| --- | --- |
| `main` | Best effort |
| `v0.1.x` | Best effort private-preview support |

No production support SLA exists yet.

## Reporting A Vulnerability

Use GitHub private vulnerability reporting if available for this repository, or
open a minimal public issue that says a private security report is needed
without disclosing exploit details.

Useful reports include:

- a short impact summary;
- affected command or crate;
- reproduction steps;
- whether a malicious PR, plugin, runner, or bundle reader is required;
- the Pramaan receipt or bundle fields that were forged, hidden, or leaked.

## Disclosure

Pramaan will prioritize issues that affect bundle integrity, receipt
trustworthiness, verifier sandboxing, redaction, plugin trust, or CI identity.
Until the project has a formal release process, fixes are handled on `main`
with clear changelog entries.

Pramaan produces evidence, not correctness proof. Security fixes should preserve
that wording.
