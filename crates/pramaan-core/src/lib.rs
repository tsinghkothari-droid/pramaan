use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const RECEIPT_SCHEMA_VERSION: &str = "pramaan.receipt.v1";
pub const CLAIM_SCOPE_SCHEMA_VERSION: &str = "pramaan.claim_scope.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Passed,
    Failed,
    Skipped,
    NotApplicable,
    TimedOut,
    Error,
}

impl StageStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::NotApplicable => "not_applicable",
            Self::TimedOut => "timed_out",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaticHallucinationCategory {
    BrokenImport,
    UndefinedSymbol,
}

impl StaticHallucinationCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrokenImport => "broken_import",
            Self::UndefinedSymbol => "undefined_symbol",
        }
    }
}

pub fn classify_static_hallucinations(output: &str) -> Vec<StaticHallucinationCategory> {
    let lower = output.to_lowercase();
    let mut categories = Vec::new();

    if lower.contains("modulenotfounderror")
        || lower.contains("importerror")
        || lower.contains("no module named")
        || lower.contains("unresolved import")
        || lower.contains("cannot find module")
        || lower.contains("failed to resolve")
        || lower.contains("unresolved module")
        || lower.contains("could not find")
    {
        categories.push(StaticHallucinationCategory::BrokenImport);
    }

    if lower.contains("nameerror")
        || lower.contains("undefined name")
        || lower.contains("cannot find name")
        || lower.contains("not found in this scope")
        || lower.contains("cannot find function")
        || lower.contains("cannot find value")
        || lower.contains("unresolved name")
    {
        categories.push(StaticHallucinationCategory::UndefinedSymbol);
    }

    categories.sort_by_key(|category| category.as_str());
    categories.dedup();
    categories
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolIdentity {
    pub name: String,
    pub version: String,
}

impl ToolIdentity {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputRef {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputRef {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptSummary {
    pub title: String,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub schema_version: String,
    pub stage: String,
    pub status: StageStatus,
    pub tool: ToolIdentity,
    pub started_at: String,
    pub ended_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    pub inputs: Vec<InputRef>,
    pub outputs: Vec<OutputRef>,
    pub artifacts: Vec<ArtifactRef>,
    pub summary: ReceiptSummary,
    pub limitations: Vec<String>,
    pub mitigated_risks: Vec<String>,
    pub residual_risks: Vec<String>,
    pub not_applicable_risks: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl Receipt {
    #[allow(clippy::too_many_arguments)]
    pub fn synthetic(
        stage: impl Into<String>,
        status: StageStatus,
        base_ref: impl Into<String>,
        head_ref: impl Into<String>,
        outputs: Vec<OutputRef>,
        artifacts: Vec<ArtifactRef>,
        summary: ReceiptSummary,
        risks: RiskRefs,
    ) -> Self {
        let now = timestamp(Utc::now());

        Self {
            schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
            stage: stage.into(),
            status,
            tool: ToolIdentity::new("pramaan-cli", env!("CARGO_PKG_VERSION")),
            started_at: now.clone(),
            ended_at: now,
            exit_code: Some(0),
            inputs: vec![
                InputRef {
                    name: "base".to_string(),
                    value: base_ref.into(),
                    digest: None,
                },
                InputRef {
                    name: "head".to_string(),
                    value: head_ref.into(),
                    digest: None,
                },
            ],
            outputs,
            artifacts,
            summary,
            limitations: vec![
                "Synthetic Phase 1 receipt only; no repository checks were executed.".to_string(),
                "Risk IDs are sample references used to verify the receipt contract.".to_string(),
            ],
            mitigated_risks: risks.mitigated,
            residual_risks: risks.residual,
            not_applicable_risks: risks.not_applicable,
            metadata: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskRefs {
    pub mitigated: Vec<String>,
    pub residual: Vec<String>,
    pub not_applicable: Vec<String>,
}

impl RiskRefs {
    pub fn sample() -> Self {
        Self {
            mitigated: vec!["R-001".to_string(), "R-014".to_string()],
            residual: vec!["R-049".to_string(), "R-057".to_string()],
            not_applicable: vec!["R-081".to_string()],
        }
    }

    pub fn claim_scope_sample() -> Self {
        Self {
            mitigated: vec!["R-003".to_string()],
            residual: vec!["R-090".to_string()],
            not_applicable: vec!["R-081".to_string()],
        }
    }
}

pub fn risk_family(risk_id: &str) -> &'static str {
    match risk_id {
        "R-001" | "R-002" | "R-003" | "R-004" | "R-005" | "R-006" | "R-007" | "R-008" | "R-009"
        | "R-010" => "claim_scope",
        "R-011" | "R-012" | "R-013" | "R-014" | "R-015" | "R-016" | "R-017" | "R-018" | "R-019"
        | "R-020" => "oracle_integrity",
        "R-021" | "R-022" | "R-023" | "R-024" | "R-025" | "R-026" | "R-027" | "R-028" | "R-029"
        | "R-030" => "sandbox_reproducibility",
        "R-031" | "R-032" | "R-033" | "R-034" | "R-035" | "R-036" | "R-037" | "R-038" | "R-039"
        | "R-040" => "static_hallucination",
        "R-041" | "R-042" | "R-043" | "R-044" | "R-045" | "R-046" | "R-047" | "R-048" | "R-049"
        | "R-050" => "public_api_compatibility",
        "R-051" | "R-052" | "R-053" | "R-054" | "R-055" | "R-056" | "R-057" | "R-058" | "R-059"
        | "R-060" => "runtime_behavior",
        "R-061" | "R-062" | "R-063" | "R-064" | "R-065" | "R-066" | "R-067" | "R-068" | "R-069"
        | "R-070" => "mutation_quality",
        "R-071" | "R-072" | "R-073" | "R-074" | "R-075" | "R-076" | "R-077" | "R-078" | "R-079"
        | "R-080" => "property_fuzz",
        "R-081" | "R-082" | "R-083" | "R-084" | "R-085" | "R-086" | "R-087" | "R-088" | "R-089"
        | "R-090" => "bundle_integrity",
        "R-091" | "R-092" | "R-093" | "R-094" | "R-095" => "ci_supply_chain",
        "R-096" | "R-097" | "R-098" | "R-099" | "R-100" => "demo_corpus",
        _ => "unknown",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub kind: String,
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimScope {
    pub schema_version: String,
    pub source_refs: Vec<SourceRef>,
    pub expected_behavior: Vec<String>,
    pub out_of_scope_behavior: Vec<String>,
    pub touched_public_apis: Vec<String>,
    pub extraction_method: String,
    pub confidence: ClaimConfidence,
    pub limitations: Vec<String>,
}

impl ClaimScope {
    pub fn synthetic(base_ref: impl Into<String>, head_ref: impl Into<String>) -> Self {
        Self {
            schema_version: CLAIM_SCOPE_SCHEMA_VERSION.to_string(),
            source_refs: vec![
                SourceRef {
                    kind: "base_ref".to_string(),
                    reference: base_ref.into(),
                },
                SourceRef {
                    kind: "head_ref".to_string(),
                    reference: head_ref.into(),
                },
            ],
            expected_behavior: vec![
                "Phase 1 synthetic verification records the requested base and head refs."
                    .to_string(),
                "Future stages can replace this claim with PR title/body and issue evidence."
                    .to_string(),
            ],
            out_of_scope_behavior: vec![
                "No real sandbox, static, oracle, mutation, or fuzz checks run in Plan 02."
                    .to_string(),
            ],
            touched_public_apis: vec!["pramaan verify --base --head --out".to_string()],
            extraction_method: "synthetic_cli_arguments".to_string(),
            confidence: ClaimConfidence::Medium,
            limitations: vec![
                "Claim scope is generated from CLI arguments only for Phase 1.".to_string(),
            ],
        }
    }
}

pub fn timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_serializes_contract_fields() {
        let receipt = Receipt::synthetic(
            "synthetic",
            StageStatus::Passed,
            "HEAD~1",
            "HEAD",
            vec![OutputRef {
                name: "receipt".to_string(),
                path: "receipts/synthetic.receipt.json".to_string(),
                digest: None,
            }],
            vec![],
            ReceiptSummary {
                title: "Synthetic stage passed".to_string(),
                details: "No real checks executed.".to_string(),
            },
            RiskRefs::sample(),
        );

        let value = serde_json::to_value(receipt).expect("receipt serializes");

        assert_eq!(value["schema_version"], RECEIPT_SCHEMA_VERSION);
        assert_eq!(value["status"], "passed");
        assert_eq!(value["mitigated_risks"][0], "R-001");
        assert_eq!(value["residual_risks"][0], "R-049");
        assert_eq!(value["not_applicable_risks"][0], "R-081");
    }

    #[test]
    fn claim_scope_serializes_contract_fields() {
        let value = serde_json::to_value(ClaimScope::synthetic("main", "feature"))
            .expect("claim scope serializes");

        assert_eq!(value["schema_version"], CLAIM_SCOPE_SCHEMA_VERSION);
        assert_eq!(value["confidence"], "medium");
        assert_eq!(
            value["touched_public_apis"][0],
            "pramaan verify --base --head --out"
        );
    }

    #[test]
    fn example_fixtures_use_expected_schema_versions_and_risk_ids() {
        let fixtures = [
            (
                include_str!("../../../examples/fixtures/receipt.synthetic.json"),
                "pramaan.receipt.v1",
            ),
            (
                include_str!("../../../examples/fixtures/claim_scope.synthetic.json"),
                "pramaan.claim_scope.v1",
            ),
            (
                include_str!("../../../examples/fixtures/risk_taxonomy.synthetic.json"),
                "pramaan.risk_taxonomy.v1",
            ),
        ];

        for (raw, schema_version) in fixtures {
            let value: serde_json::Value = serde_json::from_str(raw).expect("fixture parses");

            assert_eq!(value["schema_version"], schema_version);
            assert_no_correctness_claims(&value);
            assert_risk_ids_are_stable(&value);
        }
    }

    #[test]
    fn known_risk_ids_map_to_families() {
        assert_eq!(risk_family("R-001"), "claim_scope");
        assert_eq!(risk_family("R-011"), "oracle_integrity");
        assert_eq!(risk_family("R-038"), "static_hallucination");
        assert_eq!(risk_family("R-090"), "bundle_integrity");
        assert_eq!(risk_family("R-999"), "unknown");
    }

    #[test]
    fn static_hallucination_classifier_finds_imports_and_symbols() {
        let categories = classify_static_hallucinations(
            "ModuleNotFoundError: No module named 'ghost'\nerror[E0425]: cannot find value `x` in this scope",
        );

        assert_eq!(
            categories,
            vec![
                StaticHallucinationCategory::BrokenImport,
                StaticHallucinationCategory::UndefinedSymbol
            ]
        );
    }

    fn assert_no_correctness_claims(value: &serde_json::Value) {
        let text = value.to_string().to_lowercase();
        assert!(
            !text.contains("definitely correct"),
            "fixture must not claim correctness"
        );
    }

    fn assert_risk_ids_are_stable(value: &serde_json::Value) {
        match value {
            serde_json::Value::String(text) if text.starts_with("R-") => {
                assert_eq!(text.len(), 5, "risk id should look like R-001");
                assert!(text[2..]
                    .chars()
                    .all(|character| character.is_ascii_digit()));
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    assert_risk_ids_are_stable(item);
                }
            }
            serde_json::Value::Object(entries) => {
                for item in entries.values() {
                    assert_risk_ids_are_stable(item);
                }
            }
            _ => {}
        }
    }
}
