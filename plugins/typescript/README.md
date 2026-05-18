# Pramaan TypeScript Plugin

This directory is reserved for the future TypeScript and JavaScript verification plugin.

Phase 1 does not implement real TypeScript checks. The intended boundary is:

- run configured type, lint, oracle, mutation, and differential checks;
- convert tool output into Pramaan stage receipts;
- report skipped or not-applicable checks explicitly when tooling is absent;
- never claim a change is correct, only what evidence was produced.
