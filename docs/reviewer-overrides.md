# Reviewer Overrides

Human reviewers can approve a bundle with residual risk, but the decision must
not disappear into a PR comment. Phase 34 records overrides as first-class JSON
evidence.

```powershell
cargo run -p pramaan-cli -- feedback override `
  --bundle target/pramaan `
  --stage oracle_integrity `
  --risk R-014 `
  --reviewer github:user/octocat `
  --reason "Accepted because the changed path is staging-only" `
  --linked-outcome merged
```

The command writes:

```text
target/pramaan/feedback/reviewer-override.json
```

The override includes:

- bundle ID and manifest digest;
- stage;
- accepted risk IDs;
- decision;
- reviewer identity source;
- timestamp;
- rationale;
- linked outcome when known;
- whether this should update calibration.

Agents may surface the override option, but they must not grant it. A reviewer
override is a signed-review workflow input, not a way for an agent to make red
evidence green.
