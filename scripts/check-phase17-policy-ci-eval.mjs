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

requireTokens("schemas/policy_profile.schema.json", [
  "hard_gate_statuses",
  "warning_statuses",
  "required_stages",
  "security_sensitive_paths",
]);

requireTokens("schemas/vsa.schema.json", [
  "verification_result",
  "resource_uri",
  "policy",
  "manifest_digest",
]);

requireTokens("schemas/sarif_export.schema.json", ["SARIF", "runs", "results"]);

requireTokens("crates/pramaan-core/src/lib.rs", [
  "evaluate_policy",
  "ci_hardening_flags_untrusted_pr_workflow_hazards",
  "security_sensitive_policy_escalates_agentic_workflow_risk",
  "stage_budget_exhausted",
]);

requireTokens("crates/pramaan-cli/src/main.rs", [
  "PolicyCommands::Explain",
  "ExportCommands::Sarif",
  "ExportCommands::Rego",
]);

requireTokens("docs/policy.md", ["hard-fail", "warning", "Rego"]);
requireTokens("docs/policy-packs.md", ["startup-fast", "security-sensitive", "fintech-strict"]);
requireTokens("docs/redaction.md", ["internal-full", "reviewer-redacted", "public-demo", "summary-only"]);
requireTokens("docs/github-action.md", ["attest", "policy-profile", "permissions"]);
requireTokens("docs/benchmark-report-template.md", ["False positives", "False negatives", "Reviewer time-to-understand"]);
requireTokens("docs/benchmark-integrity.md", ["Benchmark Integrity", "overfit", "Phase 40"]);

const corpusText = read("corpus/adversarial-scenarios-v0.1.json");
if (corpusText) {
  const corpus = JSON.parse(corpusText);
  const categories = new Set((corpus.scenarios ?? []).map((item) => item.category));
  const failureModes = new Set((corpus.scenarios ?? []).map((item) => item.failure_mode));
  for (const category of [
    "ci_supply_chain",
    "verifier_abuse",
    "secure_code",
    "feedback_calibration",
  ]) {
    if (!categories.has(category)) errors.push(`corpus missing category ${category}`);
  }
  for (const failureMode of [
    "required_stage_disabled",
    "benchmark_overfitting",
    "redaction_leak",
    "untracked_override",
  ]) {
    if (!failureModes.has(failureMode)) errors.push(`corpus missing failure mode ${failureMode}`);
  }
}

if (errors.length > 0) {
  console.error("Phase 17 policy/CI/eval check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Phase 17 policy/CI/eval evidence ok");
