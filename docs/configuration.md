# Pramaan Configuration

Pramaan now has a small `.pramaan.toml` reader for private-preview operator
defaults. It is intentionally narrow: command-line flags still win for explicit
one-off runs, and unsupported keys are ignored rather than treated as a stable
configuration contract.

Supported keys:

```toml
[policy]
profile = "private-preview"

[redaction]
profile = "reviewer-redacted"

[mutation]
enabled = false

[fuzz]
seed = 12345

[stages]
skip = ["static_checks", "oracle", "fuzz"]

[reports]
markdown = "target/pramaan/reviewer-report.md"
html = "target/pramaan/reviewer-report.html"
```

`pramaan verify --config .pramaan.toml` loads these defaults before running the
bundle. Repeated `--skip-stage` flags are merged with `[stages].skip`, and
`--with-mutation` still enables mutation even if the config does not.

Report paths in `[reports]` are written after verification using the same local
reviewer renderer as `pramaan report markdown` and `pramaan report html`.

Run `pramaan doctor --config .pramaan.toml --out doctor.json` before CI rollout
to record the parsed config, visible residual risks, and local tool
availability.

This config does not yet load external policy files, custom risk weights,
stage budgets, forge credentials, or production signing identity. Those remain
future hardening items.
