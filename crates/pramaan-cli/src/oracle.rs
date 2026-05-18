use anyhow::{Context, Result};
use chrono::Utc;
use pramaan_bundle::sha256_hex;
use pramaan_core::{
    diff_oracle_snapshots, discover_oracle_snapshot, oracle_mitigated_risks, timestamp,
    ArtifactRef, InputRef, OracleDiff, OutputRef, Receipt, ReceiptSummary, StageStatus,
    ToolIdentity, RECEIPT_SCHEMA_VERSION,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run_oracle(base_repo: PathBuf, head_repo: PathBuf, out: PathBuf) -> Result<()> {
    let base_repo = base_repo
        .canonicalize()
        .with_context(|| format!("resolving base repository {}", base_repo.display()))?;
    let head_repo = head_repo
        .canonicalize()
        .with_context(|| format!("resolving head repository {}", head_repo.display()))?;

    fs::create_dir_all(&out).with_context(|| format!("creating {}", out.display()))?;
    let diff = diff_oracle_snapshots(
        discover_oracle_snapshot(&base_repo)
            .with_context(|| format!("discovering oracle data in {}", base_repo.display()))?,
        discover_oracle_snapshot(&head_repo)
            .with_context(|| format!("discovering oracle data in {}", head_repo.display()))?,
    );

    let diff_path = out.join("oracle-diff.json");
    write_json(&diff_path, &diff)?;
    let diff_digest = digest_file(&diff_path)?;

    let receipt_path = out.join("receipts").join("oracle-integrity.receipt.json");
    let receipt = oracle_receipt(&base_repo, &head_repo, &diff_path, &diff_digest, &diff);
    write_json(&receipt_path, &receipt)?;

    render_oracle_summary(&base_repo, &head_repo, &out, &receipt_path, &diff);
    Ok(())
}

fn oracle_receipt(
    base_repo: &Path,
    head_repo: &Path,
    diff_path: &Path,
    diff_digest: &str,
    diff: &OracleDiff,
) -> Receipt {
    let started_at = Utc::now();
    let residual_risks = diff
        .findings
        .iter()
        .flat_map(|finding| finding.risk_ids.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let status = if diff.findings.is_empty() {
        StageStatus::Passed
    } else {
        StageStatus::Failed
    };
    let changed_artifacts = diff
        .findings
        .iter()
        .filter(|finding| finding.test_name.is_none())
        .count();

    Receipt {
        schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
        stage: "oracle_integrity".to_string(),
        status,
        tool: ToolIdentity::new("pramaan-oracle", env!("CARGO_PKG_VERSION")),
        started_at: timestamp(started_at),
        ended_at: timestamp(Utc::now()),
        exit_code: Some(if status == StageStatus::Passed { 0 } else { 1 }),
        inputs: vec![
            InputRef {
                name: "base_repo".to_string(),
                value: portable_path(base_repo),
                digest: None,
            },
            InputRef {
                name: "head_repo".to_string(),
                value: portable_path(head_repo),
                digest: None,
            },
        ],
        outputs: vec![OutputRef {
            name: "oracle_diff".to_string(),
            path: portable_path(diff_path),
            digest: Some(diff_digest.to_string()),
        }],
        artifacts: vec![ArtifactRef {
            name: "oracle_diff_json".to_string(),
            path: portable_path(diff_path),
            media_type: Some("application/json".to_string()),
            digest: Some(diff_digest.to_string()),
        }],
        summary: ReceiptSummary {
            title: if diff.findings.is_empty() {
                "Oracle integrity passed".to_string()
            } else {
                "Oracle integrity failed".to_string()
            },
            details: format!(
                "Discovered {} base tests, {} head tests, {} sensitive artifacts, and {} oracle findings.",
                diff.base.tests.len(),
                diff.head.tests.len(),
                diff.base.artifacts.len() + diff.head.artifacts.len(),
                diff.findings.len()
            ),
        },
        limitations: vec![
            "Oracle weakening detection uses deterministic heuristics for Python and JS/TS test syntax; it is conservative and review-oriented.".to_string(),
            "Claim-vs-oracle scope mismatch risks are covered by risk IDs but require claim-scope evidence from adjacent stages for final judgment.".to_string(),
        ],
        mitigated_risks: oracle_mitigated_risks(),
        residual_risks,
        not_applicable_risks: Vec::new(),
        agent_author: None,
        reviewer_override: None,
        multi_agent_provenance: Vec::new(),
        plugin_identity: None,
        plugin_permissions: None,
        evidence_sensitivity: None,
        redaction_manifest: None,
        policy_decision: None,
        stage_budget: None,
        metadata: BTreeMap::from([
            ("base_tests".to_string(), diff.base.tests.len().to_string()),
            ("head_tests".to_string(), diff.head.tests.len().to_string()),
            ("findings".to_string(), diff.findings.len().to_string()),
            (
                "sensitive_artifact_findings".to_string(),
                changed_artifacts.to_string(),
            ),
        ]),
    }
}

fn render_oracle_summary(
    base_repo: &Path,
    head_repo: &Path,
    out: &Path,
    receipt_path: &Path,
    diff: &OracleDiff,
) {
    println!("Pramaan oracle integrity complete");
    println!("base_repo: {}", base_repo.display());
    println!("head_repo: {}", head_repo.display());
    println!("bundle: {}", out.display());
    println!("receipt: {}", receipt_path.display());
    println!();
    println!(
        "Tests: base={} head={}",
        diff.base.tests.len(),
        diff.head.tests.len()
    );
    println!(
        "Sensitive artifacts: base={} head={}",
        diff.base.artifacts.len(),
        diff.head.artifacts.len()
    );
    println!("Findings: {}", diff.findings.len());

    if !diff.findings.is_empty() {
        println!();
        println!("{:<32} {:<28} {}", "kind", "test", "path");
        for finding in &diff.findings {
            println!(
                "{:<32} {:<28} {}",
                finding.kind.as_str(),
                finding.test_name.as_deref().unwrap_or("-"),
                finding.path
            );
        }
    }
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating parent directory {}", parent.display()))?;
    }

    let bytes = serde_json::to_vec_pretty(value).context("serializing JSON artifact")?;
    fs::write(path, bytes).with_context(|| format!("writing {}", path.display()))
}

fn digest_file(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(sha256_hex(bytes))
}

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
