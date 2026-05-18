# Python oracle heuristics

Pramaan's Phase 3 Python oracle pass discovers `test_*.py` and `*_test.py` files, fingerprints each `test_*` function, and flags review-worthy weakening:

- deleted tests;
- added `pytest.mark.skip`, `pytest.mark.xfail`, or `pytest.skip`;
- reduced `pytest.mark.parametrize` cases;
- weaker assertions such as equality or containment checks becoming truthy checks;
- removed `pytest.raises` exception oracles.

The Rust engine owns execution and receipts; this plugin folder documents the language-specific contract for future parser-backed adapters.
