use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pramaan_bundle::{
    build_manifest, read_manifest, sha256_hex, verify_bundle, write_manifest, BundleBuildOptions,
    MANIFEST_FILE_NAME,
};
use pramaan_core::{
    default_policy_profile, evaluate_default_policy, risk_family, AgentAttribution, ArtifactRef,
    AttributionConfidence, ClaimScope, EvidenceSensitivity, OutputRef, PluginIdentity,
    PluginPermissions, PolicyDecision, PolicyStageEvidence, Receipt, ReceiptSummary,
    RedactionManifest, RiskRefs, StageBudget, StageStatus,
};
use pramaan_sandbox::{SandboxPlan, SandboxRunner};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod fuzz;
mod mutation;
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
    Bundle(BundleArgs),
    StaticChecks(StaticChecksArgs),
    Oracle(OracleArgs),
    Mutation(MutationArgs),
    Fuzz(FuzzArgs),
    Policy(PolicyArgs),
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
struct BundleArgs {
    #[command(subcommand)]
    command: BundleCommands,
}

#[derive(Debug, Subcommand)]
enum BundleCommands {
    Verify(BundleVerifyArgs),
}

#[derive(Debug, Parser)]
struct BundleVerifyArgs {
    path: PathBuf,
}

#[derive(Debug, Parser)]
struct PolicyArgs {
    #[command(subcommand)]
    command: PolicyCommands,
}

#[derive(Debug, Subcommand)]
enum PolicyCommands {
    Explain(PolicyExplainArgs),
}

#[derive(Debug, Parser)]
struct PolicyExplainArgs {
    bundle: PathBuf,
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

#[derive(Debug, Parser)]
struct MutationArgs {
    #[arg(long, default_value = ".")]
    repo: PathBuf,
    #[arg(long, default_value = "target/pramaan/mutation")]
    out: PathBuf,
    #[arg(long = "changed-file")]
    changed_files: Vec<String>,
    #[arg(long, default_value_t = 120000)]
    timeout_ms: u64,
    #[arg(long, default_value_t = 70)]
    kill_threshold: u8,
}

#[derive(Debug, Parser)]
struct FuzzArgs {
    #[arg(long)]
    base_repo: PathBuf,
    #[arg(long)]
    head_repo: PathBuf,
    #[arg(long)]
    claim_scope: Option<PathBuf>,
    #[arg(long, default_value = "target/pramaan/fuzz")]
    out: PathBuf,
    #[arg(long, default_value_t = 1337)]
    seed: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify(args) => run_verify(args),
        Commands::Bundle(args) => run_bundle(args),
        Commands::StaticChecks(args) => static_checks::run_static_checks(args.repo, args.out),
        Commands::Oracle(args) => oracle::run_oracle(args.base_repo, args.head_repo, args.out),
        Commands::Mutation(args) => mutation::run_mutation(
            args.repo,
            args.out,
            args.changed_files,
            args.timeout_ms,
            args.kill_threshold,
        ),
        Commands::Fuzz(args) => fuzz::run_fuzz(
            args.base_repo,
            args.head_repo,
            args.claim_scope,
            args.out,
            args.seed,
        ),
        Commands::Policy(args) => run_policy(args),
    }
}

fn run_policy(args: PolicyArgs) -> Result<()> {
    match args.command {
        PolicyCommands::Explain(args) => run_policy_explain(args.bundle),
    }
}

fn run_policy_explain(bundle: PathBuf) -> Result<()> {
    let manifest_path = if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle
    };
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let stages = manifest
        .stages
        .iter()
        .map(|stage| PolicyStageEvidence {
            id: stage.id.clone(),
            status: stage.status.clone(),
            residual_risks: stage.residual_risks.clone(),
            not_applicable_risks: stage.not_applicable_risks.clone(),
            stage_budget: stage.stage_budget.clone(),
        })
        .collect::<Vec<_>>();
    let profile = default_policy_profile();
    let evaluation = evaluate_default_policy(&stages);

    println!("Pramaan policy explanation");
    println!("manifest: {}", manifest_path.display());
    println!("policy: {}", evaluation.decision.policy_id);
    println!("decision: {}", evaluation.decision.decision);
    println!("required_stages: {}", profile.required_stages.join(", "));
    println!(
        "hard_gate_statuses: {}",
        profile.hard_gate_statuses.join(", ")
    );
    println!("warning_statuses: {}", profile.warning_statuses.join(", "));
    println!("sla_classes:");
    for class in &profile.sla_classes {
        println!(
            "  {}: <= {} changed lines, target {}ms, max {}ms",
            class.name, class.changed_lines_max, class.target_ms, class.max_ms
        );
    }
    print_policy_items("hard_failures", &evaluation.decision.hard_failures);
    print_policy_items("warnings", &evaluation.decision.warnings);
    print_policy_items("waived", &evaluation.decision.waived);
    Ok(())
}

fn print_policy_items(label: &str, items: &[String]) {
    println!("{label}:");
    if items.is_empty() {
        println!("  none");
    } else {
        for item in items {
            println!("  - {item}");
        }
    }
}

fn run_bundle(args: BundleArgs) -> Result<()> {
    match args.command {
        BundleCommands::Verify(args) => {
            let report = verify_bundle(&args.path).context("verifying bundle manifest")?;
            println!("Pramaan bundle verification complete");
            println!("manifest: {}", report.manifest_path.display());
            println!("receipts_checked: {}", report.checked_receipts);
            println!("artifacts_checked: {}", report.checked_artifacts);
            Ok(())
        }
    }
}

fn run_verify(args: VerifyArgs) -> Result<()> {
    let receipt_dir = args.out.join("receipts");
    fs::create_dir_all(&receipt_dir)
        .with_context(|| format!("creating output directory {}", receipt_dir.display()))?;

    let claim_scope = claim_scope_from_context(&args.base, &args.head)?;
    let claim_scope_path = args.out.join("claim_scope.synthetic.json");
    write_json(&claim_scope_path, &claim_scope)?;
    let claim_scope_digest = digest_file(&claim_scope_path)?;

    let claim_receipt_path = receipt_dir.join("claim-scope.receipt.json");
    let claim_summary = if claim_scope.extraction_method == "synthetic_cli_arguments" {
        ReceiptSummary {
            title: "Synthetic claim scope emitted".to_string(),
            details: "Claim scope was generated from CLI refs only; no PR metadata was inspected."
                .to_string(),
        }
    } else {
        ReceiptSummary {
            title: "PR-grounded claim scope emitted".to_string(),
            details: format!(
                "Claim scope used {} source references and detected {} touched public API entries.",
                claim_scope.source_refs.len(),
                claim_scope.touched_public_apis.len()
            ),
        }
    };
    let mut claim_receipt = Receipt::synthetic(
        "claim_scope",
        StageStatus::Passed,
        &args.base,
        &args.head,
        vec![OutputRef {
            name: "claim_scope".to_string(),
            path: bundle_path(&args.out, &claim_scope_path),
            digest: Some(claim_scope_digest),
        }],
        vec![ArtifactRef {
            name: "claim_scope_json".to_string(),
            path: bundle_path(&args.out, &claim_scope_path),
            media_type: Some("application/json".to_string()),
            digest: None,
        }],
        claim_summary,
        RiskRefs::claim_scope_sample(),
    );
    add_synthetic_trust_hooks(&mut claim_receipt);
    write_json(&claim_receipt_path, &claim_receipt)?;

    let sandbox_dir = args.out.join("sandbox");
    let mut sandbox_runner = SandboxRunner::new(
        std::env::current_dir().context("resolving current repository directory")?,
        &sandbox_dir,
    );
    if let Ok(image_name) = std::env::var("PRAMAAN_IMAGE_NAME") {
        if !image_name.trim().is_empty() {
            sandbox_runner = sandbox_runner.with_image_name(image_name);
        }
    }
    if let Ok(image_digest) = std::env::var("PRAMAAN_IMAGE_DIGEST") {
        if !image_digest.trim().is_empty() {
            sandbox_runner = sandbox_runner.with_image_digest(image_digest);
        }
    }
    if let Ok(network_policy) = std::env::var("PRAMAAN_NETWORK_POLICY") {
        if !network_policy.trim().is_empty() {
            sandbox_runner = sandbox_runner.with_network_policy(network_policy);
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
            path: bundle_path(&args.out, &sandbox_evidence_path),
            digest: Some(sandbox_evidence_digest),
        }],
        artifacts: vec![ArtifactRef {
            name: "sandbox_evidence_json".to_string(),
            path: bundle_path(&args.out, &sandbox_evidence_path),
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
            path: bundle_path(&args.out, &args.out),
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

    let manifest = build_manifest(
        &args.out,
        BundleBuildOptions::synthetic(args.base.clone(), args.head.clone()),
    )
    .context("building bundle manifest")?;
    let manifest_path = write_manifest(&args.out, &manifest).context("writing bundle manifest")?;

    render_summary(
        &args,
        &[
            (&claim_receipt, &claim_receipt_path),
            (&sandbox_receipt, &sandbox_receipt_path),
            (&synthetic_receipt, &synthetic_receipt_path),
        ],
        &manifest_path,
    );

    Ok(())
}

fn add_synthetic_trust_hooks(receipt: &mut Receipt) {
    receipt.agent_author = Some(AgentAttribution {
        product: "Codex".to_string(),
        model_family: Some("gpt-5".to_string()),
        model_version: None,
        execution_mode: "synthetic_verify".to_string(),
        prompt_context_hash: None,
        commit_provenance: None,
        source: "local_cli".to_string(),
        confidence: AttributionConfidence::Unknown,
    });
    receipt.plugin_identity = Some(PluginIdentity {
        name: "pramaan-core".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        provenance: "workspace".to_string(),
        signature: None,
        sandbox_boundary: "in_process".to_string(),
    });
    receipt.plugin_permissions = Some(PluginPermissions {
        may_emit_receipts: true,
        may_emit_artifacts: true,
        may_read_previous_receipts: false,
        may_modify_previous_receipts: false,
        may_modify_manifest: false,
    });
    receipt.evidence_sensitivity = Some(EvidenceSensitivity::Internal);
    receipt.redaction_manifest = Some(RedactionManifest {
        profile: "internal-full".to_string(),
        redacted_fields: Vec::new(),
        hashed_fields: Vec::new(),
        policy: "pramaan-redaction-v0".to_string(),
    });
    receipt.policy_decision = Some(PolicyDecision {
        decision: "informational".to_string(),
        policy_id: "pramaan-default-v0".to_string(),
        hard_failures: Vec::new(),
        warnings: vec!["synthetic_evidence_only".to_string()],
        waived: Vec::new(),
    });
    receipt.stage_budget = Some(StageBudget {
        target_ms: 30_000,
        max_ms: 60_000,
        consumed_ms: 0,
        exhausted: false,
        timeout_reason: None,
        partial_evidence: true,
    });
}

fn claim_scope_from_context(base_ref: &str, head_ref: &str) -> Result<ClaimScope> {
    let mut scope = ClaimScope::synthetic(base_ref, head_ref);
    let mut source_refs = scope.source_refs.clone();
    let mut expected_behavior = Vec::new();
    let mut limitations = Vec::new();
    let mut risk_refs = Vec::new();

    if let Ok(event_path) = std::env::var("GITHUB_EVENT_PATH") {
        if !event_path.trim().is_empty() {
            let path = PathBuf::from(&event_path);
            if path.exists() {
                let event: serde_json::Value = serde_json::from_slice(
                    &fs::read(&path)
                        .with_context(|| format!("reading GitHub event {}", path.display()))?,
                )
                .context("parsing GitHub event JSON")?;
                if let Some(title) = event
                    .pointer("/pull_request/title")
                    .and_then(serde_json::Value::as_str)
                {
                    expected_behavior.push(format!("PR title: {title}"));
                }
                if let Some(body) = event
                    .pointer("/pull_request/body")
                    .and_then(serde_json::Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                {
                    expected_behavior.push(format!("PR body: {}", body.trim()));
                    for issue in linked_issue_refs(body) {
                        source_refs.push(pramaan_core::SourceRef {
                            kind: "linked_issue".to_string(),
                            reference: issue,
                        });
                    }
                }
                source_refs.push(pramaan_core::SourceRef {
                    kind: "github_event".to_string(),
                    reference: event_path,
                });
            }
        }
    }

    if let Ok(title) = std::env::var("PRAMAAN_PR_TITLE") {
        if !title.trim().is_empty() {
            expected_behavior.push(format!("PR title: {}", title.trim()));
        }
    }
    if let Ok(body) = std::env::var("PRAMAAN_PR_BODY") {
        if !body.trim().is_empty() {
            expected_behavior.push(format!("PR body: {}", body.trim()));
            for issue in linked_issue_refs(&body) {
                source_refs.push(pramaan_core::SourceRef {
                    kind: "linked_issue".to_string(),
                    reference: issue,
                });
            }
        }
    }
    if let Some(issue_text) = read_optional_text_env("PRAMAAN_ISSUE_TEXT", "PRAMAAN_ISSUE_PATH")? {
        expected_behavior.push(format!(
            "Linked issue: {}",
            first_non_empty_line(&issue_text)
        ));
        source_refs.push(pramaan_core::SourceRef {
            kind: "issue_text".to_string(),
            reference: "PRAMAAN_ISSUE_TEXT_OR_PATH".to_string(),
        });
    }
    if let Some(scope_note) = read_optional_scope_note()? {
        expected_behavior.push(format!(
            "Maintainer scope note: {}",
            first_non_empty_line(&scope_note)
        ));
        source_refs.push(pramaan_core::SourceRef {
            kind: "maintainer_scope_note".to_string(),
            reference: ".pramaan-scope.md".to_string(),
        });
    }

    let public_apis = changed_public_apis(base_ref, head_ref).unwrap_or_else(|error| {
        limitations.push(format!("Changed public API detection failed: {error}"));
        risk_refs.push("R-004".to_string());
        Vec::new()
    });

    if !expected_behavior.is_empty() {
        scope.expected_behavior = expected_behavior;
        scope.extraction_method = "github_event_or_environment".to_string();
        scope.confidence = pramaan_core::ClaimConfidence::High;
    } else {
        limitations.push(
            "No PR title, PR body, issue text, or maintainer scope note was available; claim scope is low confidence."
                .to_string(),
        );
        risk_refs.extend(["R-001".to_string(), "R-002".to_string()]);
        scope.confidence = pramaan_core::ClaimConfidence::Low;
    }
    if public_apis.is_empty() {
        scope.touched_public_apis = Vec::new();
        limitations.push("No changed public APIs were detected by deterministic scan.".to_string());
    } else {
        if !scope_mentions_changed_api(&scope.expected_behavior, &public_apis) {
            limitations.push(
                "Changed public APIs were detected, but claim text does not mention matching symbols; semantic scope mismatch needs review."
                    .to_string(),
            );
            risk_refs.push("R-007".to_string());
        }
        scope.touched_public_apis = public_apis;
    }
    scope.source_refs = source_refs;
    scope.limitations = limitations;
    risk_refs.sort();
    risk_refs.dedup();
    scope.risk_refs = risk_refs;
    Ok(scope)
}

fn read_optional_text_env(value_var: &str, path_var: &str) -> Result<Option<String>> {
    if let Ok(value) = std::env::var(value_var) {
        if !value.trim().is_empty() {
            return Ok(Some(value));
        }
    }
    if let Ok(path) = std::env::var(path_var) {
        if !path.trim().is_empty() {
            return Ok(Some(
                fs::read_to_string(&path).with_context(|| format!("reading {path_var} {path}"))?,
            ));
        }
    }
    Ok(None)
}

fn read_optional_scope_note() -> Result<Option<String>> {
    if let Some(value) = read_optional_text_env("PRAMAAN_SCOPE_NOTE", "PRAMAAN_SCOPE_NOTE_PATH")? {
        return Ok(Some(value));
    }
    let path = PathBuf::from(".pramaan-scope.md");
    if path.exists() {
        return Ok(Some(
            fs::read_to_string(&path).context("reading .pramaan-scope.md")?,
        ));
    }
    Ok(None)
}

fn first_non_empty_line(text: &str) -> String {
    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("empty note")
        .chars()
        .take(240)
        .collect()
}

fn scope_mentions_changed_api(expected_behavior: &[String], public_apis: &[String]) -> bool {
    let haystack = expected_behavior.join("\n").to_ascii_lowercase();
    public_apis.iter().any(|api| {
        api.split("::")
            .last()
            .map(|symbol| {
                symbol
                    .split(['(', '{', ':', ' '])
                    .find(|part| part.chars().any(char::is_alphanumeric))
                    .unwrap_or(symbol)
                    .to_ascii_lowercase()
            })
            .is_some_and(|symbol| haystack.contains(&symbol))
    })
}

fn linked_issue_refs(text: &str) -> Vec<String> {
    let tokens = text
        .split_whitespace()
        .map(|token| {
            token.trim_matches(|ch: char| {
                matches!(ch, ',' | '.' | ')' | '(' | '[' | ']' | ':' | ';')
            })
        })
        .collect::<Vec<_>>();
    let mut refs = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        let lower = token.to_ascii_lowercase();
        if lower.starts_with('#') || lower.contains("/issues/") {
            refs.push((*token).to_string());
        } else if matches!(lower.as_str(), "fixes" | "closes" | "refs" | "references") {
            if let Some(next) = tokens.get(index + 1) {
                if next.starts_with('#') || next.contains("/issues/") {
                    refs.push((*next).to_string());
                }
            }
        }
    }
    refs.sort();
    refs.dedup();
    refs
}

fn changed_public_apis(base_ref: &str, head_ref: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", base_ref, head_ref])
        .output()
        .context("running git diff for changed public APIs")?;
    if !output.status.success() {
        anyhow::bail!(
            "git diff failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let mut apis = Vec::new();
    for relative in String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
    {
        let path = PathBuf::from(relative);
        if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("py" | "ts" | "tsx" | "rs")
        ) {
            apis.extend(public_symbols_in_file(relative, &path));
        }
    }
    apis.sort();
    apis.dedup();
    Ok(apis)
}

fn public_symbols_in_file(relative: &str, path: &Path) -> Vec<String> {
    let Ok(text) = fs::read_to_string(path) else {
        return vec![relative.to_string()];
    };
    let extension = path.extension().and_then(|extension| extension.to_str());
    let mut symbols = Vec::new();
    for line in text.lines().map(str::trim) {
        match extension {
            Some("py") if line.starts_with("def ") || line.starts_with("class ") => {
                symbols.push(format!("{relative}::{line}"));
            }
            Some("ts" | "tsx") if line.starts_with("export ") => {
                symbols.push(format!("{relative}::{line}"));
            }
            Some("rs") if line.starts_with("pub ") => {
                symbols.push(format!("{relative}::{line}"));
            }
            _ => {}
        }
    }
    if symbols.is_empty() {
        vec![relative.to_string()]
    } else {
        symbols
    }
}

fn render_summary(args: &VerifyArgs, receipts: &[(&Receipt, &Path)], manifest_path: &Path) {
    println!("Pramaan verification bundle emitted");
    println!("base: {}", args.base);
    println!("head: {}", args.head);
    println!("bundle: {}", args.out.display());
    println!("manifest: {}", manifest_path.display());
    println!();
    println!("Stages");
    println!("{:<24} {:<16} receipt", "stage", "status");

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
    println!(
        "{:<12} {}",
        "not_applicable",
        format_family_counts(&risks.not_applicable)
    );
    println!();
    println!(
        "Note: Pramaan records evidence and residual risk; it does not prove the code correct."
    );
}

#[derive(Default)]
struct RiskSummary {
    mitigated: BTreeMap<&'static str, usize>,
    residual: BTreeMap<&'static str, usize>,
    skipped: BTreeMap<&'static str, usize>,
    not_applicable: BTreeMap<&'static str, usize>,
}

fn summarize_risks(receipts: &[(&Receipt, &Path)]) -> RiskSummary {
    let mut summary = RiskSummary::default();

    for (receipt, _) in receipts {
        count_families(&receipt.mitigated_risks, &mut summary.mitigated);
        if receipt.status == StageStatus::Skipped {
            count_families(&receipt.residual_risks, &mut summary.skipped);
        } else {
            count_families(&receipt.residual_risks, &mut summary.residual);
        }
        count_families(&receipt.not_applicable_risks, &mut summary.not_applicable);
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

fn bundle_path(bundle_root: &Path, path: &Path) -> String {
    path.strip_prefix(bundle_root)
        .map(|relative| {
            if relative.as_os_str().is_empty() {
                ".".to_string()
            } else {
                portable_path(relative)
            }
        })
        .unwrap_or_else(|_| portable_path(path))
}
