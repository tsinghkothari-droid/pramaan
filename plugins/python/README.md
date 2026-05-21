# Pramaan Python Static Adapter

Phase 2 static skeleton implemented by `pramaan static-checks`.

Discovery:

- `compileall`: applicable when `*.py` files are present; command is
  `python -m compileall -q .`.
- `ruff`: applicable only when Ruff config is found in `pyproject.toml`,
  `ruff.toml`, or `.ruff.toml`; command is `ruff check .`.
- `mypy`: applicable only when Mypy config is found in `pyproject.toml`,
  `mypy.ini`, or `.mypy.ini`; command is `mypy .`.
- `pyright`: applicable only when Pyright config is found in
  `pyrightconfig.json` or `pyproject.toml`; command is `pyright .`.

Receipt behavior:

- no Python files => `not_applicable`;
- configured check with missing executable => `skipped` with residual risk;
- command failure => `failed`, with broken import or undefined symbol metadata
  when diagnostics support that classification.

Depth status:

- oracle, mutation, and property/fuzz details live in the sibling plugin
  README files;
- missing tools must stay visible as skipped evidence;
- this is private-preview depth, not a claim that every Python framework is
  fully covered.
