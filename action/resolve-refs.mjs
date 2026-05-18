#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

function parseArgs(argv) {
  const args = new Map();
  for (let index = 2; index < argv.length; index += 1) {
    const key = argv[index];
    if (!key.startsWith("--")) {
      throw new Error(`unexpected argument ${key}`);
    }
    args.set(key.slice(2), argv[index + 1] ?? "");
    index += 1;
  }
  return args;
}

function eventRefs(eventPath) {
  if (!eventPath || !fs.existsSync(eventPath)) {
    return {};
  }
  const event = JSON.parse(fs.readFileSync(eventPath, "utf8"));
  return {
    base: event.pull_request?.base?.sha,
    head: event.pull_request?.head?.sha,
  };
}

function appendOutput(path, pairs) {
  if (!path) {
    return;
  }
  const lines = Object.entries(pairs)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");
  fs.appendFileSync(path, `${lines}\n`);
}

export function resolveRefs({ baseInput, headInput, eventPath, env }) {
  const refs = eventRefs(eventPath);
  const base =
    baseInput ||
    env.PRAMAAN_BASE_REF ||
    refs.base ||
    env.GITHUB_BASE_REF ||
    "HEAD~1";
  const head =
    headInput ||
    env.PRAMAAN_HEAD_REF ||
    refs.head ||
    env.GITHUB_HEAD_REF ||
    env.GITHUB_SHA ||
    "HEAD";

  return { base_ref: base, head_ref: head };
}

if (process.argv[1] && path.resolve(fileURLToPath(import.meta.url)) === path.resolve(process.argv[1])) {
  const args = parseArgs(process.argv);
  const refs = resolveRefs({
    baseInput: args.get("base") ?? "",
    headInput: args.get("head") ?? "",
    eventPath: process.env.GITHUB_EVENT_PATH,
    env: process.env,
  });
  appendOutput(args.get("github-output"), refs);
  console.log(`Pramaan refs: base=${refs.base_ref} head=${refs.head_ref}`);
}
