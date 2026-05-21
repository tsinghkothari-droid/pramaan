# Fail Summary Example

```text
Pramaan Proof Bundle

Final status: failed
Policy profile: private-preview

Blocking stage:
- oracle_integrity failed

Why:
- test_login_rejects_invalid_token was changed from a strict equality
  assertion to a truthiness assertion.
- pytest skip marker was added to the original failing regression test.

Residual risk families:
- R-031 test_oracle_weakened
- R-032 skipped_test_added

Reviewer action:
- Restore the original failing test unchanged.
- Fix the production behavior.
- Rerun Pramaan and inspect the new oracle receipt.
```
