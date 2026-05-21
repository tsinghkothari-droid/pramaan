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

function requireText(rel, tokens) {
  const text = read(rel);
  for (const token of tokens) {
    if (!text.includes(token)) {
      errors.push(`${rel} missing token: ${token}`);
    }
  }
}

requireText("crates/pramaan-bundle/src/lib.rs", [
  "VSA_SCHEMA_VERSION",
  "IN_TOTO_STATEMENT_TYPE",
  "SLSA_VSA_PREDICATE_TYPE",
  "verify_offline_attestation",
  "VSA subject digest does not match current bundle manifest",
]);

requireText("crates/pramaan-cli/tests/smoke.rs", [
  "bundle attest",
  "verify-offline",
  "tampered VSA should fail",
  "VSA result mismatch",
]);

requireText("action.yml", [
  "Create a GitHub artifact attestation",
  "bundle attest",
]);

requireText("docs/signing.md", [
  "bundle attest",
  "bundle verify-offline",
  "future production trust anchors",
]);

requireText("docs/attestation.md", [
  "GitHub artifact attestation",
  "local/offline VSA",
]);

requireText("docs/adversarial-corpus.md", [
  "corpus/adversarial-scenarios-v0.1.json",
  "node scripts/check-adversarial-corpus.mjs",
]);

const corpusText = read("corpus/adversarial-scenarios-v0.1.json");
if (corpusText) {
  const corpus = JSON.parse(corpusText);
  if (corpus.schema_version !== "pramaan.adversarial_corpus/v1") {
    errors.push("corpus schema_version is not pramaan.adversarial_corpus/v1");
  }
  if (!Array.isArray(corpus.scenarios) || corpus.scenarios.length < 25) {
    errors.push("adversarial corpus has fewer than 25 scenarios");
  }
  const statuses = new Set((corpus.scenarios ?? []).map((item) => item.status));
  if (!statuses.has("implemented_demo") || !statuses.has("scenario_spec")) {
    errors.push("adversarial corpus must distinguish executable demos from scenario specs");
  }
}

if (errors.length > 0) {
  console.error("Phase 14 evidence check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Phase 14 attestation/corpus evidence ok");
