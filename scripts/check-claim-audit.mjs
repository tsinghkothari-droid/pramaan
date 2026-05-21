import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const statusPath = path.join(root, "STATUS.md");
const auditPath = path.join(root, "docs", "claim-audit.md");

const status = fs.readFileSync(statusPath, "utf8");
const audit = fs.readFileSync(auditPath, "utf8");

const allowedLabels = new Set([
  "executable-test",
  "checked-fixture",
  "manual-proof",
  "implemented-untested",
  "partial",
  "planned",
  "experimental",
  "accepted-risk",
]);

const capabilityRows = status
  .split(/\r?\n/)
  .filter((line) => line.startsWith("| ") && !line.includes("---") && !line.includes("Capability"))
  .map((line) => line.split("|").map((cell) => cell.trim()))
  .map((cells) => ({ capability: cells[1], status: cells[2] }))
  .filter((row) => row.capability && row.status);

const auditRows = audit
  .split(/\r?\n/)
  .filter((line) => line.startsWith("| CLAIM-"))
  .map((line) => line.split("|").map((cell) => cell.trim()))
  .map((cells) => ({
    id: cells[1],
    sourceClaim: cells[3],
    label: cells[4],
    status: cells[8],
  }));

const missing = [];
for (const row of capabilityRows) {
  const needle = `STATUS: ${row.capability}`;
  if (!auditRows.some((auditRow) => auditRow.sourceClaim === needle)) {
    missing.push(needle);
  }
}

const badLabels = auditRows.filter((row) => !allowedLabels.has(row.label));
const staleClaims = auditRows.filter(
  (row) => row.label === "false-or-stale" || /unresolved/i.test(row.status),
);
const untestedImplemented = auditRows.filter(
  (row) =>
    row.sourceClaim.startsWith("STATUS:") &&
    row.sourceClaim.includes("Implemented") &&
    row.label === "implemented-untested",
);

if (missing.length > 0 || badLabels.length > 0 || staleClaims.length > 0 || untestedImplemented.length > 0) {
  if (missing.length > 0) {
    console.error("Missing STATUS.md claim-audit rows:");
    for (const item of missing) console.error(`- ${item}`);
  }
  if (badLabels.length > 0) {
    console.error("Unknown claim-audit labels:");
    for (const row of badLabels) console.error(`- ${row.id}: ${row.label}`);
  }
  if (staleClaims.length > 0) {
    console.error("False/stale or unresolved claims remain:");
    for (const row of staleClaims) console.error(`- ${row.id}: ${row.status}`);
  }
  if (untestedImplemented.length > 0) {
    console.error("Implemented STATUS claims without evidence:");
    for (const row of untestedImplemented) console.error(`- ${row.id}`);
  }
  process.exit(1);
}

console.log(`Claim audit ok: ${auditRows.length} claims, ${capabilityRows.length} STATUS rows covered.`);
