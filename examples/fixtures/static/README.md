# Static Adapter Fixtures

These tiny projects exercise Phase 2 static adapter discovery:

- `python/` has Python files plus a Ruff config so `compileall` can run and
  Ruff can either run or emit a skipped receipt when unavailable.
- `typescript/` has package and TypeScript configs plus a broken import for
  hallucination classification when `tsc` is available.
- `rust/` has a Cargo manifest plus a broken import so Cargo diagnostics can
  classify static hallucination evidence.
