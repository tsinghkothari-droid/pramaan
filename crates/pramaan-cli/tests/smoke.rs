use std::fs;
use std::path::PathBuf;
use std::process::Command;

use pramaan_bundle::{build_manifest, write_manifest, BundleBuildOptions};
use pramaan_core::{Receipt, ReceiptSummary, RiskRefs, StageStatus};
use serde_json::json;

#[test]
fn verify_writes_receipts_and_prints_a_claim_disciplined_summary() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let out = workspace
        .join("target")
        .join("pramaan-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "verify",
            "--base",
            "HEAD",
            "--head",
            "HEAD",
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan verify");

    assert!(
        output.status.success(),
        "verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan verification bundle emitted"));
    assert!(stdout.contains("bundle:"));
    assert!(stdout.contains("claim_scope"));
    assert!(stdout.contains("synthetic_verification"));
    assert!(stdout.contains("passed"));
    assert!(stdout.contains("not_applicable"));
    assert!(stdout.contains("Risk families"));
    assert!(stdout.contains("mitigated"));
    assert!(stdout.contains("residual"));
    assert!(stdout.contains("skipped"));
    assert!(stdout.contains("does not prove the code correct"));

    let claim_scope_path = out.join("claim_scope.synthetic.json");
    let claim_receipt_path = out.join("receipts").join("claim-scope.receipt.json");
    let synthetic_receipt_path = out
        .join("receipts")
        .join("synthetic-verification.receipt.json");
    let manifest_path = out.join("bundle.manifest.json");

    assert!(claim_scope_path.exists(), "claim scope should be written");
    assert!(
        claim_receipt_path.exists(),
        "claim receipt should be written"
    );
    assert!(
        synthetic_receipt_path.exists(),
        "synthetic receipt should be written"
    );
    assert!(manifest_path.exists(), "bundle manifest should be written");

    let bundle_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args(["bundle", "verify", out.to_str().expect("utf-8 output path")])
        .output()
        .expect("run pramaan bundle verify");

    assert!(
        bundle_output.status.success(),
        "bundle verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&bundle_output.stdout),
        String::from_utf8_lossy(&bundle_output.stderr)
    );

    let bundle_stdout = String::from_utf8_lossy(&bundle_output.stdout);
    assert!(bundle_stdout.contains("Pramaan bundle verification complete"));
    assert!(bundle_stdout.contains("receipts_checked:"));
    assert!(bundle_stdout.contains("artifacts_checked:"));

    let policy_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "policy",
            "explain",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan policy explain");

    assert!(
        policy_output.status.success(),
        "policy explain failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&policy_output.stdout),
        String::from_utf8_lossy(&policy_output.stderr)
    );
    let policy_stdout = String::from_utf8_lossy(&policy_output.stdout);
    assert!(policy_stdout.contains("Pramaan policy explanation"));
    assert!(policy_stdout.contains("policy: pramaan-default-v0"));
    assert!(policy_stdout.contains("decision: warning"));
    assert!(policy_stdout.contains("required_stages: claim_scope, sandbox_setup"));
    assert!(policy_stdout.contains("partial_evidence:claim_scope"));

    let agent_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "agent",
            "explain",
            "--bundle",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan agent explain");

    assert!(
        agent_output.status.success(),
        "agent explain failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&agent_output.stdout),
        String::from_utf8_lossy(&agent_output.stderr)
    );
    let agent_decision_path = out.join("agent-decision.json");
    assert!(agent_decision_path.exists(), "agent decision JSON exists");
    let agent_decision: serde_json::Value =
        serde_json::from_slice(&fs::read(agent_decision_path).expect("read agent decision"))
            .expect("agent decision json");
    assert_eq!(
        agent_decision["schema_version"],
        "pramaan.agent_decision.v1"
    );
    assert_eq!(agent_decision["decision"], "warn");
    assert!(agent_decision["agent_message"]
        .as_str()
        .expect("agent message")
        .contains("Do not present this as cleanly verified"));

    let claim_receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(claim_receipt_path).expect("read claim receipt"))
            .expect("claim receipt json");
    assert_eq!(claim_receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(claim_receipt["stage"], "claim_scope");
    assert_eq!(claim_receipt["status"], "passed");
    assert!(claim_receipt["residual_risks"].is_array());
    assert_eq!(claim_receipt["agent_author"]["product"], "Codex");
    assert_eq!(claim_receipt["plugin_identity"]["name"], "pramaan-core");
    assert_eq!(claim_receipt["evidence_sensitivity"], "internal");
    assert_eq!(
        claim_receipt["policy_decision"]["decision"],
        "informational"
    );
    assert_eq!(claim_receipt["stage_budget"]["partial_evidence"], true);
    let mut normalized_claim_receipt = claim_receipt.clone();
    normalized_claim_receipt["started_at"] = json!("<normalized>");
    normalized_claim_receipt["ended_at"] = json!("<normalized>");
    assert_eq!(
        normalized_claim_receipt,
        json!({
            "schema_version": "pramaan.receipt.v1",
            "stage": "claim_scope",
            "status": "passed",
            "tool": {
                "name": "pramaan-cli",
                "version": env!("CARGO_PKG_VERSION")
            },
            "started_at": "<normalized>",
            "ended_at": "<normalized>",
            "exit_code": 0,
            "inputs": [
                {
                    "name": "base",
                    "value": "HEAD"
                },
                {
                    "name": "head",
                    "value": "HEAD"
                }
            ],
            "outputs": [
                {
                    "name": "claim_scope",
                    "path": "claim_scope.synthetic.json",
                    "digest": "sha256:35ded69ccefbcd37465aac8843f5823de7165bfec5eabeae140697229dc167dc"
                }
            ],
            "artifacts": [
                {
                    "name": "claim_scope_json",
                    "path": "claim_scope.synthetic.json",
                    "media_type": "application/json"
                }
            ],
            "summary": {
                "title": "Synthetic claim scope emitted",
                "details": "Claim scope was generated from CLI refs only; no PR metadata was inspected."
            },
            "limitations": [
                "Synthetic Phase 1 receipt only; no repository checks were executed.",
                "Risk IDs are sample references used to verify the receipt contract."
            ],
            "mitigated_risks": [
                "R-003"
            ],
            "residual_risks": [
                "R-090"
            ],
            "not_applicable_risks": [
                "R-081"
            ],
            "agent_author": {
                "product": "Codex",
                "model_family": "gpt-5",
                "execution_mode": "synthetic_verify",
                "source": "local_cli",
                "confidence": "unknown"
            },
            "plugin_identity": {
                "name": "pramaan-core",
                "version": "0.1.0",
                "provenance": "workspace",
                "sandbox_boundary": "in_process"
            },
            "plugin_permissions": {
                "may_emit_receipts": true,
                "may_emit_artifacts": true,
                "may_read_previous_receipts": false,
                "may_modify_previous_receipts": false,
                "may_modify_manifest": false
            },
            "evidence_sensitivity": "internal",
            "redaction_manifest": {
                "profile": "internal-full",
                "redacted_fields": [],
                "hashed_fields": [],
                "policy": "pramaan-redaction-v0"
            },
            "policy_decision": {
                "decision": "informational",
                "policy_id": "pramaan-default-v0",
                "hard_failures": [],
                "warnings": [
                    "synthetic_evidence_only"
                ],
                "waived": []
            },
            "stage_budget": {
                "target_ms": 30000,
                "max_ms": 60000,
                "consumed_ms": 0,
                "exhausted": false,
                "partial_evidence": true
            }
        })
    );
    let claim_scope: serde_json::Value =
        serde_json::from_slice(&fs::read(claim_scope_path).expect("read claim scope"))
            .expect("claim scope json");
    assert_eq!(claim_scope["confidence"], "low");
    assert!(claim_scope["risk_refs"]
        .as_array()
        .expect("claim risk refs")
        .iter()
        .any(|risk| risk == "R-001"));

    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("read manifest"))
            .expect("manifest json");
    assert_eq!(manifest["agent_attribution"][0]["product"], "Codex");
    assert_eq!(manifest["plugin_identities"][0]["name"], "pramaan-core");
    assert_eq!(manifest["redaction_manifest"]["profile"], "internal-full");
    assert_eq!(manifest["policy_decision"]["decision"], "informational");
    assert_eq!(manifest["stage_budgets"][0]["partial_evidence"], true);

    let confidence_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "confidence",
            "explain",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan confidence explain");

    assert!(
        confidence_output.status.success(),
        "confidence explain failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&confidence_output.stdout),
        String::from_utf8_lossy(&confidence_output.stderr)
    );

    let confidence_stdout = String::from_utf8_lossy(&confidence_output.stdout);
    assert!(confidence_stdout.contains("Pramaan confidence explanation complete"));
    assert!(confidence_stdout.contains("decision:"));

    let confidence_json_path = out.join("confidence.json");
    let confidence_md_path = out.join("confidence.md");
    let confidence_receipt_path = out.join("receipts").join("confidence-vote.receipt.json");
    assert!(confidence_json_path.exists(), "confidence JSON exists");
    assert!(confidence_md_path.exists(), "confidence markdown exists");
    assert!(
        confidence_receipt_path.exists(),
        "confidence receipt exists"
    );

    let confidence: serde_json::Value =
        serde_json::from_slice(&fs::read(confidence_json_path).expect("read confidence JSON"))
            .expect("confidence JSON");
    assert_eq!(confidence["schema_version"], "pramaan.confidence.v1");
    assert_eq!(
        confidence["algorithm_version"],
        "pramaan-confidence-v0.1-uncalibrated"
    );
    assert_eq!(confidence["calibration"]["status"], "uncalibrated");
    assert!(confidence["votes"].as_array().unwrap().len() >= 2);
    assert!(fs::read_to_string(confidence_md_path)
        .expect("read confidence markdown")
        .contains("not a proof"));

    let updated_manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(&manifest_path).expect("read updated manifest"))
            .expect("updated manifest json");
    assert!(updated_manifest["artifacts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|artifact| artifact["path"] == "confidence.json"));
    assert!(updated_manifest["receipts"]
        .as_array()
        .unwrap()
        .iter()
        .any(|receipt| receipt["path"] == "receipts/confidence-vote.receipt.json"));

    let updated_bundle_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args(["bundle", "verify", out.to_str().expect("utf-8 output path")])
        .output()
        .expect("run pramaan bundle verify after confidence");
    assert!(
        updated_bundle_output.status.success(),
        "updated bundle verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&updated_bundle_output.stdout),
        String::from_utf8_lossy(&updated_bundle_output.stderr)
    );
}

#[test]
fn agent_explain_blocks_weakened_oracle_bundle() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let out = workspace
        .join("target")
        .join("pramaan-agent-block-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean agent output");
    }
    fs::create_dir_all(out.join("receipts")).expect("create receipt dir");

    let receipts = [
        Receipt::synthetic(
            "claim_scope",
            StageStatus::Passed,
            "base",
            "head",
            vec![],
            vec![],
            ReceiptSummary {
                title: "Claim scope passed".to_string(),
                details: "Agent fixture claim scope.".to_string(),
            },
            RiskRefs {
                mitigated: vec!["R-003".to_string()],
                residual: vec![],
                not_applicable: vec![],
            },
        ),
        Receipt::synthetic(
            "sandbox_setup",
            StageStatus::Passed,
            "base",
            "head",
            vec![],
            vec![],
            ReceiptSummary {
                title: "Sandbox passed".to_string(),
                details: "Agent fixture sandbox.".to_string(),
            },
            RiskRefs {
                mitigated: vec!["R-021".to_string()],
                residual: vec![],
                not_applicable: vec![],
            },
        ),
        Receipt::synthetic(
            "oracle_integrity",
            StageStatus::Failed,
            "base",
            "head",
            vec![],
            vec![],
            ReceiptSummary {
                title: "Oracle integrity failed".to_string(),
                details: "A test assertion was weakened.".to_string(),
            },
            RiskRefs {
                mitigated: vec![],
                residual: vec!["R-011".to_string(), "R-014".to_string()],
                not_applicable: vec![],
            },
        ),
    ];

    for receipt in receipts {
        let path = out
            .join("receipts")
            .join(format!("{}.receipt.json", receipt.stage.replace('_', "-")));
        fs::write(
            path,
            serde_json::to_vec_pretty(&receipt).expect("serialize fixture receipt"),
        )
        .expect("write fixture receipt");
    }
    let manifest = build_manifest(
        &out,
        BundleBuildOptions::synthetic("base".to_string(), "head".to_string()),
    )
    .expect("build fixture manifest");
    write_manifest(&out, &manifest).expect("write fixture manifest");

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "agent",
            "explain",
            "--bundle",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan agent explain");

    assert!(
        output.status.success(),
        "agent explain failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let decision: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("agent decision stdout is JSON");
    assert_eq!(decision["decision"], "block");
    assert!(decision["blocking_stages"]
        .as_array()
        .expect("blocking stages array")
        .iter()
        .any(|stage| stage == "oracle_integrity"));
    assert!(decision["required_actions"]
        .as_array()
        .expect("required actions")
        .iter()
        .any(|action| action
            .as_str()
            .expect("action string")
            .contains("Restore or strengthen")));
}

#[test]
fn probe_plan_preserves_ai_candidates_as_pending_evidence() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let out = workspace
        .join("target")
        .join("pramaan-probe-plan-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean probe output");
    }
    fs::create_dir_all(out.join("receipts")).expect("create probe receipt dir");
    fs::create_dir_all(out.join("tests")).expect("create probe test dir");
    fs::create_dir_all(out.join("src")).expect("create probe source dir");
    fs::write(
        out.join("tests").join("test_checkout.py"),
        "def test_checkout(): pass\n",
    )
    .expect("write probe python target");
    fs::write(
        out.join("src").join("pricing.ts"),
        "export const price = 1;\n",
    )
    .expect("write probe typescript target");

    let oracle_receipt = Receipt::synthetic(
        "oracle_integrity",
        StageStatus::Failed,
        "base",
        "head",
        vec![],
        vec![pramaan_core::ArtifactRef {
            name: "changed_test".to_string(),
            path: "tests/test_checkout.py".to_string(),
            media_type: Some("text/x-python".to_string()),
            digest: None,
        }],
        ReceiptSummary {
            title: "Oracle integrity failed".to_string(),
            details: "Assertion weakening remains.".to_string(),
        },
        RiskRefs {
            mitigated: vec![],
            residual: vec!["R-014".to_string()],
            not_applicable: vec![],
        },
    );
    let fuzz_receipt = Receipt::synthetic(
        "differential_fuzz",
        StageStatus::Skipped,
        "base",
        "head",
        vec![],
        vec![pramaan_core::ArtifactRef {
            name: "changed_source".to_string(),
            path: "src/pricing.ts".to_string(),
            media_type: Some("text/typescript".to_string()),
            digest: None,
        }],
        ReceiptSummary {
            title: "Fuzz skipped".to_string(),
            details: "fast-check was unavailable.".to_string(),
        },
        RiskRefs {
            mitigated: vec![],
            residual: vec!["R-075".to_string()],
            not_applicable: vec![],
        },
    );

    for receipt in [oracle_receipt, fuzz_receipt] {
        let path = out
            .join("receipts")
            .join(format!("{}.receipt.json", receipt.stage.replace('_', "-")));
        fs::write(
            path,
            serde_json::to_vec_pretty(&receipt).expect("serialize probe fixture receipt"),
        )
        .expect("write probe fixture receipt");
    }
    let manifest = build_manifest(
        &out,
        BundleBuildOptions::synthetic("base".to_string(), "head".to_string()),
    )
    .expect("build probe fixture manifest");
    write_manifest(&out, &manifest).expect("write probe fixture manifest");

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "probe",
            "plan",
            "--bundle",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan probe plan");

    assert!(
        output.status.success(),
        "probe plan failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan AI probe plan complete"));
    assert!(stdout.contains("provider_trusted_for_decision: false"));

    let plan_path = out.join("probes").join("ai-probe-plan.json");
    let receipt_path = out
        .join("receipts")
        .join("ai-probe-generation.receipt.json");
    assert!(plan_path.exists(), "probe plan exists");
    assert!(receipt_path.exists(), "probe receipt exists");

    let plan: serde_json::Value =
        serde_json::from_slice(&fs::read(plan_path).expect("read probe plan"))
            .expect("probe plan json");
    assert_eq!(plan["schema_version"], "pramaan.probe.v1");
    assert_eq!(plan["provider"]["trusted_for_decision"], false);
    assert_eq!(plan["accepted_count"], 0);
    assert_eq!(plan["rejected_count"], 0);
    assert_eq!(plan["pending_count"], 2);
    assert!(plan["provider"]["prompt_hash"]
        .as_str()
        .expect("prompt hash")
        .starts_with("sha256:"));
    assert!(plan["probes"]
        .as_array()
        .expect("probe array")
        .iter()
        .any(|probe| probe["kind"] == "fixture_snapshot_challenge"
            && probe["language"] == "python"
            && probe["sandbox_status"] == "requires_execution"
            && probe["kept_or_rejected"] == "pending_execution"));
    assert!(plan["probes"]
        .as_array()
        .expect("probe array")
        .iter()
        .any(|probe| probe["kind"] == "differential_input" && probe["language"] == "typescript"));

    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).expect("read probe receipt"))
            .expect("probe receipt json");
    assert_eq!(receipt["stage"], "ai_probe_generation");
    assert_eq!(receipt["status"], "passed");
    assert_eq!(
        receipt["metadata"]["provider_trusted_for_decision"],
        "false"
    );
    assert!(receipt["residual_risks"]
        .as_array()
        .expect("residual risks")
        .iter()
        .any(|risk| risk == "R-075"));

    let bundle_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args(["bundle", "verify", out.to_str().expect("utf-8 output path")])
        .output()
        .expect("run bundle verify after probe plan");
    assert!(
        bundle_output.status.success(),
        "bundle verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&bundle_output.stdout),
        String::from_utf8_lossy(&bundle_output.stderr)
    );
}

#[test]
fn bundle_verify_fails_when_artifact_is_tampered() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let out = workspace
        .join("target")
        .join("pramaan-bundle-tamper-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean tamper output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "verify",
            "--base",
            "HEAD",
            "--head",
            "HEAD",
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan verify");

    assert!(
        output.status.success(),
        "verify failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(
        out.join("claim_scope.synthetic.json"),
        b"{\"tampered\":true}",
    )
    .expect("tamper artifact");

    let verify_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args(["bundle", "verify", out.to_str().expect("utf-8 output path")])
        .output()
        .expect("run pramaan bundle verify on tampered bundle");

    assert!(
        !verify_output.status.success(),
        "tampered bundle should fail\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&verify_output.stdout),
        String::from_utf8_lossy(&verify_output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&verify_output.stderr).contains("digest mismatch"),
        "stderr should describe digest mismatch: {}",
        String::from_utf8_lossy(&verify_output.stderr)
    );
}

#[test]
fn static_checks_emit_fixture_receipts_and_classify_broken_imports() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixtures = workspace
        .join("examples")
        .join("fixtures")
        .join("static")
        .join("rust");
    let out = workspace
        .join("target")
        .join("pramaan-static-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean static smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "static-checks",
            "--repo",
            fixtures.to_str().expect("utf-8 fixture path"),
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan static-checks");

    assert!(
        output.status.success(),
        "static-checks failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan static checks complete"));
    assert!(stdout.contains("static_rust_cargo_check"));
    assert!(stdout.contains("Hallucination categories"));

    let receipt_dir = out.join("receipts").join("static");
    let rust_receipt_path = receipt_dir.join("rust-cargo-check.receipt.json");
    assert!(
        rust_receipt_path.exists(),
        "Rust cargo check receipt exists"
    );

    let rust_receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(rust_receipt_path).expect("read rust receipt"))
            .expect("rust receipt json");
    assert_eq!(rust_receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(rust_receipt["status"], "failed");
    assert_eq!(
        rust_receipt["metadata"]["hallucination_categories"],
        "broken_import,nonexistent_import"
    );
}

#[test]
fn static_checks_emit_python_fixture_receipts() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixtures = workspace
        .join("examples")
        .join("fixtures")
        .join("static")
        .join("python");
    let out = workspace
        .join("target")
        .join("pramaan-static-python-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean Python static smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "static-checks",
            "--repo",
            fixtures.to_str().expect("utf-8 fixture path"),
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan static-checks for Python fixture");

    assert!(
        output.status.success(),
        "static-checks failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let python_receipt_path = out
        .join("receipts")
        .join("static")
        .join("python-compileall.receipt.json");
    assert!(
        python_receipt_path.exists(),
        "Python compile receipt exists"
    );

    let python_receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(python_receipt_path).expect("read Python receipt"))
            .expect("Python receipt json");
    assert_eq!(python_receipt["schema_version"], "pramaan.receipt.v1");
    assert!(["passed", "skipped", "failed", "not_applicable"]
        .contains(&python_receipt["status"].as_str().expect("status string")));
}

#[test]
fn oracle_emits_failed_receipt_for_weakened_fixture_pair() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixture_root = workspace.join("examples").join("fixtures").join("oracle");
    let out = workspace
        .join("target")
        .join("pramaan-oracle-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean oracle smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "oracle",
            "--base-repo",
            fixture_root
                .join("base")
                .to_str()
                .expect("utf-8 base fixture path"),
            "--head-repo",
            fixture_root
                .join("head")
                .to_str()
                .expect("utf-8 head fixture path"),
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan oracle");

    assert!(
        output.status.success(),
        "oracle command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan oracle integrity complete"));
    assert!(stdout.contains("deleted_test"));
    assert!(stdout.contains("renamed_test"));
    assert!(stdout.contains("removed_error_path"));
    assert!(stdout.contains("removed_boundary_case"));
    assert!(stdout.contains("weakened_assertion"));
    assert!(stdout.contains("sensitive_artifact_changed"));
    assert!(stdout.contains("Assertion signal weakened"));
    assert!(stdout.contains("fnv64:"));

    let diff_path = out.join("oracle-diff.json");
    let receipt_path = out.join("receipts").join("oracle-integrity.receipt.json");
    assert!(diff_path.exists(), "oracle diff exists");
    assert!(receipt_path.exists(), "oracle receipt exists");

    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).expect("read oracle receipt"))
            .expect("oracle receipt json");
    assert_eq!(receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(receipt["stage"], "oracle_integrity");
    assert_eq!(receipt["status"], "failed");
    assert!(receipt["mitigated_risks"]
        .as_array()
        .expect("mitigated risks")
        .iter()
        .any(|risk| risk == "R-020"));
    assert!(receipt["residual_risks"]
        .as_array()
        .expect("residual risks")
        .iter()
        .any(|risk| risk == "R-087"));
}

#[test]
fn fuzz_emits_replayable_divergence_receipt_for_python_fixture_pair() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixture_root = workspace.join("examples").join("fixtures").join("fuzz");
    let out = workspace
        .join("target")
        .join("pramaan-fuzz-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean fuzz smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "fuzz",
            "--base-repo",
            fixture_root
                .join("base")
                .to_str()
                .expect("utf-8 base fixture path"),
            "--head-repo",
            fixture_root
                .join("head")
                .to_str()
                .expect("utf-8 head fixture path"),
            "--claim-scope",
            fixture_root
                .join("claim_scope.json")
                .to_str()
                .expect("utf-8 claim scope path"),
            "--seed",
            "4242",
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan fuzz");

    assert!(
        output.status.success(),
        "fuzz failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan differential fuzz complete"));
    assert!(stdout.contains("expected"));
    assert!(stdout.contains("unexpected"));
    assert!(stdout.contains("replay:"));

    let evidence_path = out.join("differential-fuzz.json");
    let receipt_path = out.join("receipts").join("differential-fuzz.receipt.json");
    assert!(evidence_path.exists(), "fuzz evidence exists");
    assert!(receipt_path.exists(), "fuzz receipt exists");

    let evidence: serde_json::Value =
        serde_json::from_slice(&fs::read(evidence_path).expect("read fuzz evidence"))
            .expect("fuzz evidence json");
    assert_eq!(evidence["seed"], 4242);
    assert_eq!(evidence["adapter_availability"]["tool_backed"], false);
    assert!(evidence["adapter_availability"]["reason"]
        .as_str()
        .unwrap()
        .contains("deterministic replay evidence"));
    assert!(evidence["corpus_hash"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert!(evidence["replay_path"]
        .as_str()
        .unwrap()
        .contains("fuzz-replay"));
    assert!(evidence["divergences"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["classification"] == "expected"));
    assert!(evidence["divergences"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["classification"] == "unexpected"));
    let first_divergence = evidence["divergences"]
        .as_array()
        .unwrap()
        .first()
        .expect("at least one divergence");
    let case_id = format!(
        "{}#{}",
        first_divergence["stable_id"].as_str().expect("stable id"),
        first_divergence["input"]["index"]
            .as_u64()
            .expect("input index")
    );

    let replay_output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "replay",
            out.to_str().expect("utf-8 output path"),
            "--case",
            &case_id,
        ])
        .output()
        .expect("run pramaan replay");
    assert!(
        replay_output.status.success(),
        "replay failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&replay_output.stdout),
        String::from_utf8_lossy(&replay_output.stderr)
    );
    let replay_stdout = String::from_utf8_lossy(&replay_output.stdout);
    assert!(replay_stdout.contains("Pramaan replay case"));
    assert!(replay_stdout.contains(&case_id));
    assert!(replay_stdout.contains("mode: metadata_replay"));

    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).expect("read fuzz receipt"))
            .expect("fuzz receipt json");
    assert_eq!(receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(receipt["stage"], "differential_fuzz");
    assert_eq!(receipt["status"], "failed");
    assert_eq!(receipt["metadata"]["seed"], "4242");
    assert_eq!(receipt["metadata"]["tool_backed"], "false");
    assert!(receipt["residual_risks"]
        .as_array()
        .unwrap()
        .iter()
        .any(|risk| risk == "R-075"));
}

#[test]
fn fuzz_marks_unsafe_discovery_not_applicable() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let unsafe_fixture = workspace
        .join("examples")
        .join("fixtures")
        .join("fuzz")
        .join("unsafe");
    let out = workspace
        .join("target")
        .join("pramaan-fuzz-na-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean fuzz not-applicable output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "fuzz",
            "--base-repo",
            unsafe_fixture.to_str().expect("utf-8 unsafe fixture path"),
            "--head-repo",
            unsafe_fixture.to_str().expect("utf-8 unsafe fixture path"),
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan fuzz not applicable");

    assert!(
        output.status.success(),
        "fuzz not-applicable failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("not_applicable"));

    let receipt_path = out.join("receipts").join("differential-fuzz.receipt.json");
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).expect("read fuzz receipt"))
            .expect("fuzz receipt json");
    assert_eq!(receipt["status"], "not_applicable");
    assert!(receipt["not_applicable_risks"]
        .as_array()
        .unwrap()
        .iter()
        .any(|risk| risk == "R-073"));
}

#[test]
fn fuzz_emits_typescript_fast_check_compatible_fields() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixture_root = workspace.join("examples").join("fixtures").join("fuzz");
    let out = workspace
        .join("target")
        .join("pramaan-fuzz-ts-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean TypeScript fuzz output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "fuzz",
            "--base-repo",
            fixture_root
                .join("base-ts")
                .to_str()
                .expect("utf-8 base TypeScript fixture path"),
            "--head-repo",
            fixture_root
                .join("head-ts")
                .to_str()
                .expect("utf-8 head TypeScript fixture path"),
            "--claim-scope",
            fixture_root
                .join("claim_scope_ts.json")
                .to_str()
                .expect("utf-8 claim scope path"),
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan fuzz for TypeScript");

    assert!(
        output.status.success(),
        "typescript fuzz failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence_path = out.join("differential-fuzz.json");
    let evidence: serde_json::Value =
        serde_json::from_slice(&fs::read(evidence_path).expect("read TypeScript fuzz evidence"))
            .expect("TypeScript fuzz evidence json");
    assert_eq!(evidence["adapter"], "deterministic_simulated");
    assert_eq!(
        evidence["adapter_availability"]["selected_mode"],
        "deterministic_simulated"
    );
    assert_eq!(evidence["adapter_availability"]["tool_backed"], false);
    assert!(evidence["example_database_path"]
        .as_str()
        .unwrap()
        .contains("hypothesis-example-db-or-fast-check-path"));
    assert!(evidence["divergences"]
        .as_array()
        .unwrap()
        .iter()
        .all(|item| item["classification"] == "expected"));
}

#[test]
fn mutation_emits_diff_scoped_receipts_with_budget_metadata() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root")
        .to_path_buf();
    let fixtures = workspace.join("examples").join("fixtures").join("mutation");
    let out = workspace
        .join("target")
        .join("pramaan-mutation-smoke-tests")
        .join(format!("{}", std::process::id()));

    if out.exists() {
        fs::remove_dir_all(&out).expect("clean mutation smoke output");
    }

    let output = Command::new(env!("CARGO_BIN_EXE_pramaan"))
        .current_dir(&workspace)
        .args([
            "mutation",
            "--repo",
            fixtures.to_str().expect("utf-8 fixture path"),
            "--changed-file",
            "python/checkout.py",
            "--changed-file",
            "typescript/src/checkout.ts",
            "--changed-file",
            "rust/src/lib.rs",
            "--timeout-ms",
            "1000",
            "--out",
            out.to_str().expect("utf-8 output path"),
        ])
        .output()
        .expect("run pramaan mutation");

    assert!(
        output.status.success(),
        "mutation failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pramaan mutation checks complete"));
    assert!(stdout.contains("mutation_python_mutmut"));
    assert!(stdout.contains("mutation_typescript_stryker"));
    assert!(stdout.contains("mutation_rust_cargo_mutants"));
    assert!(stdout.contains("R-068"));
    assert!(stdout.contains("R-072"));

    for file_name in [
        "python-mutmut.receipt.json",
        "typescript-stryker.receipt.json",
        "rust-cargo-mutants.receipt.json",
    ] {
        let receipt_path = out.join("receipts").join("mutation").join(file_name);
        assert!(receipt_path.exists(), "{file_name} should exist");
        let receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(receipt_path).expect("read mutation receipt"))
                .expect("mutation receipt json");
        assert_eq!(receipt["schema_version"], "pramaan.receipt.v1");
        assert!(receipt["stage"]
            .as_str()
            .expect("stage")
            .starts_with("mutation_"));
        assert!(receipt["metadata"]["mutants_total"].is_string());
        assert!(receipt["metadata"]["mutants_killed"].is_string());
        assert!(receipt["metadata"]["mutants_survived"].is_string());
        assert!(receipt["metadata"]["mutants_timed_out"].is_string());
        assert!(receipt["metadata"]["mutants_unviable"].is_string());
        assert!(receipt["metadata"]["mutants_skipped"].is_string());
        assert!(receipt["metadata"]["timeout_ms"].is_string());
        assert!(receipt["metadata"]["filter_mode"].is_string());
        assert!(receipt["metadata"]["cache_mode"].is_string());
        assert!(receipt["metadata"]["evidence_mode"].is_string());
        assert!(receipt["metadata"]["risk_ids"]
            .as_str()
            .expect("risk ids metadata")
            .contains("R-068"));
        if ["skipped", "not_applicable"].contains(&receipt["status"].as_str().unwrap()) {
            assert!(
                receipt["mitigated_risks"].as_array().unwrap().is_empty(),
                "skipped mutation tools must not count as mitigated evidence"
            );
            assert!(receipt["not_applicable_risks"]
                .as_array()
                .unwrap()
                .iter()
                .any(|risk| risk == "R-068"));
        }
        for risk_id in ["R-068", "R-069", "R-070", "R-071", "R-072"] {
            let risk_is_present = ["mitigated_risks", "residual_risks", "not_applicable_risks"]
                .iter()
                .any(|bucket| {
                    receipt[*bucket]
                        .as_array()
                        .expect("risk bucket")
                        .iter()
                        .any(|risk| risk == risk_id)
                });
            assert!(
                risk_is_present,
                "{risk_id} should be present in a receipt risk bucket"
            );
        }
    }
}
