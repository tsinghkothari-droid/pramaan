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
