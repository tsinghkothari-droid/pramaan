import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import { renderSummary } from "./render-summary.mjs";
import { resolveRefs } from "./resolve-refs.mjs";

test("renderSummary emphasizes failed stages and residual risk families", () => {
  const markdown = renderSummary({
    baseRef: "base-sha",
    headRef: "head-sha",
    logText: "line one\nline two",
    manifest: {
      final_status: "failed",
      bundle_id: "bundle_test",
      repository: { base_ref: "main", head_ref: "feature" },
      stages: [
        {
          id: "static_python",
          status: "failed",
          residual_risks: ["R-031", "R-045", "R-087"],
          mitigated_risks: ["R-001"],
        },
        {
          id: "claim_scope",
          status: "passed",
          residual_risks: [],
          mitigated_risks: ["R-002"],
        },
      ],
      receipts: [{ path: "receipts/static.receipt.json" }],
      artifacts: [{ path: "static.json" }, { path: "claim.json" }],
      risk_summary: {
        mitigated: ["R-001", "R-002"],
        residual: ["R-031", "R-045", "R-087"],
        skipped: [],
        not_applicable: ["R-090"],
      },
      summary: { residual_risk_note: "Static check failed." },
      policy_decision: {
        decision: "failed",
        policy_id: "pramaan-default-v0",
        hard_failures: ["stage_status:static_python:failed"],
        warnings: ["residual_risk:static_python:R-031,R-045,R-087"],
      },
      integrity: {
        manifest_digest: { value: "a".repeat(64) },
        artifact_attestation: { provider: "github_actions", status: "not_requested" },
      },
    },
  });

  assert.match(markdown, /Final status: \*\*failed\*\*/);
  assert.match(markdown, /Compared refs: `base-sha` -> `head-sha`/);
  assert.match(markdown, /static_python/);
  assert.match(markdown, /static_hallucination \(1\), public_api_compatibility \(1\), bundle_integrity \(1\)/);
  assert.match(markdown, /Receipts: 1/);
  assert.match(markdown, /Artifacts: 2/);
  assert.match(markdown, /github_actions: not_requested/);
  assert.match(markdown, /Policy decision: \*\*failed\*\*/);
  assert.match(markdown, /stage_status:static_python:failed/);
  assert.doesNotMatch(markdown, /\| claim_scope \| OK passed/);
});

test("renderSummary states when there are no actionable stages", () => {
  const markdown = renderSummary({
    manifest: {
      final_status: "passed",
      stages: [{ id: "claim_scope", status: "passed", residual_risks: [], mitigated_risks: [] }],
      risk_summary: { mitigated: [], residual: [], skipped: [], not_applicable: [] },
      integrity: {},
      summary: {},
    },
  });

  assert.match(markdown, /\| none \| none \| none \| none \|/);
  assert.match(markdown, /\| residual \| none \|/);
});

test("resolveRefs prefers explicit inputs, then event refs, then environment fallbacks", () => {
  assert.deepEqual(
    resolveRefs({
      baseInput: "input-base",
      headInput: "input-head",
      eventPath: "",
      env: { GITHUB_SHA: "sha" },
    }),
    { base_ref: "input-base", head_ref: "input-head" },
  );

  assert.deepEqual(
    resolveRefs({
      baseInput: "",
      headInput: "",
      eventPath: "",
      env: { PRAMAAN_BASE_REF: "env-base", PRAMAAN_HEAD_REF: "env-head" },
    }),
    { base_ref: "env-base", head_ref: "env-head" },
  );
});

test("composite action exposes production gate inputs and uploads before failing", () => {
  const actionYaml = fs.readFileSync(new URL("../action.yml", import.meta.url), "utf8");

  assert.match(actionYaml, /out-dir:/);
  assert.match(actionYaml, /upload-bundle:/);
  assert.match(actionYaml, /fail-on:/);
  assert.match(actionYaml, /cargo build --locked -p pramaan-cli/);
  assert.match(actionYaml, /target\/debug\/pramaan verify/);
  assert.match(actionYaml, /target\/debug\/pramaan policy explain/);

  const uploadIndex = actionYaml.indexOf("name: Upload proof bundle");
  const failIndex = actionYaml.indexOf("name: Apply failure policy");
  assert.ok(uploadIndex > 0, "upload step should exist");
  assert.ok(failIndex > 0, "failure policy step should exist");
  assert.ok(uploadIndex < failIndex, "bundle upload should happen before fail-on exits");
});
