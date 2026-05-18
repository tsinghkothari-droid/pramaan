use chrono::Utc;
use pramaan_core::{
    timestamp, AgentAttribution, AgentProvenanceEntry, EvidenceSensitivity, PluginIdentity,
    PolicyDecision, Receipt, RedactionManifest, ReviewerOverride, StageBudget,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const BUNDLE_SCHEMA_VERSION: &str = "pramaan.bundle.v1";
pub const MANIFEST_FILE_NAME: &str = "bundle.manifest.json";

#[derive(Debug, Error)]
pub enum BundleError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("JSON error in {path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("bundle schema error: {0}")]
    Schema(String),
    #[error("bundle integrity error: {0}")]
    Integrity(String),
}

pub type Result<T> = std::result::Result<T, BundleError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Digest {
    pub algorithm: String,
    pub value: String,
}

impl Digest {
    pub fn sha256(bytes: impl AsRef<[u8]>) -> Self {
        let digest = Sha256::digest(bytes.as_ref());
        Self {
            algorithm: "sha256".to_string(),
            value: format!("{digest:x}"),
        }
    }

    pub fn prefixed(&self) -> String {
        format!("{}:{}", self.algorithm, self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestRef {
    pub name: String,
    pub path: String,
    pub digest: Digest,
    pub media_type: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryRef {
    pub path: String,
    pub base_ref: String,
    pub head_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolIdentity {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageManifest {
    pub id: String,
    pub status: String,
    pub receipt_path: String,
    pub tool: ToolIdentity,
    pub mitigated_risks: Vec<String>,
    pub residual_risks: Vec<String>,
    pub not_applicable_risks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seeds: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub corpus_hashes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_identity: Option<PluginIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_decision: Option<PolicyDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_budget: Option<StageBudget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_sensitivity: Option<EvidenceSensitivity>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskSummary {
    pub mitigated: Vec<String>,
    pub residual: Vec<String>,
    pub skipped: Vec<String>,
    pub not_applicable: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleSummary {
    pub claim: String,
    pub evidence: Vec<String>,
    pub residual_risk_note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SigningMetadata {
    pub mode: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signable_digest: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactAttestation {
    pub provider: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparency_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleIntegrity {
    pub manifest_digest: Digest,
    pub signing: Option<SigningMetadata>,
    pub artifact_attestation: Option<ArtifactAttestation>,
}

impl SigningMetadata {
    pub fn local_dev_signable(signable_digest: Digest) -> Self {
        Self {
            mode: "local_dev".to_string(),
            status: "signable".to_string(),
            dev_mode: Some(true),
            signable_digest: Some(signable_digest),
            note: Some(
                "Local dev signing metadata only; this is not CI-backed provenance.".to_string(),
            ),
            signature_path: None,
            certificate_path: None,
        }
    }
}

impl ArtifactAttestation {
    #[allow(clippy::too_many_arguments)]
    pub fn github_actions(
        status: impl Into<String>,
        issuer: impl Into<String>,
        subject: impl Into<String>,
        workflow: impl Into<String>,
        repository: impl Into<String>,
        commit_sha: impl Into<String>,
        transparency_mode: impl Into<String>,
    ) -> Self {
        Self {
            provider: "github_actions".to_string(),
            status: status.into(),
            attestation_path: None,
            issuer: Some(issuer.into()),
            subject: Some(subject.into()),
            workflow: Some(workflow.into()),
            repository: Some(repository.into()),
            commit_sha: Some(commit_sha.into()),
            transparency_mode: Some(transparency_mode.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleManifest {
    pub schema_version: String,
    pub bundle_id: String,
    pub run_id: String,
    pub created_at: String,
    pub final_status: String,
    pub repository: RepositoryRef,
    pub tool_versions: Vec<ToolIdentity>,
    pub stages: Vec<StageManifest>,
    pub receipts: Vec<ManifestRef>,
    pub artifacts: Vec<ManifestRef>,
    pub risk_summary: RiskSummary,
    pub summary: BundleSummary,
    pub integrity: BundleIntegrity,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent_attribution: Vec<AgentAttribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewer_overrides: Vec<ReviewerOverride>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub multi_agent_provenance: Vec<AgentProvenanceEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plugin_identities: Vec<PluginIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_manifest: Option<RedactionManifest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_decision: Option<PolicyDecision>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_budgets: Vec<StageBudget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleBuildOptions {
    pub bundle_id: String,
    pub run_id: String,
    pub repository: RepositoryRef,
}

impl BundleBuildOptions {
    pub fn synthetic(base_ref: impl Into<String>, head_ref: impl Into<String>) -> Self {
        let stamp = Utc::now().timestamp();
        Self {
            bundle_id: format!("bundle_{stamp}"),
            run_id: format!("run_{stamp}"),
            repository: RepositoryRef {
                path: ".".to_string(),
                base_ref: base_ref.into(),
                head_ref: head_ref.into(),
                base_sha: None,
                head_sha: None,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    pub manifest_path: PathBuf,
    pub checked_receipts: usize,
    pub checked_artifacts: usize,
}

pub fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    Digest::sha256(bytes).prefixed()
}

pub fn build_manifest(bundle_root: &Path, options: BundleBuildOptions) -> Result<BundleManifest> {
    let receipt_paths = collect_receipts(&bundle_root.join("receipts"))?;
    let mut receipts = Vec::new();
    let mut stages = Vec::new();
    let mut artifact_candidates = BTreeSet::<String>::new();
    let mut tools = BTreeSet::<(String, String)>::new();
    let mut mitigated = BTreeSet::new();
    let mut residual = BTreeSet::new();
    let mut skipped = BTreeSet::new();
    let mut not_applicable = BTreeSet::new();
    let mut agent_attribution = Vec::new();
    let mut reviewer_overrides = Vec::new();
    let mut multi_agent_provenance = Vec::new();
    let mut plugin_identities = Vec::new();
    let mut redaction_manifest = None;
    let mut policy_decision = None;
    let mut stage_budgets = Vec::new();

    for receipt_path in receipt_paths {
        let receipt = read_receipt(&receipt_path)?;
        let receipt_ref = manifest_ref_for_path(bundle_root, &receipt_path, "application/json")?;
        let relative_receipt_path = receipt_ref.path.clone();

        for risk_id in &receipt.mitigated_risks {
            mitigated.insert(risk_id.clone());
        }
        if receipt.status.as_str() == "skipped" {
            for risk_id in &receipt.residual_risks {
                skipped.insert(risk_id.clone());
            }
        } else {
            for risk_id in &receipt.residual_risks {
                residual.insert(risk_id.clone());
            }
        }
        for risk_id in &receipt.not_applicable_risks {
            not_applicable.insert(risk_id.clone());
        }

        tools.insert((receipt.tool.name.clone(), receipt.tool.version.clone()));
        if let Some(agent) = receipt.agent_author.clone() {
            agent_attribution.push(agent);
        }
        if let Some(reviewer_override) = receipt.reviewer_override.clone() {
            reviewer_overrides.push(reviewer_override);
        }
        multi_agent_provenance.extend(receipt.multi_agent_provenance.clone());
        if let Some(plugin_identity) = receipt.plugin_identity.clone() {
            plugin_identities.push(plugin_identity);
        }
        if redaction_manifest.is_none() {
            redaction_manifest = receipt.redaction_manifest.clone();
        }
        if policy_decision.is_none() {
            policy_decision = receipt.policy_decision.clone();
        }
        if let Some(stage_budget) = receipt.stage_budget.clone() {
            stage_budgets.push(stage_budget);
        }
        stages.push(stage_manifest(&receipt, relative_receipt_path));
        receipts.push(receipt_ref);

        for output in &receipt.outputs {
            if is_file_like_path(&output.path) {
                artifact_candidates.insert(output.path.clone());
            }
        }
        for artifact in &receipt.artifacts {
            if is_file_like_path(&artifact.path) {
                artifact_candidates.insert(artifact.path.clone());
            }
        }
    }

    let artifacts = artifact_candidates
        .iter()
        .filter_map(|path| resolve_reference_path(bundle_root, path))
        .filter(|path| path.is_file())
        .map(|path| manifest_ref_for_path(bundle_root, &path, media_type_for_path(&path)))
        .collect::<Result<Vec<_>>>()?;

    let final_status = final_status(&stages);
    let summary = BundleSummary {
        claim: "Pramaan verification bundle emitted from recorded stage receipts.".to_string(),
        evidence: stages
            .iter()
            .map(|stage| format!("{}: {}", stage.id, stage.status))
            .collect(),
        residual_risk_note: if residual.is_empty() {
            "No residual risks were reported by included receipts.".to_string()
        } else {
            format!(
                "Included receipts report residual risk IDs: {}.",
                residual.iter().cloned().collect::<Vec<_>>().join(", ")
            )
        },
    };

    let mut manifest = BundleManifest {
        schema_version: BUNDLE_SCHEMA_VERSION.to_string(),
        bundle_id: options.bundle_id,
        run_id: options.run_id,
        created_at: timestamp(Utc::now()),
        final_status,
        repository: options.repository,
        tool_versions: tools
            .into_iter()
            .map(|(name, version)| ToolIdentity {
                name,
                version,
                runtime: Some("rust".to_string()),
            })
            .collect(),
        stages,
        receipts,
        artifacts,
        risk_summary: RiskSummary {
            mitigated: mitigated.into_iter().collect(),
            residual: residual.into_iter().collect(),
            skipped: skipped.into_iter().collect(),
            not_applicable: not_applicable.into_iter().collect(),
        },
        summary,
        integrity: BundleIntegrity {
            manifest_digest: placeholder_manifest_digest(),
            signing: Some(SigningMetadata::local_dev_signable(
                placeholder_manifest_digest(),
            )),
            artifact_attestation: Some(ArtifactAttestation {
                provider: "none".to_string(),
                status: "not_requested".to_string(),
                attestation_path: None,
                issuer: None,
                subject: None,
                workflow: None,
                repository: None,
                commit_sha: None,
                transparency_mode: None,
            }),
        },
        agent_attribution,
        reviewer_overrides,
        multi_agent_provenance,
        plugin_identities,
        redaction_manifest,
        policy_decision,
        stage_budgets,
    };
    let signable_digest = manifest_digest(&manifest)?;
    if let Some(signing) = manifest.integrity.signing.as_mut() {
        signing.signable_digest = Some(signable_digest);
    }
    manifest.integrity.manifest_digest = manifest_digest(&manifest)?;
    validate_manifest_shape(&manifest)?;
    Ok(manifest)
}

pub fn write_manifest(bundle_root: &Path, manifest: &BundleManifest) -> Result<PathBuf> {
    let manifest_path = bundle_root.join(MANIFEST_FILE_NAME);
    let bytes = serde_json::to_vec_pretty(manifest).map_err(|source| BundleError::Json {
        path: manifest_path.clone(),
        source,
    })?;
    fs::write(&manifest_path, bytes).map_err(|source| BundleError::Io {
        path: manifest_path.clone(),
        source,
    })?;
    Ok(manifest_path)
}

pub fn verify_bundle(path: &Path) -> Result<VerificationReport> {
    let manifest_path = if path.is_dir() {
        path.join(MANIFEST_FILE_NAME)
    } else {
        path.to_path_buf()
    };
    let bundle_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let manifest = read_manifest(&manifest_path)?;

    validate_manifest_shape(&manifest)?;
    let expected_manifest_digest = manifest_digest(&manifest)?;
    if manifest.integrity.manifest_digest != expected_manifest_digest {
        return Err(BundleError::Integrity(format!(
            "manifest digest mismatch: expected {}, found {}",
            manifest.integrity.manifest_digest.prefixed(),
            expected_manifest_digest.prefixed()
        )));
    }

    for receipt in &manifest.receipts {
        verify_manifest_ref(bundle_root, receipt)?;
        let receipt_path = bundle_root.join(&receipt.path);
        let _ = read_receipt(&receipt_path)?;
    }
    for artifact in &manifest.artifacts {
        verify_manifest_ref(bundle_root, artifact)?;
    }

    Ok(VerificationReport {
        manifest_path,
        checked_receipts: manifest.receipts.len(),
        checked_artifacts: manifest.artifacts.len(),
    })
}

pub fn read_manifest(path: &Path) -> Result<BundleManifest> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| BundleError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn collect_receipts(root: &Path) -> Result<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    let mut pending = vec![root.to_path_buf()];
    let mut receipts = Vec::new();
    while let Some(path) = pending.pop() {
        let entries = fs::read_dir(&path).map_err(|source| BundleError::Io {
            path: path.clone(),
            source,
        })?;
        for entry in entries {
            let entry = entry.map_err(|source| BundleError::Io {
                path: path.clone(),
                source,
            })?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                pending.push(entry_path);
            } else if entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(".receipt.json"))
            {
                receipts.push(entry_path);
            }
        }
    }
    receipts.sort();
    Ok(receipts)
}

fn read_receipt(path: &Path) -> Result<Receipt> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let receipt: Receipt = serde_json::from_slice(&bytes).map_err(|source| BundleError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    if receipt.schema_version != pramaan_core::RECEIPT_SCHEMA_VERSION {
        return Err(BundleError::Schema(format!(
            "{} has unsupported receipt schema_version {}",
            path.display(),
            receipt.schema_version
        )));
    }
    Ok(receipt)
}

fn stage_manifest(receipt: &Receipt, receipt_path: String) -> StageManifest {
    let metadata_value = |key: &str| receipt.metadata.get(key).cloned().unwrap_or_default();
    let seeds = ["seed", "fuzz_seed"]
        .iter()
        .filter_map(|key| receipt.metadata.get(*key).cloned())
        .collect::<Vec<_>>();
    let corpus_hashes = ["corpus_hash", "corpus_digest"]
        .iter()
        .filter_map(|key| receipt.metadata.get(*key).cloned())
        .chain(
            receipt
                .outputs
                .iter()
                .filter(|output| output.name.contains("corpus"))
                .filter_map(|output| output.digest.clone()),
        )
        .collect::<Vec<_>>();

    StageManifest {
        id: receipt.stage.clone(),
        status: receipt.status.as_str().to_string(),
        receipt_path,
        tool: ToolIdentity {
            name: receipt.tool.name.clone(),
            version: receipt.tool.version.clone(),
            runtime: if metadata_value("runtime").is_empty() {
                None
            } else {
                Some(metadata_value("runtime"))
            },
        },
        mitigated_risks: receipt.mitigated_risks.clone(),
        residual_risks: receipt.residual_risks.clone(),
        not_applicable_risks: receipt.not_applicable_risks.clone(),
        seeds,
        corpus_hashes,
        plugin_identity: receipt.plugin_identity.clone(),
        policy_decision: receipt.policy_decision.clone(),
        stage_budget: receipt.stage_budget.clone(),
        evidence_sensitivity: receipt.evidence_sensitivity.clone(),
    }
}

fn final_status(stages: &[StageManifest]) -> String {
    if stages.iter().any(|stage| stage.status == "error") {
        "error".to_string()
    } else if stages
        .iter()
        .any(|stage| matches!(stage.status.as_str(), "failed" | "timed_out"))
    {
        "failed".to_string()
    } else if stages.iter().all(|stage| stage.status == "passed") {
        "passed".to_string()
    } else {
        "inconclusive".to_string()
    }
}

fn manifest_ref_for_path(bundle_root: &Path, path: &Path, media_type: &str) -> Result<ManifestRef> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(ManifestRef {
        name: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("artifact")
            .to_string(),
        path: portable_relative_path(bundle_root, path),
        digest: Digest::sha256(&bytes),
        media_type: media_type.to_string(),
        size_bytes: bytes.len() as u64,
    })
}

fn verify_manifest_ref(bundle_root: &Path, manifest_ref: &ManifestRef) -> Result<()> {
    if manifest_ref.digest.algorithm != "sha256" {
        return Err(BundleError::Schema(format!(
            "{} uses unsupported digest algorithm {}",
            manifest_ref.path, manifest_ref.digest.algorithm
        )));
    }

    let path = bundle_root.join(&manifest_ref.path);
    let bytes = fs::read(&path).map_err(|source| BundleError::Io {
        path: path.clone(),
        source,
    })?;
    let actual = Digest::sha256(&bytes);
    if actual != manifest_ref.digest {
        return Err(BundleError::Integrity(format!(
            "{} digest mismatch: expected {}, found {}",
            manifest_ref.path,
            manifest_ref.digest.prefixed(),
            actual.prefixed()
        )));
    }
    if bytes.len() as u64 != manifest_ref.size_bytes {
        return Err(BundleError::Integrity(format!(
            "{} size mismatch: expected {}, found {}",
            manifest_ref.path,
            manifest_ref.size_bytes,
            bytes.len()
        )));
    }
    Ok(())
}

fn manifest_digest(manifest: &BundleManifest) -> Result<Digest> {
    let mut normalized = manifest.clone();
    normalized.integrity.manifest_digest = placeholder_manifest_digest();
    let bytes = serde_json::to_vec(&normalized).map_err(|source| BundleError::Json {
        path: PathBuf::from(MANIFEST_FILE_NAME),
        source,
    })?;
    Ok(Digest::sha256(bytes))
}

fn placeholder_manifest_digest() -> Digest {
    Digest {
        algorithm: "sha256".to_string(),
        value: "0".repeat(64),
    }
}

fn validate_manifest_shape(manifest: &BundleManifest) -> Result<()> {
    if manifest.schema_version != BUNDLE_SCHEMA_VERSION {
        return Err(BundleError::Schema(format!(
            "unsupported schema_version {}",
            manifest.schema_version
        )));
    }
    if !manifest.bundle_id.starts_with("bundle_") {
        return Err(BundleError::Schema(
            "bundle_id must start with bundle_".to_string(),
        ));
    }
    if !manifest.run_id.starts_with("run_") {
        return Err(BundleError::Schema(
            "run_id must start with run_".to_string(),
        ));
    }
    if manifest.receipts.is_empty() {
        return Err(BundleError::Schema(
            "manifest must reference at least one receipt".to_string(),
        ));
    }
    if !matches!(
        manifest.final_status.as_str(),
        "passed" | "failed" | "inconclusive" | "error"
    ) {
        return Err(BundleError::Schema(format!(
            "invalid final_status {}",
            manifest.final_status
        )));
    }
    for reference in manifest.receipts.iter().chain(manifest.artifacts.iter()) {
        if reference.path.trim().is_empty() || reference.name.trim().is_empty() {
            return Err(BundleError::Schema(
                "manifest references must have non-empty name and path".to_string(),
            ));
        }
        if reference.digest.algorithm != "sha256" || reference.digest.value.len() != 64 {
            return Err(BundleError::Schema(format!(
                "{} must use a sha256 digest with 64 hex characters",
                reference.path
            )));
        }
        if !reference
            .digest
            .value
            .chars()
            .all(|character| character.is_ascii_hexdigit())
        {
            return Err(BundleError::Schema(format!(
                "{} digest is not hexadecimal",
                reference.path
            )));
        }
    }
    Ok(())
}

fn resolve_reference_path(bundle_root: &Path, raw_path: &str) -> Option<PathBuf> {
    let path = Path::new(raw_path);
    if path.is_absolute() && path.exists() {
        return Some(path.to_path_buf());
    }

    let direct = bundle_root.join(raw_path);
    if direct.exists() {
        return Some(direct);
    }

    let file_name = path.file_name()?.to_str()?;
    let by_name = find_file_by_name(bundle_root, file_name)?;
    Some(by_name)
}

fn find_file_by_name(root: &Path, file_name: &str) -> Option<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let entries = fs::read_dir(&path).ok()?;
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                pending.push(entry_path);
            } else if entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == file_name)
            {
                return Some(entry_path);
            }
        }
    }
    None
}

fn is_file_like_path(path: &str) -> bool {
    !path.trim().is_empty() && !path.ends_with('/') && !path.contains("inode/directory")
}

fn media_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("json") => "application/json",
        Some("txt") | Some("log") => "text/plain",
        _ => "application/octet-stream",
    }
}

fn portable_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pramaan_core::{
        AgentAttribution, AttributionConfidence, EvidenceSensitivity, OutputRef, PluginIdentity,
        PolicyDecision, ReceiptSummary, RedactionManifest, RiskRefs, StageBudget, StageStatus,
    };

    #[test]
    fn hashes_with_sha256_prefix() {
        assert_eq!(
            sha256_hex("pramaan"),
            "sha256:8c5f42527c4bdf03d8d19b4d8e25f530367751d7ea75ccdaf760b50482449cc0"
        );
    }

    #[test]
    fn local_dev_signing_metadata_is_marked_signable_and_not_ci_backed() {
        let signing = SigningMetadata::local_dev_signable(Digest::sha256("signable"));

        assert_eq!(signing.mode, "local_dev");
        assert_eq!(signing.status, "signable");
        assert_eq!(signing.dev_mode, Some(true));
        assert_eq!(
            signing.signable_digest.as_ref().map(Digest::prefixed),
            Some(sha256_hex("signable"))
        );
        assert!(signing
            .note
            .as_ref()
            .expect("dev signing note")
            .contains("not CI-backed"));
    }

    #[test]
    fn github_attestation_metadata_carries_ci_identity_fields() {
        let metadata = ArtifactAttestation::github_actions(
            "present",
            "https://token.actions.githubusercontent.com",
            "repo:pramaan/pramaan:ref:refs/heads/main",
            "pramaan.yml",
            "pramaan/pramaan",
            "0123456789abcdef0123456789abcdef01234567",
            "public_rekor",
        );
        let value = serde_json::to_value(metadata).expect("serialize metadata");

        assert_eq!(value["provider"], "github_actions");
        assert_eq!(
            value["issuer"],
            "https://token.actions.githubusercontent.com"
        );
        assert_eq!(value["subject"], "repo:pramaan/pramaan:ref:refs/heads/main");
        assert_eq!(value["workflow"], "pramaan.yml");
        assert_eq!(value["repository"], "pramaan/pramaan");
        assert_eq!(
            value["commit_sha"],
            "0123456789abcdef0123456789abcdef01234567"
        );
        assert_eq!(value["transparency_mode"], "public_rekor");
    }

    #[test]
    fn bundle_verification_fails_after_receipt_tamper() {
        let root = std::env::temp_dir().join(format!("pramaan-bundle-test-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("clean temp bundle");
        }
        fs::create_dir_all(root.join("receipts")).expect("receipt dir");
        fs::write(root.join("artifact.json"), br#"{"ok":true}"#).expect("artifact");

        let receipt = Receipt::synthetic(
            "claim_scope",
            StageStatus::Passed,
            "HEAD",
            "HEAD",
            vec![OutputRef {
                name: "artifact".to_string(),
                path: "artifact.json".to_string(),
                digest: None,
            }],
            vec![],
            ReceiptSummary {
                title: "Synthetic".to_string(),
                details: "Synthetic receipt.".to_string(),
            },
            RiskRefs::sample(),
        );
        let receipt_path = root.join("receipts").join("claim.receipt.json");
        fs::write(
            &receipt_path,
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let manifest =
            build_manifest(&root, BundleBuildOptions::synthetic("HEAD", "HEAD")).expect("manifest");
        write_manifest(&root, &manifest).expect("write manifest");
        verify_bundle(&root).expect("valid bundle verifies");

        fs::write(&receipt_path, b"{\"tampered\":true}").expect("tamper receipt");
        let error = verify_bundle(&root).expect_err("tamper should fail");
        assert!(error.to_string().contains("digest mismatch"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn manifest_aggregates_phase_16a_trust_hooks() {
        let root =
            std::env::temp_dir().join(format!("pramaan-phase16a-test-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("clean temp bundle");
        }
        fs::create_dir_all(root.join("receipts")).expect("receipt dir");

        let mut receipt = Receipt::synthetic(
            "claim_scope",
            StageStatus::Passed,
            "HEAD",
            "HEAD",
            vec![],
            vec![],
            ReceiptSummary {
                title: "Synthetic".to_string(),
                details: "Synthetic receipt with trust hooks.".to_string(),
            },
            RiskRefs::sample(),
        );
        receipt.agent_author = Some(AgentAttribution {
            product: "Codex".to_string(),
            model_family: Some("gpt-5".to_string()),
            model_version: None,
            execution_mode: "autonomous_phase".to_string(),
            prompt_context_hash: None,
            commit_provenance: None,
            source: "local_cli".to_string(),
            confidence: AttributionConfidence::Unknown,
        });
        receipt.plugin_identity = Some(PluginIdentity {
            name: "pramaan-core".to_string(),
            version: "0.1.0".to_string(),
            provenance: "workspace".to_string(),
            signature: None,
            sandbox_boundary: "in_process".to_string(),
        });
        receipt.evidence_sensitivity = Some(EvidenceSensitivity::Internal);
        receipt.redaction_manifest = Some(RedactionManifest {
            profile: "reviewer-redacted".to_string(),
            redacted_fields: vec!["environment.variables.SECRET_TOKEN".to_string()],
            hashed_fields: vec!["repository.path".to_string()],
            policy: "pramaan-redaction-v0".to_string(),
        });
        receipt.policy_decision = Some(PolicyDecision {
            decision: "warning".to_string(),
            policy_id: "pramaan-default-v0".to_string(),
            hard_failures: vec![],
            warnings: vec!["synthetic_evidence_only".to_string()],
            waived: vec![],
        });
        receipt.stage_budget = Some(StageBudget {
            target_ms: 30_000,
            max_ms: 60_000,
            consumed_ms: 1_000,
            exhausted: false,
            timeout_reason: None,
            partial_evidence: true,
        });

        fs::write(
            root.join("receipts").join("claim.receipt.json"),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let manifest =
            build_manifest(&root, BundleBuildOptions::synthetic("HEAD", "HEAD")).expect("manifest");

        assert_eq!(manifest.agent_attribution.len(), 1);
        assert_eq!(manifest.agent_attribution[0].product, "Codex");
        assert_eq!(manifest.plugin_identities.len(), 1);
        assert_eq!(manifest.plugin_identities[0].name, "pramaan-core");
        assert_eq!(
            manifest
                .redaction_manifest
                .as_ref()
                .expect("redaction")
                .profile,
            "reviewer-redacted"
        );
        assert_eq!(
            manifest.policy_decision.as_ref().expect("policy").decision,
            "warning"
        );
        assert_eq!(manifest.stage_budgets.len(), 1);
        assert!(manifest.stage_budgets[0].partial_evidence);
        assert_eq!(
            manifest.stages[0].evidence_sensitivity,
            Some(EvidenceSensitivity::Internal)
        );

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }
}
