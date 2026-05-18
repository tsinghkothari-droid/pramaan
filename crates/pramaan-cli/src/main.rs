use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pramaan_bundle::sha256_hex;
use pramaan_core::{
    risk_family, ArtifactRef, ClaimScope, OutputRef, Receipt, ReceiptSummary, RiskRefs, StageStatus,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify(args) => run_verify(args),
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
