# Policy Packs

Pramaan policy packs are deterministic gate profiles. They are not LLM judges,
and they cannot override hard evidence such as weakened tests or bundle tamper.

## Built-In Profiles

| Profile | Use |
| --- | --- |
| `startup-fast` | Fast feedback for small teams. Failed/error stages block; timeouts are easier to treat as follow-up. |
| `open-source-maintainer` | Public OSS review where skipped tools must be visible but not every missing local tool should block. |
| `security-sensitive` | Auth, crypto, secrets, workflow, parser, and oracle-sensitive changes. Escalates security-sensitive and agentic workflow-injection risks. |
| `fintech-strict` | Regulated/high-risk code paths. Requires deeper fuzz/mutation evidence and hard-fails skipped/not-applicable stages. |
| `private-preview` | Honest early-pilot mode. This is the default because Pramaan is still pre-Alpha. |

List profiles:

```powershell
cargo run -p pramaan-cli -- policy list
```

Explain a bundle with a specific profile:

```powershell
cargo run -p pramaan-cli -- policy explain target/pramaan --profile security-sensitive
```

## GitHub Action

```yaml
- uses: pramaan/pramaan@v0
  with:
    policy-profile: security-sensitive
```

The profile currently affects policy explanation and exported decision
artifacts. The action's `fail-on` setting still controls job failure behavior
from the bundle `final_status`.
