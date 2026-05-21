import fs from "node:fs";
import path from "node:path";

const root = process.cwd();

const requiredDocs = [
  "docs/operator-guide.md",
  "docs/plugin-author-guide.md",
  "docs/security-model.md",
  "docs/enterprise-deployment.md",
  "docs/troubleshooting.md",
  "docs/release.md",
  "docs/rendered-examples/README.md",
  "docs/rendered-examples/pass-summary.md",
  "docs/rendered-examples/warning-summary.md",
  "docs/rendered-examples/fail-summary.md",
  "docs/rendered-examples/bundle-inspection.md",
];

const failures = [];
for (const relative of requiredDocs) {
  if (!fs.existsSync(path.join(root, relative))) {
    failures.push(`Missing required Phase 35 doc: ${relative}`);
  }
}

const filesToCheck = ["README.md", ...requiredDocs];
const markdownLink = /!?\[[^\]]*\]\(([^)]+)\)/g;

for (const relative of filesToCheck) {
  const absolute = path.join(root, relative);
  if (!fs.existsSync(absolute)) continue;
  const text = fs.readFileSync(absolute, "utf8");
  for (const match of text.matchAll(markdownLink)) {
    const raw = match[1].trim();
    if (
      raw.startsWith("http://") ||
      raw.startsWith("https://") ||
      raw.startsWith("mailto:") ||
      raw.startsWith("#")
    ) {
      continue;
    }
    const target = raw.split("#")[0];
    if (!target) continue;
    const resolved = path.resolve(path.dirname(absolute), decodeURIComponent(target));
    if (!resolved.startsWith(root) || !fs.existsSync(resolved)) {
      failures.push(`${relative} has broken link: ${raw}`);
    }
  }
}

if (failures.length > 0) {
  for (const failure of failures) console.error(failure);
  process.exit(1);
}

console.log(`Phase 35 docs ok: ${requiredDocs.length} required docs, ${filesToCheck.length} files link-checked.`);
