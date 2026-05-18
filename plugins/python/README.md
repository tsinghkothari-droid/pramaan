# Pramaan Python Plugin

This directory is reserved for the future Python verification plugin.

Phase 1 does not implement real Python checks. The intended boundary is:

- run configured Python compile, type, lint, mutation, and differential checks;
- convert tool output into Pramaan stage receipts;
- report skipped or not-applicable checks explicitly when tooling is absent;
- never replace execution evidence with an LLM-only judgment.
