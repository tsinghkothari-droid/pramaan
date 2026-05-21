# Phase 36: Language Plugin Depth for Python, TypeScript, and Rust

## Goal

Deepen the first three language paths before adding Go or Java.

## Research Drivers

- The product moat is reliable, language-specific evidence depth, not a long
  list of shallow language logos.
- Secure-code studies show vulnerability patterns vary by language and
  framework.

## Tasks Covered

- Python plugin quality.
- TypeScript plugin quality.
- Rust plugin quality.
- Accountability for static checks, oracle integrity, mutation, fuzz/property,
  and fixtures in each plugin.

## Files to Change

- `plugins/python/`
- `plugins/typescript/`
- `plugins/rust/`
- `crates/pramaan-core/`
- `examples/`
- `docs/languages.md`
- `TASKS.md`

## Implementation Steps

1. Define per-language support matrices with implemented, partial, skipped, and
   planned cells.
2. Add framework-sensitive fixtures for pytest, unittest, Jest, Vitest, cargo
   tests, and snapshot libraries.
3. Improve changed-function detection and diff scoping per language.
4. Add tool availability and version fingerprint tests.
5. Keep Go/Java blocked until support matrices are credible.

## Verification

- Each language has fixture coverage for static, oracle, mutation, and
  property/fuzz evidence.
- Missing tools remain visible as skipped evidence.
- Language docs match claim audit status.

## Exit Criteria

Python, TypeScript, and Rust are credible supported lanes rather than thin demo
paths.
