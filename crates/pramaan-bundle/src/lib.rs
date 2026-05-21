use chrono::Utc;
use pramaan_core::{
    canonical_json_bytes, redact_sensitive_text, timestamp, validate_plugin_receipt_trust,
    AgentAttribution, AgentProvenanceEntry, ArtifactRef, EvidenceSensitivity, OutputRef,
    PluginIdentity, PolicyDecision, Receipt, ReceiptSummary, RedactionManifest, ReviewerOverride,
    RiskRefs, StageBudget, StageStatus, REDACTION_POLICY_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use thiserror::Error;

pub const BUNDLE_SCHEMA_VERSION: &str = "pramaan.bundle.v1";
pub const VSA_SCHEMA_VERSION: &str = "pramaan.vsa.v1";
pub const MANIFEST_FILE_NAME: &str = "bundle.manifest.json";
pub const ATTESTATIONS_DIR_NAME: &str = "attestations";
pub const VSA_FILE_NAME: &str = "bundle.vsa.json";
pub const IN_TOTO_FILE_NAME: &str = "bundle.in-toto.json";
pub const IN_TOTO_STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
pub const SLSA_VSA_PREDICATE_TYPE: &str = "https://slsa.dev/verification_summary/v1";

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InTotoSubject {
    pub name: String,
    pub digest: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsaVerifier {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsaPolicy {
    pub id: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VsaConfidenceArtifact {
    pub path: String,
    pub digest: Digest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationSummaryAttestation {
    pub schema_version: String,
    pub predicate_type: String,
    pub subject: Vec<InTotoSubject>,
    pub verifier: VsaVerifier,
    pub time_verified: String,
    pub policy: VsaPolicy,
    pub verification_result: String,
    pub manifest_digest: Digest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_artifact: Option<VsaConfidenceArtifact>,
    pub verified_levels: Vec<String>,
    pub dependency_levels: Vec<String>,
    pub resource_uri: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InTotoStatement {
    #[serde(rename = "_type")]
    pub statement_type: String,
    pub subject: Vec<InTotoSubject>,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub predicate: VerificationSummaryAttestation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineAttestationReport {
    pub vsa_path: PathBuf,
    pub statement_path: PathBuf,
    pub manifest_digest: Digest,
    pub verification_result: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionExportReport {
    pub bundle_root: PathBuf,
    pub manifest_path: PathBuf,
    pub profile: String,
    pub redacted_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RedactionExportManifest {
    schema_version: String,
    profile: String,
    source_manifest_digest: Digest,
    redacted_files: Vec<String>,
    policy: String,
}

pub fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    Digest::sha256(bytes).prefixed()
}

pub fn export_redacted_bundle(
    source_bundle: &Path,
    out_dir: &Path,
    profile: &str,
) -> Result<RedactionExportReport> {
    validate_redaction_profile(profile)?;
    if out_dir.exists() {
        return Err(BundleError::Schema(format!(
            "redaction output {} already exists",
            out_dir.display()
        )));
    }

    verify_bundle(source_bundle)?;
    let source_manifest_path = if source_bundle.is_dir() {
        source_bundle.join(MANIFEST_FILE_NAME)
    } else {
        source_bundle.to_path_buf()
    };
    let source_root = source_manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let source_manifest = read_manifest(&source_manifest_path)?;
    copy_dir_all(source_root, out_dir)?;

    let copied_manifest_path = out_dir.join(MANIFEST_FILE_NAME);
    if copied_manifest_path.exists() {
        fs::remove_file(&copied_manifest_path).map_err(|source| BundleError::Io {
            path: copied_manifest_path.clone(),
            source,
        })?;
    }
    let copied_attestations_dir = out_dir.join(ATTESTATIONS_DIR_NAME);
    if copied_attestations_dir.exists() {
        fs::remove_dir_all(&copied_attestations_dir).map_err(|source| BundleError::Io {
            path: copied_attestations_dir.clone(),
            source,
        })?;
    }

    let mut redacted_files = Vec::new();
    redact_bundle_files(out_dir, profile, &mut redacted_files)?;
    redacted_files.sort();
    redacted_files.dedup();

    let export_manifest_dir = out_dir.join("redaction");
    fs::create_dir_all(&export_manifest_dir).map_err(|source| BundleError::Io {
        path: export_manifest_dir.clone(),
        source,
    })?;
    let export_manifest_path = export_manifest_dir.join("export-manifest.json");
    let export_manifest = RedactionExportManifest {
        schema_version: "pramaan.redaction_export.v1".to_string(),
        profile: profile.to_string(),
        source_manifest_digest: source_manifest.integrity.manifest_digest.clone(),
        redacted_files: redacted_files.clone(),
        policy: REDACTION_POLICY_VERSION.to_string(),
    };
    write_json_file(&export_manifest_path, &export_manifest)?;

    let receipt_dir = out_dir.join("receipts");
    fs::create_dir_all(&receipt_dir).map_err(|source| BundleError::Io {
        path: receipt_dir.clone(),
        source,
    })?;
    let relative_export_manifest = portable_relative_path(out_dir, &export_manifest_path);
    let mut receipt = Receipt::synthetic(
        "bundle_redaction",
        StageStatus::Passed,
        source_manifest.repository.base_ref.clone(),
        source_manifest.repository.head_ref.clone(),
        vec![OutputRef {
            name: "redaction_export_manifest".to_string(),
            path: relative_export_manifest.clone(),
            digest: Some(manifest_file_digest(&export_manifest_path)?),
        }],
        vec![ArtifactRef {
            name: "redaction_export_manifest".to_string(),
            path: relative_export_manifest,
            media_type: Some("application/json".to_string()),
            digest: None,
        }],
        ReceiptSummary {
            title: "Bundle redaction export complete".to_string(),
            details: format!(
                "Profile {profile} redacted {} file(s); old attestations were removed because redaction changes manifest hashes.",
                redacted_files.len()
            ),
        },
        RiskRefs {
            mitigated: vec!["R-072".to_string()],
            residual: vec![],
            not_applicable: vec![],
        },
    );
    receipt.evidence_sensitivity = Some(EvidenceSensitivity::Redacted);
    receipt.redaction_manifest = Some(RedactionManifest {
        profile: profile.to_string(),
        redacted_fields: redacted_files.clone(),
        hashed_fields: vec!["source_manifest_digest".to_string()],
        policy: REDACTION_POLICY_VERSION.to_string(),
    });
    write_json_file(&receipt_dir.join("bundle-redaction.receipt.json"), &receipt)?;

    let manifest = build_manifest(
        out_dir,
        BundleBuildOptions {
            bundle_id: format!("bundle_redacted_{}", Utc::now().timestamp()),
            run_id: format!("run_redacted_{}", Utc::now().timestamp()),
            repository: source_manifest.repository,
        },
    )?;
    let manifest_path = write_manifest(out_dir, &manifest)?;
    verify_bundle(out_dir)?;

    Ok(RedactionExportReport {
        bundle_root: out_dir.to_path_buf(),
        manifest_path,
        profile: profile.to_string(),
        redacted_files,
    })
}

pub fn emit_offline_attestations(bundle_root: &Path) -> Result<OfflineAttestationReport> {
    verify_bundle(bundle_root)?;
    let manifest_path = bundle_root.join(MANIFEST_FILE_NAME);
    let mut manifest = read_manifest(&manifest_path)?;
    let statement_relative_path = format!("{ATTESTATIONS_DIR_NAME}/{IN_TOTO_FILE_NAME}");

    manifest.integrity.artifact_attestation = Some(ArtifactAttestation {
        provider: "slsa".to_string(),
        status: "present".to_string(),
        attestation_path: Some(statement_relative_path),
        issuer: None,
        subject: Some(MANIFEST_FILE_NAME.to_string()),
        workflow: None,
        repository: None,
        commit_sha: None,
        transparency_mode: Some("none".to_string()),
    });
    manifest.integrity.manifest_digest = manifest_digest(&manifest)?;
    write_manifest(bundle_root, &manifest)?;

    let summary = verification_summary_for_manifest(&manifest);
    let statement = InTotoStatement {
        statement_type: IN_TOTO_STATEMENT_TYPE.to_string(),
        subject: summary.subject.clone(),
        predicate_type: SLSA_VSA_PREDICATE_TYPE.to_string(),
        predicate: summary.clone(),
    };

    let attestations_dir = bundle_root.join(ATTESTATIONS_DIR_NAME);
    fs::create_dir_all(&attestations_dir).map_err(|source| BundleError::Io {
        path: attestations_dir.clone(),
        source,
    })?;
    let vsa_path = attestations_dir.join(VSA_FILE_NAME);
    let statement_path = attestations_dir.join(IN_TOTO_FILE_NAME);
    write_json_file(&vsa_path, &summary)?;
    write_json_file(&statement_path, &statement)?;

    verify_offline_attestations(bundle_root)
}

pub fn verify_offline_attestations(bundle_root: &Path) -> Result<OfflineAttestationReport> {
    verify_bundle(bundle_root)?;
    let manifest_path = bundle_root.join(MANIFEST_FILE_NAME);
    let manifest = read_manifest(&manifest_path)?;
    let attestation = manifest
        .integrity
        .artifact_attestation
        .as_ref()
        .ok_or_else(|| {
            BundleError::Integrity("manifest has no attestation metadata".to_string())
        })?;
    if attestation.status != "present" && attestation.status != "verified" {
        return Err(BundleError::Integrity(format!(
            "attestation status {} is not verifiable offline",
            attestation.status
        )));
    }
    let statement_relative_path = attestation
        .attestation_path
        .as_ref()
        .ok_or_else(|| BundleError::Integrity("attestation_path is missing".to_string()))?;
    if !is_safe_relative_manifest_path(statement_relative_path) {
        return Err(BundleError::Schema(format!(
            "{statement_relative_path} must be a relative path inside the bundle"
        )));
    }

    let statement_path = bundle_root.join(statement_relative_path);
    let statement_dir = statement_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| bundle_root.join(ATTESTATIONS_DIR_NAME));
    let vsa_path = statement_dir.join(VSA_FILE_NAME);
    let statement: InTotoStatement = read_json_file(&statement_path)?;
    let vsa: VerificationSummaryAttestation = read_json_file(&vsa_path)?;

    if statement.statement_type != IN_TOTO_STATEMENT_TYPE {
        return Err(BundleError::Integrity(format!(
            "unsupported in-toto statement type {}",
            statement.statement_type
        )));
    }
    if statement.predicate_type != SLSA_VSA_PREDICATE_TYPE {
        return Err(BundleError::Integrity(format!(
            "unsupported predicate type {}",
            statement.predicate_type
        )));
    }
    if statement.predicate != vsa {
        return Err(BundleError::Integrity(
            "in-toto predicate does not match bundle VSA artifact".to_string(),
        ));
    }

    let expected = verification_summary_for_manifest(&manifest);
    if vsa.subject != expected.subject {
        return Err(BundleError::Integrity(
            "VSA subject digest does not match current bundle manifest".to_string(),
        ));
    }
    if vsa.manifest_digest != expected.manifest_digest {
        return Err(BundleError::Integrity(format!(
            "VSA manifest digest mismatch: expected {}, found {}",
            expected.manifest_digest.prefixed(),
            vsa.manifest_digest.prefixed()
        )));
    }
    if vsa.verification_result != expected.verification_result {
        return Err(BundleError::Integrity(format!(
            "VSA result mismatch: expected {}, found {}",
            expected.verification_result, vsa.verification_result
        )));
    }
    if vsa.confidence_artifact != expected.confidence_artifact {
        return Err(BundleError::Integrity(
            "VSA confidence artifact reference does not match bundle manifest".to_string(),
        ));
    }

    Ok(OfflineAttestationReport {
        vsa_path,
        statement_path,
        manifest_digest: expected.manifest_digest,
        verification_result: expected.verification_result,
    })
}

fn validate_redaction_profile(profile: &str) -> Result<()> {
    if matches!(
        profile,
        "internal-full" | "reviewer-redacted" | "public-demo" | "summary-only"
    ) {
        Ok(())
    } else {
        Err(BundleError::Schema(format!(
            "unsupported redaction profile {profile}; expected internal-full, reviewer-redacted, public-demo, or summary-only"
        )))
    }
}

fn copy_dir_all(source: &Path, destination: &Path) -> Result<()> {
    if destination.starts_with(source) {
        return Err(BundleError::Schema(format!(
            "redaction output {} must not be inside source bundle {}",
            destination.display(),
            source.display()
        )));
    }
    fs::create_dir_all(destination).map_err(|source_error| BundleError::Io {
        path: destination.to_path_buf(),
        source: source_error,
    })?;
    for entry in fs::read_dir(source).map_err(|source_error| BundleError::Io {
        path: source.to_path_buf(),
        source: source_error,
    })? {
        let entry = entry.map_err(|source_error| BundleError::Io {
            path: source.to_path_buf(),
            source: source_error,
        })?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir_all(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path).map_err(|source_error| BundleError::Io {
                path: destination_path,
                source: source_error,
            })?;
        }
    }
    Ok(())
}

fn redact_bundle_files(root: &Path, profile: &str, redacted_files: &mut Vec<String>) -> Result<()> {
    if profile == "internal-full" {
        return Ok(());
    }

    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(&path).map_err(|source| BundleError::Io {
            path: path.clone(),
            source,
        })? {
            let entry = entry.map_err(|source| BundleError::Io {
                path: path.clone(),
                source,
            })?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                pending.push(entry_path);
            } else if should_redact_file(&entry_path) {
                let redacted =
                    if entry_path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                        redact_json_file(&entry_path)?
                    } else {
                        redact_text_file(&entry_path)?
                    };
                if redacted {
                    redacted_files.push(portable_relative_path(root, &entry_path));
                }
            }
        }
    }
    Ok(())
}

fn should_redact_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("json" | "md" | "txt" | "log" | "yml" | "yaml")
    )
}

fn redact_json_file(path: &Path) -> Result<bool> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
        return redact_text_file(path);
    };
    let original = value.clone();
    redact_json_value(&mut value);
    if value == original {
        return Ok(false);
    }
    write_json_file(path, &value)?;
    Ok(true)
}

fn redact_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(text) => {
            *text = redact_sensitive_text(text);
        }
        serde_json::Value::Array(items) => {
            for item in items {
                redact_json_value(item);
            }
        }
        serde_json::Value::Object(map) => {
            for value in map.values_mut() {
                redact_json_value(value);
            }
        }
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {}
    }
}

fn redact_text_file(path: &Path) -> Result<bool> {
    let text = fs::read_to_string(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let redacted = redact_sensitive_text(&text);
    if redacted == text {
        return Ok(false);
    }
    fs::write(path, redacted).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(true)
}

fn manifest_file_digest(path: &Path) -> Result<String> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(Digest::sha256(bytes).prefixed())
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
        let plugin_findings = validate_plugin_receipt_trust(&receipt);
        if let Some(finding) = plugin_findings
            .iter()
            .find(|finding| matches!(finding.severity.as_str(), "critical" | "high"))
        {
            return Err(BundleError::Schema(format!(
                "{} failed plugin trust validation: {} {}",
                receipt_path.display(),
                finding.id,
                finding.message
            )));
        }
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
            if artifact.media_type.as_deref() == Some("inode/directory") {
                continue;
            }
            if is_file_like_path(&artifact.path) {
                artifact_candidates.insert(artifact.path.clone());
            }
        }
    }

    let mut artifacts = Vec::new();
    for raw_path in &artifact_candidates {
        let Some(path) = resolve_reference_path(bundle_root, raw_path)? else {
            return Err(BundleError::Schema(format!(
                "receipt-declared artifact {raw_path} was not found in the bundle"
            )));
        };
        if !path.is_file() {
            return Err(BundleError::Schema(format!(
                "receipt-declared artifact {raw_path} is not a file"
            )));
        }
        artifacts.push(manifest_ref_for_path(
            bundle_root,
            &path,
            media_type_for_path(&path),
        )?);
    }

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
    let bytes = canonical_json_bytes(&normalized).map_err(|source| BundleError::Json {
        path: PathBuf::from(MANIFEST_FILE_NAME),
        source,
    })?;
    Ok(Digest::sha256(bytes))
}

fn verification_summary_for_manifest(manifest: &BundleManifest) -> VerificationSummaryAttestation {
    let mut subject_digest = BTreeMap::new();
    subject_digest.insert(
        manifest.integrity.manifest_digest.algorithm.clone(),
        manifest.integrity.manifest_digest.value.clone(),
    );
    let verification_result = vsa_result_for_manifest(manifest);
    VerificationSummaryAttestation {
        schema_version: VSA_SCHEMA_VERSION.to_string(),
        predicate_type: SLSA_VSA_PREDICATE_TYPE.to_string(),
        subject: vec![InTotoSubject {
            name: MANIFEST_FILE_NAME.to_string(),
            digest: subject_digest,
        }],
        verifier: VsaVerifier {
            id: "pramaan-offline-verifier".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
        time_verified: timestamp(Utc::now()),
        policy: VsaPolicy {
            id: "pramaan-local-offline-v0".to_string(),
            uri: "https://docs.pramaan.dev/policies/local-offline-v0".to_string(),
        },
        verification_result: verification_result.clone(),
        manifest_digest: manifest.integrity.manifest_digest.clone(),
        confidence_artifact: confidence_artifact_ref(manifest),
        verified_levels: vec!["bundle_hash_integrity".to_string()],
        dependency_levels: Vec::new(),
        resource_uri: MANIFEST_FILE_NAME.to_string(),
        summary: format!(
            "Offline VSA records Pramaan bundle hash integrity as {verification_result}; it is evidence, not a correctness proof."
        ),
    }
}

fn vsa_result_for_manifest(manifest: &BundleManifest) -> String {
    if matches!(manifest.final_status.as_str(), "failed" | "error") {
        "FAILED".to_string()
    } else if manifest.final_status == "passed"
        && manifest.risk_summary.residual.is_empty()
        && manifest.risk_summary.skipped.is_empty()
    {
        "PASSED".to_string()
    } else {
        "WARNING".to_string()
    }
}

fn confidence_artifact_ref(manifest: &BundleManifest) -> Option<VsaConfidenceArtifact> {
    manifest
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "confidence.json")
        .map(|artifact| VsaConfidenceArtifact {
            path: artifact.path.clone(),
            digest: artifact.digest.clone(),
        })
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|source| BundleError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    fs::write(path, bytes).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let bytes = fs::read(path).map_err(|source| BundleError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| BundleError::Json {
        path: path.to_path_buf(),
        source,
    })
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
        if !is_safe_relative_manifest_path(&reference.path) {
            return Err(BundleError::Schema(format!(
                "{} must be a relative path inside the bundle",
                reference.path
            )));
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

fn is_safe_relative_manifest_path(raw_path: &str) -> bool {
    let path = Path::new(raw_path);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}

fn resolve_reference_path(bundle_root: &Path, raw_path: &str) -> Result<Option<PathBuf>> {
    let path = Path::new(raw_path);
    if path.is_absolute() && path.exists() {
        let canonical_bundle_root =
            bundle_root
                .canonicalize()
                .map_err(|source| BundleError::Io {
                    path: bundle_root.to_path_buf(),
                    source,
                })?;
        let canonical_path = path.canonicalize().map_err(|source| BundleError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        if !canonical_path.starts_with(&canonical_bundle_root) {
            return Err(BundleError::Schema(format!(
                "{raw_path} must stay inside the bundle root"
            )));
        }
        return Ok(Some(canonical_path));
    }

    let direct = bundle_root.join(raw_path);
    if direct.exists() {
        return Ok(Some(direct));
    }

    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return Ok(None);
    };
    let matches = find_files_by_name(bundle_root, file_name)?;
    match matches.len() {
        0 => Ok(None),
        1 => Ok(matches.into_iter().next()),
        _ => Err(BundleError::Schema(format!(
            "{raw_path} is ambiguous inside the bundle; use an exact relative path"
        ))),
    }
}

fn find_files_by_name(root: &Path, file_name: &str) -> Result<Vec<PathBuf>> {
    let mut pending = vec![root.to_path_buf()];
    let mut found = Vec::new();
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
                .is_some_and(|name| name == file_name)
            {
                found.push(entry_path);
            }
        }
    }
    Ok(found)
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
    if let (Ok(root), Ok(path)) = (root.canonicalize(), path.canonicalize()) {
        if let Ok(relative) = path.strip_prefix(&root) {
            return relative.to_string_lossy().replace('\\', "/");
        }
    }
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
    fn bundle_verification_fails_when_artifact_is_missing() {
        let root = write_test_bundle("pramaan-missing-artifact-test");

        fs::remove_file(root.join("artifact.json")).expect("remove artifact");
        let error = verify_bundle(&root).expect_err("missing artifact should fail");
        assert!(error.to_string().contains("I/O error"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn build_manifest_fails_when_receipt_declares_missing_file_artifact() {
        let root = std::env::temp_dir().join(format!(
            "pramaan-missing-declared-artifact-test-{}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("clean temp bundle");
        }
        fs::create_dir_all(root.join("receipts")).expect("receipt dir");

        let receipt = Receipt::synthetic(
            "claim_scope",
            StageStatus::Passed,
            "HEAD",
            "HEAD",
            vec![OutputRef {
                name: "missing_artifact".to_string(),
                path: "missing.json".to_string(),
                digest: None,
            }],
            vec![],
            ReceiptSummary {
                title: "Synthetic".to_string(),
                details: "Synthetic receipt.".to_string(),
            },
            RiskRefs::sample(),
        );
        fs::write(
            root.join("receipts").join("claim.receipt.json"),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let error = build_manifest(&root, BundleBuildOptions::synthetic("HEAD", "HEAD"))
            .expect_err("missing declared artifact should fail");
        assert!(error
            .to_string()
            .contains("receipt-declared artifact missing.json was not found"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn bundle_verification_rejects_manifest_path_escape() {
        let root = write_test_bundle("pramaan-path-escape-test");
        let manifest_path = root.join(MANIFEST_FILE_NAME);
        let mut manifest = read_manifest(&manifest_path).expect("read manifest");
        manifest.artifacts[0].path = "../artifact.json".to_string();
        write_manifest(&root, &manifest).expect("write invalid manifest");

        let error = verify_bundle(&root).expect_err("path escape should fail");
        assert!(error
            .to_string()
            .contains("must be a relative path inside the bundle"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn build_manifest_rejects_duplicate_basename_artifact_resolution() {
        let root = std::env::temp_dir().join(format!(
            "pramaan-duplicate-basename-test-{}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("clean temp bundle");
        }
        fs::create_dir_all(root.join("receipts")).expect("receipt dir");
        fs::create_dir_all(root.join("left")).expect("left dir");
        fs::create_dir_all(root.join("right")).expect("right dir");
        fs::write(root.join("left").join("output.json"), br#"{"left":true}"#).expect("left");
        fs::write(root.join("right").join("output.json"), br#"{"right":true}"#).expect("right");

        let receipt = Receipt::synthetic(
            "claim_scope",
            StageStatus::Passed,
            "HEAD",
            "HEAD",
            vec![OutputRef {
                name: "ambiguous_artifact".to_string(),
                path: "output.json".to_string(),
                digest: None,
            }],
            vec![],
            ReceiptSummary {
                title: "Synthetic".to_string(),
                details: "Synthetic receipt.".to_string(),
            },
            RiskRefs::sample(),
        );
        fs::write(
            root.join("receipts").join("claim.receipt.json"),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let error = build_manifest(&root, BundleBuildOptions::synthetic("HEAD", "HEAD"))
            .expect_err("duplicate basename should fail");
        assert!(error.to_string().contains("ambiguous inside the bundle"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn bundle_verification_fails_after_signing_metadata_tamper() {
        let root = write_test_bundle("pramaan-signing-tamper-test");
        let manifest_path = root.join(MANIFEST_FILE_NAME);
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest_path).expect("read manifest"))
                .expect("manifest json");
        value["integrity"]["signing"]["note"] =
            serde_json::Value::String("tampered signing note".to_string());
        fs::write(
            &manifest_path,
            serde_json::to_vec_pretty(&value).expect("manifest bytes"),
        )
        .expect("write tampered manifest");

        let error = verify_bundle(&root).expect_err("signing metadata tamper should fail");
        assert!(error.to_string().contains("manifest digest mismatch"));

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
        receipt.plugin_permissions = Some(pramaan_core::PluginPermissions {
            may_emit_receipts: true,
            may_emit_artifacts: true,
            may_read_previous_receipts: false,
            may_modify_previous_receipts: false,
            may_modify_manifest: false,
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

    #[test]
    fn build_manifest_rejects_dangerous_plugin_permissions() {
        let root =
            std::env::temp_dir().join(format!("pramaan-plugin-trust-test-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).expect("clean temp bundle");
        }
        fs::create_dir_all(root.join("receipts")).expect("receipt dir");

        let mut receipt = Receipt::synthetic(
            "mutation_python",
            StageStatus::Passed,
            "base",
            "head",
            vec![],
            vec![],
            ReceiptSummary {
                title: "Malicious plugin pass".to_string(),
                details: "Plugin tried to claim broad mutation evidence.".to_string(),
            },
            RiskRefs::sample(),
        );
        receipt.plugin_identity = Some(PluginIdentity {
            name: "evil-mutator".to_string(),
            version: "0.0.1".to_string(),
            provenance: "untrusted-github".to_string(),
            signature: None,
            sandbox_boundary: "none".to_string(),
        });
        receipt.plugin_permissions = Some(pramaan_core::PluginPermissions {
            may_emit_receipts: true,
            may_emit_artifacts: true,
            may_read_previous_receipts: true,
            may_modify_previous_receipts: true,
            may_modify_manifest: true,
        });
        fs::write(
            root.join("receipts").join("evil.receipt.json"),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let error = build_manifest(&root, BundleBuildOptions::synthetic("base", "head"))
            .expect_err("dangerous plugin receipt should fail");
        assert!(error.to_string().contains("plugin trust validation"));

        fs::remove_dir_all(&root).expect("cleanup temp bundle");
    }

    #[test]
    fn checked_in_receipt_and_bundle_fixtures_are_serde_compatible() {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root")
            .to_path_buf();
        let examples = workspace.join("examples");
        let mut receipt_paths = Vec::new();
        collect_by_suffix(&examples, ".receipt.json", &mut receipt_paths);
        assert!(!receipt_paths.is_empty(), "expected checked-in receipts");

        for path in receipt_paths {
            let bytes = fs::read(&path).expect("read receipt fixture");
            let receipt: Receipt = serde_json::from_slice(&bytes)
                .unwrap_or_else(|error| panic!("{} should parse: {error}", path.display()));
            assert_eq!(receipt.schema_version, pramaan_core::RECEIPT_SCHEMA_VERSION);
        }

        let bundle_fixture = examples.join("fixtures").join("bundle.synthetic.json");
        let bytes = fs::read(&bundle_fixture).expect("read bundle fixture");
        let manifest: BundleManifest =
            serde_json::from_slice(&bytes).expect("bundle fixture should parse");
        assert_eq!(manifest.schema_version, BUNDLE_SCHEMA_VERSION);
        assert!(!manifest.agent_attribution.is_empty());
    }

    fn write_test_bundle(prefix: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("{prefix}-{}", std::process::id()));
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
        fs::write(
            root.join("receipts").join("claim.receipt.json"),
            serde_json::to_vec_pretty(&receipt).expect("receipt json"),
        )
        .expect("write receipt");

        let manifest =
            build_manifest(&root, BundleBuildOptions::synthetic("HEAD", "HEAD")).expect("manifest");
        write_manifest(&root, &manifest).expect("write manifest");
        verify_bundle(&root).expect("valid test bundle verifies");
        root
    }

    fn collect_by_suffix(root: &Path, suffix: &str, found: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(root) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_by_suffix(&path, suffix, found);
            } else if path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(suffix))
            {
                found.push(path);
            }
        }
    }
}
