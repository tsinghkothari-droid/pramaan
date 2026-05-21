# Pramaan Done Gate

Run this before telling the user a code task is complete:

```bash
pramaan agent done-gate --base "${BASE_REF:-HEAD~1}" --head "${HEAD_REF:-HEAD}" --out target/pramaan-agent
```

Then inspect:

```bash
cat target/pramaan-agent/agent-decision.json
```

If `decision` is `block`, stop and repair the blocking stages. If `decision` is
`warn`, report the residual risks explicitly. If `decision` is `pass`, summarize
the evidence without claiming the change is proven correct.
