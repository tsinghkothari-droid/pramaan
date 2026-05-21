import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
const fixtureFlagIndex = args.indexOf("--fixture");
const requestedFixture =
  fixtureFlagIndex >= 0 && args[fixtureFlagIndex + 1]
    ? args[fixtureFlagIndex + 1]
    : null;

const root = process.cwd();
const corpusPath = path.join(root, "corpus", "competitor-gap-fixtures.v0.1.json");
const corpus = JSON.parse(fs.readFileSync(corpusPath, "utf8"));

const requiredGapCategories = new Set([
  "oracle_integrity",
  "static_hallucination",
  "ci_supply_chain",
  "bundle_integrity",
  "policy_confidence",
]);
const requiredAdjacentCategories = new Set([
  "ai_pr_reviewer",
  "test_change_monitor",
  "ci_quality_aggregator",
  "ci_status_check",
  "status_dashboard",
]);
const allowedStatuses = new Set([
  "implemented_demo",
  "implemented_fixture",
  "metadata_fixture",
]);
const allowedDecisions = new Set(["fail", "warn"]);
const errors = [];

function fail(message) {
  errors.push(message);
}

if (corpus.schema_version !== "pramaan.competitor_gap/v1") {
  fail("schema_version must be pramaan.competitor_gap/v1");
}

if (!Array.isArray(corpus.fixtures)) {
  fail("fixtures must be an array");
} else if (corpus.fixtures.length < (corpus.minimum_fixture_count ?? 0)) {
  fail(
    `expected at least ${corpus.minimum_fixture_count} fixtures, found ${corpus.fixtures.length}`,
  );
}

const ids = new Set();
const names = new Set();
const gapCategories = new Set();
const adjacentCategories = new Set();

for (const fixture of corpus.fixtures ?? []) {
  if (!/^CG-\d{3}$/.test(fixture.id ?? "")) {
    fail(`bad fixture id: ${fixture.id}`);
  }
  if (ids.has(fixture.id)) {
    fail(`duplicate fixture id: ${fixture.id}`);
  }
  ids.add(fixture.id);

  const normalizedName = String(fixture.name ?? "").trim().toLowerCase();
  if (!normalizedName) {
    fail(`${fixture.id}: missing name`);
  } else if (names.has(normalizedName)) {
    fail(`${fixture.id}: duplicate name ${fixture.name}`);
  }
  names.add(normalizedName);

  for (const field of [
    "gap_category",
    "adjacent_tool_category",
    "ordinary_surface",
    "pramaan_signal",
    "validation_command",
  ]) {
    if (typeof fixture[field] !== "string" || fixture[field].trim() === "") {
      fail(`${fixture.id}: missing ${field}`);
    }
  }

  if (!allowedStatuses.has(fixture.status)) {
    fail(`${fixture.id}: unsupported status ${fixture.status}`);
  }
  if (!allowedDecisions.has(fixture.expected_decision)) {
    fail(`${fixture.id}: unsupported expected_decision ${fixture.expected_decision}`);
  }

  if (!Array.isArray(fixture.fixture_paths) || fixture.fixture_paths.length === 0) {
    fail(`${fixture.id}: fixture_paths must be non-empty`);
  } else {
    for (const fixturePath of fixture.fixture_paths) {
      if (path.isAbsolute(fixturePath) || fixturePath.includes("..")) {
        fail(`${fixture.id}: fixture path must be repo-relative and safe: ${fixturePath}`);
        continue;
      }
      if (!fs.existsSync(path.join(root, fixturePath))) {
        fail(`${fixture.id}: missing fixture path ${fixturePath}`);
      }
    }
  }

  if (!Array.isArray(fixture.risk_ids) || fixture.risk_ids.length === 0) {
    fail(`${fixture.id}: risk_ids must be non-empty`);
  } else {
    for (const risk of fixture.risk_ids) {
      if (!/^R-(00[1-9]|0[1-9][0-9]|100)$/.test(risk)) {
        fail(`${fixture.id}: unknown risk id shape ${risk}`);
      }
    }
  }

  gapCategories.add(fixture.gap_category);
  adjacentCategories.add(fixture.adjacent_tool_category);
}

for (const category of requiredGapCategories) {
  if (!gapCategories.has(category)) {
    fail(`missing gap category ${category}`);
  }
}

for (const category of requiredAdjacentCategories) {
  if (!adjacentCategories.has(category)) {
    fail(`missing adjacent tool category ${category}`);
  }
}

if (requestedFixture && !ids.has(requestedFixture)) {
  fail(`fixture not found: ${requestedFixture}`);
}

if (errors.length > 0) {
  console.error("Competitor-gap fixture validation failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

if (requestedFixture) {
  const fixture = corpus.fixtures.find((item) => item.id === requestedFixture);
  console.log(`${fixture.id}: ${fixture.name}`);
  console.log(`gap_category: ${fixture.gap_category}`);
  console.log(`adjacent_tool_category: ${fixture.adjacent_tool_category}`);
  console.log(`expected_decision: ${fixture.expected_decision}`);
  console.log(`risks: ${fixture.risk_ids.join(", ")}`);
} else {
  console.log("Competitor-gap fixtures ok");
  console.log(`fixtures: ${corpus.fixtures.length}`);
  console.log(`gap_categories: ${[...gapCategories].sort().join(", ")}`);
  console.log(`adjacent_tool_categories: ${[...adjacentCategories].sort().join(", ")}`);
}
