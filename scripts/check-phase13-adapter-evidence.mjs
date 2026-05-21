#!/usr/bin/env node
import fs from "node:fs";

const required = [
  ["crates/pramaan-cli/src/mutation.rs", ["mutmut", "StrykerJS", "cargo-mutants", "raw_output_digest", "timeout_ms"]],
  ["crates/pramaan-cli/src/fuzz.rs", ["Hypothesis", "fast-check", "tool_generated_case_count", "raw_output_digest", "run_with_timeout"]],
  ["plugins/python/mutation/README.md", ["mutmut", "timeout", "changed"]],
  ["plugins/typescript/mutation/README.md", ["Stryker", "timeout", "changed"]],
  ["plugins/rust/mutation/README.md", ["cargo-mutants", "timeout"]],
  ["plugins/python/fuzz/README.md", ["Hypothesis", "seed", "replay"]],
  ["plugins/typescript/fuzz/README.md", ["fast-check", "seed", "replay"]],
  ["examples/fixtures/mutation/README.md", ["mutmut", "StrykerJS", "cargo-mutants"]],
  ["examples/fixtures/fuzz/claim_scope.json", ["expected_behavior", "out_of_scope_behavior"]],
];

const failures = [];
for (const [file, tokens] of required) {
  if (!fs.existsSync(file)) {
    failures.push(`${file}: missing`);
    continue;
  }
  const text = fs.readFileSync(file, "utf8");
  for (const token of tokens) {
    if (!text.includes(token)) {
      failures.push(`${file}: missing token ${token}`);
    }
  }
}

if (failures.length > 0) {
  console.error("Phase 13 adapter evidence check failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("Phase 13 adapter evidence ok");
