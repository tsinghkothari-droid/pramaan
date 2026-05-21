# Pramaan Tasks to Serious v1

This file tracks the concrete work needed to make Pramaan a serious production
verification layer for AI-authored pull requests.

## Strategic Re-Think: Are We on the Right Track?

Current answer: **yes, if Pramaan stays narrow and evidence-first.**

The strong product is not "AI proves code correct." The strong product is:

> Pramaan creates an auditable evidence bundle for AI-authored pull requests so
> reviewers can see what was checked, what was skipped, what was weakened, what
> risks remain, and whether the bundle itself can be trusted.

That direction is worth continuing because it targets a real adoption blocker:
teams do not merely need more code generation; they need reviewable evidence
when generated code claims to be safe. The risk is scope creep. Pramaan should
not become a generic agent platform, a full CI replacement, a new programming
language, or a universal security scanner before the core PR-verification loop
is trusted.

### Research Discipline

For a project like this, the appropriate amount of research is **four focused
research phases before build commitment**, then short research refreshes only
when a phase has a concrete unknown.

The four research phases are:

1. **Problem and buyer validation:** who has this pain, what they currently do,
   what would make them trust or reject Pramaan.
2. **Technical feasibility and prior art:** papers, tools, benchmarks, CI
   systems, mutation/fuzz/property tools, Sigstore/SLSA/in-toto, and competing
   products.
3. **Failure-mode and threat research:** adversarial PRs, weak oracles, poisoned
   fixtures, hallucinated APIs, malicious CI execution, plugin poisoning,
   redaction failures, and reviewer alert fatigue.
4. **Pilot/eval research:** 3-5 real repositories, 25-100 adversarial fixtures,
   runtime baselines, false-positive/false-negative measurements, and reviewer
   time-to-understand.

After those four research phases, new research must be tied to a deliverable:
a schema field, risk ID, fixture, policy rule, plugin contract, demo, or
benchmark. Broad "keep researching" work should pause until the alpha loop runs
on real repositories.

## How To Execute This File

Use `TASKS.md` in this order:

1. **Current execution order:** follow the phase queue below. This is the
   source of truth for what to build next.
2. **Current Alpha decision:** check whether the public Alpha blockers are
   still open before making external claims.
3. **Active task buckets:** work the detailed checklist for the owning phase.
4. **Release gates:** after each phase, update Alpha, Real MVP, and Serious v1
   gate status if the evidence changed.
5. **Completed historical sections:** keep them for auditability, but do not
   re-open completed P0/P1 work unless the claim audit or pilot evidence says
   the implementation is stale.

For autonomous continuation before Phase 36, use the paste-ready prompt in
`.planning/AUTONOMOUS_GSD_BEFORE_PHASE_36_PROMPT.md`. It orders the remaining
pre-36 phases, keeps the proof/evidence language honest, and requires a
separate verified commit and push per phase.

### Priority Completion Snapshot

This table is the quick answer for whether P0, P1, and P2 are complete.

| Priority | Status | Meaning | Still blocking |
| --- | --- | --- | --- |
| P0 | Complete for private technical preview | Product thesis, killer demo, receipt trust, GitHub Action readiness, policy/SLA, assertion truth audit, three external local pilots, and a live workflow-dispatch Action proof are done. | Public review still needs the final readiness report and explicit risk wording, not more hidden green claims. |
| P1 | Private-preview sufficient, not fully closed | Sandbox, claim scope, static checks, oracle integrity, mutation adapters, deterministic property/fuzz evidence, parser-backed oracle subset evidence with parser metadata, bounded Hypothesis/fast-check harness execution when tools are installed, bounded AI-probe sandbox execution, and the first auditable confidence artifact are usable with honest skipped-tool receipts. Phase 28.15 closes the review-found fuzz harness truthfulness gaps: tool-backed failures affect verdict evidence, timeouts are enforced, harness errors become receipts, dynamic JS evaluation is removed, and tool-generated counts are structured. | Full compiler AST extraction remains split to a heavier Phase 27.2/36 path; production sandboxing for property tools and arbitrary generated code remains open. |
| P2 | Not complete | P2 is the trust/adoption layer after the core loop: signing, redaction, plugin trust, SARIF/policy integration, corpus, calibration, docs, and language depth. Phase 28.5 has started the trust bridge, but the rest remains open. | Phases 29-36. |
| P3 | Not started as product scope | Multi-forge, multi-agent provenance, and adapter certification are later expansion tracks. | Phases 37-39. |

Do not mark P2 complete until signed/attested bundles, redaction profiles,
plugin trust, SARIF/policy export, 25+ adversarial scenarios, calibration, and
operator docs have all landed with tests or checked fixtures.

### Current Execution Order

Build Pramaan in this order. Do not jump ahead unless the phase explicitly says
it can run in parallel.

| Order | Phase | What to make | Why now | Blocks |
| --- | --- | --- | --- | --- |
| 1 | Phase 26 | External local pilot reports for Python, TypeScript, and Rust repositories | Public Alpha needed real-repo evidence beyond internal fixtures. | Public Alpha evidence |
| 1.1 | Phase 26.1 | Live GitHub Action proof on a real PR or PR-like branch | Live workflow-dispatch evidence exists; a PR-event demo remains useful in Phase 26.4. | Public Alpha claims |
| 1.2 | Phase 26.2 | Competitive benchmark and prior-art matrix | The repo should prove where Pramaan is different from review assistants, test monitors, and attestation tools. | Positioning and scope discipline |
| 1.3 | Phase 26.3 | Competitor-gap fixtures | "Pramaan catches what X misses" must be executable evidence, not a marketing sentence. | Demo credibility |
| 1.4 | Phase 26.4 | Minimum lovable verifier loop | One command, one report, one proof bundle, one killer demo should feel complete before expanding. | Real MVP wedge |
| 1.5 | Phase 26.5 | Agent harness interface for Claude Code, Codex, Cursor-style agents, and custom harnesses | Agents should call Pramaan before claiming done. | Agent adoption |
| 2 | Phase 27 | Parser-backed subset oracle extractors | Confidence and signing are weaker if oracle evidence is still parser-light. | Phase 28.5 confidence inputs |
| 2.1 | Phase 27.1 | Parser metadata and full-AST dependency decision | Full AST support needs dependency and runtime justification before public claims change. | Public full-AST claims |
| 3 | Phase 28 | Recorded-case replay for differential fuzz evidence | Confidence needs replayable generated-case evidence, even before real harness execution. | Phase 28.5 confidence inputs |
| 3.1 | Phase 28.1 | Safe Hypothesis/fast-check harness execution | Real tool-backed campaigns need sandboxed generated harnesses. | Public tool-backed property claims |
| 3.15 | Phase 28.15 | Fuzz harness truthfulness review gate | Tool-backed failures must affect verdicts, timeouts must be real, and harness errors must become receipts before public claims. | Public tool-backed property claims |
| 3.25 | Phase 28.25 | AI evidence-seeking probe generator | AI should generate better probes, but only executed probes count. | Phase 28.26 execution |
| 3.26 | Phase 28.26 | Sandbox execution for generated probes | Safe-marker bounded probes now compile/run before they can be kept; rejected probes preserve failure reasons. | Public-review probe honesty |
| 4 | Phase 28.5 | Auditable confidence vote and `confidence.schema.json` | The score must be decomposed before it is signed or marketed. | Phase 29 signed confidence |
| 5 | Phase 29 | Local/offline in-toto/SLSA VSA and offline verify landed; production Sigstore/cosign identity remains planned. | Trust evidence must leave CI as a verifiable artifact. | Real MVP trust gate |
| 6 | Phase 30 | Redaction profiles and public-safe bundle export landed; summary-only artifact minimization remains later hardening. | External pilots and demos need shareable bundles. | Public bundle sharing |
| 7 | Phase 31 | Plugin protocol schema, trust validator, dangerous-permission rejection, and malicious-plugin fixture landed. | Plugin expansion is dangerous before verifier supply-chain trust exists. | Third-party plugin work |
| 8 | Phase 32 | SARIF export, Rego policy export, and agentic workflow-injection checks landed. | Findings should appear in existing security review surfaces. | Enterprise/security adoption |
| 8.5 | Phase 32.5 | Built-in policy packs, profile CLI, fixtures, and Action profile input landed. | Teams need risk-tuned policies without editing code. | Enterprise/security adoption |
| 8.75 | Phase 32.75 | Anti-gaming and verifier-abuse hardening | Malicious or over-optimized PRs must not game skipped stages, config, fixtures, hooks, or receipts. | Corpus and trust hardening |
| 9 | Phase 33 | 25-scenario adversarial and secure-code corpus | Real MVP needs broader failure-mode proof before scale claims. | Phase 40 corpus 100 |
| 10 | Phase 34 | Reviewer overrides, baselines, drift, and confidence calibration | Prevent alert fatigue and make confidence less hand-tuned. | Real MVP calibration gate |
| 11 | Phase 35 | Operator docs, screenshots, troubleshooting, and release packaging | External maintainers need to install and inspect bundles unaided. | Real MVP adoption gate |
| 11.5 | Phase 35.5 | Reviewer UX and local HTML report | A bundle must be understood in 30 seconds without a hosted dashboard. | Real MVP adoption gate |
| 11.6 | Phase 35.6 | License-safe reviewer interface patterns | Borrow mature pull-request tooling ergonomics without importing names, text, prompts, configuration, screenshots, or source code. | Real MVP adoption gate |
| 11.7 | Phase 35.7 | Machine verification and human sign-off gate | Agents must produce evidence, but humans must approve meaning, risk, usefulness, public claims, and release readiness. | Real MVP governance gate |
| 12 | Phase 36 | Python/TypeScript/Rust plugin depth | Depth beats adding shallow Go/Java too early. | Language expansion |
| 13 | Phase 37 | Provider-neutral forge design and GitLab support plan | Avoid hard-coding GitHub before enterprise pilots. | GitLab/Gitea/Bitbucket work |
| 14 | Phase 38 | Multi-agent provenance and handoff tracking | Agent chains need attribution before trend analysis becomes serious. | Serious v1 provenance |
| 15 | Phase 39 | Bounded adapter certification mode | Keep adjacent adapter work useful but out of Alpha scope. | Adapter product expansion |
| 16 | Phase 40 | Corpus 100, benchmark-integrity, cross-platform CI, final claim audit, Serious v1 decision | Forces a release decision from evidence instead of enthusiasm. | Serious v1 |

Execution guardrails:

- Start with Phase 26 unless the user explicitly asks for a narrower phase.
- Do not market public Alpha from live Action evidence alone; Phase 26.1 is
  complete, but public review still needs the remaining pre-36 readiness phases.
- Do not claim Pramaan is better than adjacent review assistants before Phase 26.2 and Phase
  26.3 create a benchmark matrix and executable competitor-gap fixtures.
- Do not broaden engines before Phase 26.4 proves the minimum lovable verifier
  loop is coherent end to end.
- Do not ask coding agents to self-certify completion before Phase 26.5 lands.
- Do not count AI-generated probes as evidence unless `pramaan probe execute`
  records `executed_passed`; rejected or pending probes are residual risk.
- Do not sign or attest confidence scores before Phase 28.5 exists.
- Do not offer enterprise policy-pack claims before Phase 32.5 exists.
- Do not expand plugins before Phase 31 defines trust and isolation.
- Do not publish public-demo bundles before Phase 30 redaction tests pass.
- Do not dashboard-first the product; Phase 35.5 must ship static report and PR
  summary UX first.
- Do not treat any agent-written phase as human-approved; Phase 35.7 requires
  machine-verification evidence plus human sign-off for public claims, releases,
  Marketplace publishing, and Serious v1 gates.
- Do not call the project Serious v1 before Phase 40 closes or rejects the
  release gate.
- Use `.planning/AUTONOMOUS_GSD_BEFORE_PHASE_36_PROMPT.md` when handing the
  remaining pre-36 sequence to a fresh autonomous agent.

### P0 Product Direction Tasks

- [x] Declare the narrow product thesis in `README.md`, `STATUS.md`, and
  `.planning/STATE.md`: Pramaan is a PR evidence-bundle verifier, not a
  correctness oracle or generic CI replacement.
- [x] Add explicit non-goals: no automatic merge authority, no "proved correct"
  claims, no generic agent registry work inside the core v0.1 path, and no
  dashboard-first roadmap before CLI/Action trust is real.
- [x] Define the first ideal customer profile: teams reviewing AI-authored PRs
  in Python, TypeScript, or Rust repos with CI already in place.
- [x] Define the first killer workflow: "GitHub green, Pramaan red" for a
  weakened-test PR, with a bundle understood in under 30 seconds.
- [x] Create a research sufficiency checklist: at least 40 source-backed notes,
  30 mapped failure modes, 10 competing/prior-art tools, 25 adversarial
  fixtures, 3 pilot repositories, and measured runtime/noise baselines.
- [x] Stop broad research once the sufficiency checklist is met; convert every
  remaining research question into an experiment, fixture, policy rule, or doc.
- [x] Add pivot/kill criteria: if reviewers cannot understand a bundle in under
  30 seconds, if runtime regularly exceeds the SLA, or if skipped stages look
  like passes, pause feature expansion and fix trust/UX first.
- [x] Review the phase plan every 4 build phases and remove or merge phases that
  do not directly improve evidence quality, reviewer trust, runtime, or adoption.

### Completed P0/P1 GSD Track

This track is historical. It explains how P0/P1 became private-preview-ready
and why public Alpha is still blocked. A phase should only be reopened if new
pilot evidence or the claim audit shows the implementation is stale.

| GSD phase | Priority | What it completes |
| --- | --- | --- |
| Phase 18 | P0 | Product thesis, non-goals, `STATUS.md`, current-vs-planned README honesty, ICP, killer workflow, research sufficiency, pivot criteria. |
| Phase 19 | P0 | Receipt golden tests, canonical serialization/hashing plan, fixture drift controls, schema/runtime consistency guardrails. |
| Phase 20 | P0 | Performance SLA, stage budgets, default policy profile, `pramaan policy explain`, hard-gate vs warning-gate behavior. |
| Phase 21 | P1 | Sandbox hardening, OCI/container identity, dirty-after-run detection, verifier threat model, redaction, CI hardening checks. |
| Phase 22 | P1 | Claim-scope issue ingestion, scope-note support, vague-claim warnings, semantic mismatch signal, relaxed static-config and security-sensitive diff classification. |
| Phase 22.5 | P0 | Assertion truth audit gate: every public claim must be backed by executable evidence, a checked fixture, a manual proof command, or a clear partial/planned/experimental label. |
| Phase 23 | P1 | Structured oracle extractor evidence for Python, TypeScript, and Rust with golden fixtures; full compiler AST integrations remain a hardening follow-up. |
| Phase 24 | P1 | Tool-backed mutation when adapters exist, deterministic replay/property evidence, budgets, thresholds, replay metadata, and honest skipped/timeout receipts. |
| Phase 25 | P0/P1 gate | Internal pilot validation, P0/P1 acceptance report, unresolved-risk register, and private-preview/public-Alpha decision. |

Execution rule: do not start P2 signing/attestation expansion or dashboard work
until the Phase 25 P0/P1 gate says the core PR-verification loop is trustworthy
enough for external users. Current Phase 25 decision: **private technical
preview is reasonable; public Alpha remains blocked** until three external
repository pilots are measured and the remaining public-claim gaps stay
narrowed in `docs/claim-audit.md`.

### Research-Driven GSD Continuation Track

Current internet research was refreshed on 2026-05-21 in
`.planning/research/GSD_PHASE_RESEARCH_REFRESH_2026-05-21.md`. The refresh
does not change Pramaan's thesis: Pramaan produces evidence, not a correctness
proof. It does change the remaining build order. The next track turns every
unfinished task family below into an executable GSD phase.

| GSD phase | Priority | What it completes |
| --- | --- | --- |
| Phase 26 | Alpha gate | Three external real-repository local pilots, runtime/noise metrics, and a public-Alpha no-go update. |
| Phase 26.1 | Alpha gate | Live GitHub Action proof, uploaded bundle artifact, job summary evidence, and a public-Alpha go/no-go update. |
| Phase 26.2 | Positioning | Competitive benchmark matrix against adjacent review-assistant, quality-reporting, test-monitoring, and attestation tool categories. |
| Phase 26.3 | Demo credibility | Executable "Pramaan catches what X misses" fixtures for weakened assertions, skipped tests, fixture drift, false-green CI, and unsigned aggregate reports. |
| Phase 26.4 | Product wedge | Minimum lovable verifier loop: one command, one report, one proof bundle, one killer demo, and a 30-second reviewer pass. |
| Phase 26.5 | Agent adoption | `pramaan agent done-gate`, agent decision schema, `AGENTS.md`, Claude Code hook/command templates, and blocked-agent fixtures. |
| Phase 27 | P1 hardening | Parser-backed subset oracle extractors for Python, TypeScript, and Rust with negative fixtures and honest residual full-AST split. |
| Phase 27.1 | P1 hardening split | Parser-version evidence, unsupported-syntax metadata, disagreement reporting fields, and full-AST dependency decisions; full compiler integrations remain split. |
| Phase 28 | P1 hardening | Recorded-case replay CLI contracts for differential fuzz evidence, with real harness execution split honestly. |
| Phase 28.1 | P1 hardening split | First bounded Hypothesis and fast-check generated-harness execution path with tool-version/raw-output evidence; Phase 28.15 owns review-found verdict/timeout/truthfulness gaps. |
| Phase 28.15 | P1 corrective review gate | Completed truthfulness repair for harness failure promotion, real timeout enforcement, structured harness-error receipts, JS evaluation safety, and clearer tool-generated count metadata. |
| Phase 28.25 | 10x evidence depth | AI-generated probes for tests, properties, differential inputs, and security checks; only sandbox-executed probes count. |
| Phase 28.26 | 10x evidence depth | Bounded generated-probe execution, rejected-probe preservation, and execution report validation. |
| Phase 28.5 | P1/P2 trust | Auditable confidence-vote algorithm, hard-gate rules, weak-signal aggregation, statistical intervals, and `confidence.schema.json`. |
| Phase 29 | P2 trust | Local/offline SLSA VSA-style output, in-toto wrapping, composite-action attestation emission, and offline bundle verification; production Sigstore/cosign identity remains a hardening follow-up. |
| Phase 30 | P2 trust | Redaction profiles, redacted bundle export, manifest rebuild, and public-demo scrub tests for secrets, private paths, internal hosts, and CI metadata. |
| Phase 31 | P2 security | Plugin protocol schema, plugin identity/provenance validation, least-privilege receipt permissions, and malicious plugin rejection. |
| Phase 32 | P2 security | SARIF export, Rego policy export, claim-scope workflow-injection checks, and GitHub code-scanning guidance. |
| Phase 32.5 | Enterprise adoption | Built-in policy packs, profile fixtures, `policy list`, `policy explain --profile`, and GitHub Action `policy-profile`. |
| Phase 32.75 | Trust hardening | Anti-gaming fixtures and verifier-abuse checks for weakened config, hidden skipped stages, hook bypass, fixture poisoning, receipt tampering, and benchmark overfitting. |
| Phase 33 | P2 evals | Adversarial corpus expansion to 25 scenarios, including secure-code and malicious-verifier cases. |
| Phase 34 | P2 feedback | Reviewer override persistence, repo baselines, calibration, drift exports, and agent-author trend metrics. |
| Phase 35 | P2 adoption | Operator guide, plugin-author guide, security model, troubleshooting docs, screenshots, and release packaging. |
| Phase 35.5 | Adoption UX | Local HTML report and PR markdown report with blockers, warnings, ran/skipped stages, oracle changes, replay commands, and override fields. |
| Phase 35.6 | Adoption UX and license hygiene | PR URL entrypoint, original reviewer commands, `.pramaan.toml`, persistent summaries, and public docs that avoid named adjacent-project references. |
| Phase 35.7 | Governance gate | Machine-verification and human-signoff templates plus docs defining what agents can prove and what humans must approve. |
| Phase 36 | P2 language depth | Deep Python, TypeScript, and Rust plugin quality before Go/Java expansion. |
| Phase 37 | P3 enterprise | Provider-neutral forge abstraction, GitLab attestation/OIDC design, and Gitea/Bitbucket notes. |
| Phase 38 | P3 provenance | Multi-agent provenance chains, intermediate commit attribution, and handoff metadata. |
| Phase 39 | P3 adjacent | Keep adapter certification bounded, with proof-bundle examples and adapter risk taxonomy. |
| Phase 40 | Serious v1 gate | 100-scenario corpus, benchmark-integrity checks, cross-platform CI, final claim audit, and Serious v1 go/no-go decision. |

Execution rule: **Public review can only be described as "ready with risks" if
the dated public-review readiness report says so after full verification.** Do
not expand the plugin ecosystem until Phase 31 defines plugin trust and
isolation. Do not claim production-safe bundle sharing until Phase 30 redaction
tests pass. Do not claim Serious v1 until Phase 40 closes the 100-scenario
corpus and final release gate. Phase 28.5 must land before Phase 29 so any
confidence vote is digest-linked and attestable instead of being an unsigned
UI-only score.

Unfinished task-family mapping:

| Open task family | Owning phases |
| --- | --- |
| Public Alpha blockers | 26, 27, 28, 29 |
| Competitive benchmark and prior art | 26.2 |
| Competitor-gap proof fixtures | 26.3 |
| Minimum lovable verifier packaging | 26.4 |
| Agent harness for coding agents | 26.5 |
| Full AST/parser oracle integrations | 27.1 |
| Real Hypothesis/fast-check campaigns | 28.1 |
| AI evidence-seeking probe generation and bounded execution | 28.25, 28.26 |
| Auditable confidence vote and scoring schema | 28.5, 34 |
| Attestation and signing | 29 |
| Redaction profiles | 30 |
| Plugin trust and verifier security | 31 |
| CI/security review integrations | 32 |
| Policy packs and enterprise profiles | 32.5 |
| Anti-gaming and verifier-abuse hardening | 32.75, 33 |
| Adversarial corpus and secure-code scenarios | 33, 40 |
| Reviewer feedback, calibration, and drift | 34 |
| Documentation and adoption | 35 |
| Reviewer UX and local reports | 35.5 |
| Python/TypeScript/Rust language depth | 36 |
| Non-GitHub enterprise support | 37 |
| Multi-agent provenance | 38 |
| Adapter certification | 39 |
| Serious v1 release gates | 40 |

### Current Alpha Decision Snapshot

**Decision:** private technical preview yes, public Alpha no-go.

Evidence already in the repo:

- [x] P0/P1 phases 18-25 have landed with summaries, docs, and verification.
- [x] Claim audit covers 56 claims and all 28 `STATUS.md` capability rows.
- [x] Internal pilot runs cover oracle, mutation, Python fuzz, and TypeScript
  fuzz fixtures with runtimes recorded.
- [x] Three external local pilots cover Python, TypeScript, and Rust
  repositories with runtime, skipped-stage, noise, and residual-risk notes.
- [x] Missing mutation/property tools are visible evidence, not hidden passes.
- [x] Public copy is narrowed around signing, full AST parsing, and real
  Hypothesis/fast-check execution.

Public Alpha blockers:

- [x] Run Pramaan on three external real repositories and record runtime,
  skipped stages, false positives, false negatives, and reviewer
  time-to-understand.
- [x] Add safe generated-harness execution for Hypothesis and fast-check when
  tools are installed; keep missing-tool fallback visible as residual evidence.
- [x] Add parser-version evidence, unsupported-syntax metadata, disagreement
  reporting fields, and dependency decision docs for Python, TypeScript, and
  Rust.
- [ ] Add full compiler/parser AST integrations for Python, TypeScript, and
  Rust, or keep those claims explicitly planned.
- [x] Add local/offline VSA and in-toto attestation evidence, while keeping
  production Sigstore/GitHub identity claims explicitly planned.
- [x] Prove the GitHub Action through a live `workflow_dispatch` run with an
  uploaded bundle and rendered summary; a PR-event demo remains useful for
  Phase 26.4.
- [x] Add an auditable confidence-vote schema and receipt, or keep all
  confidence-score claims explicitly absent from public Alpha copy.

### Right-Direction Phase Task Breakdowns

These tasks expand the right-direction phases that make Pramaan useful to real
AI coding agents and real reviewers. They are planned, not complete.

#### Phase 26.2: Competitive Benchmark and Prior-Art Matrix

- [x] Add `docs/competitive-benchmark.md`.
- [x] Compare Pramaan against adjacent review-assistant, quality-reporting,
  test-change monitoring, test-generation, mutation, property/fuzz, and
  supply-chain attestation categories.
- [x] For each tool category, record overlap, what Pramaan should reuse, what Pramaan
  should not duplicate, and what evidence gap remains.
- [x] Add a "not a competitor" section for tools that are primitives rather
  than replacement products.
- [x] Add an adoption-positioning table: review assistant, CI quality aggregator,
  supply-chain attestor, evidence-bundle verifier.
- [x] Add claim-audit rows for any README/marketing claim that says Pramaan is
  broader, stronger, or more comprehensive than existing tools.
- [x] Add a maintenance note requiring this benchmark to refresh before public
  Alpha and Serious v1.

#### Phase 26.3: Competitor-Gap Fixtures

- [x] Add fixture scenarios where ordinary AI PR review would likely pass but
  Pramaan should block or warn.
- [x] Cover weakened assertions, added skips, fixture/snapshot drift,
  hallucinated API usage, false-green CI, unsigned aggregate quality reports,
  and hidden skipped verification stages.
- [x] Map every scenario to stable risk IDs and expected Pramaan receipts.
- [x] Add a report that says which public-tool category would miss or
  under-report each scenario.
- [x] Add validation that stale or duplicate gap fixtures fail through
  `node scripts/check-competitor-gap-fixtures.mjs`; CI wiring remains a Phase
  26.4/35.5 packaging follow-up.
- [x] Keep competitor names out of brittle test names where possible; test the
  failure mode, not another project's implementation detail.

#### Phase 26.4: Minimum Lovable Verifier Loop

- [x] Define the one-command path from checkout to proof bundle:
  `powershell -ExecutionPolicy Bypass -File scripts/run-minimum-lovable-loop.ps1`.
- [x] Define the one-report path for a reviewer to understand blockers in under
  30 seconds.
- [x] Pick one killer demo as the canonical first-run example and keep all
  public quickstart docs pointed at it.
- [x] Ensure the generated bundle contains receipts, manifest, policy summary,
  confidence explanation when available, and clear skipped-stage evidence.
- [x] Add a manual UAT script where a fresh reviewer runs the command, opens
  the report, and explains the blocker without reading source code first.
- [x] Treat any skipped stage that looks like a pass as a release blocker.
- [x] Defer broad language expansion until this loop feels complete.

#### Phase 26.5: Agent Harness Interface

- [x] Add `schemas/agent_decision.schema.json` with `decision`, `reason`,
  `bundle_path`, `blocking_stages`, `warnings`, `required_actions`,
  `agent_message`, and `human_override_allowed`.
- [x] Add `pramaan agent done-gate --base <ref> --head <ref> --out <dir>`.
- [x] Add `pramaan agent explain --bundle <path>`.
- [x] Add deterministic `pass`, `warn`, and `block` mapping from existing
  policy, bundle, and oracle evidence.
- [x] Add an `AGENTS.md` template that tells Codex-style agents not to claim
  done while Pramaan blocks.
- [x] Add Claude Code command/hook templates under docs or `.claude/commands/`
  if they can be kept provider-optional.
- [x] Add a blocked-agent fixture where a weakened-test PR returns
  `decision=block`.
- [x] Add a warning fixture where skipped optional tools return
  `decision=warn`.
- [x] Add docs explaining the agent harness is an evidence gate, not an agent
  self-certification loop.

#### Phase 28.25: AI Evidence-Seeking Probe Generator

- [x] Add `schemas/probe.schema.json`.
- [x] Add `pramaan probe plan --bundle <path>` to produce risk-targeted probe
  plans.
- [x] Support probe kinds: regression assertion, property invariant,
  differential input, security sink/source check, mutation-targeted test, and
  fixture/snapshot challenge.
- [x] Store prompt hash and model/provider metadata without making provider
  output trusted evidence.
- [x] Run safe-marker generated probes in isolated temp test locations.
- [x] Reject probes that do not compile, do not run, or do not exercise changed
  behavior.
- [x] Preserve pending generated probes and execution requirement as evidence;
  rejected probes now keep compile/run/static rejection reasons.
- [ ] Mutation-test or differential-test accepted probes where practical.
- [x] Emit `ai_probe_generation` receipts with accepted/rejected/pending counts, risk
  IDs, and artifact hashes.
- [x] Document that AI proposes probes, but only sandbox-executed probes count.
- [x] Execute `Phase 28.26`: bounded sandbox execution for generated probes
  and compile/run/static rejection reasons.
- [ ] Add mutation/differential validation for accepted probes where practical.

#### Phase 32.5: Policy Pack Library and Enterprise Profiles

- [x] Add `schemas/policy_profile.schema.json`.
- [x] Create `policy/startup-fast.json`, `policy/open-source-maintainer.json`,
  `policy/security-sensitive.json`, `policy/fintech-strict.json`, and
  `policy/private-preview.json`.
- [x] Add `pramaan policy list`.
- [x] Add `pramaan policy explain --profile <id>`.
- [x] Add GitHub Action `policy-profile` input.
- [ ] Add policy fixture bundles for pass, warn, fail, waiver, and
  security-sensitive escalation.
- [ ] Add parity tests between default Rust policy behavior and exported policy
  fixtures.
- [x] Document when each policy pack should be used.
- [x] Keep policy packs as deterministic gates; no LLM judge can override hard
  gates.

#### Phase 32.75: Anti-Gaming and Verifier-Abuse Hardening

- [x] Add malicious PR fixtures for workflow bypass, action wrapper weakening,
  altered fixtures, changed verification scripts, schema weakening, and corpus
  overfitting.
- [x] Add verifier-abuse fixture coverage for hidden skipped stages, forged
  script output, schema weakening, poisoned examples, and benchmark overfitting.
- [x] Add policy rules that make suspicious verifier-surface changes
  warning-or-blocking depending on policy profile.
- [x] Ensure skipped required stages cannot silently improve confidence.
- [x] Ensure generated receipts cannot be overwritten by plugins or PR code.
- [x] Add docs explaining Pramaan's adversary model: careless AI, overfitted AI,
  malicious contributor, and compromised verification plugin.
- [x] Feed accepted scenarios into Phase 33 and Phase 40 corpus counts through
  `corpus/verifier-abuse-fixtures.v0.1.json`.

#### Phase 35.5: Reviewer UX and Local HTML Report

- [x] Add `pramaan report html --bundle <path> --out <report.html>`.
- [x] Add `pramaan report markdown --bundle <path>`.
- [x] Group report content into blockers, warnings, ran/skipped stages, oracle
  changes, replay commands, and human override fields.
- [x] Show oracle-integrity failures before lower-signal evidence.
- [x] Add copyable replay commands for mutation, fuzz, and property failures.
- [x] Add static fixture reports for pass, warn, and fail bundles.
- [x] Update the GitHub Action markdown summary to mirror the report hierarchy.
- [x] Add a smoke test that the weakened-test report includes blocker, oracle
  finding, and replay or inspection paths.
- [x] Add docs that the local report is the first UX surface; hosted dashboard
  is later.

#### Phase 35.6: License-Safe Reviewer Interface Patterns

- [x] Add or stage `pramaan verify-pr --url <pull-request-url>` as the clean
  PR URL entrypoint.
- [x] Define original reviewer commands: `/pramaan verify`,
  `/pramaan explain`, `/pramaan replay`, and `/pramaan policy <profile>`.
- [x] Add `.pramaan.toml` documentation for policy profile, stage budgets,
  redaction, mutation opt-in, report behavior, and summary update behavior.
- [x] Update the Action/report plan so repeated runs update one persistent
  summary instead of creating noisy repeated comments.
- [x] Keep all public docs category-level: no copied prompts, config keys,
  command names, screenshots, text, or branded terminology from adjacent
  projects.
- [x] Add a verification check that risky adjacent-project names do not appear
  in selected public-facing docs.

Phase 35.6 is complete as a docs/interface-contract slice. Runtime `verify-pr`,
`doctor`, `.pramaan.toml` loading, and forge-specific persistent PR summary
updates remain future implementation work.

#### Phase 35.7: Machine Verification and Human Sign-Off Gate

- [x] Add `docs/human-signoff.md`.
- [x] Add `.planning/templates/MACHINE_VERIFICATION.md`.
- [x] Add `.planning/templates/HUMAN_SIGNOFF.md`.
- [x] Require future GSD phase closeouts to include machine evidence and a
  prepared human sign-off artifact.
- [x] Update autonomous continuation instructions so agents do not
  self-approve human-only decisions.
- [x] Block public claims, releases, Marketplace publishing, and Serious v1
  decisions until human sign-off exists.

### P0: Assertion Truth Audit Gate

This gate is now documented and remains a release blocker before public Alpha.
The repo must keep proving its own claims the same way Pramaan expects
AI-authored PRs to prove theirs.

- [x] Create `docs/claim-audit.md` with stable claim IDs for README, STATUS,
  TASKS, ROADMAP, docs, schemas, examples, and Action promises.
- [x] Classify every claim as `executable-test`, `checked-fixture`,
  `manual-proof`, `implemented-untested`, `partial`, `planned`,
  `experimental`, or `false-or-stale`.
- [x] Downgrade or remove every `false-or-stale` public claim.
- [x] Add tests or fixtures for high-risk `implemented-untested` claims:
  canonical hashing, policy decisions, sandbox evidence, redaction, claim
  scope, static security signals, oracle weakening, bundle verification, and
  Action summary rendering.
- [x] Make Alpha release impossible while any public `implemented` claim lacks
  evidence or an accepted-risk owner.
- [x] Record final audit counts: total claims, tested claims, fixture-backed
  claims, manual-proof claims, downgraded claims, and unresolved risks.

## P0: Killer Demo

- [x] Build a standalone demo repository where normal CI passes but Pramaan fails because a test assertion was weakened.
- [x] Add a second demo where a snapshot or fixture change silently approves wrong behavior.
- [x] Add a third demo where a fake import/API passes superficial review but fails static/hallucination checks.
- [x] Create a short reviewer walkthrough that shows the proof bundle can be understood in under 30 seconds.
- [x] Add generated example bundles for each demo scenario.

## P0: Receipt and Bundle Trust

- [x] Freeze receipt schema version `0.1`.
- [x] Add agent-author attribution before schema freeze: agent product, model family/version when available, execution mode, prompt/context hash, and commit provenance.
- [x] Add reviewer override capture before schema freeze: override decision, reason, reviewer identity source, timestamp, risk IDs accepted, and linked merge outcome.
- [x] Add schema compatibility tests for all checked-in fixture receipts.
- [x] Add golden tests that diff generated receipts against approved fixtures.
- [x] Add artifact graph support so every receipt can point to hashed logs, corpora, and tool outputs.
- [x] Add bundle-level verification summary with mitigated, residual, skipped, and not-applicable risk families.
- [x] Add tamper tests for missing artifacts, modified receipts, modified manifests, and changed signing metadata.

## P0: GitHub Action Readiness

- [x] Make the action install or download the Pramaan CLI deterministically.
- [x] Add `base-ref`, `head-ref`, `out-dir`, `fail-on`, and `upload-bundle` inputs.
- [x] Define hard performance SLA targets for PR use: target runtime, max runtime, per-stage budget, and behavior when a budget is exhausted.
- [x] Add default policy-as-code profile for hard gates, warning gates, waiver rules, stage requirements, and security-sensitive paths.
- [x] Add `pramaan policy explain` so reviewers can see why a bundle failed, warned, or passed under a policy.
- [x] Upload the proof bundle as a GitHub Actions artifact.
- [x] Render a concise PR summary focused on failed stages and residual risks.
- [x] Add permissions documentation for pull requests from forks.
- [x] Add a minimal example workflow for Python, TypeScript, and Rust repositories.

## P1: Sandbox and Environment Evidence

- [x] Capture OS, architecture, shell, timezone, locale, and toolchain versions.
- [x] Record base/head commit IDs and dirty/untracked file state.
- [x] Hash dependency manifests and lockfiles.
- [x] Detect lockfile changes and mark dependency-drift risks.
- [x] Capture container image names and digests when supplied by CI/container environment metadata.
- [x] Auto-detect OCI/container identity when CI does not provide image metadata explicitly.
- [x] Add network policy evidence: disabled, allowed, observed, or unknown.
- [x] Detect source changes after a stage runs and mark dirty-after-run risk.
- [x] Threat-model the verifier as an attack target, including malicious PR code exploiting mutation engines, fuzzers, parsers, test runners, or plugin hooks.
- [x] Add PII/secrets scrubbing rules for environment evidence, logs, network endpoints, internal hostnames, paths, and artifact metadata before bundle emission.
- [x] Add CI hardening checks for untrusted PR execution: least-privilege token permissions, `pull_request_target` hazards, cache poisoning, unpinned actions, artifact retention, and self-hosted runner warnings.

## P1: Claim Scope

- [x] Parse PR title and body from GitHub Actions context.
- [x] Ingest linked issue text when available.
- [x] Detect changed public APIs for Python, TypeScript, and Rust.
- [x] Add low-confidence claim-scope warnings for vague or missing PR descriptions.
- [x] Allow maintainers to provide a scope note file for expected and out-of-scope behavior.
- [x] Map claim-scope warnings to stable risk IDs.
- [x] Add semantic claim-implementation mismatch detection as a bounded signal: compare stated intent, touched APIs, tests, and changed behavior without making it a sole merge gate.

## P1: Static and Hallucination Checks

- [x] Python: integrate `compileall`, `ruff`, `mypy`, and `pyright` when configured.
- [x] TypeScript: integrate package-manager detection, `tsc --noEmit`, and ESLint when configured.
- [x] Rust: integrate `cargo check`, `cargo test --no-run`, and `cargo clippy` when configured.
- [x] Classify failures as `invented_api`, `invalid_parameter`, `undefined_symbol`, `nonexistent_import`, `resource_mismatch`, `logic_mismatch`, or `unknown`.
- [x] Detect relaxed static-check configuration in the PR.
- [x] Emit skipped receipts when tools are unavailable instead of hiding missing checks.
- [x] Add security-sensitive diff classification for auth, authorization, cryptography, SQL/query construction, subprocess, filesystem, deserialization, secrets, network, and permissions.

## P1: Oracle Integrity

- [x] Python: implement deterministic diff for `pytest` assertions, skips, xfails, raises, and parametrized cases.
- [x] TypeScript: implement deterministic diff for Jest, Vitest, and common `expect` patterns.
- [x] Rust: detect weakened `assert!`, `assert_eq!`, panic tests, `#[ignore]`, and snapshot/fixture changes.
- [x] Detect deleted tests and renamed tests through stable body fingerprints.
- [x] Classify fixture and snapshot diffs as oracle-sensitive.
- [x] Detect removed boundary cases, error cases, and parameter values.
- [x] Add reviewer-facing summaries that explain exactly which assertion or oracle artifact changed.
- [x] Replace opaque heuristic oracle scanning with parser-backed subset extractor evidence for Python, TypeScript, and Rust: extractor labels, assertion-signal kinds, strength scores, signal hashes, skip markers, comment/string filtering, and multiline assertion grouping.
- [x] Add parser metadata fields for parser version, fallback reason,
  unsupported syntax, and disagreement count so full-AST residual risk is
  visible in `oracle-diff.json`.
- [x] Add dependency decision docs and metadata fixtures for decorators,
  generated tests, TypeScript computed names, and Rust macro-generated tests.
- [ ] Add full compiler/parser AST integrations for Python, TypeScript, and
  Rust with subprocess/dependency pinning; current v0.1 remains a
  parser-backed subset, not a full-AST proof.

## P1: Mutation Adapters

- [x] Python: run `mutmut` on changed files when the tool is installed; otherwise emit a skipped receipt.
- [x] TypeScript: run StrykerJS in diff-scoped mode where possible.
- [x] Rust: run `cargo-mutants` on changed crates/modules when available.
- [x] Record mutants created, killed, survived, timed out, skipped, and unviable.
- [x] Record mutation threshold, timeout policy, incremental-cache state, filtering mode, raw-output path, and raw-output digest when executed.
- [x] Add equivalent-mutant and requires-review classifications where tool output supports it.
- [x] Keep stage budgets strict enough for pull-request CI.
- [x] Ensure skipped/missing mutation tools never count as mitigated evidence.

## P1: Property, Fuzz, and Differential Checks

- [x] Python: auto-discover eligible pure functions and run deterministic differential replay checks.
- [x] TypeScript: auto-discover eligible pure functions and run deterministic differential replay checks.
- [x] Python: execute a first bounded Hypothesis subprocess harness when
  Hypothesis is installed and eligible pure-function candidates exist.
- [x] TypeScript: execute a first bounded fast-check subprocess harness when
  fast-check is installed and eligible pure-function candidates exist.
- [x] Promote Hypothesis/fast-check harness-discovered failures into canonical
  divergence, replay, counterexample, residual-risk, confidence, and policy
  evidence.
- [x] Enforce real subprocess timeouts for Python and Node tool harnesses.
- [x] Convert harness nonzero exits/timeouts into structured receipt evidence
  instead of opaque command aborts.
- [x] Remove dynamic JavaScript `Function(...)` evaluation or isolate it behind
  a documented verifier-security boundary.
- [x] Store tool version, generated cases, timeout status, raw-output digest,
  deterministic input count, and tool-generated case count as structured
  evidence fields.
- [x] Record seeds, replay data, counterexamples, corpus hashes, generated input counts, and adapter availability.
- [x] Compare base/head outputs on identical generated inputs.
- [x] Classify divergences as expected, unexpected, or needs-review.
- [x] Add replay artifacts and `pramaan replay` metadata inspection for every
  failing generated case.

## P1/P2: Auditable Confidence Vote

Phase owner: **Phase 28.5: Auditable Confidence Vote and Calibration Schema**.

Purpose: turn Pramaan's many receipts into a signed, decomposed risk decision
that reviewers can audit. This must never be marketed as "proof of
correctness"; it is calibrated evidence about residual PR risk.

Phase prerequisites:

- [x] Phase 26 external pilot report exists or Phase 28.5 uses only explicit
  deterministic starter weights marked `uncalibrated`.
- [x] Phase 27 parser-backed oracle status is reflected as implemented,
  partial, or residual risk in the confidence inputs.
- [x] Phase 28 property/fuzz receipts expose generated-case counts, failures,
  skipped-tool status, and replay metadata needed for confidence evidence.

Core implementation:

- [x] Add a `pramaan-confidence` module or equivalent core subsystem that
  emits a confidence artifact without claiming correctness proof.
- [x] Add `schemas/confidence.schema.json` with algorithm version,
  hard-gate outcomes, weak-signal votes, stage reliability inputs, dependency
  clusters, statistical intervals, top risk drivers, top confidence drivers,
  calibration metadata, and residual-risk explanation.
- [x] Add fixture validation for `confidence.schema.json`, including required
  fields, enum values, unknown algorithm versions, and forward-compatible
  optional fields.
- [x] Implement v0.1 hard gates that cannot be averaged away: failed oracle
  integrity evidence, failed bundle/attestation-style receipts, untrusted
  plugin provenance, and exhausted evidence budgets.
- [x] Add remaining hard gates for unsupported critical evidence paths and
  finer-grained invalid-attestation policy reasons.
- [x] Implement weak-signal aggregation inspired by weak supervision:
  `risky`, `safe`, or `abstain` votes from oracle, mutation, fuzz/property,
  static, claim scope, sandbox, policy, and optional critic stages.
- [x] Define deterministic starter weights for each stage and document why
  oracle, bundle tamper, and missing critical tools carry higher weight than
  style/critic signals.
- [x] Add dependency discounts so correlated stages such as oracle, mutation,
  and property/fuzz do not get counted as independent proof.
- [x] Add skipped-stage uncertainty penalties so "tool not installed" lowers
  confidence instead of silently becoming neutral.
- [x] Use Wilson lower bounds for mutation kill confidence instead of raw
  mutation score alone.
- [x] Use the rule-of-three upper bound for zero-failure fuzz/property
  campaigns and store the generated-case count.
- [x] Penalize skipped or missing tools as uncertainty, never as pass evidence.
- [x] Emit `confidence.json` and `confidence.md` with decomposed risk drivers,
  not just a single percentage.
- [x] Add `pramaan confidence explain <bundle>` or equivalent CLI path that
  renders the confidence artifact for reviewers.
- [x] Add bundle-manifest links and artifact digests for `confidence.json` and
  `confidence.md` so Phase 29 can sign or attest them.
- [x] Add policy wiring so confidence can influence `fail`, `warn`, or `pass`
  without overriding hard gates.

Required confidence fixtures:

- [x] Hard fail: weakened assertion with otherwise clean static checks.
- [ ] Hard fail: bundle tamper or invalid bundle integrity.
- [ ] Warning: mutation survivors with clean oracle evidence.
- [x] Warning: fuzz/property stage skipped because the tool is missing.
- [ ] Warning: contradictory signals where static passes but claim scope is
  low-confidence and mutation evidence is weak.
- [ ] Pass: clean receipts with sufficient executed evidence and no hard gates.
- [x] Small-sample mutation case proving Wilson lower bound is more cautious
  than raw mutation score.
- [x] Zero-failure fuzz/property case proving rule-of-three residual-risk bound
  is recorded.
- [ ] Correlated evidence case proving oracle, mutation, and property/fuzz do
  not triple-count the same test-quality signal.

Audit and documentation:

- [x] Write `docs/confidence.md` explaining the algorithm, hard gates, weak
  votes, dependency discounts, statistical intervals, skipped-stage penalties,
  and calibration status in reviewer language.
- [x] Update `docs/claim-audit.md` so any public confidence-score wording is
  backed by executable tests, checked fixtures, or a clear planned label.
- [x] Add an example `confidence.md` output to fixture/demo evidence.
- [x] Keep initial weights deterministic and documented until Phase 34 has
  enough pilot data for calibration.
- [x] In Phase 34, evaluate calibration using Brier score, log loss, and
  reliability diagrams / expected calibration error where labeled outcomes
  exist.

Phase 28.5 acceptance criteria:

- [x] `confidence.json` validates against `schemas/confidence.schema.json`.
- [x] `confidence.md` explains the same decision in reviewer-readable language.
- [x] Confidence artifacts are deterministic for the same receipt inputs.
- [x] Hard gates always dominate the final decision.
- [x] Missing/skipped evidence is visible as uncertainty.
- [x] Phase 29 has an explicit artifact digest to sign or attest.
- [x] The public docs state that confidence is residual-risk evidence, not
  correctness proof.

## P2: Attestation and Signing

- [ ] Add Sigstore keyless signing path for local and CI runs.
- [x] Add GitHub artifact attestation integration in the composite action and
  workflow permission docs, with `attest: "true"` gated by repository policy.
- [x] Map bundle manifest fields to local/offline in-toto/SLSA-compatible
  predicates.
- [x] Add offline verification mode for downloaded bundles.
- [x] Document public-repo and private-repo attestation differences.
- [ ] Add signing identity and certificate metadata to bundle summaries.
- [x] Define plugin trust model: plugin identity, version, signature, sandbox
  boundary, receipt permissions, and tamper resistance.
- [x] Add SLSA Verification Summary Attestation output mode for Pramaan's
  final verifier decision.
- [x] Add redaction profiles: internal-full, reviewer-redacted, public-demo,
  and summary-only.

## P2: Adversarial Corpus and Evals

- [x] Expand the adversarial corpus to 25 scenarios.
- [ ] Expand the adversarial corpus to 75 scenarios.
- [ ] Expand the adversarial corpus to 100+ scenarios mapped to risk IDs.
- [ ] Add real-world replay cases from open-source bug-fix PRs.
- [ ] Add flaky-case quarantine rules.
- [ ] Track false positives, false negatives, runtime, and reviewer time-to-understand.
- [x] Create a benchmark report template.
- [x] Add repo-level baseline calibration: expected mutation survival range, expected skipped-stage profile, runtime baseline, and noise-floor warnings.
- [x] Add trend/drift export across bundles: mutation survival drift, oracle-risk drift, skipped-stage drift, runtime drift, residual-risk families, and agent-author fields.
- [ ] Add benchmark-integrity mutation harness to detect agents overfitting eval tasks or hidden-test assumptions.
- [x] Add secure-code corpus categories for removed validation, weakened authorization, unsafe deserialization, injection sanitization removal, crypto misuse, and secret exposure.

## P2: Documentation and Adoption

- [x] Write an operator guide for running Pramaan in CI.
- [x] Write a plugin-author guide.
- [x] Write a security model.
- [x] Write a threat model for malicious PR authors and compromised tools.
- [x] Write an enterprise deployment guide.
- [x] Add troubleshooting docs for slow mutation, missing tools, flaky tests, and forked PR permissions.
- [x] Add screenshots or rendered examples of PR summaries and bundle inspection.
- [x] Document non-GitHub roadmap and minimum abstraction layer for GitLab, Gitea, and Bitbucket support.
- [x] Document GitLab artifact, identity, and OIDC differences before implementing GitLab support.

## P2: Feedback, Calibration, and Drift

- [x] Persist reviewer override decisions as first-class evidence, not comments that disappear in PR history.
- [ ] Correlate override outcomes with later defects or revert signals when available.
- [x] Store per-repo baselines for mutation survival, oracle warnings, skipped stages, runtime, and static/hallucination findings.
- [x] Expose a trend export format for weekly/monthly agent-code quality drift; a hosted trend API remains future work.
- [x] Add dashboard-ready JSON/CSV metrics without making the dashboard a blocker for CLI adoption.
- [x] Track agent-author attribution in multi-bundle exports to compare failure modes by agent, model, workflow, and repository; long-lived hosted analytics remain future work.

## P2: Verifier and Plugin Security

- [x] Define a plugin protocol with least-privilege receipt-writing
  permissions.
- [x] Require plugin identity, version, provenance, and optional signature in
  every plugin-emitted receipt.
- [x] Prevent plugins from editing prior receipts or bundle manifests directly.
- [ ] Run risky parsers, test runners, mutation engines, and fuzzers behind stronger sandbox boundaries.
- [x] Add malicious-plugin fixtures to the adversarial corpus.
- [x] Add malicious-PR scenario specs to the adversarial corpus; executable
  malicious-PR repos remain future fixtures.
- [x] Add bundle redaction policy tests for secrets, internal endpoints,
  private paths, and CI metadata.

## P3: Multi-Agent and Multi-Forge Support

- [x] Model multi-agent provenance chains: code author agent, review agent, test-writing agent, and final human reviewer.
- [ ] Record intermediate commit attribution and handoff metadata where available.
- [ ] Add provider-neutral VCS interfaces before adding GitLab support.
- [ ] Add GitLab CI support after GitHub Action readiness stabilizes.
- [ ] Add Gitea and Bitbucket notes as later adoption targets, not MVP blockers.

## P2: Language Expansion

- [x] Deepen Python plugin quality before adding more languages.
- [x] Deepen TypeScript plugin quality before adding more languages.
- [x] Deepen Rust plugin quality before adding more languages.
- [ ] Add Go support after plugin protocol stability.
- [ ] Add Java support after plugin protocol stability.
- [x] Keep each language plugin accountable for static checks, oracle integrity, mutation, fuzz/property, and fixture coverage.

## P3: Adapter Certification

- [x] Keep adapter certification as an adjacent mode, not a distraction from PR verification.
- [ ] Add certification checks for MCP tool names, descriptions, schemas, auth scopes, idempotency, retry behavior, rate limits, and auditability.
- [ ] Add adapter proof-bundle examples.
- [x] Add failure-mode taxonomy for agent tool adapters.
- [ ] Integrate adapter certification only after the core PR-verification bundle is trusted.

## Release Gates

### Alpha MVP

- [x] The weakened-test demo is undeniable.
- [ ] Pramaan runs successfully on at least three selected real repositories.
- [x] Pramaan runs successfully on internal oracle, mutation, Python fuzz, and TypeScript fuzz pilot fixtures with runtimes recorded.
- [ ] GitHub Action posts a useful PR summary.
- [x] Bundle verification catches tampering.
- [x] Missing tools and skipped checks are visible.
- [x] Receipt schema includes agent attribution and reviewer override fields before v0.1 freeze.
- [x] PR runtime SLA is documented and enforced through stage budgets.
- [x] Default policy profile can explain hard-fail vs warning-only decisions.

Alpha remains blocked by the unchecked items above. Do not describe the repo as
public-Alpha-ready until those are complete.

### Real MVP

- [ ] Python, TypeScript, and Rust paths have real tool integrations.
- [x] Oracle integrity catches weakened assertions, skipped tests, and snapshot/fixture drift.
- [ ] Mutation and property/fuzz stages run within practical CI budgets.
- [ ] At least 75 adversarial scenarios exist.
- [x] Documentation is good enough for an external maintainer to install and inspect a bundle in private technical preview.
- [ ] Auditable confidence vote is decomposed, signed, and clearly labeled as
  risk evidence rather than correctness proof.
- [x] Repo-level calibration prevents obvious alert fatigue in local baseline JSON/CSV mode.
- [x] Plugin trust model prevents untrusted plugins from poisoning receipts.
- [x] CI hardening checks catch unsafe workflow patterns for untrusted PR code.
- [ ] Redaction profiles are tested before any bundle is safe to export.

### Serious v1

- [ ] Production-grade orchestrator with parallel scheduling and stage budgets.
- [ ] Hardened sandbox and environment evidence.
- [ ] Sigstore/GitHub attestation support.
- [ ] 100+ adversarial scenarios mapped to risk IDs.
- [ ] Cross-platform CI.
- [x] Security model and threat model complete for current private-preview trust boundaries.
- [ ] Public demo proves "GitHub green, Pramaan red" in under 30 seconds.
- [x] Reviewer overrides, agent attribution, baseline calibration, and drift exports are part of the proof-bundle lifecycle.
- [ ] Confidence model is calibrated on pilot data and reports Brier/log-loss
  or equivalent calibration evidence.
- [x] PII/secrets scrubbing is tested before enterprise bundle export.
- [x] Pramaan can emit a VSA-style verification summary attestation.
