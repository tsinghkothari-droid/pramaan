# Phase 9 Workstreams

| Workstream | Scope | Verification |
| --- | --- | --- |
| Receipt schema freeze | Replace richer aspirational schema with v0.1 runtime-compatible compact schema plus Phase 16a hooks | JSON parse and checked-in receipt serde compatibility |
| Fixture refresh | Update stale expected oracle receipt to current receipt shape | `checked_in_receipt_and_bundle_fixtures_are_serde_compatible` |
| Bundle path hardening | Reject manifest path escapes and ambiguous basename resolution | Rust bundle tests |
| Evidence completeness | Fail manifest build if receipt-declared file artifacts are missing | Rust bundle tests |
| Signing metadata tamper | Ensure manifest-signing metadata edits trigger digest failure | Rust bundle test |
| Docs | Document compatibility and verification limits | Markdown link check |
