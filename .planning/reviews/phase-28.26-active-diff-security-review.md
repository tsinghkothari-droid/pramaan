# Phase 28.26 Active Diff Security Review

Date: 2026-05-21

## Verdict

FAIL_FOR_SECURITY_AND_FORMATTING.

## Current Status Note

This is a historical local review artifact for the active Phase 28.26 diff. The
follow-up public-review track later kept generated-probe execution explicitly
bounded/private-preview and warning-oriented rather than production sandboxing.
The core unresolved concern still stands for Serious v1: arbitrary generated
code must not be marketed as safely sandboxed until stronger OS/container
boundaries exist.

The active `crates/pramaan-cli/src/main.rs` diff appears to implement
`pramaan probe execute`. It compiles and tests pass, but it is not ready to
ship as a verifier feature because it executes generated probe code with only a
weak string filter and no real process/container sandbox.

## Findings

1. **High: generated probe code executes without a real sandbox.**
   - Evidence: `crates/pramaan-cli/src/main.rs:743` runs probe code through
     `run_probe_command`, which spawns `python`, `node`, or `rustc` directly.
   - Evidence: `crates/pramaan-cli/src/main.rs:805` only sets
     `PRAMAAN_PROBE_NETWORK=disabled`; that is metadata, not an enforced
     network or filesystem boundary.
   - Risk: provider/agent-generated code can read local files, write outside the
     probe directory with absolute paths, consume CPU, or exploit interpreter
     behavior. This violates the Phase 28.26 goal of isolated execution.

2. **High: the static danger filter is too weak to protect execution.**
   - Evidence: `crates/pramaan-cli/src/main.rs:856` blocks a short token list
     such as `socket`, `subprocess`, `child_process`, and URLs.
   - Missing examples include Python `open`, `pathlib`, `eval`, `exec`,
     `__import__`, Node `require("fs")`, `process`, encoded strings, and many
     indirect filesystem/network paths.
   - Risk: a malicious or sloppy generated probe can pass the marker check and
     still perform unsafe behavior.

3. **Medium: passing probes can be accepted without proving changed-behavior
   relevance.**
   - Evidence: `probe_binds_to_changed_behavior` at
     `crates/pramaan-cli/src/main.rs:869` accepts textual mentions of a risk ID,
     basename, or `pramaan-bind`.
   - Risk: a probe can contain the marker and a filename comment, exit 0, and be
     counted as kept even if it does not exercise the repository or the changed
     behavior.

4. **Medium: the execution receipt still reports the stage as passed.**
   - Evidence: `crates/pramaan-cli/src/main.rs:1167` marks the receipt
     `Passed` whenever probes exist, even if every executed probe is rejected or
     failed.
   - Risk: reviewer summaries may show a green probe stage even though probe
     execution produced no usable mitigation.

5. **Low: the active diff is not rustfmt-clean.**
   - Evidence: `cargo fmt --check` reports required formatting changes in
     `crates/pramaan-cli/src/main.rs`.
   - Risk: CI will fail once fmt is enforced, even though tests and clippy pass.

## Positive Evidence

- `cargo test --workspace`: PASS.
- `cargo clippy --workspace -- -D warnings`: PASS.
- The implementation records stdout/stderr digests instead of raw logs, which is
  a good redaction instinct.
- Timeout logic exists for the probe command itself, though process-tree and
  interpreter-level isolation remain open.

## Recommendation

Before completing Phase 28.26, require either a real sandbox boundary
(`cargo`/Python/Node inside a restricted temp workspace, container, or OS-level
job object policy) or mark probe execution as experimental and warning-only.
Do not allow accepted probes to mitigate risk until they both execute safely and
prove changed-behavior binding through stronger evidence than comments or token
mentions.
