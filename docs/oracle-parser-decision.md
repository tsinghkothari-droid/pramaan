# Oracle Parser Dependency Decision

Last updated: 2026-05-21

Phase 27.1 evaluated full compiler/parser AST extraction for the first three
languages. The decision for v0.1 is conservative:

| Language | Full-parser option | Decision | Reason |
| --- | --- | --- | --- |
| Python | CPython `ast` through subprocess helper | Defer as Phase 27.2 candidate | Good fit, but needs subprocess protocol, version capture, and fixture parity before replacing the Rust subset extractor. |
| TypeScript | TypeScript compiler API through Node subprocess | Defer as Phase 27.2/36 candidate | Correct option, but adds Node/toolchain dependency and must handle project `tsconfig` safely. |
| Rust | `syn`, `ra_ap_syntax`, or rust-analyzer subprocess | Defer as Phase 27.2/36 candidate | `syn` misses full macro expansion; rust-analyzer is heavier but closer to compiler behavior. Macro-generated tests must remain explicit residual risk. |

## What Shipped Now

Pramaan's oracle evidence now records parser metadata on every extracted test:

- `parser_version`
- `fallback_reason`
- `unsupported_syntax`
- `disagreement_count`

This makes the residual risk visible in receipts and `oracle-diff.json` instead
of hiding it behind an overbroad "parser-backed" label.

Validate metadata after an oracle run:

```powershell
node scripts/check-oracle-parser-metadata.mjs target/pramaan-minimum-lovable/oracle-diff.json
```

## Claim Boundary

Pramaan may say:

- oracle extraction is deterministic and parser-backed for the supported
  subset;
- parser metadata and unsupported syntax are recorded in oracle evidence;
- full compiler AST extraction remains a planned hardening path.

Pramaan must not say:

- Python, TypeScript, or Rust oracle extraction is full compiler AST backed;
- macro-generated, dynamically generated, or custom-wrapper tests are fully
  covered;
- parser metadata is equivalent to proof of complete test discovery.
