# Pramaan TypeScript Static Adapter

Phase 2 static skeleton implemented by `pramaan static-checks`.

Discovery:

- package manager: `pnpm` when `pnpm-lock.yaml` exists, `yarn` when
  `yarn.lock` exists, otherwise `npm`.
- type check: applicable when TypeScript files, `package.json`, and
  `tsconfig.json` are present; command is `<pm> exec tsc --noEmit`.
- lint: applicable when TypeScript files and a `package.json` lint script are
  present; command is `<pm> run lint`.

Receipt behavior:

- missing TypeScript/package/config/script surfaces become `not_applicable`;
- missing package-manager executable becomes `skipped`;
- command failures are normalized into Pramaan receipts and classified as
  broken imports or undefined symbols when diagnostics support it.
