# Reviewer UX

Pramaan's first reviewer surface is local and static. It is not a hosted
dashboard and it is not a merge authority.

Generate a PR-comment-ready report:

```powershell
cargo run -p pramaan-cli -- report markdown --bundle target/pramaan --out target/pramaan/reviewer-report.md
```

Generate a local HTML report:

```powershell
cargo run -p pramaan-cli -- report html --bundle target/pramaan --out target/pramaan/reviewer-report.html
```

The report is organized for a 30-second pass:

1. Blockers
2. Warnings
3. What Ran
4. What Skipped
5. What Changed In Tests
6. Replay Commands
7. Human Override

The HTML command wraps the same markdown evidence in a portable static page so
the reviewer can inspect it without a server. The GitHub Action summary now
uses the same hierarchy for blockers, warnings, ran/skipped stages, and human
override fields.

## Human Override

Override data should be captured explicitly rather than hidden in comments:

| Field | Meaning |
| --- | --- |
| Accepted risk IDs | Risk IDs the reviewer knowingly accepts. |
| Reason | Short justification tied to repo context. |
| Reviewer identity source | GitHub actor, SSO identity, or local operator. |
| Timestamp | When the override was made. |

An override records judgment; it does not erase the original receipt evidence.
