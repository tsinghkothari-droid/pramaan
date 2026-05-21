#!/usr/bin/env node
import fs from 'node:fs';

const fixturePath = process.argv[2] || 'corpus/verifier-abuse-fixtures.v0.1.json';
const fixture = JSON.parse(fs.readFileSync(fixturePath, 'utf8'));
const scenarios = Array.isArray(fixture.scenarios) ? fixture.scenarios : [];
const failures = [];

if (fixture.schema_version !== 'pramaan.verifier_abuse_fixtures.v0.1') {
  failures.push(`unexpected schema_version ${fixture.schema_version}`);
}
if (scenarios.length < 6) {
  failures.push(`expected at least 6 verifier-abuse scenarios, got ${scenarios.length}`);
}

const requiredRisks = new Set(['R-094', 'R-095']);
const seenRisks = new Set();
const requiredPathPrefixes = [
  '.github/workflows/',
  'action.yml',
  'scripts/',
  'schemas/',
  'examples/',
  'corpus/',
];
const seenPathPrefixes = new Set();

for (const scenario of scenarios) {
  if (!scenario.id || !scenario.name || !scenario.attack) {
    failures.push(`scenario ${scenario.id || '<missing id>'} is missing id/name/attack`);
  }
  const paths = Array.isArray(scenario.changed_paths) ? scenario.changed_paths : [];
  if (paths.length === 0) {
    failures.push(`${scenario.id} has no changed_paths`);
  }
  for (const path of paths) {
    for (const prefix of requiredPathPrefixes) {
      if (path === prefix || path.startsWith(prefix)) {
        seenPathPrefixes.add(prefix);
      }
    }
  }
  const risks = Array.isArray(scenario.expected_risks) ? scenario.expected_risks : [];
  for (const risk of risks) {
    seenRisks.add(risk);
  }
  if (!risks.every((risk) => requiredRisks.has(risk))) {
    failures.push(`${scenario.id} uses unexpected risk ids ${risks.join(',')}`);
  }
  if (scenario.expected_policy?.['security-sensitive'] !== 'fail') {
    failures.push(`${scenario.id} must fail under security-sensitive policy`);
  }
}

for (const risk of requiredRisks) {
  if (!seenRisks.has(risk)) {
    failures.push(`missing expected risk ${risk}`);
  }
}
for (const prefix of requiredPathPrefixes) {
  if (!seenPathPrefixes.has(prefix)) {
    failures.push(`missing changed path family ${prefix}`);
  }
}

if (failures.length) {
  console.error('Verifier-abuse fixture validation failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(`Verifier-abuse fixtures ok: ${scenarios.length} scenarios, risks=${[...seenRisks].sort().join(',')}`);
