# Risk Taxonomy

Pramaan uses stable risk IDs so receipts can describe mitigated, residual, and
skipped risk without collapsing the run into a single score.

The source register for Phase 1 planning is:

```text
.planning/research/TOP_100_FLAWS_AND_MITIGATIONS_2026-05-18.md
```

The synthetic fixture at `examples/fixtures/risk_taxonomy.synthetic.json` is a
small validation subset of that model. It is not the full register.

## Families

Risk IDs are grouped into families that line up with Pramaan stages:

| Range | Family | Typical owner |
| --- | --- | --- |
| R-001..R-010 | claim_scope | claim-scope receipt |
| R-011..R-020 | oracle_integrity | oracle checks |
| R-021..R-030 | sandbox_reproducibility | sandbox/worktree setup |
| R-031..R-040 | static_hallucination | static checks |
| R-041..R-050 | public_api_compatibility | API compatibility checks |
| R-051..R-060 | runtime_behavior | runtime and replay checks |
| R-061..R-070 | mutation_quality | mutation checks |
| R-071..R-080 | property_fuzz | property, fuzz, differential checks |
| R-081..R-090 | bundle_integrity | receipt and bundle integrity |
| R-091..R-095 | ci_supply_chain | CI and dependency trust |
| R-096..R-100 | demo_corpus | demo and evaluation corpus |

The CLI and bundle summaries report these families in four buckets:

- `mitigated`: the stage produced evidence that addresses the risk.
- `residual`: the risk remains open or only partially addressed.
- `skipped`: the stage deliberately skipped a check and records the still-open
  risk family.
- `not_applicable`: the risk family genuinely does not apply to the stage.

## Receipt References

Every stage receipt may include:

```json
{
  "mitigated_risks": ["R-001"],
  "residual_risks": ["R-049"],
  "not_applicable_risks": ["R-081"]
}
```

These arrays are review signals, not a mathematical proof. A risk can move from
residual to mitigated only when the stage has concrete evidence for that claim.

## Oracle Integrity Notes

The Phase 12 oracle engine treats `R-011..R-020` as partially mitigated when it
extracts concrete test-oracle evidence: deleted tests, renamed tests, added
skips/todos/xfails/ignores, parametrized case reductions, weakened assertion
signals, removed error paths, removed boundary cases, and changed or deleted
fixture/snapshot artifacts.

Current oracle extraction is deterministic and framework-aware by pattern, not
yet a complete AST proof. Receipts should therefore keep heuristic limitations
visible and may list the same family as both mitigated and residual when a
specific finding remains open for reviewer judgment.

## Top-100 Mapping

The top-100 register is the planning source of truth for risk ID meaning. The
Rust helper currently maps IDs to families by range so Phase 1 can summarize
synthetic receipts. Later phases should load the complete taxonomy from the
schema-backed register and validate that every receipt risk ID exists.

Until then, any new receipt language should cite risk IDs conservatively:

- Use a stable `R-000`-style ID only when it exists in the register.
- Mark uncovered checks as residual instead of implying they passed.
- Preserve failed, timed-out, and skipped receipts in the bundle.
- Avoid wording that says Pramaan proved code correctness.
