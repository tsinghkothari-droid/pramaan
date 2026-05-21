#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const publicFiles = [
  "README.md",
  "STATUS.md",
  "TASKS.md",
  "docs/PRAMAAN_INTENT.md",
  "docs/agent-harness.md",
  "docs/competitive-benchmark.md",
  "docs/quickstart.md",
  "docs/reviewer-interface.md",
];

const riskyNames = [
  "PR-Agent",
  "Qodo",
  "reviewdog",
  "Pynguin",
  "EvoSuite",
  "Testomatio",
  "OpenReview",
];

const failures = [];
for (const file of publicFiles) {
  const fullPath = path.join(root, file);
  if (!fs.existsSync(fullPath)) {
    failures.push(`${file}: missing public doc`);
    continue;
  }
  const text = fs.readFileSync(fullPath, "utf8");
  for (const riskyName of riskyNames) {
    if (text.includes(riskyName)) {
      failures.push(`${file}: contains adjacent-project name ${riskyName}`);
    }
  }
}

if (failures.length > 0) {
  console.error("License-safe public docs check failed:");
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log("License-safe public docs ok");
