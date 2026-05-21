import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const evidencePath = process.argv[2]
  ? path.resolve(root, process.argv[2])
  : path.join(root, "target", "pramaan-fuzz", "differential-fuzz.json");

const evidence = JSON.parse(fs.readFileSync(evidencePath, "utf8"));
const errors = [];

function fail(message) {
  errors.push(message);
}

if (evidence.schema_version !== "pramaan.differential_fuzz.v1") {
  fail("unexpected differential fuzz schema_version");
}

if (!evidence.adapter_availability) {
  fail("missing adapter_availability");
} else {
  const availability = evidence.adapter_availability;
  if (typeof availability.tool_backed !== "boolean") {
    fail("adapter_availability.tool_backed must be boolean");
  }
  if (typeof availability.reason !== "string" || availability.reason.trim() === "") {
    fail("adapter_availability.reason must explain tool-backed or fallback mode");
  }
  if (availability.tool_backed) {
    for (const token of [
      "safe generated harness executed",
      "tool_version=",
      "generated_cases=",
      "raw_output_digest=sha256:",
      "harness_path=",
      "raw_output_path=",
    ]) {
      if (!availability.reason.includes(token)) {
        fail(`tool-backed evidence reason missing ${token}`);
      }
    }
    if (!["hypothesis", "fast_check"].includes(evidence.adapter)) {
      fail(`tool-backed adapter must be hypothesis or fast_check, got ${evidence.adapter}`);
    }
  } else {
    if (!availability.reason.includes("deterministic replay evidence was selected")) {
      fail("fallback evidence must explicitly say deterministic replay was selected");
    }
    if (evidence.adapter !== "deterministic_simulated") {
      fail(`fallback adapter should be deterministic_simulated, got ${evidence.adapter}`);
    }
  }
}

if (typeof evidence.generated_input_count !== "number" || evidence.generated_input_count <= 0) {
  fail("generated_input_count must be positive");
}
if (typeof evidence.corpus_hash !== "string" || !evidence.corpus_hash.startsWith("sha256:")) {
  fail("corpus_hash must be sha256-prefixed");
}
if (!Array.isArray(evidence.limitations) || evidence.limitations.length === 0) {
  fail("limitations must remain visible");
}

if (errors.length > 0) {
  console.error("Fuzz harness evidence validation failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log("Fuzz harness evidence ok");
console.log(`adapter: ${evidence.adapter}`);
console.log(`tool_backed: ${evidence.adapter_availability.tool_backed}`);
console.log(`generated_input_count: ${evidence.generated_input_count}`);
