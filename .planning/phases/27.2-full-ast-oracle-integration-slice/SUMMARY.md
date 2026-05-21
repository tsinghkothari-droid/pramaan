# Phase 27.2 Summary - Full AST Oracle Integration Slice

Status: `PLANNED_SPLIT`

Created on: 2026-05-21

This phase was split from the Phase 27.1 residual. The current repository has
parser-backed subset extractors with parser metadata, but not full compiler AST
integrations.

The next executable slice should start with Python AST via a bounded subprocess
helper, then use that protocol to decide whether TypeScript compiler API or Rust
rust-analyzer/syn comes next.

No production claim changes yet.
