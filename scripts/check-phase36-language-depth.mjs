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

requireTokens("docs/languages.md", [
  "Python",
  "TypeScript",
  "Rust",
  "compileall",
  "pyright",
  "StrykerJS",
  "cargo-mutants",
  "Hypothesis",
  "fast-check",
  "Go and Java remain blocked",
]);

requireTokens("docs/language-readiness-gates.md", [
  "Missing tools must emit skipped or residual-risk receipts",
  "Full compiler-AST parsing is a separate hardening gate",
]);

requireTokens("crates/pramaan-cli/src/static_checks.rs", [
  "python-compileall",
  "python-ruff",
  "python-mypy",
  "python-pyright",
  "typescript-tsc",
  "rust-cargo-check",
  "rust-cargo-clippy",
]);

requireTokens("crates/pramaan-core/src/lib.rs", [
  "python_indent_parser_v2",
  "typescript_balanced_call_parser_v2",
  "rust_attribute_brace_parser_v2",
  "normalize_mutmut_output",
  "normalize_stryker_output",
  "normalize_cargo_mutants_output",
  "pub enum FuzzLanguage",
  "Hypothesis",
  "FastCheck",
]);

requireTokens("crates/pramaan-cli/src/fuzz.rs", [
  "Hypothesis",
  "fast-check",
  "tool_generated_case_count",
  "deterministic_input_count",
  "run_with_timeout",
]);

requireTokens("crates/pramaan-cli/src/mutation.rs", [
  "mutmut",
  "StrykerJS",
  "cargo-mutants",
  "raw_output_digest",
  "timeout_ms",
]);

for (const rel of [
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
  "plugins/rust/fuzz/README.md",
]) {
  read(rel);
}

if (errors.length > 0) {
  console.error("Phase 36 language depth check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Phase 36 language depth evidence ok");
