#!/usr/bin/env node
import fs from 'node:fs';

const [planPath] = process.argv.slice(2);

if (!planPath) {
  console.error('usage: node scripts/check-ai-probe-execution.mjs <executed-ai-probe-plan.json>');
  process.exit(2);
}

const plan = JSON.parse(fs.readFileSync(planPath, 'utf8'));
const probes = Array.isArray(plan.probes) ? plan.probes : [];
const accepted = probes.filter((probe) => probe.kept_or_rejected === 'kept');
const rejected = probes.filter((probe) => probe.kept_or_rejected === 'rejected');
const pending = probes.filter((probe) => probe.kept_or_rejected === 'pending_execution');

const failures = [];

if (plan.schema_version !== 'pramaan.probe.v1') {
  failures.push(`unexpected schema_version: ${plan.schema_version}`);
}
if (plan.accepted_count !== accepted.length) {
  failures.push(`accepted_count=${plan.accepted_count} but found ${accepted.length}`);
}
if (plan.rejected_count !== rejected.length) {
  failures.push(`rejected_count=${plan.rejected_count} but found ${rejected.length}`);
}
if (plan.pending_count !== pending.length) {
  failures.push(`pending_count=${plan.pending_count} but found ${pending.length}`);
}
if (pending.length > 0) {
  failures.push(`executed plans must not leave pending probes: ${pending.map((p) => p.probe_id).join(', ')}`);
}
for (const probe of accepted) {
  if (probe.sandbox_status !== 'executed_passed') {
    failures.push(`${probe.probe_id} is kept without executed_passed`);
  }
  if (!String(probe.execution_result || '').includes('executed_passed')) {
    failures.push(`${probe.probe_id} lacks executed_passed result text`);
  }
}
for (const probe of rejected) {
  if (!probe.rejection_reason) {
    failures.push(`${probe.probe_id} is rejected without rejection_reason`);
  }
  if (!['executed_failed', 'rejected_static'].includes(probe.sandbox_status)) {
    failures.push(`${probe.probe_id} rejected with invalid sandbox_status ${probe.sandbox_status}`);
  }
}

if (failures.length) {
  console.error('AI probe execution validation failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log(
  `AI probe execution validation ok: accepted=${accepted.length}, rejected=${rejected.length}, pending=${pending.length}`,
);
