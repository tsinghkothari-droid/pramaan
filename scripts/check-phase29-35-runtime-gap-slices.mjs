import fs from "node:fs";

const required = [
  ["crates/pramaan-cli/src/main.rs", "Commands::Doctor"],
  ["crates/pramaan-cli/src/main.rs", "BundleCommands::CosignPlan"],
  ["crates/pramaan-cli/src/main.rs", "fn run_bundle_cosign_plan"],
  ["crates/pramaan-cli/src/main.rs", "fn run_doctor"],
  ["crates/pramaan-cli/src/main.rs", "fn parse_pramaan_config"],
  ["crates/pramaan-cli/tests/smoke.rs", "doctor_reports_config_and_tool_availability"],
  ["crates/pramaan-cli/tests/smoke.rs", "verify_loads_config_for_skips_seed_and_reports"],
  ["crates/pramaan-cli/tests/smoke.rs", "bundle_cosign_plan_records_readiness_without_claiming_identity_proof"],
  ["docs/configuration.md", "pramaan doctor"],
  ["docs/cosign-signing.md", "does not itself prove CI identity"],
  ["STATUS.md", "Cosign signing readiness plan"],
  ["STATUS.md", "Runtime doctor and config loading"],
  [".planning/phases/29.1-production-cosign-signing-slice/SUMMARY.md", "PASS_WITH_RISKS"],
  [".planning/phases/35.8-runtime-reviewer-commands-and-config/SUMMARY.md", "PASS_WITH_RISKS"],
];

const missing = [];
for (const [file, token] of required) {
  if (!fs.existsSync(file)) {
    missing.push(`${file}: missing file`);
    continue;
  }
  const text = fs.readFileSync(file, "utf8");
  if (!text.includes(token)) {
    missing.push(`${file}: missing token ${JSON.stringify(token)}`);
  }
}

if (missing.length > 0) {
  console.error("Phase 29.1/35.8 runtime gap validation failed:");
  for (const item of missing) console.error(`- ${item}`);
  process.exit(1);
}

console.log("Phase 29.1/35.8 runtime gap validation ok.");
