use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use pramaan_bundle::{
    build_manifest, read_manifest, sha256_hex, verify_bundle, write_manifest, BundleBuildOptions,
    BundleManifest, MANIFEST_FILE_NAME,
};
use pramaan_core::{
    build_agent_decision, build_confidence_artifact, canonical_json_bytes, default_policy_profile,
    evaluate_default_policy, render_confidence_markdown, risk_family, timestamp, AgentAttribution,
    ArtifactRef, AttributionConfidence, ClaimScope, ConfidenceDecision, EvidenceSensitivity,
    FuzzDivergence, FuzzRunEvidence, OutputRef, PluginIdentity, PluginPermissions, PolicyDecision,
    PolicyStageEvidence, ProbeCandidate, ProbeDecision, ProbeKind, ProbeLanguage,
    ProbePlanArtifact, ProbeProvider, ProbeSandboxStatus, Receipt, ReceiptSummary,
    RedactionManifest, RiskRefs, StageBudget, StageStatus, ToolIdentity, PROBE_SCHEMA_VERSION,
    RECEIPT_SCHEMA_VERSION,
};
use pramaan_sandbox::{SandboxPlan, SandboxRunner};
use std::collections::{BTreeMap, BTreeSet};
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
    Confidence(ConfidenceArgs),
    Agent(AgentArgs),
    Probe(ProbeArgs),
    Replay(ReplayArgs),
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
struct ConfidenceArgs {
    #[command(subcommand)]
    command: ConfidenceCommands,
}

#[derive(Debug, Subcommand)]
enum ConfidenceCommands {
    Explain(ConfidenceExplainArgs),
}

#[derive(Debug, Parser)]
struct ConfidenceExplainArgs {
    bundle: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct AgentArgs {
    #[command(subcommand)]
    command: AgentCommands,
}

#[derive(Debug, Subcommand)]
enum AgentCommands {
    DoneGate(AgentDoneGateArgs),
    Explain(AgentExplainArgs),
}

#[derive(Debug, Parser)]
struct AgentDoneGateArgs {
    #[arg(long)]
    base: String,
    #[arg(long)]
    head: String,
    #[arg(long, default_value = "target/pramaan-agent")]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct AgentExplainArgs {
    #[arg(long)]
    bundle: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProbeArgs {
    #[command(subcommand)]
    command: ProbeCommands,
}

#[derive(Debug, Subcommand)]
enum ProbeCommands {
    Plan(ProbePlanArgs),
}

#[derive(Debug, Parser)]
struct ProbePlanArgs {
    #[arg(long)]
    bundle: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ReplayArgs {
    bundle: PathBuf,
    #[arg(long)]
    case: String,
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
        Commands::Confidence(args) => run_confidence(args),
        Commands::Agent(args) => run_agent(args),
        Commands::Probe(args) => run_probe(args),
        Commands::Replay(args) => run_replay(args),
    }
}

fn run_replay(args: ReplayArgs) -> Result<()> {
    let evidence_path = resolve_fuzz_evidence_path(&args.bundle)?;
    let evidence: FuzzRunEvidence = serde_json::from_slice(
        &fs::read(&evidence_path)
            .with_context(|| format!("reading fuzz evidence {}", evidence_path.display()))?,
    )
    .with_context(|| format!("parsing fuzz evidence {}", evidence_path.display()))?;
    let divergence = evidence
        .divergences
        .iter()
        .find(|item| replay_case_matches(item, &args.case))
        .with_context(|| {
            format!(
                "case {} not found in {}",
                args.case,
                evidence_path.display()
            )
        })?;

    println!("Pramaan replay case");
    println!("evidence: {}", evidence_path.display());
    println!("case_id: {}", replay_case_id(divergence));
    println!("stable_id: {}", divergence.stable_id);
    println!("classification: {}", divergence.classification.as_str());
    println!("function: {}", divergence.function_name);
    println!("path: {}", divergence.path);
    println!(
        "input: {}",
        serde_json::to_string(&divergence.input).context("serializing replay input")?
    );
    println!("base_output: {}", divergence.base_output);
    println!("head_output: {}", divergence.head_output);
    println!("rationale: {}", divergence.rationale);
    println!("mode: metadata_replay");
    Ok(())
}

fn resolve_fuzz_evidence_path(bundle: &Path) -> Result<PathBuf> {
    if bundle.is_file() {
        return Ok(bundle.to_path_buf());
    }
    let direct = bundle.join("differential-fuzz.json");
    if direct.exists() {
        return Ok(direct);
    }
    let nested = bundle.join("fuzz").join("differential-fuzz.json");
    if nested.exists() {
        return Ok(nested);
    }
    anyhow::bail!(
        "could not find differential-fuzz.json in {}",
        bundle.display()
    )
}

fn replay_case_matches(divergence: &FuzzDivergence, requested: &str) -> bool {
    requested == replay_case_id(divergence)
        || requested == divergence.stable_id
        || requested == divergence.function_name
}

fn replay_case_id(divergence: &FuzzDivergence) -> String {
    format!("{}#{}", divergence.stable_id, divergence.input.index)
}

fn run_probe(args: ProbeArgs) -> Result<()> {
    match args.command {
        ProbeCommands::Plan(args) => run_probe_plan(args),
    }
}

fn run_probe_plan(args: ProbePlanArgs) -> Result<()> {
    let manifest_path = resolve_manifest_path(&args.bundle);
    let bundle_root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let receipts = read_bundle_receipts(&bundle_root, &manifest)?;
    let prompt_hash = probe_prompt_hash(&manifest, &receipts)?;
    let artifact = build_probe_plan_artifact(&bundle_root, &receipts, &prompt_hash);
    let write_into_bundle = args.out.is_none();
    let out_dir = args.out.unwrap_or_else(|| bundle_root.join("probes"));
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("creating probe output directory {}", out_dir.display()))?;

    let plan_path = out_dir.join("ai-probe-plan.json");
    write_json(&plan_path, &artifact)?;
    let plan_digest = digest_file(&plan_path)?;
    let receipt_path = if write_into_bundle {
        bundle_root
            .join("receipts")
            .join("ai-probe-generation.receipt.json")
    } else {
        out_dir
            .join("receipts")
            .join("ai-probe-generation.receipt.json")
    };
    let receipt =
        probe_generation_receipt(&manifest, &artifact, &plan_path, &plan_digest, &bundle_root);
    write_json(&receipt_path, &receipt)?;

    if write_into_bundle {
        let updated_manifest = build_manifest(
            &bundle_root,
            BundleBuildOptions {
                bundle_id: manifest.bundle_id,
                run_id: manifest.run_id,
                repository: manifest.repository,
            },
        )
        .context("rebuilding bundle manifest with probe plan artifacts")?;
        write_manifest(&bundle_root, &updated_manifest)
            .context("writing updated bundle manifest")?;
    }

    println!("Pramaan AI probe plan complete");
    println!("manifest: {}", manifest_path.display());
    println!("probe_plan: {}", plan_path.display());
    println!("probes: {}", artifact.probes.len());
    println!("accepted: {}", artifact.accepted_count);
    println!("rejected: {}", artifact.rejected_count);
    println!("pending_execution: {}", artifact.pending_count);
    println!("provider_trusted_for_decision: false");
    Ok(())
}

fn resolve_manifest_path(bundle: &Path) -> PathBuf {
    if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle.to_path_buf()
    }
}

fn probe_prompt_hash(manifest: &BundleManifest, receipts: &[Receipt]) -> Result<String> {
    let receipt_rows = receipts
        .iter()
        .map(|receipt| {
            serde_json::json!({
                "stage": receipt.stage,
                "status": receipt.status.as_str(),
                "residual_risks": receipt.residual_risks,
                "not_applicable_risks": receipt.not_applicable_risks,
                "limitations": receipt.limitations,
            })
        })
        .collect::<Vec<_>>();
    let material = serde_json::json!({
        "generator": "pramaan-ai-probe-plan-v0.1",
        "manifest_digest": manifest.integrity.manifest_digest.prefixed(),
        "final_status": manifest.final_status,
        "risk_summary": manifest.risk_summary,
        "receipts": receipt_rows,
    });
    let bytes = canonical_json_bytes(&material).context("canonicalizing probe prompt material")?;
    Ok(sha256_hex(bytes))
}

fn build_probe_plan_artifact(
    bundle_root: &Path,
    receipts: &[Receipt],
    prompt_hash: &str,
) -> ProbePlanArtifact {
    let probes = build_probe_candidates(receipts, prompt_hash);
    let accepted_count = probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::Kept)
        .count();
    let rejected_count = probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::Rejected)
        .count();
    let pending_count = probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::PendingExecution)
        .count();

    ProbePlanArtifact {
        schema_version: PROBE_SCHEMA_VERSION.to_string(),
        generator_version: "pramaan-ai-probe-plan-v0.1".to_string(),
        source_bundle: portable_path(bundle_root),
        generated_at: timestamp(Utc::now()),
        provider: ProbeProvider {
            name: "provider_neutral".to_string(),
            mode: "deterministic_risk_targeting".to_string(),
            model: None,
            prompt_hash: prompt_hash.to_string(),
            trusted_for_decision: false,
        },
        probes,
        accepted_count,
        rejected_count,
        pending_count,
        limitations: vec![
            "AI/provider output is not trusted evidence; generated probes must execute before they mitigate risk.".to_string(),
            "Phase 28.25 creates a provider-neutral probe plan. Safe sandbox execution is split to a follow-up phase.".to_string(),
        ],
    }
}

fn build_probe_candidates(receipts: &[Receipt], prompt_hash: &str) -> Vec<ProbeCandidate> {
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for receipt in receipts {
        for risk_id in receipt
            .residual_risks
            .iter()
            .chain(receipt.not_applicable_risks.iter())
        {
            if seen.insert((receipt.stage.clone(), risk_id.clone())) {
                rows.push((receipt, risk_id.clone()));
            }
        }
    }

    rows.into_iter()
        .enumerate()
        .map(|(index, (receipt, risk_id))| {
            let kind = probe_kind_for(&receipt.stage, &risk_id);
            let language = probe_language_for(receipt);
            let target_files = probe_target_files(receipt);
            ProbeCandidate {
                probe_id: format!("probe-{index:03}-{}", risk_id.to_ascii_lowercase()),
                risk_ids: vec![risk_id.clone()],
                kind,
                language,
                target_files: target_files.clone(),
                prompt_hash: prompt_hash.to_string(),
                candidate_code: probe_candidate_code(kind, language, &risk_id, &target_files),
                sandbox_status: ProbeSandboxStatus::RequiresExecution,
                execution_result: "not_executed: candidate must run in an isolated temp test location before it can mitigate risk.".to_string(),
                kept_or_rejected: ProbeDecision::PendingExecution,
                rejection_reason: None,
            }
        })
        .collect()
}

fn probe_kind_for(stage: &str, risk_id: &str) -> ProbeKind {
    match risk_family(risk_id) {
        "oracle_integrity" => {
            if stage.contains("oracle") {
                ProbeKind::FixtureSnapshotChallenge
            } else {
                ProbeKind::RegressionAssertion
            }
        }
        "mutation_quality" => ProbeKind::MutationTargetedTest,
        "property_fuzz" => ProbeKind::DifferentialInput,
        "runtime_behavior" => ProbeKind::PropertyInvariant,
        "static_hallucination" | "public_api_compatibility" => ProbeKind::RegressionAssertion,
        "ci_supply_chain" | "bundle_integrity" => ProbeKind::SecuritySinkSourceCheck,
        _ => ProbeKind::RegressionAssertion,
    }
}

fn probe_language_for(receipt: &Receipt) -> ProbeLanguage {
    let mut paths = Vec::new();
    paths.extend(receipt.outputs.iter().map(|output| output.path.as_str()));
    paths.extend(
        receipt
            .artifacts
            .iter()
            .map(|artifact| artifact.path.as_str()),
    );
    for path in paths {
        if path.ends_with(".py") {
            return ProbeLanguage::Python;
        }
        if path.ends_with(".ts") || path.ends_with(".tsx") || path.ends_with(".js") {
            return ProbeLanguage::TypeScript;
        }
        if path.ends_with(".rs") {
            return ProbeLanguage::Rust;
        }
    }
    ProbeLanguage::Unknown
}

fn probe_target_files(receipt: &Receipt) -> Vec<String> {
    let mut paths = receipt
        .outputs
        .iter()
        .map(|output| output.path.clone())
        .chain(
            receipt
                .artifacts
                .iter()
                .map(|artifact| artifact.path.clone()),
        )
        .filter(|path| {
            path.ends_with(".py")
                || path.ends_with(".ts")
                || path.ends_with(".tsx")
                || path.ends_with(".js")
                || path.ends_with(".rs")
                || path.ends_with(".json")
                || path.ends_with(".snap")
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if paths.is_empty() {
        paths.push("unknown_changed_files".to_string());
    }
    paths
}

fn probe_candidate_code(
    kind: ProbeKind,
    language: ProbeLanguage,
    risk_id: &str,
    target_files: &[String],
) -> String {
    format!(
        "// Pramaan provider-neutral probe skeleton\n// kind: {}\n// language: {}\n// risk: {}\n// targets: {}\n// Generate an isolated test/property/input that exercises changed behavior.\n// This skeleton is not mitigation until sandbox execution records a passing result.",
        kind.as_str(),
        language.as_str(),
        risk_id,
        target_files.join(", ")
    )
}

fn probe_generation_receipt(
    manifest: &BundleManifest,
    artifact: &ProbePlanArtifact,
    plan_path: &Path,
    plan_digest: &str,
    bundle_root: &Path,
) -> Receipt {
    let now = timestamp(Utc::now());
    let residual_risks = artifact
        .probes
        .iter()
        .flat_map(|probe| probe.risk_ids.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut metadata = BTreeMap::new();
    metadata.insert(
        "accepted_count".to_string(),
        artifact.accepted_count.to_string(),
    );
    metadata.insert(
        "rejected_count".to_string(),
        artifact.rejected_count.to_string(),
    );
    metadata.insert(
        "pending_count".to_string(),
        artifact.pending_count.to_string(),
    );
    metadata.insert(
        "provider_trusted_for_decision".to_string(),
        artifact.provider.trusted_for_decision.to_string(),
    );
    metadata.insert(
        "prompt_hash".to_string(),
        artifact.provider.prompt_hash.clone(),
    );

    Receipt {
        schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
        stage: "ai_probe_generation".to_string(),
        status: if artifact.probes.is_empty() {
            StageStatus::NotApplicable
        } else {
            StageStatus::Passed
        },
        tool: ToolIdentity::new("pramaan-ai-probe-plan", env!("CARGO_PKG_VERSION")),
        started_at: now.clone(),
        ended_at: now,
        exit_code: Some(0),
        inputs: vec![pramaan_core::InputRef {
            name: "bundle_manifest".to_string(),
            value: MANIFEST_FILE_NAME.to_string(),
            digest: Some(manifest.integrity.manifest_digest.prefixed()),
        }],
        outputs: vec![OutputRef {
            name: "ai_probe_plan".to_string(),
            path: bundle_path(bundle_root, plan_path),
            digest: Some(plan_digest.to_string()),
        }],
        artifacts: vec![ArtifactRef {
            name: "ai_probe_plan_json".to_string(),
            path: bundle_path(bundle_root, plan_path),
            media_type: Some("application/json".to_string()),
            digest: Some(plan_digest.to_string()),
        }],
        summary: ReceiptSummary {
            title: "AI evidence-seeking probe plan emitted".to_string(),
            details: format!(
                "{} probes require sandbox execution; zero AI proposals are trusted as mitigation.",
                artifact.pending_count
            ),
        },
        limitations: artifact.limitations.clone(),
        mitigated_risks: Vec::new(),
        residual_risks,
        not_applicable_risks: Vec::new(),
        agent_author: None,
        reviewer_override: None,
        multi_agent_provenance: Vec::new(),
        plugin_identity: None,
        plugin_permissions: None,
        evidence_sensitivity: Some(EvidenceSensitivity::Internal),
        redaction_manifest: None,
        policy_decision: None,
        stage_budget: None,
        metadata,
    }
}

fn run_agent(args: AgentArgs) -> Result<()> {
    match args.command {
        AgentCommands::DoneGate(args) => run_agent_done_gate(args),
        AgentCommands::Explain(args) => run_agent_explain(args.bundle, args.out),
    }
}

fn run_agent_done_gate(args: AgentDoneGateArgs) -> Result<()> {
    let out = args.out.clone();
    run_verify(VerifyArgs {
        base: args.base,
        head: args.head,
        out: out.clone(),
    })?;
    run_agent_explain(out, None)
}

fn run_agent_explain(bundle: PathBuf, out: Option<PathBuf>) -> Result<()> {
    let manifest_path = if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle.clone()
    };
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let stages = policy_stage_evidence_from_manifest(&manifest);
    let decision = build_agent_decision(portable_path(&bundle), &stages);
    let out_path = out.or_else(|| bundle.is_dir().then(|| bundle.join("agent-decision.json")));
    if let Some(path) = out_path {
        write_json(&path, &decision)?;
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&decision).context("serializing agent decision")?
    );
    Ok(())
}

fn run_confidence(args: ConfidenceArgs) -> Result<()> {
    match args.command {
        ConfidenceCommands::Explain(args) => run_confidence_explain(args.bundle, args.out),
    }
}

fn run_confidence_explain(bundle: PathBuf, out: Option<PathBuf>) -> Result<()> {
    let manifest_path = if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle
    };
    let bundle_root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let receipts = read_bundle_receipts(&bundle_root, &manifest)?;
    let artifact = build_confidence_artifact(&receipts);
    let markdown = render_confidence_markdown(&artifact);
    let write_into_bundle = out.is_none();
    let out_dir = out.unwrap_or_else(|| bundle_root.clone());
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("creating confidence output directory {}", out_dir.display()))?;

    let json_path = out_dir.join("confidence.json");
    let markdown_path = out_dir.join("confidence.md");
    write_json(&json_path, &artifact)?;
    fs::write(&markdown_path, markdown)
        .with_context(|| format!("writing {}", markdown_path.display()))?;

    if write_into_bundle {
        let json_digest = digest_file(&json_path)?;
        let markdown_digest = digest_file(&markdown_path)?;
        let receipt_path = bundle_root
            .join("receipts")
            .join("confidence-vote.receipt.json");
        let receipt = confidence_receipt(
            &manifest,
            &artifact,
            &json_path,
            &json_digest,
            &markdown_path,
            &markdown_digest,
            &bundle_root,
        );
        write_json(&receipt_path, &receipt)?;
        let updated_manifest = build_manifest(
            &bundle_root,
            BundleBuildOptions {
                bundle_id: manifest.bundle_id,
                run_id: manifest.run_id,
                repository: manifest.repository,
            },
        )
        .context("rebuilding bundle manifest with confidence artifacts")?;
        write_manifest(&bundle_root, &updated_manifest)
            .context("writing updated bundle manifest")?;
    }

    println!("Pramaan confidence explanation complete");
    println!("manifest: {}", manifest_path.display());
    println!("confidence_json: {}", json_path.display());
    println!("confidence_md: {}", markdown_path.display());
    println!("decision: {}", artifact.decision.as_str());
    println!("confidence_score: {}", artifact.confidence_score);
    println!("residual_risk_score: {}", artifact.residual_risk_score);
    Ok(())
}

fn read_bundle_receipts(bundle_root: &Path, manifest: &BundleManifest) -> Result<Vec<Receipt>> {
    let mut receipts = Vec::new();
    for receipt_ref in &manifest.receipts {
        let path = bundle_root.join(&receipt_ref.path);
        let receipt: Receipt = serde_json::from_slice(
            &fs::read(&path).with_context(|| format!("reading receipt {}", path.display()))?,
        )
        .with_context(|| format!("parsing receipt {}", path.display()))?;
        if receipt.schema_version != RECEIPT_SCHEMA_VERSION {
            anyhow::bail!(
                "{} has unsupported receipt schema_version {}",
                path.display(),
                receipt.schema_version
            );
        }
        if receipt.stage != "confidence_vote" {
            receipts.push(receipt);
        }
    }
    Ok(receipts)
}

fn policy_stage_evidence_from_manifest(manifest: &BundleManifest) -> Vec<PolicyStageEvidence> {
    manifest
        .stages
        .iter()
        .map(|stage| PolicyStageEvidence {
            id: stage.id.clone(),
            status: stage.status.clone(),
            residual_risks: stage.residual_risks.clone(),
            not_applicable_risks: stage.not_applicable_risks.clone(),
            stage_budget: stage.stage_budget.clone(),
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn confidence_receipt(
    manifest: &BundleManifest,
    artifact: &pramaan_core::ConfidenceArtifact,
    json_path: &Path,
    json_digest: &str,
    markdown_path: &Path,
    markdown_digest: &str,
    bundle_root: &Path,
) -> Receipt {
    let now = timestamp(Utc::now());
    let residual_risks = confidence_residual_risks(artifact);
    Receipt {
        schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
        stage: "confidence_vote".to_string(),
        status: match artifact.decision {
            ConfidenceDecision::Fail => StageStatus::Failed,
            ConfidenceDecision::Warn | ConfidenceDecision::Pass => StageStatus::Passed,
        },
        tool: ToolIdentity::new("pramaan-confidence", env!("CARGO_PKG_VERSION")),
        started_at: now.clone(),
        ended_at: now,
        exit_code: Some(if artifact.decision == ConfidenceDecision::Fail {
            1
        } else {
            0
        }),
        inputs: vec![pramaan_core::InputRef {
            name: "bundle_manifest".to_string(),
            value: MANIFEST_FILE_NAME.to_string(),
            digest: Some(manifest.integrity.manifest_digest.prefixed()),
        }],
        outputs: vec![
            OutputRef {
                name: "confidence_json".to_string(),
                path: bundle_path(bundle_root, json_path),
                digest: Some(json_digest.to_string()),
            },
            OutputRef {
                name: "confidence_markdown".to_string(),
                path: bundle_path(bundle_root, markdown_path),
                digest: Some(markdown_digest.to_string()),
            },
        ],
        artifacts: vec![
            ArtifactRef {
                name: "confidence_json".to_string(),
                path: bundle_path(bundle_root, json_path),
                media_type: Some("application/json".to_string()),
                digest: Some(json_digest.to_string()),
            },
            ArtifactRef {
                name: "confidence_markdown".to_string(),
                path: bundle_path(bundle_root, markdown_path),
                media_type: Some("text/markdown".to_string()),
                digest: Some(markdown_digest.to_string()),
            },
        ],
        summary: ReceiptSummary {
            title: format!("Confidence vote: {}", artifact.decision.as_str()),
            details: format!(
                "Confidence score {}, residual risk score {}, hard gates {}, votes {}, calibration {}.",
                artifact.confidence_score,
                artifact.residual_risk_score,
                artifact.hard_gates.len(),
                artifact.votes.len(),
                artifact.calibration.status
            ),
        },
        limitations: artifact.limitations.clone(),
        mitigated_risks: confidence_mitigated_risks(artifact),
        residual_risks,
        not_applicable_risks: Vec::new(),
        agent_author: None,
        reviewer_override: None,
        multi_agent_provenance: Vec::new(),
        plugin_identity: None,
        plugin_permissions: None,
        evidence_sensitivity: Some(EvidenceSensitivity::Internal),
        redaction_manifest: None,
        policy_decision: Some(PolicyDecision {
            decision: artifact.decision.as_str().to_string(),
            policy_id: "pramaan-confidence-v0.1".to_string(),
            hard_failures: artifact
                .hard_gates
                .iter()
                .map(|gate| gate.id.clone())
                .collect(),
            warnings: artifact.skipped_evidence.clone(),
            waived: Vec::new(),
        }),
        stage_budget: None,
        metadata: BTreeMap::from([
            (
                "algorithm_version".to_string(),
                artifact.algorithm_version.clone(),
            ),
            (
                "confidence_score".to_string(),
                artifact.confidence_score.to_string(),
            ),
            (
                "residual_risk_score".to_string(),
                artifact.residual_risk_score.to_string(),
            ),
            (
                "calibration_status".to_string(),
                artifact.calibration.status.clone(),
            ),
        ]),
    }
}

fn confidence_residual_risks(artifact: &pramaan_core::ConfidenceArtifact) -> Vec<String> {
    let mut risks = artifact
        .hard_gates
        .iter()
        .flat_map(|gate| gate.risk_ids.clone())
        .chain(
            artifact
                .top_risk_drivers
                .iter()
                .flat_map(|driver| driver.risk_ids.clone()),
        )
        .collect::<Vec<_>>();
    risks.sort();
    risks.dedup();
    risks
}

fn confidence_mitigated_risks(artifact: &pramaan_core::ConfidenceArtifact) -> Vec<String> {
    let mut risks = artifact
        .top_confidence_drivers
        .iter()
        .flat_map(|driver| driver.risk_ids.clone())
        .collect::<Vec<_>>();
    risks.sort();
    risks.dedup();
    risks
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
    let stages = policy_stage_evidence_from_manifest(&manifest);
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
