# Plugin Author Guide

Pramaan plugins are subprocess adapters. They call existing tools, emit
structured receipts, and stay inside a narrow trust boundary.

## What A Plugin May Do

- read the request payload from stdin or an assigned input file;
- run a language tool such as `mutmut`, StrykerJS, `cargo-mutants`,
  Hypothesis, or fast-check;
- write artifacts only under its assigned output directory;
- emit receipts that include `plugin_identity` and `plugin_permissions`.

## What A Plugin Must Not Do

- modify previous receipts;
- modify `bundle.manifest.json`;
- write absolute paths or `..` traversal paths into artifacts;
- hide missing tools as successful verification;
- claim mitigation for a risk unless tool-backed evidence exists.

The protocol schema is [plugin_protocol.schema.json](../schemas/plugin_protocol.schema.json).
The trust model and allowed permissions are summarized in [Plugin And Adapter
Evidence](plugins.md).

## Minimal Receipt Rules

Every plugin-emitted receipt should include:

- stable `schema_version`;
- stage and tool identity;
- `plugin_identity` with name, version, provenance, sandbox boundary, and
  optional signature;
- `plugin_permissions`;
- input hashes and artifact hashes for any evidence used in a policy decision;
- explicit skipped or residual risks when a tool is missing, timed out, or not
  applicable.

## Versioning

Plugins should version the adapter separately from the underlying tool. For
example, a Python mutation plugin can be `pramaan-python-mutation@0.1.0` while
recording the installed `mutmut` version in the receipt.

## Testing Checklist

- missing tool emits a skipped receipt, not a pass;
- timeout emits `timed_out` or residual risk;
- artifact paths are relative and canonical;
- dangerous permissions are rejected by bundle construction;
- receipts round-trip through the core receipt schema;
- policy summaries still expose skipped and residual risks.
