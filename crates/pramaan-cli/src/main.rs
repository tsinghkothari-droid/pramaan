use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pramaan_bundle::sha256_hex;
use pramaan_core::{
    risk_family, ArtifactRef, ClaimScope, OutputRef, Receipt, ReceiptSummary, RiskRefs, StageStatus,
};
use pramaan_sandbox::{SandboxPlan, SandboxRunner};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

mod oracle;
mod static_checks;

#[derive(Debug, Parser)]
#[command(name = "pramaan")]
#[command(about = "Execution-grounded receipts for AI-generated code changes.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Verify(VerifyArgs),
    StaticChecks(StaticChecksArgs),
    Oracle(OracleArgs),
}

#[derive(Debug, Parser)]
struct VerifyArgs {
    #[arg(long)]
    base: String,
    #[arg(long)]
    head: String,
    #[arg(long, default_value = "target/pramaan")]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct StaticChecksArgs {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[arg(long, default_value = "target/pramaan/static")]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct OracleArgs {
    #[arg(long)]
    base_repo: PathBuf,
    #[arg(long)]
    head_repo: PathBuf,
    #[arg(long, default_value = "target/pramaan/oracle")]
    out: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify(args) => run_verify(args),
        Commands::StaticChecks(args) => static_checks::run_static_checks(args.repo, args.out),
        Commands::Oracle(args) => oracle::run_oracle(args.base_repo, args.head_repo, args.out),
    }
}

fn run_verify(args: VerifyArgs) -> Result<()> {
    let receipt_dir = args.out.join("receipts");
    fs::create_dir_all(&receipt_dir)
        .with_context(|| format!("creating output directory {}", receipt_dir.display()))?;

    let claim_scope = ClaimScope::synthetic(&args.base, &args.head);
    let claim_scope_path = args.out.join("claim_scope.synthetic.json");
    write_json(&claim_scope_path, &claim_scope)?;
    let claim_scope_digest = digest_file(&claim_scope_path)?;

    let claim_receipt_path = receipt_dir.join("claim-scope.receipt.json");
    let claim_receipt = Receipt::synthetic(
        "claim_scope",
        StageStatus::Passed,
        &args.base,
        &args.head,
        vec![OutputRef {
            name: "claim_scope".to_string(),
            path: portable_path(&claim_scope_path),
            digest: Some(claim_scope_digest),
        }],
        vec![ArtifactRef {
            name: "claim_scope_json".to_string(),
            path: portable_path(&claim_scope_path),
            media_type: Some("application/json".to_string()),
            digest: None,
        }],
        ReceiptSummary {
            title: "Synthetic claim scope emitted".to_string(),
            details: "Claim scope was generated from CLI refs only; no PR metadata was inspected."
                .to_string(),
        },
        RiskRefs::claim_scope_sample(),
    );
    write_json(&claim_receipt_path, &claim_receipt)?;

    let sandbox_dir = args.out.join("sandbox");
    let mut sandbox_runner = SandboxRunner::new(
        std::env::current_dir().context("resolving current repository directory")?,
        &sandbox_dir,
    );
    if let Ok(image_digest) = std::env::var("PRAMAAN_IMAGE_DIGEST") {
        if !image_digest.trim().is_empty() {
            sandbox_runner = sandbox_runner.with_image_digest(image_digest);
        }
    }
    let sandbox_run = sandbox_runner
        .prepare(&SandboxPlan::isolated_worktree(&args.base, &args.head))
        .context("preparing isolated base/head sandbox worktrees")?;
    let sandbox_evidence_path = sandbox_dir.join("sandbox-evidence.json");
    write_json(&sandbox_evidence_path, &sandbox_run.evidence)?;
    let sandbox_evidence_digest = digest_file(&sandbox_evidence_path)?;

    let sandbox_receipt_path = receipt_dir.join("sandbox-setup.receipt.json");
    let sandbox_receipt = Receipt {
        schema_version: pramaan_core::RECEIPT_SCHEMA_VERSION.to_string(),
        stage: "sandbox_setup".to_string(),
        status: StageStatus::Passed,
        tool: pramaan_core::ToolIdentity::new("pramaan-sandbox", env!("CARGO_PKG_VERSION")),
        started_at: claim_receipt.ended_at.clone(),
        ended_at: claim_receipt.ended_at.clone(),
        exit_code: Some(0),
        inputs: vec![
            pramaan_core::InputRef {
                name: "base".to_string(),
                value: args.base.clone(),
                digest: Some(sandbox_run.evidence.base.commit_sha.clone()),
            },
            pramaan_core::InputRef {
                name: "head".to_string(),
                value: args.head.clone(),
                digest: Some(sandbox_run.evidence.head.commit_sha.clone()),
            },
        ],
        outputs: vec![OutputRef {
            name: "sandbox_evidence".to_string(),
            path: portable_path(&sandbox_evidence_path),
            digest: Some(sandbox_evidence_digest),
        }],
        artifacts: vec![ArtifactRef {
            name: "sandbox_evidence_json".to_string(),
            path: portable_path(&sandbox_evidence_path),
            media_type: Some("application/json".to_string()),
            digest: None,
        }],
        summary: ReceiptSummary {
            title: "Sandbox worktrees prepared".to_string(),
            details: format!(
                "Base {} and head {} were materialized into isolated worktrees; hermetic={}.",
                sandbox_run.evidence.base.commit_sha,
                sandbox_run.evidence.head.commit_sha,
                sandbox_run.evidence.hermetic
            ),
        },
        limitations: sandbox_run.evidence.limitations.clone(),
        mitigated_risks: sandbox_run.evidence.mitigated_risks.clone(),
        residual_risks: sandbox_run.evidence.residual_risks.clone(),
        not_applicable_risks: sandbox_run.evidence.not_applicable_risks.clone(),
        metadata: BTreeMap::from([
            (
                "base_worktree".to_string(),
                sandbox_run.evidence.base.path.clone(),
            ),
            (
                "head_worktree".to_string(),
                sandbox_run.evidence.head.path.clone(),
            ),
            (
                "source_dirty".to_string(),
                sandbox_run.evidence.source_dirty_state.is_dirty.to_string(),
            ),
        ]),
    };
    write_json(&sandbox_receipt_path, &sandbox_receipt)?;

    let synthetic_receipt_path = receipt_dir.join("synthetic-verification.receipt.json");
    let synthetic_receipt = Receipt::synthetic(
        "synthetic_verification",
        StageStatus::NotApplicable,
        &args.base,
        &args.head,
        vec![OutputRef {
            name: "synthetic_receipt".to_string(),
            path: portable_path(&synthetic_receipt_path),
            digest: None,
        }],
        vec![ArtifactRef {
            name: "phase_1_contract".to_string(),
            path: portable_path(&args.out),
            media_type: Some("inode/directory".to_string()),
            digest: None,
        }],
        ReceiptSummary {
            title: "Synthetic verifier placeholder completed".to_string(),
            details:
                "No repository checks ran; this receipt exercises status, artifact, and risk fields."
                    .to_string(),
        },
        RiskRefs::sample(),
    );
    write_json(&synthetic_receipt_path, &synthetic_receipt)?;

    render_summary(
        &args,
        &[
            (&claim_receipt, &claim_receipt_path),
            (&sandbox_receipt, &sandbox_receipt_path),
            (&synthetic_receipt, &synthetic_receipt_path),
        ],
    );

    Ok(())
}

fn render_summary(args: &VerifyArgs, receipts: &[(&Receipt, &Path)]) {
    println!("Pramaan synthetic verification complete");
    println!("base: {}", args.base);
    println!("head: {}", args.head);
    println!("bundle: {}", args.out.display());
    println!();
    println!("Stages");
    println!("{:<24} {:<16} {}", "stage", "status", "receipt");

    for (receipt, path) in receipts {
        println!(
            "{:<24} {:<16} {}",
            receipt.stage,
            receipt.status.as_str(),
            path.display()
        );
    }

    let risks = summarize_risks(receipts);

    println!();
    println!("Risk families");
    println!(
        "{:<12} {}",
        "mitigated",
        format_family_counts(&risks.mitigated)
    );
    println!(
        "{:<12} {}",
        "residual",
        format_family_counts(&risks.residual)
    );
    println!("{:<12} {}", "skipped", format_family_counts(&risks.skipped));
    println!();
    println!("Note: synthetic Phase 1 receipts record evidence shape only; they do not prove the code correct.");
}

#[derive(Default)]
struct RiskSummary {
    mitigated: BTreeMap<&'static str, usize>,
    residual: BTreeMap<&'static str, usize>,
    skipped: BTreeMap<&'static str, usize>,
}

fn summarize_risks(receipts: &[(&Receipt, &Path)]) -> RiskSummary {
    let mut summary = RiskSummary::default();

    for (receipt, _) in receipts {
        count_families(&receipt.mitigated_risks, &mut summary.mitigated);
        count_families(&receipt.residual_risks, &mut summary.residual);
        count_families(&receipt.not_applicable_risks, &mut summary.skipped);
    }

    summary
}

fn count_families(risk_ids: &[String], counts: &mut BTreeMap<&'static str, usize>) {
    for risk_id in risk_ids {
        *counts.entry(risk_family(risk_id)).or_default() += 1;
    }
}

fn format_family_counts(counts: &BTreeMap<&'static str, usize>) -> String {
    if counts.is_empty() {
        return "none".to_string();
    }

    counts
        .iter()
        .map(|(family, count)| format!("{family}({count})"))
        .collect::<Vec<_>>()
        .join(", ")
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
