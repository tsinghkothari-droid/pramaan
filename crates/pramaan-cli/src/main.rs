use anyhow::{Context, Result};
use chrono::Utc;
use clap::{Parser, Subcommand};
use pramaan_bundle::{
    build_manifest, emit_offline_attestations, export_redacted_bundle, read_manifest, sha256_hex,
    verify_bundle, verify_offline_attestations, write_manifest, BundleBuildOptions, BundleManifest,
    MANIFEST_FILE_NAME,
};
use pramaan_core::risks::{
    CLAIM_SCOPE_API_NOT_MENTIONED, CLAIM_SCOPE_LOW_CONFIDENCE, CLAIM_SCOPE_NO_PR_METADATA,
    CLAIM_SCOPE_PUBLIC_API_DETECTION_FAILED,
};
use pramaan_core::{
    build_agent_decision, build_confidence_artifact, builtin_policy_profiles, canonical_json_bytes,
    compare_to_baseline, detect_agentic_workflow_injection, detect_verifier_abuse_paths,
    evaluate_calibration, evaluate_policy, policy_profile_by_id, render_confidence_markdown,
    risk_family, timestamp, AgentAttribution, ArtifactRef, AttributionConfidence,
    BundleFeedbackMetrics, CalibrationObservation, ClaimScope, ConfidenceDecision,
    EvidenceSensitivity, FeedbackReport, FuzzDivergence, FuzzRunEvidence, OutputRef,
    OverrideDecision, PluginIdentity, PluginPermissions, PolicyDecision, PolicyStageEvidence,
    ProbeCandidate, ProbeDecision, ProbeKind, ProbeLanguage, ProbePlanArtifact, ProbeProvider,
    ProbeSandboxStatus, Receipt, ReceiptSummary, RedactionManifest, RepoBaseline, ReviewerOverride,
    ReviewerOverrideEvidence, RiskRefs, StageBudget, StageStatus, ToolIdentity,
    FEEDBACK_SCHEMA_VERSION, PROBE_SCHEMA_VERSION, RECEIPT_SCHEMA_VERSION,
};
use pramaan_sandbox::{SandboxPlan, SandboxRunner};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
    Export(ExportArgs),
    Feedback(FeedbackArgs),
}

#[derive(Debug, Parser)]
struct VerifyArgs {
    #[arg(long)]
    base: String,
    #[arg(long)]
    head: String,
    #[arg(long, default_value = "target/pramaan")]
    out: PathBuf,
    /// Skip a stage by id. May be passed multiple times. Known ids:
    /// static_checks, oracle, fuzz, mutation.
    #[arg(long = "skip-stage")]
    skip_stages: Vec<String>,
    /// Run diff-scoped mutation testing (mutmut / StrykerJS / cargo-mutants).
    /// Off by default because mutation runs can take minutes per language.
    #[arg(long = "with-mutation", default_value_t = false)]
    with_mutation: bool,
    /// Deterministic seed for the differential-fuzz stage.
    #[arg(long = "fuzz-seed", default_value_t = 1337)]
    fuzz_seed: u64,
}

#[derive(Debug, Parser)]
struct BundleArgs {
    #[command(subcommand)]
    command: BundleCommands,
}

#[derive(Debug, Subcommand)]
enum BundleCommands {
    Verify(BundleVerifyArgs),
    Attest(BundleAttestArgs),
    VerifyOffline(BundleVerifyOfflineArgs),
    ExportRedacted(BundleExportRedactedArgs),
}

#[derive(Debug, Parser)]
struct BundleVerifyArgs {
    path: PathBuf,
}

#[derive(Debug, Parser)]
struct BundleAttestArgs {
    path: PathBuf,
}

#[derive(Debug, Parser)]
struct BundleVerifyOfflineArgs {
    path: PathBuf,
}

#[derive(Debug, Parser)]
struct BundleExportRedactedArgs {
    path: PathBuf,
    #[arg(long)]
    profile: String,
    #[arg(long)]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct ExportArgs {
    #[command(subcommand)]
    command: ExportCommands,
}

#[derive(Debug, Subcommand)]
enum ExportCommands {
    Sarif(ExportSarifArgs),
    Rego(ExportRegoArgs),
}

#[derive(Debug, Parser)]
struct ExportSarifArgs {
    bundle: PathBuf,
    #[arg(long)]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct ExportRegoArgs {
    #[arg(long)]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct FeedbackArgs {
    #[command(subcommand)]
    command: FeedbackCommands,
}

#[derive(Debug, Subcommand)]
enum FeedbackCommands {
    Override(FeedbackOverrideArgs),
    Analyze(FeedbackAnalyzeArgs),
}

#[derive(Debug, Parser)]
struct FeedbackOverrideArgs {
    #[arg(long)]
    bundle: PathBuf,
    #[arg(long)]
    stage: String,
    #[arg(long = "risk")]
    risks: Vec<String>,
    #[arg(long)]
    reason: String,
    #[arg(long)]
    reviewer: String,
    #[arg(long, default_value = "approved_despite_risk")]
    decision: String,
    #[arg(long)]
    linked_outcome: Option<String>,
    #[arg(long, default_value_t = true)]
    update_calibration: bool,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct FeedbackAnalyzeArgs {
    #[arg(long = "bundle", required = true)]
    bundles: Vec<PathBuf>,
    #[arg(long)]
    baseline: Option<PathBuf>,
    #[arg(long)]
    observations: Option<PathBuf>,
    #[arg(long, default_value = "target/pramaan-feedback")]
    out: PathBuf,
}

#[derive(Debug, Parser)]
struct PolicyArgs {
    #[command(subcommand)]
    command: PolicyCommands,
}

#[derive(Debug, Subcommand)]
enum PolicyCommands {
    Explain(PolicyExplainArgs),
    List,
}

#[derive(Debug, Parser)]
struct PolicyExplainArgs {
    bundle: PathBuf,
    #[arg(long, default_value = "private-preview")]
    profile: String,
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
    Execute(ProbeExecuteArgs),
}

#[derive(Debug, Parser)]
struct ProbePlanArgs {
    #[arg(long)]
    bundle: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProbeExecuteArgs {
    #[arg(long)]
    plan: PathBuf,
    #[arg(long)]
    bundle: PathBuf,
    #[arg(long)]
    out: Option<PathBuf>,
    #[arg(long, default_value_t = 5000)]
    timeout_ms: u64,
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
        Commands::Export(args) => run_export(args),
        Commands::Feedback(args) => run_feedback(args),
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
        ProbeCommands::Execute(args) => run_probe_execute(args),
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
    let receipt = probe_generation_receipt(
        &manifest,
        &artifact,
        &plan_path,
        &plan_digest,
        None,
        &bundle_root,
    );
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

fn run_probe_execute(args: ProbeExecuteArgs) -> Result<()> {
    let manifest_path = resolve_manifest_path(&args.bundle);
    let bundle_root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let mut artifact: ProbePlanArtifact = serde_json::from_slice(
        &fs::read(&args.plan)
            .with_context(|| format!("reading probe plan {}", args.plan.display()))?,
    )
    .with_context(|| format!("parsing probe plan {}", args.plan.display()))?;
    if artifact.schema_version != PROBE_SCHEMA_VERSION {
        anyhow::bail!(
            "{} has unsupported probe schema_version {}",
            args.plan.display(),
            artifact.schema_version
        );
    }

    let out_dir = args
        .out
        .unwrap_or_else(|| bundle_root.join("probes").join("executed"));
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("creating probe execution directory {}", out_dir.display()))?;
    let sandbox_root = out_dir.join("sandbox");
    if sandbox_root.exists() {
        fs::remove_dir_all(&sandbox_root)
            .with_context(|| format!("cleaning probe sandbox {}", sandbox_root.display()))?;
    }
    fs::create_dir_all(&sandbox_root)
        .with_context(|| format!("creating probe sandbox {}", sandbox_root.display()))?;

    let mut executions = Vec::new();
    for probe in &mut artifact.probes {
        let result = execute_probe_candidate(probe, &sandbox_root, args.timeout_ms)?;
        probe.sandbox_status = result.sandbox_status;
        probe.kept_or_rejected = result.decision;
        probe.execution_result = result.execution_result.clone();
        probe.rejection_reason = result.rejection_reason.clone();
        executions.push(result);
    }
    recount_probe_artifact(&mut artifact);
    artifact.generated_at = timestamp(Utc::now());
    if !artifact
        .limitations
        .iter()
        .any(|item| item.contains("safe-marker"))
    {
        artifact.limitations.push(
            "Phase 28.26 executes only safe-marker bounded probes; unmarked or dangerous candidates are preserved as rejected evidence.".to_string(),
        );
    }

    let executed_plan_path = out_dir.join("ai-probe-plan.executed.json");
    write_json(&executed_plan_path, &artifact)?;
    let executed_plan_digest = digest_file(&executed_plan_path)?;
    let report = ProbeExecutionReport {
        schema_version: PROBE_SCHEMA_VERSION.to_string(),
        generated_at: timestamp(Utc::now()),
        timeout_ms: args.timeout_ms,
        sandbox_root: portable_path(&sandbox_root),
        accepted_count: artifact.accepted_count,
        rejected_count: artifact.rejected_count,
        pending_count: artifact.pending_count,
        executions,
    };
    let report_path = out_dir.join("ai-probe-execution.json");
    write_json(&report_path, &report)?;
    let report_digest = digest_file(&report_path)?;

    let receipt_path = if out_dir.starts_with(&bundle_root) {
        bundle_root
            .join("receipts")
            .join("ai-probe-generation.receipt.json")
    } else {
        out_dir
            .join("receipts")
            .join("ai-probe-generation.receipt.json")
    };
    let receipt = probe_generation_receipt(
        &manifest,
        &artifact,
        &executed_plan_path,
        &executed_plan_digest,
        Some((&report_path, &report_digest)),
        &bundle_root,
    );
    write_json(&receipt_path, &receipt)?;

    if out_dir.starts_with(&bundle_root) {
        let updated_manifest = build_manifest(
            &bundle_root,
            BundleBuildOptions {
                bundle_id: manifest.bundle_id,
                run_id: manifest.run_id,
                repository: manifest.repository,
            },
        )
        .context("rebuilding bundle manifest with executed probe artifacts")?;
        write_manifest(&bundle_root, &updated_manifest)
            .context("writing updated bundle manifest")?;
    }

    println!("Pramaan AI probe execution complete");
    println!("manifest: {}", manifest_path.display());
    println!("probe_plan: {}", executed_plan_path.display());
    println!("execution_report: {}", report_path.display());
    println!("accepted: {}", artifact.accepted_count);
    println!("rejected: {}", artifact.rejected_count);
    println!("pending_execution: {}", artifact.pending_count);
    println!("provider_trusted_for_decision: false");
    Ok(())
}

#[derive(Debug, Serialize)]
struct ProbeExecutionReport {
    schema_version: String,
    generated_at: String,
    timeout_ms: u64,
    sandbox_root: String,
    accepted_count: usize,
    rejected_count: usize,
    pending_count: usize,
    executions: Vec<ProbeExecutionRecord>,
}

#[derive(Debug, Serialize)]
struct ProbeExecutionRecord {
    probe_id: String,
    language: String,
    command: Vec<String>,
    sandbox_status: ProbeSandboxStatus,
    decision: ProbeDecision,
    execution_result: String,
    rejection_reason: Option<String>,
    stdout_digest: Option<String>,
    stderr_digest: Option<String>,
    materialized_path: Option<String>,
    duration_ms: u128,
}

fn execute_probe_candidate(
    probe: &ProbeCandidate,
    sandbox_root: &Path,
    timeout_ms: u64,
) -> Result<ProbeExecutionRecord> {
    let started = Instant::now();
    let probe_dir = sandbox_root.join(sanitize_probe_id(&probe.probe_id));
    fs::create_dir_all(&probe_dir)
        .with_context(|| format!("creating probe sandbox {}", probe_dir.display()))?;

    let mut record = ProbeExecutionRecord {
        probe_id: probe.probe_id.clone(),
        language: probe.language.as_str().to_string(),
        command: Vec::new(),
        sandbox_status: ProbeSandboxStatus::RejectedStatic,
        decision: ProbeDecision::Rejected,
        execution_result: String::new(),
        rejection_reason: None,
        stdout_digest: None,
        stderr_digest: None,
        materialized_path: None,
        duration_ms: 0,
    };

    if probe.candidate_code.contains("Generate an isolated")
        || !probe.candidate_code.contains("pramaan-accepted-probe")
    {
        record.execution_result =
            "rejected_static: candidate is a skeleton or lacks the pramaan-accepted-probe marker"
                .to_string();
        record.rejection_reason = Some("candidate_skeleton_or_missing_safe_marker".to_string());
        record.duration_ms = started.elapsed().as_millis();
        return Ok(record);
    }
    if let Some(reason) = dangerous_probe_token(&probe.candidate_code) {
        record.execution_result = format!("rejected_static: dangerous token {reason}");
        record.rejection_reason = Some(format!("dangerous_token:{reason}"));
        record.duration_ms = started.elapsed().as_millis();
        return Ok(record);
    }
    if !probe_binds_to_changed_behavior(probe) {
        record.execution_result =
            "rejected_static: probe did not mention a risk id, target basename, or pramaan-bind"
                .to_string();
        record.rejection_reason = Some("no_changed_behavior_binding".to_string());
        record.duration_ms = started.elapsed().as_millis();
        return Ok(record);
    }

    let (file_name, command, args): (&str, &str, Vec<String>) = match probe.language {
        ProbeLanguage::Python => ("test_probe.py", "python", vec!["test_probe.py".to_string()]),
        ProbeLanguage::TypeScript => ("probe.test.js", "node", vec!["probe.test.js".to_string()]),
        ProbeLanguage::Rust => (
            "probe_test.rs",
            "rustc",
            vec![
                "--crate-type".to_string(),
                "lib".to_string(),
                "--emit".to_string(),
                "metadata".to_string(),
                "probe_test.rs".to_string(),
            ],
        ),
        ProbeLanguage::Unknown => {
            record.execution_result = "rejected_static: unknown probe language".to_string();
            record.rejection_reason = Some("unknown_language".to_string());
            record.duration_ms = started.elapsed().as_millis();
            return Ok(record);
        }
    };

    let probe_path = probe_dir.join(file_name);
    fs::write(&probe_path, &probe.candidate_code)
        .with_context(|| format!("materializing probe {}", probe_path.display()))?;
    let mut full_command = vec![command.to_string()];
    full_command.extend(args.iter().cloned());
    record.command = full_command;
    record.materialized_path = Some(portable_path(&probe_path));

    match run_probe_command(command, &args, &probe_dir, timeout_ms) {
        Ok(output) => {
            record.stdout_digest = Some(sha256_hex(output.stdout.as_bytes()));
            record.stderr_digest = Some(sha256_hex(output.stderr.as_bytes()));
            record.duration_ms = output.duration_ms;
            if output.timed_out {
                record.sandbox_status = ProbeSandboxStatus::ExecutedFailed;
                record.execution_result = format!(
                    "executed_failed: timed out after {}ms; stdout_digest={}; stderr_digest={}",
                    timeout_ms,
                    record.stdout_digest.clone().unwrap_or_default(),
                    record.stderr_digest.clone().unwrap_or_default()
                );
                record.rejection_reason = Some("execution_timeout".to_string());
            } else if output.exit_code == Some(0) {
                record.sandbox_status = ProbeSandboxStatus::ExecutedPassed;
                record.decision = ProbeDecision::Kept;
                record.execution_result = format!(
                    "executed_passed: command exited 0; stdout_digest={}; stderr_digest={}",
                    record.stdout_digest.clone().unwrap_or_default(),
                    record.stderr_digest.clone().unwrap_or_default()
                );
            } else {
                record.sandbox_status = ProbeSandboxStatus::ExecutedFailed;
                record.execution_result = format!(
                    "executed_failed: exit_code={:?}; stdout_digest={}; stderr_digest={}",
                    output.exit_code,
                    record.stdout_digest.clone().unwrap_or_default(),
                    record.stderr_digest.clone().unwrap_or_default()
                );
                record.rejection_reason = Some("execution_failed".to_string());
            }
        }
        Err(error) => {
            record.sandbox_status = ProbeSandboxStatus::ExecutedFailed;
            record.execution_result = format!("executed_failed: {error:#}");
            record.rejection_reason = Some("execution_error".to_string());
            record.duration_ms = started.elapsed().as_millis();
        }
    }

    Ok(record)
}

struct ProbeCommandOutput {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    duration_ms: u128,
    timed_out: bool,
}

fn run_probe_command(
    command: &str,
    args: &[String],
    cwd: &Path,
    timeout_ms: u64,
) -> Result<ProbeCommandOutput> {
    let started = Instant::now();
    let mut child = Command::new(command)
        .args(args)
        .current_dir(cwd)
        .env("PRAMAAN_PROBE_NETWORK", "disabled")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning probe command {command}"))?;

    let timeout = Duration::from_millis(timeout_ms.max(1));
    loop {
        if child.try_wait()?.is_some() {
            let output = child
                .wait_with_output()
                .context("collecting probe command output")?;
            return Ok(ProbeCommandOutput {
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                duration_ms: started.elapsed().as_millis(),
                timed_out: false,
            });
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let output = child
                .wait_with_output()
                .context("collecting timed-out probe command output")?;
            return Ok(ProbeCommandOutput {
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                duration_ms: started.elapsed().as_millis(),
                timed_out: true,
            });
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn sanitize_probe_id(probe_id: &str) -> String {
    probe_id
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn dangerous_probe_token(code: &str) -> Option<&'static str> {
    let lowered = code.to_ascii_lowercase();
    let blocked = [
        "socket",
        "subprocess",
        "os.system",
        "child_process",
        "std::process",
        "std::net",
        "requests.",
        "fetch(",
        "http://",
        "https://",
        "remove_file",
        "remove_dir",
        "unlink(",
        "rmdir(",
        "shutil.rmtree",
    ];
    blocked
        .iter()
        .find(|token| lowered.contains(**token))
        .copied()
}

fn probe_binds_to_changed_behavior(probe: &ProbeCandidate) -> bool {
    if probe.candidate_code.contains("pramaan-bind") {
        return true;
    }
    probe
        .risk_ids
        .iter()
        .any(|risk_id| probe.candidate_code.contains(risk_id))
        || probe.target_files.iter().any(|target| {
            Path::new(target)
                .file_name()
                .and_then(|name| name.to_str())
                .map(|basename| probe.candidate_code.contains(basename))
                .unwrap_or(false)
        })
}

fn recount_probe_artifact(artifact: &mut ProbePlanArtifact) {
    artifact.accepted_count = artifact
        .probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::Kept)
        .count();
    artifact.rejected_count = artifact
        .probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::Rejected)
        .count();
    artifact.pending_count = artifact
        .probes
        .iter()
        .filter(|probe| probe.kept_or_rejected == ProbeDecision::PendingExecution)
        .count();
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
    execution_report: Option<(&Path, &str)>,
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

    let mut outputs = vec![OutputRef {
        name: "ai_probe_plan".to_string(),
        path: bundle_path(bundle_root, plan_path),
        digest: Some(plan_digest.to_string()),
    }];
    let mut artifacts = vec![ArtifactRef {
        name: "ai_probe_plan_json".to_string(),
        path: bundle_path(bundle_root, plan_path),
        media_type: Some("application/json".to_string()),
        digest: Some(plan_digest.to_string()),
    }];
    if let Some((report_path, report_digest)) = execution_report {
        outputs.push(OutputRef {
            name: "ai_probe_execution_report".to_string(),
            path: bundle_path(bundle_root, report_path),
            digest: Some(report_digest.to_string()),
        });
        artifacts.push(ArtifactRef {
            name: "ai_probe_execution_json".to_string(),
            path: bundle_path(bundle_root, report_path),
            media_type: Some("application/json".to_string()),
            digest: Some(report_digest.to_string()),
        });
    }

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
        outputs,
        artifacts,
        summary: ReceiptSummary {
            title: if execution_report.is_some() {
                "AI evidence-seeking probes sandbox executed".to_string()
            } else {
                "AI evidence-seeking probe plan emitted".to_string()
            },
            details: format!(
                "accepted={}, rejected={}, pending={}; provider output remains untrusted for final decisions.",
                artifact.accepted_count, artifact.rejected_count, artifact.pending_count
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
        skip_stages: Vec::new(),
        with_mutation: false,
        fuzz_seed: 1337,
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
        PolicyCommands::Explain(args) => run_policy_explain(args.bundle, args.profile),
        PolicyCommands::List => {
            for profile in builtin_policy_profiles() {
                println!("{}", profile.policy_id);
            }
            Ok(())
        }
    }
}

fn run_policy_explain(bundle: PathBuf, profile_id: String) -> Result<()> {
    let manifest_path = if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle
    };
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let stages = policy_stage_evidence_from_manifest(&manifest);
    let profile = policy_profile_by_id(&profile_id)
        .with_context(|| format!("unknown policy profile {profile_id}"))?;
    let evaluation = evaluate_policy(&profile, &stages);

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
    println!(
        "hard_gate_risk_ids: {}",
        profile.hard_gate_risk_ids.join(", ")
    );
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

fn run_feedback(args: FeedbackArgs) -> Result<()> {
    match args.command {
        FeedbackCommands::Override(args) => run_feedback_override(args),
        FeedbackCommands::Analyze(args) => run_feedback_analyze(args),
    }
}

fn run_feedback_override(args: FeedbackOverrideArgs) -> Result<()> {
    let manifest_path = resolve_manifest_path(&args.bundle);
    let bundle_root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
    let decision = parse_override_decision(&args.decision)?;
    let evidence = ReviewerOverrideEvidence {
        schema_version: FEEDBACK_SCHEMA_VERSION.to_string(),
        bundle_id: manifest.bundle_id,
        manifest_digest: manifest.integrity.manifest_digest.prefixed(),
        stage: args.stage,
        override_record: ReviewerOverride {
            decision,
            accepted_risk_ids: args.risks,
            reviewer_identity_source: args.reviewer,
            timestamp: timestamp(Utc::now()),
            reason: args.reason,
            linked_outcome: args.linked_outcome,
            update_calibration: args.update_calibration,
        },
    };
    let out_path = args
        .out
        .unwrap_or_else(|| bundle_root.join("feedback").join("reviewer-override.json"));
    write_json(&out_path, &evidence)?;

    println!("Pramaan reviewer override recorded");
    println!("override: {}", out_path.display());
    println!("bundle_id: {}", evidence.bundle_id);
    println!("stage: {}", evidence.stage);
    println!(
        "accepted_risks: {}",
        evidence.override_record.accepted_risk_ids.join(", ")
    );
    println!(
        "update_calibration: {}",
        evidence.override_record.update_calibration
    );
    Ok(())
}

fn parse_override_decision(value: &str) -> Result<OverrideDecision> {
    match value {
        "approved_despite_risk" | "approved" | "accept" => {
            Ok(OverrideDecision::ApprovedDespiteRisk)
        }
        "rejected" | "reject" => Ok(OverrideDecision::Rejected),
        "needs_follow_up" | "follow_up" => Ok(OverrideDecision::NeedsFollowUp),
        _ => anyhow::bail!(
            "unknown override decision {value}; use approved_despite_risk, rejected, or needs_follow_up"
        ),
    }
}

#[derive(Debug, Deserialize)]
struct CalibrationObservationFile {
    observations: Vec<CalibrationObservation>,
}

fn run_feedback_analyze(args: FeedbackAnalyzeArgs) -> Result<()> {
    fs::create_dir_all(&args.out)
        .with_context(|| format!("creating feedback output directory {}", args.out.display()))?;
    let baseline = match args.baseline {
        Some(path) => Some(read_json_file::<RepoBaseline>(&path)?),
        None => None,
    };
    let observations = match args.observations {
        Some(path) => read_json_file::<CalibrationObservationFile>(&path)?.observations,
        None => Vec::new(),
    };

    let mut metrics = Vec::new();
    let mut drift_warnings = Vec::new();
    for bundle in args.bundles {
        let manifest_path = resolve_manifest_path(&bundle);
        let bundle_root = manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let manifest = read_manifest(&manifest_path).context("reading bundle manifest")?;
        let receipts = read_bundle_receipts(&bundle_root, &manifest)?;
        let metric = feedback_metrics_for_bundle(&bundle_root, &manifest, &receipts)?;
        if let Some(baseline) = &baseline {
            drift_warnings.extend(compare_to_baseline(&metric, baseline));
        }
        metrics.push(metric);
    }

    let report = FeedbackReport {
        schema_version: FEEDBACK_SCHEMA_VERSION.to_string(),
        bundle_count: metrics.len(),
        metrics,
        drift_warnings,
        calibration: evaluate_calibration(&observations),
        reviewer_overrides: Vec::new(),
        limitations: vec![
            "Phase 34 feedback analysis is local JSON/CSV evidence, not a hosted dashboard.".to_string(),
            "Calibration uses only supplied labeled outcomes; absent labels keep calibration in no_labeled_outcomes mode.".to_string(),
        ],
    };

    let report_path = args.out.join("feedback-report.json");
    write_json(&report_path, &report)?;
    let csv_path = args.out.join("feedback-metrics.csv");
    fs::write(&csv_path, feedback_metrics_csv(&report.metrics))
        .with_context(|| format!("writing {}", csv_path.display()))?;

    println!("Pramaan feedback analysis complete");
    println!("report: {}", report_path.display());
    println!("csv: {}", csv_path.display());
    println!("bundles: {}", report.bundle_count);
    println!("drift_warnings: {}", report.drift_warnings.len());
    println!("calibration_status: {}", report.calibration.status);
    Ok(())
}

fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path).with_context(|| format!("reading {}", path.display()))?)
        .with_context(|| format!("parsing {}", path.display()))
}

fn feedback_metrics_for_bundle(
    bundle_root: &Path,
    manifest: &BundleManifest,
    receipts: &[Receipt],
) -> Result<BundleFeedbackMetrics> {
    let mut residual_by_family = BTreeMap::new();
    let mut mutation_survivors = 0;
    let mut oracle_residual_risks = 0;
    let mut static_residual_risks = 0;
    let mut skipped_stages = 0;
    for stage in &manifest.stages {
        if stage.status == "skipped" {
            skipped_stages += 1;
        }
        if stage.id.contains("mutation") {
            mutation_survivors += stage
                .residual_risks
                .iter()
                .filter(|risk| risk.as_str() == "R-068")
                .count() as u64;
        }
        if stage.id.contains("oracle") {
            oracle_residual_risks += stage.residual_risks.len() as u64;
        }
        if stage.id.contains("static") {
            static_residual_risks += stage.residual_risks.len() as u64;
        }
        for risk in &stage.residual_risks {
            *residual_by_family
                .entry(risk_family(risk).to_string())
                .or_default() += 1;
        }
    }

    let runtime_ms = manifest
        .stage_budgets
        .iter()
        .map(|budget| budget.consumed_ms)
        .sum();
    let confidence_score = read_confidence_score(bundle_root)?;
    let agent_author = receipts
        .iter()
        .find_map(|receipt| receipt.agent_author.as_ref())
        .map(|agent| {
            let model = agent
                .model_version
                .as_deref()
                .or(agent.model_family.as_deref())
                .unwrap_or("unknown_model");
            format!("{}:{model}", agent.product)
        });

    Ok(BundleFeedbackMetrics {
        bundle_id: manifest.bundle_id.clone(),
        repository: manifest.repository.path.clone(),
        final_status: manifest.final_status.clone(),
        runtime_ms,
        confidence_score,
        mutation_survivors,
        oracle_residual_risks,
        skipped_stages,
        static_residual_risks,
        residual_by_family,
        agent_author,
    })
}

fn read_confidence_score(bundle_root: &Path) -> Result<Option<u8>> {
    let path = bundle_root.join("confidence.json");
    if !path.exists() {
        return Ok(None);
    }
    let value: serde_json::Value = read_json_file(&path)?;
    Ok(value
        .get("confidence_score")
        .and_then(serde_json::Value::as_u64)
        .and_then(|score| u8::try_from(score).ok()))
}

fn feedback_metrics_csv(metrics: &[BundleFeedbackMetrics]) -> String {
    let mut csv = String::from(
        "bundle_id,repository,final_status,runtime_ms,confidence_score,mutation_survivors,oracle_residual_risks,skipped_stages,static_residual_risks,agent_author\n",
    );
    for metric in metrics {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            csv_cell(&metric.bundle_id),
            csv_cell(&metric.repository),
            csv_cell(&metric.final_status),
            metric.runtime_ms,
            metric
                .confidence_score
                .map(|score| score.to_string())
                .unwrap_or_default(),
            metric.mutation_survivors,
            metric.oracle_residual_risks,
            metric.skipped_stages,
            metric.static_residual_risks,
            csv_cell(metric.agent_author.as_deref().unwrap_or(""))
        ));
    }
    csv
}

fn csv_cell(value: &str) -> String {
    if value.chars().any(|ch| matches!(ch, ',' | '"' | '\n')) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

fn run_export(args: ExportArgs) -> Result<()> {
    match args.command {
        ExportCommands::Sarif(args) => run_export_sarif(args.bundle, args.out),
        ExportCommands::Rego(args) => run_export_rego(args.out),
    }
}

fn run_export_sarif(bundle: PathBuf, out: PathBuf) -> Result<()> {
    let manifest_path = if bundle.is_dir() {
        bundle.join(MANIFEST_FILE_NAME)
    } else {
        bundle
    };
    let manifest = read_manifest(&manifest_path).context("reading bundle manifest for SARIF")?;
    let sarif = sarif_for_manifest(&manifest);
    write_json(&out, &sarif)?;
    println!("Pramaan SARIF export complete");
    println!("sarif: {}", out.display());
    Ok(())
}

fn run_export_rego(out: PathBuf) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating parent directory {}", parent.display()))?;
    }
    fs::write(&out, DEFAULT_POLICY_REGO)
        .with_context(|| format!("writing Rego policy {}", out.display()))?;
    println!("Pramaan Rego policy export complete");
    println!("rego: {}", out.display());
    Ok(())
}

fn sarif_for_manifest(manifest: &BundleManifest) -> serde_json::Value {
    let mut rules = BTreeMap::<String, serde_json::Value>::new();
    let mut results = Vec::new();
    for stage in &manifest.stages {
        let risk_ids = stage
            .residual_risks
            .iter()
            .chain(stage.not_applicable_risks.iter())
            .cloned()
            .collect::<BTreeSet<_>>();
        for risk_id in risk_ids {
            rules.entry(risk_id.clone()).or_insert_with(|| {
                serde_json::json!({
                    "id": risk_id,
                    "name": format!("Pramaan risk {}", risk_id),
                    "shortDescription": {
                        "text": format!("{} risk family", risk_family(&risk_id))
                    },
                    "help": {
                        "text": "Pramaan emits evidence and residual risk; it does not prove code correctness."
                    }
                })
            });
            results.push(serde_json::json!({
                "ruleId": risk_id,
                "level": sarif_level_for_status(&stage.status),
                "message": {
                    "text": format!("Stage `{}` reported {} risk `{}`.", stage.id, stage.status, risk_id)
                },
                "locations": [
                    {
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": &stage.receipt_path
                            }
                        }
                    }
                ],
                "properties": {
                    "stage": &stage.id,
                    "status": &stage.status,
                    "tool": &stage.tool.name
                }
            }));
        }
    }
    serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [
            {
                "tool": {
                    "driver": {
                        "name": "Pramaan",
                        "informationUri": "https://github.com/tsinghkothari-droid/pramaan",
                        "rules": rules.into_values().collect::<Vec<_>>()
                    }
                },
                "results": results
            }
        ]
    })
}

fn sarif_level_for_status(status: &str) -> &'static str {
    match status {
        "failed" | "error" | "timed_out" => "error",
        "skipped" | "not_applicable" => "warning",
        _ => "note",
    }
}

const DEFAULT_POLICY_REGO: &str = r#"package pramaan.default

default decision := "pass"

hard_statuses := {"failed", "error", "timed_out"}

decision := "fail" if {
  some stage in input.stages
  hard_statuses[stage.status]
}

decision := "fail" if {
  some required in {"claim_scope", "sandbox_setup"}
  not stage_ids[required]
}

decision := "warning" if {
  decision != "fail"
  count(input.risk_summary.residual) > 0
}

stage_ids[id] if {
  some stage in input.stages
  id := stage.id
}
"#;

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
        BundleCommands::Attest(args) => {
            let report = emit_offline_attestations(&args.path)
                .context("emitting offline attestation material")?;
            println!("Pramaan offline attestation complete");
            println!("vsa: {}", report.vsa_path.display());
            println!("in_toto: {}", report.statement_path.display());
            println!("manifest_digest: {}", report.manifest_digest.prefixed());
            println!("verification_result: {}", report.verification_result);
            println!("note: local/offline attestation is evidence, not a correctness proof");
            Ok(())
        }
        BundleCommands::VerifyOffline(args) => {
            let report = verify_offline_attestations(&args.path)
                .context("verifying offline attestation material")?;
            println!("Pramaan offline attestation verification complete");
            println!("vsa: {}", report.vsa_path.display());
            println!("in_toto: {}", report.statement_path.display());
            println!("manifest_digest: {}", report.manifest_digest.prefixed());
            println!("verification_result: {}", report.verification_result);
            Ok(())
        }
        BundleCommands::ExportRedacted(args) => {
            let report = export_redacted_bundle(&args.path, &args.out, &args.profile)
                .context("exporting redacted bundle")?;
            println!("Pramaan redacted bundle export complete");
            println!("bundle: {}", report.bundle_root.display());
            println!("manifest: {}", report.manifest_path.display());
            println!("profile: {}", report.profile);
            println!("redacted_files: {}", report.redacted_files.len());
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

    let (base_worktree, head_worktree) = sandbox_run.worktree_paths();
    let base_worktree = base_worktree.to_path_buf();
    let head_worktree = head_worktree.to_path_buf();

    let skip: BTreeSet<String> = args.skip_stages.iter().map(|s| s.to_lowercase()).collect();
    let mut stages_run: Vec<&'static str> = Vec::new();

    if !skip.contains("static_checks") {
        static_checks::run_static_checks(head_worktree.clone(), args.out.clone())
            .context("running static_checks stage")?;
        stages_run.push("static_checks");
    }

    if !skip.contains("oracle") {
        oracle::run_oracle(
            base_worktree.clone(),
            head_worktree.clone(),
            args.out.clone(),
        )
        .context("running oracle stage")?;
        stages_run.push("oracle");
    }

    if !skip.contains("fuzz") {
        fuzz::run_fuzz(
            base_worktree.clone(),
            head_worktree.clone(),
            Some(claim_scope_path.clone()),
            args.out.clone(),
            args.fuzz_seed,
        )
        .context("running fuzz stage")?;
        stages_run.push("fuzz");
    }

    if args.with_mutation && !skip.contains("mutation") {
        mutation::run_mutation(
            head_worktree.clone(),
            args.out.clone(),
            Vec::new(),
            120_000,
            70,
        )
        .context("running mutation stage")?;
        stages_run.push("mutation");
    }

    if stages_run.is_empty() {
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
                title: "Synthetic verifier fallback".to_string(),
                details: format!(
                    "All real stages were skipped (--skip-stage: {}). Bundle reflects partial evidence only.",
                    args.skip_stages.join(",")
                ),
            },
            RiskRefs::sample(),
        );
        write_json(&synthetic_receipt_path, &synthetic_receipt)?;
    }

    let manifest = build_manifest(
        &args.out,
        BundleBuildOptions::synthetic(args.base.clone(), args.head.clone()),
    )
    .context("building bundle manifest")?;
    let manifest_path = write_manifest(&args.out, &manifest).context("writing bundle manifest")?;

    render_summary(&args, &manifest, &manifest_path, &stages_run);

    Ok(())
}

fn add_synthetic_trust_hooks(receipt: &mut Receipt) {
    receipt.agent_author = agent_attribution_from_env();
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

fn agent_attribution_from_env() -> Option<AgentAttribution> {
    let product = read_non_empty_env("PRAMAAN_AGENT_PRODUCT")?;
    let model_family = read_non_empty_env("PRAMAAN_AGENT_MODEL_FAMILY");
    let model_version = read_non_empty_env("PRAMAAN_AGENT_MODEL_VERSION");
    let execution_mode =
        read_non_empty_env("PRAMAAN_AGENT_EXECUTION_MODE").unwrap_or_else(|| "unspecified".into());
    let prompt_context_hash = read_non_empty_env("PRAMAAN_AGENT_PROMPT_CONTEXT_HASH");
    let commit_provenance = read_non_empty_env("PRAMAAN_AGENT_COMMIT_PROVENANCE");
    let source = read_non_empty_env("PRAMAAN_AGENT_SOURCE").unwrap_or_else(|| "environment".into());

    Some(AgentAttribution {
        product,
        model_family,
        model_version,
        execution_mode,
        prompt_context_hash,
        commit_provenance,
        source,
        confidence: AttributionConfidence::Unknown,
    })
}

fn read_non_empty_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn claim_scope_from_context(base_ref: &str, head_ref: &str) -> Result<ClaimScope> {
    let mut scope = ClaimScope::synthetic(base_ref, head_ref);
    let mut source_refs = scope.source_refs.clone();
    let mut expected_behavior = Vec::new();
    let mut limitations = Vec::new();
    let mut risk_refs = Vec::new();
    let mut untrusted_agent_text = Vec::<(String, String)>::new();

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
                    untrusted_agent_text
                        .push(("github.pull_request.title".to_string(), title.to_string()));
                }
                if let Some(body) = event
                    .pointer("/pull_request/body")
                    .and_then(serde_json::Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                {
                    expected_behavior.push(format!("PR body: {}", body.trim()));
                    untrusted_agent_text
                        .push(("github.pull_request.body".to_string(), body.to_string()));
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
            untrusted_agent_text.push(("PRAMAAN_PR_TITLE".to_string(), title));
        }
    }
    if let Ok(body) = std::env::var("PRAMAAN_PR_BODY") {
        if !body.trim().is_empty() {
            expected_behavior.push(format!("PR body: {}", body.trim()));
            untrusted_agent_text.push(("PRAMAAN_PR_BODY".to_string(), body.clone()));
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
        untrusted_agent_text.push(("PRAMAAN_ISSUE_TEXT_OR_PATH".to_string(), issue_text));
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
        risk_refs.push(CLAIM_SCOPE_PUBLIC_API_DETECTION_FAILED.to_string());
        Vec::new()
    });

    for (source, text) in &untrusted_agent_text {
        for finding in detect_agentic_workflow_injection(source.clone(), text) {
            limitations.push(format!(
                "Agentic workflow injection signal {} from {}: {}",
                finding.id, finding.source, finding.message
            ));
            risk_refs.push(finding.risk_id);
        }
    }
    match changed_files_between_refs(base_ref, head_ref) {
        Ok(paths) => {
            for finding in detect_verifier_abuse_paths(paths) {
                limitations.push(format!(
                    "Verifier-abuse surface signal {} at {}: {}",
                    finding.id, finding.path, finding.message
                ));
                risk_refs.push(finding.risk_id);
            }
        }
        Err(error) => {
            limitations.push(format!("Verifier-surface change detection failed: {error}"));
        }
    }

    if !expected_behavior.is_empty() {
        scope.expected_behavior = expected_behavior;
        scope.extraction_method = "github_event_or_environment".to_string();
        scope.confidence = pramaan_core::ClaimConfidence::High;
    } else {
        limitations.push(
            "No PR title, PR body, issue text, or maintainer scope note was available; claim scope is low confidence."
                .to_string(),
        );
        risk_refs.extend([
            CLAIM_SCOPE_NO_PR_METADATA.to_string(),
            CLAIM_SCOPE_LOW_CONFIDENCE.to_string(),
        ]);
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
            risk_refs.push(CLAIM_SCOPE_API_NOT_MENTIONED.to_string());
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

fn changed_files_between_refs(base_ref: &str, head_ref: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", base_ref, head_ref])
        .output()
        .with_context(|| format!("running git diff --name-only {base_ref} {head_ref}"))?;
    if !output.status.success() {
        anyhow::bail!(
            "git diff --name-only failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
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

fn render_summary(
    args: &VerifyArgs,
    manifest: &BundleManifest,
    manifest_path: &Path,
    stages_run: &[&'static str],
) {
    println!("Pramaan verification bundle emitted");
    println!("base: {}", args.base);
    println!("head: {}", args.head);
    println!("bundle: {}", args.out.display());
    println!("manifest: {}", manifest_path.display());
    println!("final_status: {}", manifest.final_status);
    println!("stages_run: {}", stages_run.join(", "));
    if !args.skip_stages.is_empty() {
        println!("stages_skipped: {}", args.skip_stages.join(", "));
    }
    println!();
    println!("Stages");
    println!("{:<32} {:<16} receipt", "stage", "status");

    for stage in &manifest.stages {
        println!(
            "{:<32} {:<16} {}",
            stage.id, stage.status, stage.receipt_path
        );
    }

    let risks = summarize_risks_from_manifest(manifest);

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

fn summarize_risks_from_manifest(manifest: &BundleManifest) -> RiskSummary {
    let mut summary = RiskSummary::default();

    for stage in &manifest.stages {
        count_families(&stage.mitigated_risks, &mut summary.mitigated);
        if stage.status == "skipped" {
            count_families(&stage.residual_risks, &mut summary.skipped);
        } else {
            count_families(&stage.residual_risks, &mut summary.residual);
        }
        count_families(&stage.not_applicable_risks, &mut summary.not_applicable);
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
