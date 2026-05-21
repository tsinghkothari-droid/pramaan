# Competitive Benchmark and Prior-Art Matrix

Last refreshed: 2026-05-21

Pramaan should not pretend the market is empty. Pull-request review assistants,
test-generation tools, quality-report aggregators, and software supply-chain
attestation systems all solve adjacent problems. The defensible claim is
narrower:

> Pramaan is an evidence-bundle verifier for AI-authored pull requests. It
> combines execution receipts, oracle-integrity evidence, policy decisions, and
> hash-linked bundle verification so reviewers can inspect what ran, what was
> skipped, what was weakened, and what residual risk remains.

It is not a replacement for CI, code review, SAST, artifact attestations, or
LLM critique. It is the connective verification layer around those signals.
This document intentionally uses category-level prior art instead of named
adjacent projects so Pramaan's public copy, commands, configuration, and code
remain original to this MIT-licensed repository.

## Positioning Summary

| Category | Primary job | Evidence style | Pramaan stance |
| --- | --- | --- | --- |
| Review assistant | Explain a diff, suggest improvements, comment on pull requests | LLM analysis and review comments | Complement. Pramaan may link review comments only as weak signal, never as the sole gate. |
| Structural quality reporter | Put analyzer findings into pull-request review surfaces | Tool output converted into review comments/checks | Reuse the UX pattern. Pramaan emits receipts and SARIF/policy outputs rather than becoming another generic reporter. |
| Test-change monitor | Flag suspicious edits to tests, fixtures, or snapshots | Diff heuristics over tests and artifacts | Overlap. Pramaan's oracle-integrity stage should be deeper and risk-ID backed, but it should not replace specialized test-management workflows. |
| Test generation/amplification | Generate tests or candidate oracles | Generated tests, coverage, search-based assertions | Reuse as optional engines later. Generated tests are not proof until sandbox-executed and recorded as accepted/rejected evidence. |
| Mutation testing | Measure whether tests kill behavior perturbations | Executed mutants, killed/survived/timeouts | Use as an execution stage, scoped to the diff with explicit skipped-tool receipts. |
| Property/fuzz testing | Explore input spaces and find counterexamples | Seeds, generated inputs, shrink/corpus data | Use as execution evidence with replay data. Missing tools and timeouts remain residual risk. |
| Supply-chain attestations | Prove provenance, build identity, and verifier decisions | Signed attestations over artifacts and predicates | Reuse primitives. Pramaan should emit attestable verification summaries, not invent new crypto. |

## Feature Matrix

| Capability | Review assistant | Quality reporter | Test-change monitor | Test generator | Attestation primitive | Pramaan target |
| --- | --- | --- | --- | --- | --- | --- |
| Diff explanation | Strong | Weak | Weak | None | None | Concise summary only |
| LLM critique | Strong | None | None | Sometimes | None | Optional weak signal only |
| Real command execution | Usually limited | Delegated to tools | Usually no | Yes, for generated tests | No code validation by itself | Core requirement |
| Oracle weakening detection | Usually heuristic | Only if analyzer exists | Core overlap | Not the primary goal | None | Core requirement with stable risk IDs |
| Mutation/fuzz receipts | Usually no | Can display output | No | Adjacent | None | Core execution evidence |
| Skipped-tool visibility | Varies | Varies | Varies | Varies | Not applicable | Required residual risk |
| Bundle hash verification | Usually no | Usually no | Usually no | No | Core primitive | Core bundle contract |
| Signed provenance | Usually no | Usually no | Usually no | No | Core primitive | Reuse, emit, and verify |
| Reviewer 30-second artifact | Comments | Comments/checks | Alerts | Generated tests | Attestation metadata | One bundle/report with blockers, warnings, and replay links |
| Agent-harness support | Sometimes | No | No | No | No | First-class done gate |

## Interface Lessons To Reuse Safely

Pramaan should reuse category-level ergonomics, not code or brand language:

- one command or Action step should start verification from a pull request;
- reviewer summaries should be concise and update in place where possible;
- command names should be original, stable, and easy to remember;
- configuration should live in one documented project file;
- comments should summarize evidence, while proof bundles retain the audit log;
- generated suggestions should never outrank executed evidence.

Pramaan should not duplicate:

- general-purpose LLM review comments;
- code-writing or auto-fix loops;
- broad conversational review UX before proof bundles are trusted;
- cryptographic signing infrastructure already covered by established
  attestation primitives;
- full test-generation engines inside the core verifier.

## Evidence Gap Pramaan Targets

The proof bundle should answer questions that ordinary review surfaces can hide:

- Were tests, fixtures, snapshots, or assertions weakened?
- Did required mutation/property/fuzz stages actually run?
- Were missing tools, timeouts, and skipped stages visible as residual risk?
- Are generated probes merely candidates, or were they sandbox-executed and
  bound to changed behavior?
- Can the reviewer verify the bundle hash graph offline?
- Can policy explain why the result is pass, warn, or block?

## Adoption Positioning

| Buyer question | Adjacent tool answer | Pramaan answer |
| --- | --- | --- |
| "Can a review assistant spot issues?" | Comments on likely issues. | Useful weak signal, but not enough for merge confidence. |
| "Can findings appear in the pull request?" | Reporter/check systems can display analyzer output. | Yes, plus a durable bundle that survives the PR UI. |
| "Were tests weakened?" | Some tools flag test diffs. | Oracle integrity emits stable risk IDs and policy-visible receipts. |
| "Did generated tests actually run?" | Test generators produce candidate tests. | Only sandbox-executed accepted probes count as evidence. |
| "Can I trust the artifact?" | Attestation systems prove provenance/signature properties. | Pramaan reuses those primitives for its evidence bundle and verifier decision. |
| "Should I merge?" | Some tools provide recommendations. | Pramaan provides evidence and policy status, not merge authority. |

## Public Claim Rules

Pramaan can honestly say:

- it is evidence-bundle infrastructure for AI-authored pull requests;
- it combines execution receipts, oracle integrity, policy explanation, and
  bundle verification;
- it reuses supply-chain attestation primitives instead of inventing crypto;
- it treats AI review and generated probes as weak or pending signal until
  execution evidence exists.

Pramaan must not say yet:

- it is the most comprehensive PR verifier;
- it proves code correct;
- it replaces review assistants, CI, SAST, or attestations;
- generated tests or probes count before sandbox execution;
- production Sigstore identity is shipped.

## Maintenance Trigger

Refresh this benchmark before:

- public Alpha announcement;
- Serious v1 decision;
- adding or changing claims that compare Pramaan to review assistants, test
  generation systems, quality aggregators, or supply-chain attestation tools;
- publishing a "Pramaan catches what X misses" demo.

Phase 26.3 must turn the strongest comparison claims into executable fixtures.
Until then, this document is positioning evidence, not proof of superiority.

## Fixture Follow-Through

Phase 26.3 adds the category-level fixture manifest:

```text
corpus/competitor-gap-fixtures.v0.1.json
```

Validate it with:

```powershell
node scripts/check-competitor-gap-fixtures.mjs
```

These fixtures are intentionally framed around adjacent tool categories rather
than claims that a named product always misses a case. They support the narrower
public claim that Pramaan targets evidence gaps ordinary review surfaces can
hide: weakened or skipped tests, oracle-sensitive artifacts, hallucinated code,
green CI without required evidence, unsigned aggregate reports, and skipped
verification rendered as success.
