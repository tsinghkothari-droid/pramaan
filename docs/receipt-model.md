# Receipt Model

Pramaan's v0.1 receipt model is deliberately modest: it records evidence about
a verification run. It does not certify that the changed code is correct.

## Terms

`ClaimScope` is the bounded statement of what the change appears to claim. In
Phase 1 synthetic runs it is generated from `--base` and `--head`; later stages
can replace that with PR title, PR body, issue links, and reviewer notes.

`Receipt` is a stage-level record. A receipt says which stage ran, what status it
reported, which inputs and outputs it referenced, which artifacts it produced,
and which risk IDs are mitigated, residual, or not applicable for that stage.

`ArtifactRef` and `OutputRef` are pointers to evidence. They should be reviewable
paths plus digests where the producing stage can calculate them. A path without a
digest is still useful during Phase 1, but it is weaker evidence.

`BundleManifest` is the run-level index. It should reference receipts and
artifacts so a reviewer or GitHub Action can inspect the run without reading
every file first.

## v0.1 Compatibility Rules

The v0.1 public contract is the compact runtime shape emitted by the Rust CLI:

- `schema_version` remains `pramaan.receipt.v1`.
- `stage` is a stable string ID such as `claim_scope`, `oracle_integrity`, or
  `differential_fuzz`.
- `started_at` and `ended_at` remain top-level RFC3339 timestamps.
- `summary` contains `title` and `details`.
- `inputs`, `outputs`, and `artifacts` are arrays of simple references with
  optional digests.
- risk buckets stay explicit: `mitigated_risks`, `residual_risks`, and
  `not_applicable_risks`.
- Phase 16a trust hooks are optional but reserved: `agent_author`,
  `reviewer_override`, `multi_agent_provenance`, `plugin_identity`,
  `plugin_permissions`, `evidence_sensitivity`, `redaction_manifest`,
  `policy_decision`, and `stage_budget`.

Compatible additions may add optional fields or new enum values only after a
schema-version decision. Incompatible changes include removing existing fields,
changing field types, hiding skipped/failed stage evidence, or replacing risk
buckets with a single score.

## How They Relate

The claim scope sets the target for later checks. Stage receipts then report
evidence against that target. The bundle manifest gathers those receipts into
one auditable directory.

```text
claim scope
  -> stage receipts
  -> evidence artifacts
  -> bundle manifest
  -> reviewer summary
```

The direction matters. Pramaan should not start with a verdict and then search
for supporting evidence. It should record what each stage actually observed and
leave residual risk visible.

## Synthetic Receipts

The current CLI command is:

```powershell
cargo run -p pramaan-cli -- verify --base HEAD --head HEAD --out target/pramaan-smoke
```

This writes a synthetic claim scope plus stage receipts. Those receipts exercise
the contract for status, artifact paths, risk references, and the Phase 16a trust
hooks. They do not prove static analysis, mutation testing, differential fuzzing,
or sandbox replay found the code correct.

## Compatibility Tests

Phase 9 pins the receipt contract in two ways:

- checked-in `*.receipt.json` fixtures under `examples/` must deserialize as
  current Pramaan receipts;
- CLI smoke tests assert that generated receipts and manifests still carry the
  expected fields.

This is a compatibility floor, not the final schema-validation story. Full JSON
Schema validation for every generated artifact remains a hardening target.

## Golden and Canonical Evidence

Phase 19 adds two stronger guardrails:

- generated smoke receipts are compared against approved golden JSON after
  volatile timestamps are normalized;
- bundle manifest digests are computed from canonical JSON bytes with sorted
  object keys instead of relying on pretty-printer output.

Receipt artifact digests still hash the exact bytes written to disk. That is
intentional for v0.1 because reviewers should be able to detect any byte-level
change to an emitted receipt. A later schema migration may split display JSON
from canonical signable payloads, but that must be explicit and fixture-backed.

To intentionally update a golden receipt, regenerate the relevant smoke fixture,
review the diff in the normalized expected JSON, and update the test and docs in
the same commit. Never update a golden fixture just to make a failing test green.

## Redaction Discipline

Receipts should not expose secrets or private infrastructure details when a
bundle is shared outside the producing CI environment. The redaction helper
covers common secret assignments such as `password=`, `token:`, `api_key=`,
`authorization:`, cache keys, and artifact URLs; common token prefixes; private
user paths such as `C:\Users\<name>`, `/Users/<name>`, and `/home/<name>`;
email-like values; private IPs; and internal hostnames.

Phase 30 adds explicit profiles: `internal-full`, `reviewer-redacted`,
`public-demo`, and `summary-only`. Exported redacted bundles get a
`bundle_redaction` receipt and a rebuilt manifest.

Redaction is not a substitute for least-privilege CI. Untrusted pull request
jobs should not receive secrets in the first place. Redaction makes accidental
bundle sharing safer; it does not make hostile code execution safe.

## Claim Scope Inputs

Phase 22 extends claim-scope evidence beyond CLI refs. The CLI can now consume:

- GitHub event title/body through `GITHUB_EVENT_PATH`;
- explicit `PRAMAAN_PR_TITLE` and `PRAMAAN_PR_BODY`;
- issue context through `PRAMAAN_ISSUE_TEXT` or `PRAMAAN_ISSUE_PATH`;
- maintainer scope notes through `PRAMAAN_SCOPE_NOTE`,
  `PRAMAAN_SCOPE_NOTE_PATH`, or `.pramaan-scope.md`.

When none of those sources exist, the claim scope is marked low confidence and
receives claim-scope risk references such as `R-001` and `R-002`. If public API
changes are detected but the claim text does not mention matching symbols,
Pramaan emits `R-007` as a bounded semantic mismatch signal. That signal should
inform review; it is not a sole merge gate.

## Oracle Extractor Evidence

Phase 23 adds structured oracle extractor evidence to `oracle-diff.json`; Phase
27 hardens that into parser-backed subset evidence for the supported test-file
patterns.
Each discovered test now carries:

- `extractor.engine`: the deterministic extractor used for that language;
- `extractor.evidence_label`: whether the evidence came from a structured
  parser or a fallback;
- `assertion_signals`: normalized assertion kinds, strength scores, and stable
  hashes for reviewer comparison;
- `skip_markers`: skip, xfail, todo, ignore, or panic markers found in the
  test block.

This improves the reviewer story without overclaiming full compiler AST
coverage. Current extractors are parser-backed subsets for Python, TypeScript,
and Rust, with comment/string filtering and multiline assertion grouping. Full
compiler-backed parser integrations remain a hardening target and should be
added only with fixtures and dependency justification.

## Mutation And Fuzz Evidence

Skipped mutation tools must not satisfy a mitigation gate. When `mutmut`,
StrykerJS, or `cargo-mutants` is missing or a language is not applicable,
mutation receipts keep `mitigated_risks` empty and place the mutation risk IDs
in `not_applicable_risks`.

Differential fuzz receipts record adapter availability separately from the
selected adapter. A deterministic replay adapter may still produce useful
base/head evidence, but it is labeled `tool_backed=false` until a safe
Hypothesis or fast-check harness actually executes.

## Claim Discipline

Receipts should use precise language:

- Say "this stage produced evidence" rather than "the change is safe".
- Say "residual risk remains" when a check did not run or could not cover a
  behavior.
- Say "not applicable" only when a risk family genuinely does not apply to the
  stage.
- Keep skipped and failed stages in the bundle so reviewers can audit absence as
  well as success.
