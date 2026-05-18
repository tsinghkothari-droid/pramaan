# Pramaan Python Mutation Adapter

Phase 4 mutation skeleton implemented by `pramaan mutation`.

Discovery:

- Python source is selected from `--changed-file` values ending in `.py`.
- If no changed files are supplied, Pramaan discovers Python files under the
  repository while ignoring `.git`, `target`, `node_modules`, virtualenvs, and
  `__pycache__`.
- The adapter runs `mutmut run --paths-to-mutate <changed files>` when mutmut is
  available.

Receipt behavior:

- no Python source => `not_applicable`;
- missing `mutmut` => `skipped` with timeout, filter, cache, skipped, and risk
  metadata preserved;
- command output is normalized into killed, survived, timed-out, unviable, and
  skipped mutant counts;
- survivor lines are classified as `review`, `test_gap`, or
  `likely_equivalent` where the output contains enough signal.

Risk coverage:

- R-068 records equivalent/survivor classification;
- R-069 records diff-scoped filtering and kill-threshold evidence;
- R-070 records cache mode and input metadata;
- R-071 records whether changed tests were present in the scoped file set;
- R-072 preserves timeout and unviable counts separately.
