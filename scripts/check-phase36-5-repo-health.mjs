import fs from "node:fs";

const required = [
  ["crates/pramaan-cli/src/main.rs", "about = \"Run the receipt-first verification pipeline"],
  ["crates/pramaan-cli/src/main.rs", "ExportCommands::Redacted"],
  ["crates/pramaan-cli/src/main.rs", "blockers: Vec<String>"],
  ["crates/pramaan-cli/src/static_checks.rs", "run_static_checks_quiet"],
  ["crates/pramaan-cli/src/static_checks.rs", "CARGO_TARGET_DIR"],
  ["crates/pramaan-cli/src/oracle.rs", "run_oracle_quiet"],
  ["crates/pramaan-cli/src/mutation.rs", "run_mutation_quiet"],
  ["crates/pramaan-cli/src/fuzz.rs", "run_fuzz_quiet"],
  ["crates/pramaan-core/src/lib.rs", "extract_static_diagnostic_codes"],
  ["crates/pramaan-core/src/risks.rs", "ORACLE_WEAKENED_ASSERTION"],
  ["SECURITY.md", "Reporting A Vulnerability"],
  ["CHANGELOG.md", "v0.1.1"],
  [".github/workflows/release.yml", "actions/attest-build-provenance@v2"],
  ["README.md", "pramaan-terminal-evidence.svg"],
  ["docs/quickstart.md", "cargo install pramaan-cli"],
  [".planning/phases/36.5-repo-health-pass-after-v0.1.1/SUMMARY.md", "PASS_WITH_RISKS"],
];

const missing = [];
for (const [file, token] of required) {
  if (!fs.existsSync(file)) {
    missing.push(`${file}: missing file`);
    continue;
  }
  const text = fs.readFileSync(file, "utf8");
  if (!text.includes(token)) missing.push(`${file}: missing token ${JSON.stringify(token)}`);
}

if (fs.existsSync("assets/readme/pramaan-generated-hero.png")) {
  missing.push("assets/readme/pramaan-generated-hero.png should be removed");
}

if (missing.length > 0) {
  console.error("Phase 36.5 repo-health validation failed:");
  for (const item of missing) console.error(`- ${item}`);
  process.exit(1);
}

console.log("Phase 36.5 repo-health validation ok.");
