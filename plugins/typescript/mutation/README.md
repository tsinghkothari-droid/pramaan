# Pramaan TypeScript Mutation Adapter

Phase 4 mutation skeleton implemented by `pramaan mutation`.

Discovery:

- JS/TS source is selected from `--changed-file` values ending in `.js`, `.jsx`,
  `.ts`, or `.tsx`.
- `package.json` must be present for the adapter to be applicable.
- Package manager detection follows the static adapter: `pnpm`, then `yarn`,
  otherwise `npm`.

Execution:

- npm projects run `npx stryker run --mutate <changed files> --incremental true`.
- pnpm/yarn projects run through the package manager's `exec` path.
- When Stryker writes `reports/mutation/mutation.json`, Pramaan prefers that
  structured report over console text.

Receipt behavior:

- missing source or package metadata => `not_applicable`;
- missing runner executable => `skipped`;
- Stryker statuses are normalized into killed, survived, timed-out, unviable,
  and skipped mutant counts;
- incremental cache intent, mutate pattern, timeout policy, and changed-test
  awareness are always recorded in receipt metadata.
