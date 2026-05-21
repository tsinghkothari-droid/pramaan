import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const errors = [];

function read(rel) {
  const abs = path.join(root, rel);
  if (!fs.existsSync(abs)) {
    errors.push(`missing ${rel}`);
    return "";
  }
  return fs.readFileSync(abs, "utf8");
}

function requireTokens(rel, tokens) {
  const text = read(rel);
  for (const token of tokens) {
    if (!text.includes(token)) errors.push(`${rel} missing token: ${token}`);
  }
}

[
  "docs/operator-guide.md",
  "docs/plugin-author-guide.md",
  "docs/security-model.md",
  "docs/threat-model.md",
  "docs/enterprise-deployment.md",
  "docs/troubleshooting.md",
  "docs/rendered-examples/README.md",
  "docs/rendered-examples/pass-summary.md",
  "docs/rendered-examples/warning-summary.md",
  "docs/rendered-examples/fail-summary.md",
  "docs/rendered-examples/bundle-inspection.md",
  "docs/language-readiness-gates.md",
  "docs/adapter-certification.md",
  "schemas/adapter_certification.schema.json",
  "examples/fixtures/adapter_certification.synthetic.json",
].forEach((rel) => read(rel));

requireTokens("docs/language-readiness-gates.md", [
  "Python",
  "TypeScript",
  "Rust",
  "Go and Java remain blocked",
  "Missing tools must emit skipped or residual-risk receipts",
  "Full compiler-AST parsing is a separate hardening gate",
]);

requireTokens("docs/adapter-certification.md", [
  "bounded Pramaan mode",
  "exposed tools",
  "auth",
  "idempotency",
  "retry",
  "rate-limit",
  "Registry and Sutra ideas remain deferred",
]);

requireTokens("schemas/adapter_certification.schema.json", [
  "pramaan.adapter_certification.v1",
  "declared_tools",
  "side_effect_profile",
  "risk_summary",
]);

const plugins = [
  "plugins/python/README.md",
  "plugins/python/mutation/README.md",
  "plugins/python/fuzz/README.md",
  "plugins/python/oracle/README.md",
  "plugins/typescript/README.md",
  "plugins/typescript/mutation/README.md",
  "plugins/typescript/fuzz/README.md",
  "plugins/typescript/oracle/README.md",
  "plugins/rust/README.md",
  "plugins/rust/mutation/README.md",
];
for (const rel of plugins) read(rel);

if (errors.length > 0) {
  console.error("Phase 15 docs/language gates check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Phase 15 docs/language gates ok");
