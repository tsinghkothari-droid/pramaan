# Autonomous GSD Prompt: Finish Every Pramaan Phase Before 36

Use this prompt in a fresh Codex/Claude Code window when you want the agent to
continue Pramaan before starting Phase 36 language-depth work.

## Paste-Ready Prompt

```text
You are working in the Pramaan repository:

  C:\Users\Tushar\Pictures\pramaan

Use $gsd-execute-phase.

Goal:
Finish every Pramaan GSD phase before Phase 36 that is planned but not honestly
implemented. Do not begin Phase 36 until all pre-36 blockers below are either
completed, split into smaller phases with blockers documented, or explicitly
marked as impossible with evidence.

Context:
Pramaan is a receipt-first verification system for AI-authored pull requests.
It produces auditable evidence bundles, not correctness proofs. Keep every
README, TASKS, STATUS, roadmap, and release claim honest.

Already executed and pushed:
- Phase 26 external local pilots
- Phase 26.5 agent harness interface
- Phase 27 parser-backed oracle subset
- Phase 28 recorded-case replay
- Phase 28.25 AI evidence-seeking probe planning
- Phase 28.5 auditable confidence bridge
- Phase 29 offline attestation/VSA
- Phase 30 redacted bundle export
- Phase 31 plugin trust
- Phase 32 SARIF/Rego/workflow security
- Phase 32.5 built-in policy packs
- Phase 33 adversarial corpus v0.1 with 25 scenarios
- Phase 34 feedback, baselines, drift, and calibration exports
- Phase 35 operator docs, rendered examples, troubleshooting, and release checklist

Execute remaining pre-36 phases in this exact order:

1. Phase 26.1: Live GitHub Action Proof
2. Phase 26.2: Competitive Benchmark and Prior-Art Matrix
3. Phase 26.3: Competitor-Gap Fixtures
4. Phase 26.4: Minimum Lovable Verifier Loop
5. Phase 27.1: Full Compiler AST Oracle Extractors
6. Phase 28.1: Safe Hypothesis / fast-check Harness Execution
7. Phase 28.26: Sandbox Execution for Generated Probes
8. Phase 32.75: Anti-Gaming and Verifier-Abuse Hardening
9. Phase 35.5: Reviewer UX and Local HTML Report

For each phase:
1. Read the phase plan in `.planning/phases/<phase>/01-PLAN.md`.
2. Read the relevant sections of `TASKS.md`, `.planning/ROADMAP.md`,
   `.planning/STATE.md`, `STATUS.md`, and docs touched by that phase.
3. Execute real code/docs/test work, not only planning text.
4. Update `TASKS.md` honestly. Do not mark a task complete unless code/docs/tests
   or checked fixtures support it.
5. Update `.planning/ROADMAP.md` and `.planning/STATE.md` when phase state
   changes.
6. Add or update `.planning/phases/<phase>/SUMMARY.md` with:
   - what landed
   - what was deferred
   - verification commands and results
   - residual risks
7. Run verification:
   - `cargo fmt --check`
   - `cargo test --workspace`
   - `cargo clippy --workspace -- -D warnings`
   - `node scripts/check-claim-audit.mjs`
   - `node --test action/render-summary.test.mjs`
   - any phase-specific validator introduced by the phase
8. Commit each phase separately with a clear message.
9. Push to GitHub after each successful phase.

If a phase cannot be completed honestly:
- split it into the smallest useful follow-up phase;
- document the blocker in the phase SUMMARY and `TASKS.md`;
- commit the planning/blocker update;
- continue only if the next phase is safe and does not depend on the blocked
  implementation.

Non-negotiable honesty rules:
- Pramaan produces evidence, not correctness proof.
- Missing tools, skipped stages, timeouts, and unavailable attestations are
  visible residual risk, not success.
- AI-generated probes do not count as mitigation until executed in a sandbox.
- Do not claim public Alpha before Phase 26.1 has live GitHub Action evidence.
- Do not claim full AST support until Phase 27.1 has real parser/compiler-backed
  fixtures.
- Do not claim real property/fuzz campaigns until Phase 28.1 executes
  Hypothesis/fast-check under budgeted harnesses.
- Do not claim generated-probe evidence until Phase 28.26 executes probes safely.
- Do not claim the reviewer UX is solved until Phase 35.5 produces local HTML
  and markdown reports that show blockers, warnings, skipped stages, oracle
  changes, replay commands, and override fields.

Before final response:
- run `git status --short`;
- summarize completed phases, blocked phases, commit hashes, pushed branch, and
  remaining work before Phase 36;
- keep unrelated user/local changes out of commits unless they are part of the
  phase being completed.
```

## Why This Exists

Phase 36 should deepen Python, TypeScript, and Rust plugins. That work should
not start while earlier adoption, live-proof, AST, fuzz, anti-gaming, and
reviewer-report gaps are still floating as loose plans.
