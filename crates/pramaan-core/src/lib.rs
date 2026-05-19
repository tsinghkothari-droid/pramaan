use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

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
    InventedApi,
    InvalidParameter,
    LogicMismatch,
    NonexistentImport,
    ResourceMismatch,
    Unknown,
    UndefinedSymbol,
}

impl StaticHallucinationCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrokenImport => "broken_import",
            Self::InventedApi => "invented_api",
            Self::InvalidParameter => "invalid_parameter",
            Self::LogicMismatch => "logic_mismatch",
            Self::NonexistentImport => "nonexistent_import",
            Self::ResourceMismatch => "resource_mismatch",
            Self::Unknown => "unknown",
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

    if lower.contains("no module named")
        || lower.contains("cannot find module")
        || lower.contains("unresolved import")
        || lower.contains("failed to resolve")
        || lower.contains("unresolved module")
    {
        categories.push(StaticHallucinationCategory::NonexistentImport);
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

    if lower.contains("no method named")
        || lower.contains("has no attribute")
        || lower.contains("property does not exist")
        || lower.contains("method does not exist")
        || lower.contains("unknown field")
        || lower.contains("no field")
    {
        categories.push(StaticHallucinationCategory::InventedApi);
    }

    if lower.contains("unexpected keyword argument")
        || lower.contains("missing required")
        || lower.contains("required positional argument")
        || lower.contains("incorrect number of arguments")
        || lower.contains("expected ")
            && (lower.contains(" arguments") || lower.contains(" argument"))
    {
        categories.push(StaticHallucinationCategory::InvalidParameter);
    }

    if lower.contains("file not found")
        || lower.contains("no such file or directory")
        || lower.contains("couldn't read")
        || lower.contains("could not read")
        || lower.contains("resource not found")
    {
        categories.push(StaticHallucinationCategory::ResourceMismatch);
    }

    if lower.contains("assertion failed")
        || lower.contains("assertion `left")
        || lower.contains("assert.strict")
        || lower.contains("expected:") && (lower.contains("actual:") || lower.contains("received:"))
    {
        categories.push(StaticHallucinationCategory::LogicMismatch);
    }

    if output.trim().is_empty() {
        categories.push(StaticHallucinationCategory::Unknown);
    }

    categories.sort_by_key(|category| category.as_str());
    categories.dedup();
    categories
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleLanguage {
    Python,
    TypeScript,
}

impl OracleLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleTestCase {
    pub language: OracleLanguage,
    pub path: String,
    pub name: String,
    pub stable_id: String,
    pub fingerprint: String,
    pub assertion_count: usize,
    pub parametrized_case_count: usize,
    pub skipped: bool,
    pub skip_reason: Option<String>,
    pub signal_tokens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleSensitiveArtifact {
    pub path: String,
    pub kind: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleSnapshot {
    pub root: String,
    pub tests: Vec<OracleTestCase>,
    pub artifacts: Vec<OracleSensitiveArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleDiff {
    pub base: OracleSnapshot,
    pub head: OracleSnapshot,
    pub findings: Vec<OracleFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleFindingKind {
    DeletedTest,
    AddedSkip,
    ParametrizedCaseReduction,
    WeakenedAssertion,
    SensitiveArtifactChanged,
    SensitiveArtifactDeleted,
}

impl OracleFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeletedTest => "deleted_test",
            Self::AddedSkip => "added_skip",
            Self::ParametrizedCaseReduction => "parametrized_case_reduction",
            Self::WeakenedAssertion => "weakened_assertion",
            Self::SensitiveArtifactChanged => "sensitive_artifact_changed",
            Self::SensitiveArtifactDeleted => "sensitive_artifact_deleted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleFinding {
    pub kind: OracleFindingKind,
    pub path: String,
    pub test_name: Option<String>,
    pub details: String,
    pub risk_ids: Vec<String>,
}

pub fn discover_oracle_snapshot(root: &Path) -> std::io::Result<OracleSnapshot> {
    let root = root.canonicalize()?;
    let mut tests = Vec::new();
    let mut artifacts = Vec::new();

    for path in walk_oracle_files(&root)? {
        let relative = portable_relative_path(&root, &path);
        if is_sensitive_artifact(&path, &relative) {
            let bytes = fs::read(&path)?;
            artifacts.push(OracleSensitiveArtifact {
                path: relative,
                kind: sensitive_artifact_kind(&path).to_string(),
                fingerprint: stable_hash_bytes(&bytes),
            });
            continue;
        }

        let Some(language) = test_language(&path, &relative) else {
            continue;
        };
        let text = fs::read_to_string(&path)?;
        tests.extend(match language {
            OracleLanguage::Python => discover_python_tests(&relative, &text),
            OracleLanguage::TypeScript => discover_typescript_tests(&relative, &text),
        });
    }

    tests.sort_by(|left, right| left.stable_id.cmp(&right.stable_id));
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));

    Ok(OracleSnapshot {
        root: root.to_string_lossy().replace('\\', "/"),
        tests,
        artifacts,
    })
}

pub fn diff_oracle_snapshots(base: OracleSnapshot, head: OracleSnapshot) -> OracleDiff {
    let mut findings = Vec::new();
    let head_tests = head
        .tests
        .iter()
        .map(|test| (test.stable_id.clone(), test))
        .collect::<BTreeMap<_, _>>();

    for base_test in &base.tests {
        let Some(head_test) = head_tests.get(&base_test.stable_id) else {
            findings.push(OracleFinding {
                kind: OracleFindingKind::DeletedTest,
                path: base_test.path.clone(),
                test_name: Some(base_test.name.clone()),
                details: "Test existed in base but was absent in head.".to_string(),
                risk_ids: vec![
                    "R-011".to_string(),
                    "R-012".to_string(),
                    "R-087".to_string(),
                ],
            });
            continue;
        };

        if !base_test.skipped && head_test.skipped {
            findings.push(OracleFinding {
                kind: OracleFindingKind::AddedSkip,
                path: head_test.path.clone(),
                test_name: Some(head_test.name.clone()),
                details: head_test.skip_reason.clone().unwrap_or_else(|| {
                    "Skip, xfail, todo, or equivalent marker was added.".to_string()
                }),
                risk_ids: vec![
                    "R-012".to_string(),
                    "R-013".to_string(),
                    "R-087".to_string(),
                ],
            });
        }

        if head_test.parametrized_case_count < base_test.parametrized_case_count {
            findings.push(OracleFinding {
                kind: OracleFindingKind::ParametrizedCaseReduction,
                path: head_test.path.clone(),
                test_name: Some(head_test.name.clone()),
                details: format!(
                    "Parametrized cases reduced from {} to {}.",
                    base_test.parametrized_case_count, head_test.parametrized_case_count
                ),
                risk_ids: vec![
                    "R-018".to_string(),
                    "R-019".to_string(),
                    "R-087".to_string(),
                ],
            });
        }

        if assertion_weakened(base_test, head_test) {
            findings.push(OracleFinding {
                kind: OracleFindingKind::WeakenedAssertion,
                path: head_test.path.clone(),
                test_name: Some(head_test.name.clone()),
                details: format!(
                    "Assertion signal weakened: assertions {} -> {}, tokens [{}] -> [{}].",
                    base_test.assertion_count,
                    head_test.assertion_count,
                    base_test.signal_tokens.join(","),
                    head_test.signal_tokens.join(",")
                ),
                risk_ids: vec![
                    "R-014".to_string(),
                    "R-015".to_string(),
                    "R-016".to_string(),
                    "R-020".to_string(),
                    "R-087".to_string(),
                ],
            });
        }
    }

    let head_artifacts = head
        .artifacts
        .iter()
        .map(|artifact| (artifact.path.clone(), artifact))
        .collect::<BTreeMap<_, _>>();

    for base_artifact in &base.artifacts {
        match head_artifacts.get(&base_artifact.path) {
            Some(head_artifact) if head_artifact.fingerprint != base_artifact.fingerprint => {
                findings.push(OracleFinding {
                    kind: OracleFindingKind::SensitiveArtifactChanged,
                    path: head_artifact.path.clone(),
                    test_name: None,
                    details: format!(
                        "{} artifact changed and can redefine expected behavior.",
                        head_artifact.kind
                    ),
                    risk_ids: vec![
                        "R-008".to_string(),
                        "R-017".to_string(),
                        "R-088".to_string(),
                    ],
                });
            }
            None => findings.push(OracleFinding {
                kind: OracleFindingKind::SensitiveArtifactDeleted,
                path: base_artifact.path.clone(),
                test_name: None,
                details: format!(
                    "{} artifact was deleted and may remove oracle coverage.",
                    base_artifact.kind
                ),
                risk_ids: vec![
                    "R-008".to_string(),
                    "R-017".to_string(),
                    "R-089".to_string(),
                ],
            }),
            _ => {}
        }
    }

    findings.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.kind.as_str().cmp(right.kind.as_str()))
            .then_with(|| left.test_name.cmp(&right.test_name))
    });

    OracleDiff {
        base,
        head,
        findings,
    }
}

pub fn oracle_mitigated_risks() -> Vec<String> {
    (4..=20)
        .chain(87..=89)
        .map(|id| format!("R-{id:03}"))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationLanguage {
    Python,
    TypeScript,
    Rust,
}

impl MutationLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::Rust => "rust",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationSummary {
    pub language: MutationLanguage,
    pub tool: String,
    pub changed_files: Vec<String>,
    pub total: usize,
    pub killed: usize,
    pub survived: usize,
    pub timed_out: usize,
    pub unviable: usize,
    pub skipped: usize,
    pub review_survivors: usize,
    pub test_gap_survivors: usize,
    pub likely_equivalent_survivors: usize,
}

impl MutationSummary {
    pub fn empty(
        language: MutationLanguage,
        tool: impl Into<String>,
        changed_files: Vec<String>,
    ) -> Self {
        Self {
            language,
            tool: tool.into(),
            changed_files,
            total: 0,
            killed: 0,
            survived: 0,
            timed_out: 0,
            unviable: 0,
            skipped: 0,
            review_survivors: 0,
            test_gap_survivors: 0,
            likely_equivalent_survivors: 0,
        }
    }

    pub fn kill_rate_percent(&self) -> Option<u8> {
        let executable = self.killed + self.survived + self.timed_out;
        if executable == 0 {
            None
        } else {
            Some(((self.killed * 100) / executable) as u8)
        }
    }
}

pub fn mutation_mitigated_risks() -> Vec<String> {
    (68..=72).map(|id| format!("R-{id:03}")).collect()
}

pub fn classify_mutation_survivor(output: &str) -> &'static str {
    let lower = output.to_lowercase();
    if lower.contains("equivalent") || lower.contains("no behavioral change") {
        "likely_equivalent"
    } else if lower.contains("assert")
        || lower.contains("coverage")
        || lower.contains("not killed")
        || lower.contains("survived")
    {
        "test_gap"
    } else {
        "review"
    }
}

pub fn normalize_mutmut_output(output: &str, changed_files: Vec<String>) -> MutationSummary {
    let mut summary = MutationSummary::empty(MutationLanguage::Python, "mutmut", changed_files);
    for line in output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("survived") || lower.contains("survivor") {
            let count = numbers_in(line).into_iter().next().unwrap_or(1);
            summary.survived += count;
            match classify_mutation_survivor(line) {
                "likely_equivalent" => summary.likely_equivalent_survivors += count,
                "test_gap" => summary.test_gap_survivors += count,
                _ => summary.review_survivors += count,
            }
        } else if lower.contains("killed") {
            summary.killed += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("timeout") || lower.contains("timed out") {
            summary.timed_out += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("incompetent") || lower.contains("unviable") {
            summary.unviable += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("skipped") {
            summary.skipped += numbers_in(line).into_iter().next().unwrap_or(1);
        }
    }
    summary.total =
        summary.killed + summary.survived + summary.timed_out + summary.unviable + summary.skipped;
    summary
}

pub fn normalize_stryker_output(output: &str, changed_files: Vec<String>) -> MutationSummary {
    let mut summary =
        MutationSummary::empty(MutationLanguage::TypeScript, "stryker-js", changed_files);
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(output) {
        accumulate_stryker_json(&value, &mut summary);
    } else {
        for line in output.lines() {
            let lower = line.to_lowercase();
            if lower.contains("killed") {
                summary.killed += numbers_in(line).into_iter().next().unwrap_or(1);
            } else if lower.contains("survived") {
                let count = numbers_in(line).into_iter().next().unwrap_or(1);
                summary.survived += count;
                summary.test_gap_survivors += count;
            } else if lower.contains("timeout") {
                summary.timed_out += numbers_in(line).into_iter().next().unwrap_or(1);
            } else if lower.contains("no coverage") || lower.contains("ignored") {
                summary.skipped += numbers_in(line).into_iter().next().unwrap_or(1);
            } else if lower.contains("compile error") || lower.contains("runtime error") {
                summary.unviable += numbers_in(line).into_iter().next().unwrap_or(1);
            }
        }
    }
    summary.total =
        summary.killed + summary.survived + summary.timed_out + summary.unviable + summary.skipped;
    summary
}

pub fn normalize_cargo_mutants_output(output: &str, changed_files: Vec<String>) -> MutationSummary {
    let mut summary =
        MutationSummary::empty(MutationLanguage::Rust, "cargo-mutants", changed_files);
    for line in output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("caught") || lower.contains("killed") {
            summary.killed += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("missed") || lower.contains("survived") {
            let count = numbers_in(line).into_iter().next().unwrap_or(1);
            summary.survived += count;
            summary.test_gap_survivors += count;
        } else if lower.contains("timeout") || lower.contains("timed out") {
            summary.timed_out += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("unviable") || lower.contains("build failed") {
            summary.unviable += numbers_in(line).into_iter().next().unwrap_or(1);
        } else if lower.contains("skipped") {
            summary.skipped += numbers_in(line).into_iter().next().unwrap_or(1);
        }
    }
    summary.total =
        summary.killed + summary.survived + summary.timed_out + summary.unviable + summary.skipped;
    summary
}

fn accumulate_stryker_json(value: &serde_json::Value, summary: &mut MutationSummary) {
    match value {
        serde_json::Value::Object(entries) => {
            if let Some(status) = entries.get("status").and_then(serde_json::Value::as_str) {
                match status.to_ascii_lowercase().as_str() {
                    "killed" => summary.killed += 1,
                    "survived" => {
                        summary.survived += 1;
                        summary.test_gap_survivors += 1;
                    }
                    "timeout" | "timedout" => summary.timed_out += 1,
                    "compileerror" | "runtimeerror" => summary.unviable += 1,
                    "ignored" | "nocoverage" => summary.skipped += 1,
                    _ => {}
                }
            }
            for item in entries.values() {
                accumulate_stryker_json(item, summary);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                accumulate_stryker_json(item, summary);
            }
        }
        _ => {}
    }
}

fn numbers_in(line: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut current = String::new();
    for character in line.chars() {
        if character.is_ascii_digit() {
            current.push(character);
        } else if !current.is_empty() {
            if let Ok(value) = current.parse::<usize>() {
                numbers.push(value);
            }
            current.clear();
        }
    }
    if !current.is_empty() {
        if let Ok(value) = current.parse::<usize>() {
            numbers.push(value);
        }
    }
    numbers
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FuzzLanguage {
    Python,
    TypeScript,
}

impl FuzzLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::TypeScript => "typescript",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FuzzAdapterMode {
    Hypothesis,
    FastCheck,
    DeterministicSimulated,
}

impl FuzzAdapterMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hypothesis => "hypothesis",
            Self::FastCheck => "fast_check",
            Self::DeterministicSimulated => "deterministic_simulated",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PureFunctionCandidate {
    pub language: FuzzLanguage,
    pub path: String,
    pub name: String,
    pub stable_id: String,
    pub parameters: Vec<String>,
    pub return_expression: String,
    pub safety_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsafeFunctionCandidate {
    pub language: FuzzLanguage,
    pub path: String,
    pub name: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzDiscovery {
    pub root: String,
    pub safe_functions: Vec<PureFunctionCandidate>,
    pub unsafe_functions: Vec<UnsafeFunctionCandidate>,
    pub not_applicable_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DivergenceClassification {
    Expected,
    Unexpected,
    NeedsReview,
}

impl DivergenceClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expected => "expected",
            Self::Unexpected => "unexpected",
            Self::NeedsReview => "needs_review",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzInputCase {
    pub index: usize,
    pub values: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzDivergence {
    pub stable_id: String,
    pub function_name: String,
    pub path: String,
    pub input: FuzzInputCase,
    pub base_output: String,
    pub head_output: String,
    pub classification: DivergenceClassification,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzRunEvidence {
    pub schema_version: String,
    pub adapter: FuzzAdapterMode,
    pub seed: u64,
    pub generated_input_count: usize,
    pub corpus_hash: String,
    pub replay_path: String,
    pub example_database_path: Option<String>,
    pub counterexample_path: Option<String>,
    pub base_discovery: FuzzDiscovery,
    pub head_discovery: FuzzDiscovery,
    pub divergences: Vec<FuzzDivergence>,
    pub limitations: Vec<String>,
}

pub fn fuzz_mitigated_risks() -> Vec<String> {
    (73..=80).map(|id| format!("R-{id:03}")).collect()
}

pub fn fuzz_not_applicable_risks() -> Vec<String> {
    vec![
        "R-073".to_string(),
        "R-074".to_string(),
        "R-077".to_string(),
        "R-080".to_string(),
    ]
}

fn assertion_weakened(base: &OracleTestCase, head: &OracleTestCase) -> bool {
    if head.assertion_count < base.assertion_count {
        return true;
    }

    let base_tokens = base.signal_tokens.iter().cloned().collect::<BTreeSet<_>>();
    let head_tokens = head.signal_tokens.iter().cloned().collect::<BTreeSet<_>>();
    let strong_tokens = [
        "equals",
        "deep_equals",
        "comparison",
        "raises",
        "throws",
        "contains",
        "snapshot",
    ];

    strong_tokens
        .iter()
        .any(|token| base_tokens.contains(*token) && !head_tokens.contains(*token))
        || (!base_tokens.contains("truthy") && head_tokens.contains("truthy"))
        || (!base_tokens.contains("always_true") && head_tokens.contains("always_true"))
}

fn discover_python_tests(path: &str, text: &str) -> Vec<OracleTestCase> {
    let lines = text.lines().collect::<Vec<_>>();
    let mut tests = Vec::new();

    let definitions = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| python_test_name(line.trim_start()).map(|name| (index, name)))
        .collect::<Vec<_>>();

    for (position, (definition_index, name)) in definitions.iter().enumerate() {
        let start = python_decorator_start(&lines, *definition_index);

        let end = definitions
            .get(position + 1)
            .map(|(next_definition, _)| python_decorator_start(&lines, *next_definition))
            .unwrap_or(lines.len());

        if start < end {
            let block = lines[start..end].join("\n");
            tests.push(build_test_case(
                OracleLanguage::Python,
                path,
                name.clone(),
                &block,
            ));
        }
    }

    tests
}

fn python_decorator_start(lines: &[&str], definition_index: usize) -> usize {
    let mut start = definition_index;
    while start > 0 {
        let previous = lines[start - 1].trim_start();
        if previous.is_empty()
            || python_test_name(previous).is_some()
            || previous.starts_with("def ")
            || previous.starts_with("async def ")
        {
            break;
        }
        start -= 1;
    }
    start
}

fn discover_typescript_tests(path: &str, text: &str) -> Vec<OracleTestCase> {
    let mut tests = Vec::new();
    let lines = text.lines().collect::<Vec<_>>();
    let mut index = 0;

    while index < lines.len() {
        if let Some(name) = typescript_test_name(lines[index]) {
            let start = index;
            index += 1;
            while index < lines.len() && typescript_test_name(lines[index]).is_none() {
                index += 1;
            }
            let block = lines[start..index].join("\n");
            tests.push(build_test_case(
                OracleLanguage::TypeScript,
                path,
                name,
                &block,
            ));
        } else {
            index += 1;
        }
    }

    tests
}

fn build_test_case(
    language: OracleLanguage,
    path: &str,
    name: String,
    block: &str,
) -> OracleTestCase {
    let normalized = normalize_test_block(block);
    let skipped = skipped_test(language, block);

    OracleTestCase {
        language,
        path: path.to_string(),
        stable_id: format!("{}::{}", path.replace('\\', "/"), name),
        fingerprint: stable_hash_text(&format!("{}:{}:{normalized}", language.as_str(), name)),
        name,
        assertion_count: assertion_count(language, block),
        parametrized_case_count: parametrized_case_count(language, block),
        skipped,
        skip_reason: skipped.then(|| skip_reason(language, block)),
        signal_tokens: signal_tokens(language, block),
    }
}

fn python_test_name(line: &str) -> Option<String> {
    let line = line.strip_prefix("async ").unwrap_or(line);
    let rest = line.strip_prefix("def test_")?;
    let end = rest.find('(')?;
    Some(format!("test_{}", &rest[..end]))
}

fn typescript_test_name(line: &str) -> Option<String> {
    let compact = line.trim_start();
    let prefixes = [
        "test.skip(",
        "it.skip(",
        "test.todo(",
        "it.todo(",
        "test(",
        "it(",
    ];
    for prefix in prefixes {
        if let Some(rest) = compact.strip_prefix(prefix) {
            let quote = rest.chars().next()?;
            if quote == '\'' || quote == '"' || quote == '`' {
                if let Some(end) = rest[1..].find(quote) {
                    return Some(rest[1..1 + end].to_string());
                }
            }
        }
    }
    None
}

fn normalize_test_block(block: &str) -> String {
    block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn assertion_count(language: OracleLanguage, block: &str) -> usize {
    match language {
        OracleLanguage::Python => block
            .lines()
            .filter(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with("assert ")
                    || trimmed.contains("pytest.raises(")
                    || trimmed.contains(".assert")
            })
            .count(),
        OracleLanguage::TypeScript => {
            count_occurrences(block, "expect(")
                + count_occurrences(block, "assert.")
                + count_occurrences(block, "assert(")
        }
    }
}

fn parametrized_case_count(language: OracleLanguage, block: &str) -> usize {
    match language {
        OracleLanguage::Python => {
            if !block.contains("parametrize") {
                return 1;
            }
            block
                .matches("),")
                .count()
                .max(block.matches("],").count())
                .max(1)
        }
        OracleLanguage::TypeScript => {
            if block.contains("test.each") || block.contains("it.each") {
                block.matches("],").count().max(1)
            } else {
                1
            }
        }
    }
}

fn skipped_test(language: OracleLanguage, block: &str) -> bool {
    match language {
        OracleLanguage::Python => {
            block.contains("@pytest.mark.skip")
                || block.contains("@pytest.mark.xfail")
                || block.contains("pytest.skip(")
        }
        OracleLanguage::TypeScript => {
            block.contains("test.skip(")
                || block.contains("it.skip(")
                || block.contains("test.todo(")
                || block.contains("it.todo(")
        }
    }
}

fn skip_reason(language: OracleLanguage, block: &str) -> String {
    match language {
        OracleLanguage::Python => {
            if block.contains("xfail") {
                "pytest xfail marker added".to_string()
            } else {
                "pytest skip marker or runtime skip added".to_string()
            }
        }
        OracleLanguage::TypeScript => {
            if block.contains(".todo(") {
                "JS/TS todo test marker added".to_string()
            } else {
                "JS/TS skip test marker added".to_string()
            }
        }
    }
}

fn signal_tokens(language: OracleLanguage, block: &str) -> Vec<String> {
    let mut tokens = BTreeSet::new();
    let signal_text = match language {
        OracleLanguage::Python => block
            .lines()
            .map(str::trim_start)
            .filter(|line| {
                line.starts_with("assert ")
                    || line.contains("pytest.raises(")
                    || line.contains(".assert")
            })
            .collect::<Vec<_>>()
            .join("\n"),
        OracleLanguage::TypeScript => block.to_string(),
    };
    let lower = signal_text.to_lowercase();

    if lower.contains("assert true") || lower.contains("expect(true)") {
        tokens.insert("always_true".to_string());
    }
    if lower.contains("assert ") && !lower.contains("==") && !lower.contains("!=") {
        tokens.insert("truthy".to_string());
    }
    if lower.contains("tobetruthy")
        || lower.contains("tobedefined")
        || lower.contains("not.tobenull")
    {
        tokens.insert("truthy".to_string());
    }
    if lower.contains("==")
        || lower.contains("!=")
        || lower.contains("assertequal")
        || lower.contains("assertnotequal")
        || lower.contains(".tobe(")
        || lower.contains(".toequal(")
    {
        tokens.insert("equals".to_string());
    }
    if lower.contains("assertgreater")
        || lower.contains("assertless")
        || lower.contains("assertgreaterequal")
        || lower.contains("assertlessequal")
    {
        tokens.insert("comparison".to_string());
    }
    if lower.contains(".tostrictequal(") || lower.contains(".tomatchobject(") {
        tokens.insert("deep_equals".to_string());
    }
    if lower.contains('>')
        || lower.contains('<')
        || lower.contains(".tobegreater")
        || lower.contains(".tobeless")
    {
        tokens.insert("comparison".to_string());
    }
    if lower.contains(" in ") || lower.contains(".tocontain") || lower.contains(".tomatch(") {
        tokens.insert("contains".to_string());
    }
    if lower.contains("pytest.raises") {
        tokens.insert("raises".to_string());
    }
    if lower.contains("tothrow") || lower.contains("assert.throws") {
        tokens.insert("throws".to_string());
    }
    if lower.contains("snapshot") || lower.contains(".tomatchsnapshot(") {
        tokens.insert("snapshot".to_string());
    }
    if language == OracleLanguage::Python && lower.contains("pytest.approx") {
        tokens.insert("approximate".to_string());
    }

    tokens.into_iter().collect()
}

fn test_language(path: &Path, relative: &str) -> Option<OracleLanguage> {
    let file_name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
    if extension.eq_ignore_ascii_case("py")
        && (file_name.starts_with("test_") || file_name.ends_with("_test.py"))
    {
        return Some(OracleLanguage::Python);
    }

    let lower = relative.to_lowercase();
    if matches!(extension, "js" | "jsx" | "ts" | "tsx")
        && (lower.contains(".test.")
            || lower.contains(".spec.")
            || lower.contains("/__tests__/")
            || lower.contains("\\__tests__\\"))
    {
        return Some(OracleLanguage::TypeScript);
    }

    None
}

fn is_sensitive_artifact(path: &Path, relative: &str) -> bool {
    let lower = relative.to_lowercase();
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
    lower.contains("__snapshots__")
        || lower.ends_with(".snap")
        || lower.ends_with(".snapshot")
        || lower.starts_with("fixtures/")
        || lower.starts_with("fixtures\\")
        || lower.contains("/fixtures/")
        || lower.contains("\\fixtures\\")
        || matches!(extension, "snap" | "snapshot" | "golden")
}

fn sensitive_artifact_kind(path: &Path) -> &'static str {
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
    match extension {
        "snap" | "snapshot" => "snapshot",
        _ => "fixture",
    }
}

fn walk_oracle_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();

            if entry_path.is_dir() {
                if matches!(
                    name.as_ref(),
                    ".git" | "target" | "node_modules" | ".venv" | "venv" | "__pycache__"
                ) {
                    continue;
                }
                stack.push(entry_path);
            } else {
                files.push(entry_path);
            }
        }
    }

    Ok(files)
}

fn portable_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn count_occurrences(text: &str, needle: &str) -> usize {
    text.match_indices(needle).count()
}

fn stable_hash_text(text: &str) -> String {
    stable_hash_bytes(text.as_bytes())
}

fn stable_hash_bytes(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv64:{hash:016x}")
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
#[serde(rename_all = "snake_case")]
pub enum AttributionConfidence {
    Low,
    Medium,
    High,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentAttribution {
    pub product: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    pub execution_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_context_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_provenance: Option<String>,
    pub source: String,
    pub confidence: AttributionConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideDecision {
    ApprovedDespiteRisk,
    Rejected,
    NeedsFollowUp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewerOverride {
    pub decision: OverrideDecision,
    pub accepted_risk_ids: Vec<String>,
    pub reviewer_identity_source: String,
    pub timestamp: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_outcome: Option<String>,
    pub update_calibration: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentProvenanceEntry {
    pub role: String,
    pub agent: AgentAttribution,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginPermissions {
    pub may_emit_receipts: bool,
    pub may_emit_artifacts: bool,
    pub may_read_previous_receipts: bool,
    pub may_modify_previous_receipts: bool,
    pub may_modify_manifest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginIdentity {
    pub name: String,
    pub version: String,
    pub provenance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub sandbox_boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSensitivity {
    Public,
    Internal,
    SecretDerived,
    Redacted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionManifest {
    pub profile: String,
    pub redacted_fields: Vec<String>,
    pub hashed_fields: Vec<String>,
    pub policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub decision: String,
    pub policy_id: String,
    pub hard_failures: Vec<String>,
    pub warnings: Vec<String>,
    pub waived: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageBudget {
    pub target_ms: u64,
    pub max_ms: u64,
    pub consumed_ms: u64,
    pub exhausted: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_reason: Option<String>,
    pub partial_evidence: bool,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_author: Option<AgentAttribution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_override: Option<ReviewerOverride>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub multi_agent_provenance: Vec<AgentProvenanceEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_identity: Option<PluginIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_permissions: Option<PluginPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_sensitivity: Option<EvidenceSensitivity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_manifest: Option<RedactionManifest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_decision: Option<PolicyDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_budget: Option<StageBudget>,
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
            agent_author: None,
            reviewer_override: None,
            multi_agent_provenance: Vec::new(),
            plugin_identity: None,
            plugin_permissions: None,
            evidence_sensitivity: None,
            redaction_manifest: None,
            policy_decision: None,
            stage_budget: None,
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
        let mut receipt = Receipt::synthetic(
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
        receipt.agent_author = Some(AgentAttribution {
            product: "Codex".to_string(),
            model_family: Some("gpt-5".to_string()),
            model_version: Some("fixture".to_string()),
            execution_mode: "autonomous_phase".to_string(),
            prompt_context_hash: Some(
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            ),
            commit_provenance: Some("commit:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string()),
            source: "ci_metadata".to_string(),
            confidence: AttributionConfidence::Medium,
        });
        receipt.plugin_permissions = Some(PluginPermissions {
            may_emit_receipts: true,
            may_emit_artifacts: true,
            may_read_previous_receipts: false,
            may_modify_previous_receipts: false,
            may_modify_manifest: false,
        });
        receipt.evidence_sensitivity = Some(EvidenceSensitivity::Internal);
        receipt.policy_decision = Some(PolicyDecision {
            decision: "warning".to_string(),
            policy_id: "pramaan-default-v0".to_string(),
            hard_failures: vec![],
            warnings: vec!["residual_risk:R-049".to_string()],
            waived: vec![],
        });
        receipt.stage_budget = Some(StageBudget {
            target_ms: 30_000,
            max_ms: 60_000,
            consumed_ms: 1_000,
            exhausted: false,
            timeout_reason: None,
            partial_evidence: false,
        });

        let value = serde_json::to_value(receipt).expect("receipt serializes");

        assert_eq!(value["schema_version"], RECEIPT_SCHEMA_VERSION);
        assert_eq!(value["status"], "passed");
        assert_eq!(value["mitigated_risks"][0], "R-001");
        assert_eq!(value["residual_risks"][0], "R-049");
        assert_eq!(value["not_applicable_risks"][0], "R-081");
        assert_eq!(value["agent_author"]["product"], "Codex");
        assert_eq!(value["plugin_permissions"]["may_modify_manifest"], false);
        assert_eq!(value["evidence_sensitivity"], "internal");
        assert_eq!(value["policy_decision"]["decision"], "warning");
        assert_eq!(value["stage_budget"]["partial_evidence"], false);
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
        assert_eq!(risk_family("R-075"), "property_fuzz");
        assert_eq!(risk_family("R-090"), "bundle_integrity");
        assert_eq!(risk_family("R-999"), "unknown");
    }

    #[test]
    fn fuzz_receipt_types_preserve_replay_and_classification_fields() {
        let evidence = FuzzRunEvidence {
            schema_version: "pramaan.differential_fuzz.v1".to_string(),
            adapter: FuzzAdapterMode::DeterministicSimulated,
            seed: 7,
            generated_input_count: 1,
            corpus_hash: "sha256:abc".to_string(),
            replay_path: "target/pramaan/fuzz/fuzz-replay.json".to_string(),
            example_database_path: Some("target/pramaan/fuzz/examples".to_string()),
            counterexample_path: Some("target/pramaan/fuzz/counterexamples.json".to_string()),
            base_discovery: FuzzDiscovery {
                root: "base".to_string(),
                safe_functions: vec![],
                unsafe_functions: vec![],
                not_applicable_reason: None,
            },
            head_discovery: FuzzDiscovery {
                root: "head".to_string(),
                safe_functions: vec![],
                unsafe_functions: vec![],
                not_applicable_reason: None,
            },
            divergences: vec![FuzzDivergence {
                stable_id: "math_ops.py::add_one".to_string(),
                function_name: "add_one".to_string(),
                path: "math_ops.py".to_string(),
                input: FuzzInputCase {
                    index: 0,
                    values: BTreeMap::from([("x".to_string(), 1)]),
                },
                base_output: "2".to_string(),
                head_output: "3".to_string(),
                classification: DivergenceClassification::Expected,
                rationale: "claim scoped".to_string(),
            }],
            limitations: vec![],
        };

        let value = serde_json::to_value(evidence).expect("fuzz evidence serializes");
        assert_eq!(value["seed"], 7);
        assert_eq!(value["adapter"], "deterministic_simulated");
        assert_eq!(value["divergences"][0]["classification"], "expected");
        assert!(fuzz_mitigated_risks().contains(&"R-079".to_string()));
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
                StaticHallucinationCategory::NonexistentImport,
                StaticHallucinationCategory::UndefinedSymbol
            ]
        );
    }

    #[test]
    fn oracle_fixture_diff_detects_weakened_tests_and_artifacts() {
        let fixture_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root")
            .join("examples")
            .join("fixtures")
            .join("oracle");
        let base = discover_oracle_snapshot(&fixture_root.join("base")).expect("base snapshot");
        let head = discover_oracle_snapshot(&fixture_root.join("head")).expect("head snapshot");
        let diff = diff_oracle_snapshots(base, head);
        let kinds = diff
            .findings
            .iter()
            .map(|finding| finding.kind)
            .collect::<BTreeSet<_>>();

        assert!(kinds.contains(&OracleFindingKind::DeletedTest));
        assert!(kinds.contains(&OracleFindingKind::AddedSkip));
        assert!(kinds.contains(&OracleFindingKind::ParametrizedCaseReduction));
        assert!(kinds.contains(&OracleFindingKind::WeakenedAssertion));
        assert!(kinds.contains(&OracleFindingKind::SensitiveArtifactChanged));
        assert!(diff
            .findings
            .iter()
            .any(|finding| finding.risk_ids.contains(&"R-087".to_string())));
        assert!(oracle_mitigated_risks().contains(&"R-020".to_string()));
        assert!(oracle_mitigated_risks().contains(&"R-089".to_string()));
    }

    #[test]
    fn mutation_normalizers_preserve_counts_and_risk_family() {
        let mutmut = normalize_mutmut_output(
            "killed 3\nsurvived 2 not killed by assertions\ntimeout 1\nincompetent 1\nskipped 1",
            vec!["checkout.py".to_string()],
        );
        assert_eq!(mutmut.killed, 3);
        assert_eq!(mutmut.survived, 2);
        assert_eq!(mutmut.timed_out, 1);
        assert_eq!(mutmut.unviable, 1);
        assert_eq!(mutmut.skipped, 1);
        assert_eq!(mutmut.kill_rate_percent(), Some(50));
        assert_eq!(risk_family("R-068"), "mutation_quality");
        assert_eq!(risk_family("R-072"), "property_fuzz");
        assert_eq!(
            mutation_mitigated_risks(),
            vec!["R-068", "R-069", "R-070", "R-071", "R-072"]
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
