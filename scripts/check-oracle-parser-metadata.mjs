import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const diffPath = process.argv[2]
  ? path.resolve(root, process.argv[2])
  : path.join(root, "target", "pramaan-minimum-lovable", "oracle-diff.json");

const diff = JSON.parse(fs.readFileSync(diffPath, "utf8"));
const tests = [...(diff.base?.tests ?? []), ...(diff.head?.tests ?? [])];
const errors = [];

if (tests.length === 0) {
  errors.push("oracle diff contains no tests");
}

for (const test of tests) {
  const extractor = test.extractor ?? {};
  if (typeof extractor.engine !== "string" || extractor.engine.trim() === "") {
    errors.push(`${test.stable_id}: missing extractor.engine`);
  }
  if (
    typeof extractor.evidence_label !== "string" ||
    !extractor.evidence_label.includes("parser_backed_subset")
  ) {
    errors.push(`${test.stable_id}: missing parser-backed subset evidence label`);
  }
  if (typeof extractor.parser_version !== "string" || extractor.parser_version.trim() === "") {
    errors.push(`${test.stable_id}: missing parser_version`);
  }
  if (extractor.parser_available !== true) {
    errors.push(`${test.stable_id}: parser_available must be true for current subset extractors`);
  }
  if (
    typeof extractor.fallback_reason !== "string" ||
    !extractor.fallback_reason.includes("Full compiler AST integration is not enabled")
  ) {
    errors.push(`${test.stable_id}: fallback_reason must keep full-AST residual risk visible`);
  }
  if (!Array.isArray(extractor.unsupported_syntax) || extractor.unsupported_syntax.length === 0) {
    errors.push(`${test.stable_id}: unsupported_syntax must be non-empty`);
  }
  if (typeof extractor.disagreement_count !== "number") {
    errors.push(`${test.stable_id}: disagreement_count must be numeric`);
  }
}

if (errors.length > 0) {
  console.error("Oracle parser metadata validation failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Oracle parser metadata ok");
console.log(`tests_checked: ${tests.length}`);
