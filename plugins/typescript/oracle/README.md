# TypeScript and JavaScript oracle heuristics

Pramaan's Phase 3 JS/TS oracle pass discovers `*.test.*`, `*.spec.*`, and `__tests__` files, fingerprints `test()` and `it()` cases, and flags review-worthy weakening:

- deleted tests;
- added `test.skip`, `it.skip`, `test.todo`, or `it.todo`;
- reduced `test.each` or `it.each` cases;
- weaker `expect` or `assert` checks such as `toBe` or `toEqual` becoming truthy checks;
- removed throw, containment, comparison, or snapshot expectations.

The Rust engine owns execution and receipts; this plugin folder documents the language-specific contract for future parser-backed adapters.
