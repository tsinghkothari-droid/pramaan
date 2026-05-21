import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
const scenarioFlagIndex = args.indexOf("--scenario");
const scenarioId =
  scenarioFlagIndex >= 0 && args[scenarioFlagIndex + 1]
    ? args[scenarioFlagIndex + 1]
    : null;

const root = process.cwd();
const corpusPath = path.join(root, "corpus", "adversarial-scenarios-v0.1.json");
const corpus = JSON.parse(fs.readFileSync(corpusPath, "utf8"));

const requiredSecureCategories = new Set([
  "validation_removal",
  "authorization_weakening",
  "unsafe_deserialization",
  "injection_sanitization_removal",
  "crypto_misuse",
  "secret_exposure",
]);
const requiredAdversaries = new Set([
  "careless_ai",
  "overfitted_ai",
  "malicious_pr",
  "compromised_plugin",
  "malicious_ci",
]);
const allowedStatuses = new Set([
  "implemented_demo",
  "implemented_fixture",
  "scenario_spec",
]);
const allowedReplayKinds = new Set(["executable_demo", "metadata_report"]);
const errors = [];

function fail(message) {
  errors.push(message);
}

if (corpus.schema_version !== "pramaan.adversarial_corpus/v1") {
  fail("schema_version must be pramaan.adversarial_corpus/v1");
}

if (!Array.isArray(corpus.scenarios)) {
  fail("scenarios must be an array");
} else if (corpus.scenarios.length < 25) {
  fail(`expected at least 25 scenarios, found ${corpus.scenarios.length}`);
}

const ids = new Set();
const secureCategories = new Set();
const adversaries = new Set();
const categories = new Set();

for (const scenario of corpus.scenarios ?? []) {
  if (!/^ADV-\d{3}$/.test(scenario.id ?? "")) {
    fail(`bad scenario id: ${scenario.id}`);
  }
  if (ids.has(scenario.id)) {
    fail(`duplicate scenario id: ${scenario.id}`);
  }
  ids.add(scenario.id);

  for (const field of [
    "name",
    "category",
    "failure_mode",
    "language",
    "adversary_model",
    "severity",
    "base_change",
    "head_change",
    "ordinary_ci_expectation",
    "pramaan_expected_finding",
    "reviewer_explanation",
    "replay_command",
  ]) {
    if (typeof scenario[field] !== "string" || scenario[field].trim() === "") {
      fail(`${scenario.id}: missing ${field}`);
    }
  }

  if (!allowedStatuses.has(scenario.status)) {
    fail(`${scenario.id}: unsupported status ${scenario.status}`);
  }
  if (!allowedReplayKinds.has(scenario.replay_kind)) {
    fail(`${scenario.id}: unsupported replay_kind ${scenario.replay_kind}`);
  }
  if (!Array.isArray(scenario.fixture_paths)) {
    fail(`${scenario.id}: fixture_paths must be an array`);
  }
  if (!Array.isArray(scenario.risk_ids) || scenario.risk_ids.length === 0) {
    fail(`${scenario.id}: risk_ids must be non-empty`);
  } else {
    for (const risk of scenario.risk_ids) {
      if (!/^R-(00[1-9]|0[1-9][0-9]|100)$/.test(risk)) {
        fail(`${scenario.id}: unknown risk id shape ${risk}`);
      }
    }
  }

  categories.add(scenario.category);
  adversaries.add(scenario.adversary_model);
  if (scenario.secure_code_category) {
    secureCategories.add(scenario.secure_code_category);
  }
}

for (const category of requiredSecureCategories) {
  if (!secureCategories.has(category)) {
    fail(`missing secure-code category ${category}`);
  }
}

for (const adversary of requiredAdversaries) {
  if (!adversaries.has(adversary)) {
    fail(`missing adversary model ${adversary}`);
  }
}

if (!categories.has("verifier_abuse") || !categories.has("ci_supply_chain")) {
  fail("missing verifier-abuse or CI-abuse scenarios");
}

if (scenarioId && !ids.has(scenarioId)) {
  fail(`scenario not found: ${scenarioId}`);
}

if (errors.length > 0) {
  console.error("Adversarial corpus validation failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

if (scenarioId) {
  const scenario = corpus.scenarios.find((item) => item.id === scenarioId);
  console.log(`${scenario.id}: ${scenario.name}`);
  console.log(`category: ${scenario.category}`);
  console.log(`status: ${scenario.status}`);
  console.log(`expected: ${scenario.pramaan_expected_finding}`);
  console.log(`risks: ${scenario.risk_ids.join(", ")}`);
} else {
  console.log("Adversarial corpus ok");
  console.log(`scenarios: ${corpus.scenarios.length}`);
  console.log(`secure_code_categories: ${[...secureCategories].sort().join(", ")}`);
  console.log(`adversary_models: ${[...adversaries].sort().join(", ")}`);
}
