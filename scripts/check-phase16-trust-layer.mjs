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

requireTokens("schemas/receipt.schema.json", [
  "agent_attribution",
  "reviewer_override",
  "multi_agent_provenance",
  "plugin_identity",
  "redaction_manifest",
  "stage_budget",
]);

requireTokens("schemas/bundle.schema.json", [
  "agent_attribution",
  "reviewer_overrides",
  "multi_agent_provenance",
  "plugin_identities",
  "redaction_manifest",
  "stage_budgets",
]);

requireTokens("examples/fixtures/receipt.synthetic.json", [
  "agent_author",
  "reviewer_override",
  "multi_agent_provenance",
  "plugin_identity",
  "redaction_manifest",
  "stage_budget",
]);

requireTokens("examples/fixtures/bundle.synthetic.json", [
  "agent_attribution",
  "reviewer_overrides",
  "multi_agent_provenance",
  "redaction_manifest",
  "stage_budgets",
]);

requireTokens("crates/pramaan-core/src/lib.rs", [
  "AgentAttribution",
  "ReviewerOverride",
  "AgentProvenanceEntry",
  "validate_plugin_receipt_trust",
  "REDACTION_POLICY_VERSION",
  "feedback_baseline_comparison_flags_drift",
]);

requireTokens("crates/pramaan-bundle/src/lib.rs", [
  "manifest_aggregates_phase_16a_trust_hooks",
  "build_manifest_rejects_dangerous_plugin_permissions",
  "redaction_manifest",
]);

requireTokens("docs/reviewer-overrides.md", [
  "Human reviewers",
  "accepted risk IDs",
  "update calibration",
]);

requireTokens("docs/calibration.md", [
  "feedback analyze",
  "drift warnings",
  "expected calibration error",
]);

requireTokens("docs/threat-model.md", [
  "malicious pull request",
  "plugin",
  "artifact",
  "fuzzer",
]);

requireTokens("docs/redaction.md", [
  "reviewer-redacted",
  "public-demo",
  "internal hosts",
]);

requireTokens("docs/enterprise-deployment.md", [
  "GitLab",
  "forge-neutral",
  "provider-neutral interface",
]);

if (errors.length > 0) {
  console.error("Phase 16 trust layer check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Phase 16 trust layer evidence ok");
