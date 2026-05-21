# Phase 36.5: Repo Health Pass After v0.1.1

## Goal

Close the rough edges that surfaced when running the full diagnostic on main at
commit `5d0e61d` (post-`v0.1.1`). All automated gates are green; the issues
below are the ones the test suite cannot catch — UX gaps, perf gaps, missing
release artifacts, and known-debt items still flagged in `STATUS.md` as
`Planned` or `Partial`.

## Diagnostic Baseline (2026-05-21)

Run before adding anything new. Snapshot to keep in this plan:

| Check | Result |
| --- | --- |
| `cargo fmt --all -- --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `cargo test --workspace -- --test-threads=1` | 84 / 84 pass |
| `cargo test --workspace` (parallel) | 84 / 84 pass — old worktree flake is gone |
| `node --test action/render-summary.test.mjs` | 4 / 4 pass |
| `node scripts/check-claim-audit.mjs` | ok (66 claims, 34 STATUS rows) |
| `pramaan verify --base HEAD~3 --head HEAD --skip-stage static_checks` | emits 4 receipts, `final_status: passed` |
| `pramaan oracle` weakened-test demo | emits `weakened_assertion` finding |
| `pramaan oracle` snapshot-drift demo | emits 2 `sensitive_artifact_changed` findings |
| `pramaan report markdown` | emits blockers/warnings/what-ran/what-skipped sections |
| `pramaan doctor` | emits JSON, warns when tools or `.pramaan.toml` missing |

**No code is currently broken in the green-build sense.** The findings below
are everything else worth a separate hardening pass before scope expansion
continues.

## Findings (Grouped by Impact)

### Group A — UX gaps that hide capabilities from a first-time user

1. **Subcommand `--help` text is empty.** `pramaan bundle --help`,
   `pramaan agent --help`, `pramaan report --help`, `pramaan probe --help`,
   `pramaan export --help`, `pramaan feedback --help` list subcommands with no
   `about` strings. A reader cannot tell what `bundle export-redacted`,
   `agent done-gate`, or `feedback override` do without reading source.
2. **Two top-level "export" surfaces are easy to confuse.**
   - `pramaan export sarif | rego` (top level)
   - `pramaan bundle export-redacted` (nested under `bundle`)
   Both contain "export" in the verb. First-time users will try
   `pramaan export --bundle ...` (fails) before discovering the redaction
   command lives under `bundle`.
3. **Verify output is five disconnected blocks of stdout.** Each stage
   (`static_checks`, `oracle`, `fuzz`, `mutation`) currently prints its own
   "Pramaan X complete" summary as it runs. The final orchestrator summary
   then prints again. Reviewers see ~30 lines of repeated headers.
4. **`pramaan doctor` mixes blocker warnings with constant-true warnings.**
   "Mutation remains opt-in; missing mutation evidence is residual risk"
   appears on every doctor run regardless of repo state. That trains
   reviewers to ignore the warnings list.

### Group B — Performance / adoption friction

5. **`pramaan verify` runs a full `cargo check` on the head worktree on every
   invocation.** The sandbox worktree has no build cache, so this is a
   cold-from-zero compile (~2 minutes on this repo). Phase B added
   `--skip-stage static_checks` as a workaround. The real fix is to share or
   pre-warm a target directory keyed on toolchain version, or to run static
   checks against the source repo when it matches head (the common PR case).
6. **`v0.1.1` is tagged but there are no release binaries.** Adopters cannot
   `cargo install pramaan-cli` (not published to crates.io) and cannot
   download a prebuilt CLI from the release page. The only install path is
   `git clone && cargo build`. This is the #1 reason an evaluator bounces.
7. **GitHub Action wraps the CLI but has not been proven on a live PR**
   (Phase 26.1 still marked alpha gate in `ROADMAP.md`). Without a recorded
   run URL + downloadable bundle artifact, the action is unverified
   infrastructure.

### Group C — Known-debt items still labeled `Planned` or `Partial`

8. **Full compiler AST oracle extractors remain `Planned`** (Phase 27.1).
   Hand-rolled per-language parsers in `pramaan-core/src/lib.rs` cover the
   parser-backed subset but miss edge cases (multi-line decorators,
   macros, JSX, `cfg`-gated tests). Each language's edge cases will silently
   fail to be detected as oracle weakening.
9. **Hallucination classification is still substring-matching English error
   text** in `pramaan-core/src/lib.rs::classify_static_hallucinations`. ruff,
   mypy, tsc, and cargo rev their error wording regularly. The classifier
   will silently rot. Real fix: parse `ruff --output-format=json`,
   `mypy --show-error-codes`, `cargo --message-format=json`, `tsc --pretty=false`
   and key off stable diagnostic codes.
10. **Named risk-ID constants in `pramaan_core::risks` only cover the 17 IDs
    that existed when Phase A4 landed.** Codex's commits between Phase B and
    `v0.1.1` (probe planning, attestation, redaction, etc.) introduced new
    bare `"R-NNN".to_string()` literals that should migrate to the registry
    so typos still fail compilation.

### Group D — Release / supply-chain hygiene

11. **No `SECURITY.md`** in repo root. For a project whose whole pitch is
    supply-chain evidence, missing the disclosure policy is a credibility
    hole.
12. **No `CHANGELOG.md`** despite `v0.1.1` tagged. Receipt schema is now
    versioned (`pramaan.receipt.v1`); adopters need to know what changed and
    whether bundles stored from earlier versions still parse.
13. **README hero image is still AI-generated art**
    (`assets/readme/pramaan-generated-hero.png`). For an "evidence not vibes"
    project, the first image being a generated render is a tonal own-goal.
    Replace with a real screenshot (PR comment summary, JSON receipt with
    risk IDs highlighted, or asciinema of the killer demo).

## Tasks Covered

- Subcommand `about` strings on every `clap` `#[command]` derivation.
- Reconciliation of `export` vs `bundle export-redacted` command surface
  (rename, alias, or doc note).
- Quiet flag on stage runners (or shared logger) so `verify` orchestration
  produces one summary block.
- Doctor warning categorization: blockers vs informational vs constant.
- Static-check perf path: cache reuse or "checkout-equals-head" fast path.
- Release packaging: `cargo install` path, prebuilt artifacts in GitHub
  release, attestation on release artifacts.
- Live GitHub Action proof recording (Phase 26.1 gate completion).
- Migrating `classify_static_hallucinations` to structured tool output.
- Extending `pramaan_core::risks` named constants to cover all production
  literals after the Codex sprint.
- `SECURITY.md`, `CHANGELOG.md`.
- README hero image replacement.

## Files To Change

- `crates/pramaan-cli/src/main.rs` — `clap` `about` strings; doctor warning
  categories; suppress per-stage chatter when called from `run_verify`.
- `crates/pramaan-cli/src/{static_checks,oracle,mutation,fuzz}.rs` —
  optional `quiet: bool` argument or `eprintln!` instead of `println!` for
  non-terminal output.
- `crates/pramaan-core/src/lib.rs` —
  `classify_static_hallucinations` rewrite around structured codes;
  add diagnostic-code -> hallucination-category map.
- `crates/pramaan-core/src/risks.rs` — named constants for new IDs
  introduced after Phase A (probe, attestation, redaction stages).
- `crates/pramaan-cli/Cargo.toml` — `categories`, `keywords`, `description`,
  `readme` fields so the crate is publishable.
- `SECURITY.md` (new).
- `CHANGELOG.md` (new). Keep-a-Changelog format keyed to receipt schema
  versions.
- `README.md` — replace hero image, link to CHANGELOG and SECURITY.
- `assets/readme/` — drop in real screenshot; remove the generated PNG.
- `.github/workflows/` — add `release.yml` that builds binaries for
  Linux / macOS / Windows on tag push and attaches them to the release.
- `docs/quickstart.md` — add `cargo install pramaan-cli` path once published.
- `.planning/STATE.md` — record Phase 36.5 entry on completion.

## Implementation Steps

### Stability and UX (Group A)

1. Add `#[command(about = "...")]` to every subcommand derive in
   `crates/pramaan-cli/src/main.rs`. Cover: `verify`, `bundle`, `bundle
   verify`, `bundle attest`, `bundle verify-offline`, `bundle export-redacted`,
   `bundle cosign-plan`, `static-checks`, `oracle`, `mutation`, `fuzz`,
   `policy explain`, `confidence explain`, `agent done-gate`, `agent explain`,
   `probe plan`, `probe execute`, `replay`, `export sarif`, `export rego`,
   `feedback override`, `feedback analyze`, `report markdown`, `report html`,
   `doctor`.
2. Decide on the export naming inconsistency. Two reasonable resolutions:
   - Rename `bundle export-redacted` to `export redacted` so all export verbs
     live at the top level.
   - Keep `bundle export-redacted` and add a top-level alias
     `export bundle --profile redacted`.
   Document the chosen name and add the other as a deprecated alias for one
   minor release.
3. Add a `--quiet` flag on each stage runner that defaults to off when
   invoked standalone and on when invoked from `run_verify`. Stage runners
   stop printing summaries when quiet; the orchestrator alone produces the
   single end-of-run summary.
4. Split `pramaan doctor` output into `blockers`, `warnings`, and
   `informational`. The constant warning about mutation opt-in moves to
   `informational`.

### Performance (Group B5)

5. Detect "head ref equals current `HEAD` of source repo" case and run
   static checks against the source dir instead of the worktree to reuse
   the existing `target/` cache. Add a `head_matches_source` flag to
   sandbox evidence so the receipt records which path was used.
6. Where the source/head paths differ, set `CARGO_TARGET_DIR` to a
   pramaan-shared dir keyed on the head SHA so subsequent verify runs warm
   the cache.

### Release plumbing (Group B6, B7)

7. Add `release.yml` workflow: on tag push (`v*`), build CLI for
   `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`,
   `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc`.
   Attach binaries to the GitHub release. Use `actions/attest-build-provenance@v2`
   so the release artifacts themselves carry Pramaan's own evidence.
8. Fill in `crates/pramaan-cli/Cargo.toml` publish metadata (description,
   homepage, repository, license, readme, keywords). Publish to crates.io
   as part of the `v0.1.2` release after the rest of this phase lands.
9. Run the GitHub Action on at least one live PR to this repository.
   Record the run URL, the uploaded `pramaan-proof-bundle` artifact, and the
   rendered summary in `.planning/reports/phase-26.1-live-action-proof.md`.

### Known-debt items (Group C)

10. Rewrite `classify_static_hallucinations` to consume structured output:
    - `ruff check --output-format=json` -> code-to-category map by rule
      family (`F4*` -> `nonexistent_import`, `F8*` -> `undefined_symbol`).
    - `mypy --show-error-codes --no-color-output` -> parse `error: ... [code]`.
    - `cargo --message-format=json` -> parse `compiler-message` entries.
    - `tsc --noEmit --pretty=false` -> parse `path(L,C): error TSnnnn:`.
    Keep the substring path as a fallback only when structured output
    parsing fails, and label the receipt accordingly.
11. Grep production code for any remaining bare `"R-NNN".to_string()` calls
    added since `risks.rs` landed; promote each to a named constant. Extend
    the `every_named_constant_is_known` test to cover the new IDs.
12. Pick the parser strategy for Phase 27.1 (subprocess vs in-process
    tree-sitter / rustpython-parser / tsc subprocess). Land at least one
    language as a proof of concept and migrate the rest in follow-ups.

### Release hygiene (Group D)

13. Author `SECURITY.md` covering supported versions, reporting channel
    (email or GitHub security advisory), and the disclosure window.
14. Author `CHANGELOG.md` (Keep-a-Changelog). Backfill entries for
    `v0.1.0` and `v0.1.1` from `git log` so adopters can map receipt-schema
    versions to releases.
15. Replace the README hero image. Candidates: PR comment screenshot from
    step 9's live action run, terminal capture of the killer demo, or
    asciinema cast embedded as SVG.

## Verification

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace -- --test-threads=1`
- `cargo test --workspace` (parallel; baseline confirms no new flake)
- `node --test action/render-summary.test.mjs`
- `node scripts/check-claim-audit.mjs`
- `pramaan --help` and every subcommand `--help` show an `about` line.
- `pramaan verify --base HEAD~3 --head HEAD` produces one clean summary
  block (no per-stage stdout).
- `pramaan doctor` separates blockers / warnings / informational.
- Release workflow run on a `v0.1.2-test` tag attaches platform binaries
  with attestations.
- Live GitHub Action run on a PR is recorded and linked from
  `.planning/reports/phase-26.1-live-action-proof.md`.
- `SECURITY.md` and `CHANGELOG.md` exist and are linked from README.
- `assets/readme/pramaan-generated-hero.png` is replaced.

## Exit Criteria

A first-time visitor can:

- read each command's purpose from `pramaan <cmd> --help` without opening
  source code;
- install Pramaan in one command (`cargo install pramaan-cli` or download a
  release binary);
- run `pramaan verify` on a real PR and see one clean summary;
- watch the GitHub Action produce evidence on this repository's own PRs;
- find the release notes (CHANGELOG.md) and disclosure path (SECURITY.md)
  without searching;
- see a real screenshot in the README instead of an AI-generated render.

The repo's automated gates remain green, and the residual `Planned` items in
`STATUS.md` shrink by at least the structured-output classifier and
named-constants items.
