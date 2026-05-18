use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    assert!(stdout.contains("Pramaan synthetic verification complete"));
    assert!(stdout.contains("bundle:"));
    assert!(stdout.contains("claim_scope"));
    assert!(stdout.contains("synthetic_verification"));
    assert!(stdout.contains("passed"));
    assert!(stdout.contains("not_applicable"));
    assert!(stdout.contains("Risk families"));
    assert!(stdout.contains("mitigated"));
    assert!(stdout.contains("residual"));
    assert!(stdout.contains("skipped"));
    assert!(stdout.contains("do not prove the code correct"));

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

    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(manifest_path).expect("read manifest"))
            .expect("manifest json");
    assert_eq!(manifest["agent_attribution"][0]["product"], "Codex");
    assert_eq!(manifest["plugin_identities"][0]["name"], "pramaan-core");
    assert_eq!(manifest["redaction_manifest"]["profile"], "internal-full");
    assert_eq!(manifest["policy_decision"]["decision"], "informational");
    assert_eq!(manifest["stage_budgets"][0]["partial_evidence"], true);
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
        "broken_import"
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
    assert!(stdout.contains("weakened_assertion"));
    assert!(stdout.contains("sensitive_artifact_changed"));

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

    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(receipt_path).expect("read fuzz receipt"))
            .expect("fuzz receipt json");
    assert_eq!(receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(receipt["stage"], "differential_fuzz");
    assert_eq!(receipt["status"], "failed");
    assert_eq!(receipt["metadata"]["seed"], "4242");
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
        assert!(receipt["metadata"]["risk_ids"]
            .as_str()
            .expect("risk ids metadata")
            .contains("R-068"));
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
