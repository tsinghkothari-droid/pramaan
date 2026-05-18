# Phase 3: Oracle Integrity and Killer Demo - Context

**Status:** Ready for execution after Phase 2

Phase 3 is the product's first "this is real" moment. It detects test weakening, deleted/skipped tests, snapshot/fixture oracle changes, and claim-vs-test mismatch. It must produce a demo where ordinary CI is green but Pramaan fails the PR.

Locked decisions:

- Oracle integrity is execution-gating.
- Claim scope is the comparison base for narrow/wide oracle risk.
- Python and TypeScript assertion weakening are required for v1.
- The demo should be simple, inspectable, and mapped to top-100 risk IDs.
