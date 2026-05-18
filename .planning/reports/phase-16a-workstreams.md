# Phase 16a Workstreams

| Workstream | Scope | Verification |
| --- | --- | --- |
| Receipt hook types | Add optional trust-hook structs and fields to `pramaan-core::Receipt` | Core serialization test |
| Bundle aggregation | Carry trust hooks from receipts into stage and manifest summaries | Bundle manifest aggregation test |
| Runtime generated evidence | Populate trust hooks in the synthetic claim-scope receipt emitted by `pramaan verify` | CLI smoke test and generated-bundle assertion |
| Public schemas | Add hook definitions to receipt and bundle JSON Schemas | JSON parse check |
| Fixtures | Populate synthetic receipt and bundle fixtures with hook examples | Existing fixture tests plus JSON parse check |
| Skeptical review | Record what is complete and what is not freeze-ready | Phase 16a unbiased review |
