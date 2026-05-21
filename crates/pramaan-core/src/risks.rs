//! Canonical Pramaan risk-ID registry.
//!
//! Every risk ID emitted into a receipt or bundle MUST appear in
//! [`KNOWN_RISK_IDS`]. Named constants below give compile-time safety for the
//! IDs referenced as semantic literals from production code; bulk replacement
//! of remaining literals can layer in over time.

/// Every risk ID currently defined by the Pramaan taxonomy.
///
/// Keep aligned with `schemas/risk_taxonomy.schema.json` and the family ranges
/// in [`crate::risk_family`].
pub const KNOWN_RISK_IDS: &[&str] = &[
    "R-001", "R-002", "R-003", "R-004", "R-005", "R-006", "R-007", "R-008", "R-009", "R-010",
    "R-011", "R-012", "R-013", "R-014", "R-015", "R-016", "R-017", "R-018", "R-019", "R-020",
    "R-021", "R-022", "R-023", "R-024", "R-025", "R-026", "R-027", "R-028", "R-029", "R-030",
    "R-031", "R-032", "R-033", "R-034", "R-035", "R-036", "R-037", "R-038", "R-039", "R-040",
    "R-041", "R-042", "R-043", "R-044", "R-045", "R-046", "R-047", "R-048", "R-049", "R-050",
    "R-051", "R-052", "R-053", "R-054", "R-055", "R-056", "R-057", "R-058", "R-059", "R-060",
    "R-061", "R-062", "R-063", "R-064", "R-065", "R-066", "R-067", "R-068", "R-069", "R-070",
    "R-071", "R-072", "R-073", "R-074", "R-075", "R-076", "R-077", "R-078", "R-079", "R-080",
    "R-081", "R-082", "R-083", "R-084", "R-085", "R-086", "R-087", "R-088", "R-089", "R-090",
    "R-091", "R-092", "R-093", "R-094", "R-095", "R-096", "R-097", "R-098", "R-099", "R-100",
];

/// Returns true if `id` is a registered Pramaan risk identifier.
pub fn is_known_risk_id(id: &str) -> bool {
    KNOWN_RISK_IDS.contains(&id)
}

// ---------------------------------------------------------------------------
// Claim-scope (R-001..R-010)
// ---------------------------------------------------------------------------
pub const CLAIM_SCOPE_NO_PR_METADATA: &str = "R-001";
pub const CLAIM_SCOPE_LOW_CONFIDENCE: &str = "R-002";
pub const CLAIM_SCOPE_SYNTHETIC_SAMPLE: &str = "R-003";
pub const CLAIM_SCOPE_PUBLIC_API_DETECTION_FAILED: &str = "R-004";
pub const CLAIM_SCOPE_API_NOT_MENTIONED: &str = "R-007";
pub const CLAIM_SCOPE_CHANGED_ARTIFACT_NOT_MENTIONED: &str = "R-008";

// ---------------------------------------------------------------------------
// Oracle integrity (R-011..R-020) and parser residuals
// ---------------------------------------------------------------------------
pub const ORACLE_DELETED_TEST: &str = "R-011";
pub const ORACLE_SKIPPED_TEST: &str = "R-012";
pub const ORACLE_REMOVED_ERROR_PATH: &str = "R-013";
pub const ORACLE_WEAKENED_ASSERTION: &str = "R-014";
pub const ORACLE_DOWNGRADED_ASSERTION: &str = "R-015";
pub const ORACLE_LOOSENED_EXPECTATION: &str = "R-016";
pub const ORACLE_SENSITIVE_ARTIFACT_CHANGED: &str = "R-017";
pub const ORACLE_CASE_REDUCTION: &str = "R-018";
pub const ORACLE_BOUNDARY_REMOVED: &str = "R-019";
pub const ORACLE_INTEGRITY_FAILED: &str = "R-020";

// ---------------------------------------------------------------------------
// Sandbox and environment evidence (R-021..R-034)
// ---------------------------------------------------------------------------
pub const SANDBOX_WORKTREE_CREATED: &str = "R-021";
pub const SANDBOX_BASE_REF_CAPTURED: &str = "R-022";
pub const SANDBOX_HEAD_REF_CAPTURED: &str = "R-023";
pub const SANDBOX_ENV_CAPTURED: &str = "R-024";
pub const SANDBOX_TOOLCHAIN_CAPTURED: &str = "R-025";
pub const SANDBOX_LOCKFILE_CAPTURED: &str = "R-026";
pub const SANDBOX_SOURCE_STATE_CAPTURED: &str = "R-027";
pub const SANDBOX_DIRTY_STATE: &str = "R-028";
pub const SANDBOX_UNTRACKED_STATE: &str = "R-029";
pub const SANDBOX_CONTAINER_IDENTITY: &str = "R-030";

// ---------------------------------------------------------------------------
// Static / hallucination (R-031..R-040)
// ---------------------------------------------------------------------------
pub const STATIC_CHECK_BASELINE_TYPE: &str = "R-031";
pub const STATIC_CHECK_BASELINE_LINT: &str = "R-032";
pub const STATIC_CHECK_FAILED: &str = "R-038";
pub const STATIC_CHECK_SECURITY_SENSITIVE: &str = "R-039";
pub const STATIC_CHECK_RELAXED_CONFIG: &str = "R-040";

// ---------------------------------------------------------------------------
// Residual review/release risks
// ---------------------------------------------------------------------------
pub const REVIEWER_OVERRIDE_REQUIRED: &str = "R-049";
pub const RELEASE_ARTIFACT_MISSING: &str = "R-057";

// ---------------------------------------------------------------------------
// Mutation quality (R-061..R-070) and adjacent property/fuzz risks
// ---------------------------------------------------------------------------
pub const MUTATION_SURVIVED: &str = "R-068";
pub const MUTATION_BELOW_KILL_THRESHOLD: &str = "R-069";
pub const MUTATION_TIMEOUT: &str = "R-072";

// ---------------------------------------------------------------------------
// Property / fuzz (R-071..R-080)
// ---------------------------------------------------------------------------
pub const FUZZ_DETERMINISTIC_SIMULATED: &str = "R-073";
pub const FUZZ_EXPECTED_DIVERGENCE: &str = "R-074";
pub const FUZZ_UNEXPECTED_DIVERGENCE: &str = "R-075";
pub const FUZZ_NO_TOOL_BACKED_ADAPTER: &str = "R-077";
pub const FUZZ_REPLAYABLE_CASES: &str = "R-079";
pub const FUZZ_DIVERGENCE_NEEDS_REVIEW: &str = "R-080";
pub const FORMAL_NOT_APPLICABLE: &str = "R-081";

// ---------------------------------------------------------------------------
// Parser, bundle, and attestation residuals
// ---------------------------------------------------------------------------
pub const ORACLE_FULL_AST_NOT_AVAILABLE: &str = "R-087";
pub const ORACLE_SNAPSHOT_REVIEW_REQUIRED: &str = "R-088";
pub const ORACLE_ARTIFACT_REVIEW_REQUIRED: &str = "R-089";
pub const BUNDLE_LOCAL_ONLY_ATTESTATION: &str = "R-090";
pub const BUNDLE_UNSIGNED: &str = "R-092";

// ---------------------------------------------------------------------------
// CI / agentic workflow supply chain (R-091..R-095)
// ---------------------------------------------------------------------------
pub const AGENTIC_WORKFLOW_INJECTION: &str = "R-093";
pub const VERIFIER_SURFACE_CHANGED: &str = "R-094";
pub const VERIFIER_STAGE_LAUNDERING: &str = "R-095";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn known_risk_ids_are_sorted_and_unique() {
        let mut ids = KNOWN_RISK_IDS.to_vec();
        let original = ids.clone();
        ids.sort();
        ids.dedup();
        assert_eq!(ids, original, "KNOWN_RISK_IDS must be sorted and unique");
    }

    #[test]
    fn known_risk_ids_match_taxonomy_pattern() {
        for id in KNOWN_RISK_IDS {
            assert!(
                id.len() == 5
                    && id.starts_with("R-")
                    && id[2..].chars().all(|c| c.is_ascii_digit()),
                "risk id {id} must match R-NNN"
            );
        }
    }

    #[test]
    fn every_named_constant_is_known() {
        for id in [
            CLAIM_SCOPE_NO_PR_METADATA,
            CLAIM_SCOPE_LOW_CONFIDENCE,
            CLAIM_SCOPE_SYNTHETIC_SAMPLE,
            CLAIM_SCOPE_PUBLIC_API_DETECTION_FAILED,
            CLAIM_SCOPE_API_NOT_MENTIONED,
            CLAIM_SCOPE_CHANGED_ARTIFACT_NOT_MENTIONED,
            ORACLE_DELETED_TEST,
            ORACLE_SKIPPED_TEST,
            ORACLE_REMOVED_ERROR_PATH,
            ORACLE_WEAKENED_ASSERTION,
            ORACLE_DOWNGRADED_ASSERTION,
            ORACLE_LOOSENED_EXPECTATION,
            ORACLE_SENSITIVE_ARTIFACT_CHANGED,
            ORACLE_CASE_REDUCTION,
            ORACLE_BOUNDARY_REMOVED,
            ORACLE_INTEGRITY_FAILED,
            SANDBOX_WORKTREE_CREATED,
            SANDBOX_BASE_REF_CAPTURED,
            SANDBOX_HEAD_REF_CAPTURED,
            SANDBOX_ENV_CAPTURED,
            SANDBOX_TOOLCHAIN_CAPTURED,
            SANDBOX_LOCKFILE_CAPTURED,
            SANDBOX_SOURCE_STATE_CAPTURED,
            SANDBOX_DIRTY_STATE,
            SANDBOX_UNTRACKED_STATE,
            SANDBOX_CONTAINER_IDENTITY,
            STATIC_CHECK_BASELINE_TYPE,
            STATIC_CHECK_BASELINE_LINT,
            STATIC_CHECK_FAILED,
            STATIC_CHECK_SECURITY_SENSITIVE,
            STATIC_CHECK_RELAXED_CONFIG,
            REVIEWER_OVERRIDE_REQUIRED,
            RELEASE_ARTIFACT_MISSING,
            MUTATION_SURVIVED,
            MUTATION_BELOW_KILL_THRESHOLD,
            MUTATION_TIMEOUT,
            FUZZ_DETERMINISTIC_SIMULATED,
            FUZZ_EXPECTED_DIVERGENCE,
            FUZZ_UNEXPECTED_DIVERGENCE,
            FUZZ_NO_TOOL_BACKED_ADAPTER,
            FUZZ_REPLAYABLE_CASES,
            FUZZ_DIVERGENCE_NEEDS_REVIEW,
            FORMAL_NOT_APPLICABLE,
            ORACLE_FULL_AST_NOT_AVAILABLE,
            ORACLE_SNAPSHOT_REVIEW_REQUIRED,
            ORACLE_ARTIFACT_REVIEW_REQUIRED,
            BUNDLE_LOCAL_ONLY_ATTESTATION,
            BUNDLE_UNSIGNED,
            AGENTIC_WORKFLOW_INJECTION,
            VERIFIER_SURFACE_CHANGED,
            VERIFIER_STAGE_LAUNDERING,
        ] {
            assert!(
                is_known_risk_id(id),
                "named constant {id} not in KNOWN_RISK_IDS"
            );
        }
    }

    #[test]
    fn phase_33_adversarial_corpus_has_required_coverage() {
        let corpus: serde_json::Value = serde_json::from_str(include_str!(
            "../../../corpus/adversarial-scenarios-v0.1.json"
        ))
        .expect("phase 33 adversarial corpus parses");
        assert_eq!(
            corpus["schema_version"], "pramaan.adversarial_corpus/v1",
            "corpus must use the v1 schema id"
        );
        let scenarios = corpus["scenarios"]
            .as_array()
            .expect("corpus scenarios must be an array");
        assert!(
            scenarios.len() >= 25,
            "phase 33 requires at least 25 scenarios"
        );

        let mut ids = BTreeSet::new();
        let mut secure_categories = BTreeSet::new();
        let mut adversary_models = BTreeSet::new();
        let mut categories = BTreeSet::new();
        for scenario in scenarios {
            let id = scenario["id"].as_str().expect("scenario id is required");
            assert!(ids.insert(id), "duplicate scenario id {id}");
            for field in [
                "name",
                "category",
                "failure_mode",
                "status",
                "language",
                "adversary_model",
                "severity",
                "base_change",
                "head_change",
                "ordinary_ci_expectation",
                "pramaan_expected_finding",
                "reviewer_explanation",
                "replay_command",
            ] {
                assert!(
                    scenario[field]
                        .as_str()
                        .is_some_and(|value| !value.is_empty()),
                    "{id} missing non-empty {field}"
                );
            }
            categories.insert(
                scenario["category"]
                    .as_str()
                    .expect("scenario category is required"),
            );
            adversary_models.insert(
                scenario["adversary_model"]
                    .as_str()
                    .expect("scenario adversary_model is required"),
            );
            if let Some(category) = scenario["secure_code_category"].as_str() {
                secure_categories.insert(category);
            }
            let risks = scenario["risk_ids"]
                .as_array()
                .expect("scenario risk_ids must be an array");
            assert!(!risks.is_empty(), "{id} must map to risk IDs");
            for risk in risks {
                let risk = risk.as_str().expect("risk id must be a string");
                assert!(is_known_risk_id(risk), "{id} uses unknown risk ID {risk}");
            }
        }

        for category in [
            "validation_removal",
            "authorization_weakening",
            "unsafe_deserialization",
            "injection_sanitization_removal",
            "crypto_misuse",
            "secret_exposure",
        ] {
            assert!(
                secure_categories.contains(category),
                "missing secure-code corpus category {category}"
            );
        }
        for adversary in [
            "careless_ai",
            "overfitted_ai",
            "malicious_pr",
            "malicious_ci",
            "compromised_plugin",
        ] {
            assert!(
                adversary_models.contains(adversary),
                "missing adversary model {adversary}"
            );
        }
        assert!(categories.contains("verifier_abuse"));
        assert!(categories.contains("ci_supply_chain"));
    }
}
