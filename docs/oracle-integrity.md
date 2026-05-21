# Oracle Integrity

Oracle integrity is Pramaan's clearest first use case: a pull request can make
ordinary CI green by weakening the test oracle instead of fixing the behavior.

## Current Extractor Contract

Phase 27 hardens the deterministic parser subset for Python, TypeScript, and
Rust test files:

| Language | Engine label | Covered subset |
| --- | --- | --- |
| Python | `python_indent_parser_v2` | `test_` functions, decorators, skip/xfail markers, `assert`, `pytest.raises`, unittest-style `.assert*`, multiline assertions. |
| TypeScript | `typescript_balanced_call_parser_v2` | `test`/`it`/`each`/`skip`/`todo` blocks, `expect`, `assert`, chained multiline matchers, snapshot-sensitive signals. |
| Rust | `rust_attribute_brace_parser_v2` | `#[test]`, async test attributes, `#[ignore]`, `#[should_panic]`, assertion macros, multiline macro assertions. |

The extractor strips comments and string literal contents before detecting skip
markers and assertions. This reduces false positives such as:

```text
"expect(value).toEqual(...)" inside a string
// it.skip("not a real skip")
```

Negative fixtures live under:

```text
examples/oracle-integrity/parser-negative-fixtures/
```

## Honest Limit

This is parser-backed subset evidence, not a full compiler AST proof. Unsupported
syntax must remain visible as residual risk until a future phase adds full
compiler/parser integrations with dependency justifications.

Receipts should keep the wording precise:

- say "parser-backed subset" for the current extractor;
- keep "full compiler AST-backed oracle extractors" marked planned;
- preserve `R-011`, `R-014`, `R-020`, and `R-087` when weakened or changed
  oracle evidence remains for review.
