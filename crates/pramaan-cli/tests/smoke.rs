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

    assert!(claim_scope_path.exists(), "claim scope should be written");
    assert!(
        claim_receipt_path.exists(),
        "claim receipt should be written"
    );
    assert!(
        synthetic_receipt_path.exists(),
        "synthetic receipt should be written"
    );

    let claim_receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(claim_receipt_path).expect("read claim receipt"))
            .expect("claim receipt json");
    assert_eq!(claim_receipt["schema_version"], "pramaan.receipt.v1");
    assert_eq!(claim_receipt["stage"], "claim_scope");
    assert_eq!(claim_receipt["status"], "passed");
    assert!(claim_receipt["residual_risks"].is_array());
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
