#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const FAMILY_RANGES = [
  [1, 10, "claim_scope"],
  [11, 20, "oracle_integrity"],
  [21, 30, "sandbox_reproducibility"],
  [31, 40, "static_hallucination"],
  [41, 50, "public_api_compatibility"],
  [51, 60, "runtime_behavior"],
  [61, 70, "mutation_quality"],
  [71, 80, "property_fuzz"],
  [81, 90, "bundle_integrity"],
  [91, 95, "ci_supply_chain"],
  [96, 100, "demo_corpus"],
];

function parseArgs(argv) {
  const args = new Map();
  for (let index = 2; index < argv.length; index += 1) {
    const key = argv[index];
    if (!key.startsWith("--")) {
      throw new Error(`unexpected argument ${key}`);
    }
    args.set(key.slice(2), argv[index + 1] ?? "");
    index += 1;
  }
  return args;
}

function riskFamily(riskId) {
  const match = /^R-(\d{3})$/.exec(riskId);
  if (!match) {
    return "unknown";
  }
  const number = Number(match[1]);
  const range = FAMILY_RANGES.find(([start, end]) => number >= start && number <= end);
  return range ? range[2] : "unknown";
}

function familyCounts(risks = []) {
  const counts = new Map();
  for (const risk of risks) {
    const family = riskFamily(risk);
    counts.set(family, (counts.get(family) ?? 0) + 1);
  }
  const order = new Map(FAMILY_RANGES.map(([, , family], index) => [family, index]));
  return [...counts.entries()].sort(([left], [right]) => {
    const leftOrder = order.get(left) ?? Number.MAX_SAFE_INTEGER;
    const rightOrder = order.get(right) ?? Number.MAX_SAFE_INTEGER;
    return leftOrder - rightOrder || left.localeCompare(right);
  });
}

function formatFamilyCounts(risks = []) {
  const counts = familyCounts(risks);
  if (counts.length === 0) {
    return "none";
  }
  return counts.map(([family, count]) => `${family} (${count})`).join(", ");
}

function statusIcon(status) {
  if (status === "passed") return "OK";
  if (status === "failed" || status === "error" || status === "timed_out") return "ACTION";
  if (status === "skipped" || status === "not_applicable") return "NOTE";
  return "CHECK";
}

function tableRows(stages) {
  if (!stages.length) {
    return "| none | none | none | none |\n";
  }
  return stages
    .map((stage) => {
      const residual = formatFamilyCounts(stage.residual_risks ?? []);
      const mitigated = formatFamilyCounts(stage.mitigated_risks ?? []);
      return `| ${stage.id} | ${statusIcon(stage.status)} ${stage.status} | ${residual} | ${mitigated} |`;
    })
    .join("\n");
}

function logTail(logText, maxLines = 12) {
  const lines = logText
    .split(/\r?\n/)
    .map((line) => line.trimEnd())
    .filter(Boolean);
  return lines.slice(-maxLines).join("\n");
}

export function renderSummary({ manifest, logText = "", baseRef = "", headRef = "" }) {
  const actionableStages = (manifest.stages ?? []).filter((stage) =>
    ["failed", "error", "timed_out", "skipped"].includes(stage.status),
  );
  const stageSection = actionableStages.length
    ? tableRows(actionableStages)
    : "| none | none | none | none |";
  const attestation = manifest.integrity?.artifact_attestation;
  const attestationText = attestation
    ? `${attestation.provider}: ${attestation.status}`
    : "not recorded";
  const digest = manifest.integrity?.manifest_digest?.value
    ? `sha256:${manifest.integrity.manifest_digest.value}`
    : "not recorded";
  const tail = logTail(logText);

  return `# Pramaan proof bundle

Final status: **${manifest.final_status ?? "unknown"}**

Compared refs: \`${baseRef || manifest.repository?.base_ref || "unknown"}\` -> \`${headRef || manifest.repository?.head_ref || "unknown"}\`

Bundle: \`${manifest.bundle_id ?? "unknown"}\`

Manifest digest: \`${digest}\`

## Failed, skipped, or incomplete stages

| Stage | Status | Residual risk families | Mitigated risk families |
| --- | --- | --- | --- |
${stageSection}

## Risk families

| Bucket | Families |
| --- | --- |
| mitigated | ${formatFamilyCounts(manifest.risk_summary?.mitigated)} |
| residual | ${formatFamilyCounts(manifest.risk_summary?.residual)} |
| skipped | ${formatFamilyCounts(manifest.risk_summary?.skipped)} |
| not_applicable | ${formatFamilyCounts(manifest.risk_summary?.not_applicable)} |

## Bundle evidence

- Receipts: ${(manifest.receipts ?? []).length}
- Artifacts: ${(manifest.artifacts ?? []).length}
- Artifact attestation: ${attestationText}
- Residual risk note: ${manifest.summary?.residual_risk_note ?? "not recorded"}

${tail ? `## CLI tail\n\n\`\`\`text\n${tail}\n\`\`\`\n` : ""}`;
}

if (process.argv[1] && path.resolve(fileURLToPath(import.meta.url)) === path.resolve(process.argv[1])) {
  const args = parseArgs(process.argv);
  const manifestPath = args.get("manifest");
  const outPath = args.get("out");
  if (!manifestPath || !outPath) {
    throw new Error("--manifest and --out are required");
  }

  const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
  const logPath = args.get("log");
  const summary = renderSummary({
    manifest,
    logText: logPath && fs.existsSync(logPath) ? fs.readFileSync(logPath, "utf8") : "",
    baseRef: args.get("base") ?? "",
    headRef: args.get("head") ?? "",
  });
  fs.writeFileSync(outPath, summary);
}
