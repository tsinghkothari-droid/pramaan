# Example Proof Outputs

These directories contain checked-in Pramaan outputs for the public demo set.

They are evidence-shape examples, not correctness proofs and not replacement
for running the commands locally.

| Directory | Scenario | Expected signal |
| --- | --- | --- |
| `weakened-assertion/` | `ADV-001` | Oracle receipt fails with `weakened_assertion`. |
| `snapshot-fixture-drift/` | `ADV-006` | Oracle receipt fails with `sensitive_artifact_changed`. |
| `hallucinated-rust/` | `ADV-003` | Static Rust receipts fail on an unresolved invented import. |

Stage-specific commands currently emit receipt/artifact directories. Full signed
bundle manifests are covered by later bundle-hardening phases.

