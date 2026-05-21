use anyhow::{Context, Result};
use chrono::Utc;
use pramaan_bundle::sha256_hex;
use pramaan_core::risks::{
    FUZZ_DETERMINISTIC_SIMULATED, FUZZ_DIVERGENCE_NEEDS_REVIEW, FUZZ_NO_TOOL_BACKED_ADAPTER,
    FUZZ_UNEXPECTED_DIVERGENCE,
};
use pramaan_core::{
    fuzz_mitigated_risks, fuzz_not_applicable_risks, timestamp, ArtifactRef,
    DivergenceClassification, FuzzAdapterAvailability, FuzzAdapterMode, FuzzDiscovery,
    FuzzDivergence, FuzzInputCase, FuzzLanguage, FuzzRunEvidence, InputRef, OutputRef,
    PureFunctionCandidate, Receipt, ReceiptSummary, StageStatus, ToolIdentity,
    UnsafeFunctionCandidate, RECEIPT_SCHEMA_VERSION,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

pub fn run_fuzz(
    base_repo: PathBuf,
    head_repo: PathBuf,
    claim_scope: Option<PathBuf>,
    out: PathBuf,
    seed: u64,
) -> Result<()> {
    let base_repo = base_repo
        .canonicalize()
        .with_context(|| format!("resolving base repository {}", base_repo.display()))?;
    let head_repo = head_repo
        .canonicalize()
        .with_context(|| format!("resolving head repository {}", head_repo.display()))?;
    fs::create_dir_all(&out).with_context(|| format!("creating {}", out.display()))?;

    let started_at = Utc::now();
    let timer = Instant::now();
    let scope = claim_scope
        .as_ref()
        .map(read_claim_scope)
        .transpose()
        .context("reading claim scope")?;

    let base_discovery = discover_safe_functions(&base_repo)?;
    let head_discovery = discover_safe_functions(&head_repo)?;
    let corpus = generated_corpus(seed);
    let replay_path = out.join("fuzz-replay.json");
    let corpus_path = out.join("fuzz-corpus.json");
    write_json(&corpus_path, &corpus)?;
    let corpus_hash = digest_file(&corpus_path)?;

    let mut divergences = compare_discoveries(&base_discovery, &head_discovery, &corpus, &scope);
    let tool_run = run_tool_backed_harness_if_available(
        &base_discovery,
        &head_discovery,
        &head_repo,
        &out,
        seed,
        scope.as_ref(),
        corpus.len(),
    )?;
    if let Some(tool_run) = &tool_run {
        divergences.extend(tool_run.divergences.iter().cloned());
        dedupe_and_sort_divergences(&mut divergences);
    } else {
        divergences.sort_by(|left, right| {
            left.stable_id
                .cmp(&right.stable_id)
                .then_with(|| left.input.index.cmp(&right.input.index))
        });
    }

    write_json(&replay_path, &divergences)?;
    let counterexample_path = if divergences.is_empty() {
        None
    } else {
        let path = out.join("counterexamples.json");
        write_json(&path, &divergences)?;
        Some(portable_path(&path))
    };
    let adapter_availability =
        adapter_availability(&base_discovery, &head_discovery, &head_repo, &tool_run);
    let adapter = adapter_availability.selected_mode;
    let not_applicable =
        base_discovery.safe_functions.is_empty() || head_discovery.safe_functions.is_empty();
    let evidence = FuzzRunEvidence {
        schema_version: "pramaan.differential_fuzz.v1".to_string(),
        adapter,
        adapter_availability,
        seed,
        generated_input_count: corpus.len(),
        deterministic_input_count: corpus.len(),
        tool_generated_case_count: tool_run.as_ref().map(|run| run.generated_cases).unwrap_or(0),
        corpus_hash: corpus_hash.clone(),
        replay_path: portable_path(&replay_path),
        example_database_path: Some(portable_path(&out.join("hypothesis-example-db-or-fast-check-path.txt"))),
        counterexample_path,
        base_discovery,
        head_discovery,
        divergences,
        limitations: vec![
            "Pure-function discovery is intentionally conservative; functions with side effects, calls, imports, async/yield, or complex bodies are marked not applicable.".to_string(),
            "When Hypothesis or fast-check project wiring is unavailable or not yet safely generated, Pramaan emits deterministic differential evidence with the same seed/replay/corpus receipt fields and labels it non-tool-backed.".to_string(),
        ],
    };

    let evidence_path = out.join("differential-fuzz.json");
    write_json(&evidence_path, &evidence)?;
    let evidence_digest = digest_file(&evidence_path)?;
    let replay_digest = digest_file(&replay_path)?;

    let receipt_path = out.join("receipts").join("differential-fuzz.receipt.json");
    let receipt = fuzz_receipt(
        &base_repo,
        &head_repo,
        claim_scope.as_deref(),
        &evidence_path,
        &evidence_digest,
        &replay_path,
        &replay_digest,
        &evidence,
        started_at,
        timer,
        not_applicable,
    );
    write_json(&receipt_path, &receipt)?;

    render_fuzz_summary(&base_repo, &head_repo, &out, &receipt_path, &evidence);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn fuzz_receipt(
    base_repo: &Path,
    head_repo: &Path,
    claim_scope: Option<&Path>,
    _evidence_path: &Path,
    evidence_digest: &str,
    _replay_path: &Path,
    replay_digest: &str,
    evidence: &FuzzRunEvidence,
    started_at: chrono::DateTime<Utc>,
    timer: Instant,
    not_applicable: bool,
) -> Receipt {
    let unexpected = evidence
        .divergences
        .iter()
        .filter(|item| item.classification == DivergenceClassification::Unexpected)
        .count();
    let needs_review = evidence
        .divergences
        .iter()
        .filter(|item| item.classification == DivergenceClassification::NeedsReview)
        .count();
    let tool_status = evidence
        .adapter_availability
        .execution_status
        .as_deref()
        .unwrap_or("not_attempted");
    let status = if not_applicable {
        StageStatus::NotApplicable
    } else if tool_status == "timeout" {
        StageStatus::TimedOut
    } else if matches!(tool_status, "failed" | "error") || unexpected > 0 {
        StageStatus::Failed
    } else {
        StageStatus::Passed
    };
    let mut inputs = vec![
        InputRef {
            name: "base_repo".to_string(),
            value: portable_path(base_repo),
            digest: None,
        },
        InputRef {
            name: "head_repo".to_string(),
            value: portable_path(head_repo),
            digest: None,
        },
    ];
    if let Some(claim_scope) = claim_scope {
        inputs.push(InputRef {
            name: "claim_scope".to_string(),
            value: portable_path(claim_scope),
            digest: None,
        });
    }

    Receipt {
        schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
        stage: "differential_fuzz".to_string(),
        status,
        tool: ToolIdentity::new("pramaan-differential-fuzz", env!("CARGO_PKG_VERSION")),
        started_at: timestamp(started_at),
        ended_at: timestamp(Utc::now()),
        exit_code: Some(if status == StageStatus::Failed { 1 } else { 0 }),
        inputs,
        outputs: vec![
            OutputRef {
                name: "fuzz_evidence".to_string(),
                path: "differential-fuzz.json".to_string(),
                digest: Some(evidence_digest.to_string()),
            },
            OutputRef {
                name: "fuzz_replay".to_string(),
                path: "fuzz-replay.json".to_string(),
                digest: Some(replay_digest.to_string()),
            },
        ],
        artifacts: vec![
            ArtifactRef {
                name: "differential_fuzz_json".to_string(),
                path: "differential-fuzz.json".to_string(),
                media_type: Some("application/json".to_string()),
                digest: Some(evidence_digest.to_string()),
            },
            ArtifactRef {
                name: "replay_artifact".to_string(),
                path: "fuzz-replay.json".to_string(),
                media_type: Some("application/json".to_string()),
                digest: Some(replay_digest.to_string()),
            },
        ],
        summary: ReceiptSummary {
            title: match status {
                StageStatus::NotApplicable => "Differential fuzz not applicable".to_string(),
                StageStatus::Failed => "Differential fuzz found unexpected divergences".to_string(),
                _ => "Differential fuzz completed".to_string(),
            },
            details: format!(
                "Adapter={}, seed={}, deterministic_inputs={}, tool_generated_cases={}, tool_status={}, corpus_hash={}, divergences={}, unexpected={}, needs_review={}.",
                evidence.adapter.as_str(),
                evidence.seed,
                evidence.deterministic_input_count,
                evidence.tool_generated_case_count,
                tool_status,
                evidence.corpus_hash,
                evidence.divergences.len(),
                unexpected,
                needs_review
            ),
        },
        limitations: evidence.limitations.clone(),
        mitigated_risks: if not_applicable {
            vec![]
        } else {
            fuzz_mitigated_risks()
        },
        residual_risks: residual_risks(evidence),
        not_applicable_risks: if not_applicable {
            fuzz_not_applicable_risks()
        } else {
            vec![]
        },
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
            ("adapter".to_string(), evidence.adapter.as_str().to_string()),
            (
                "tool_backed".to_string(),
                evidence.adapter_availability.tool_backed.to_string(),
            ),
            (
                "hypothesis_available".to_string(),
                evidence
                    .adapter_availability
                    .hypothesis_available
                    .to_string(),
            ),
            (
                "fast_check_available".to_string(),
                evidence
                    .adapter_availability
                    .fast_check_available
                    .to_string(),
            ),
            ("seed".to_string(), evidence.seed.to_string()),
            (
                "generated_input_count".to_string(),
                evidence.generated_input_count.to_string(),
            ),
            (
                "deterministic_input_count".to_string(),
                evidence.deterministic_input_count.to_string(),
            ),
            (
                "tool_generated_case_count".to_string(),
                evidence.tool_generated_case_count.to_string(),
            ),
            ("corpus_hash".to_string(), evidence.corpus_hash.clone()),
            ("replay_path".to_string(), evidence.replay_path.clone()),
            ("divergences".to_string(), evidence.divergences.len().to_string()),
            ("unexpected".to_string(), unexpected.to_string()),
            ("needs_review".to_string(), needs_review.to_string()),
            ("tool_execution_status".to_string(), tool_status.to_string()),
            (
                "tool_version".to_string(),
                evidence
                    .adapter_availability
                    .tool_version
                    .clone()
                    .unwrap_or_else(|| "unavailable".to_string()),
            ),
            (
                "duration_ms".to_string(),
                timer.elapsed().as_millis().to_string(),
            ),
        ]),
    }
}

fn residual_risks(evidence: &FuzzRunEvidence) -> Vec<String> {
    let mut risks = BTreeSet::new();
    if evidence
        .divergences
        .iter()
        .any(|item| item.classification == DivergenceClassification::Unexpected)
    {
        risks.insert(FUZZ_UNEXPECTED_DIVERGENCE.to_string());
    }
    if evidence
        .divergences
        .iter()
        .any(|item| item.classification == DivergenceClassification::NeedsReview)
    {
        risks.insert(FUZZ_DIVERGENCE_NEEDS_REVIEW.to_string());
    }
    if evidence.adapter == FuzzAdapterMode::DeterministicSimulated {
        risks.insert(FUZZ_DETERMINISTIC_SIMULATED.to_string());
        risks.insert(FUZZ_NO_TOOL_BACKED_ADAPTER.to_string());
    }
    if !matches!(
        evidence
            .adapter_availability
            .execution_status
            .as_deref()
            .unwrap_or("not_attempted"),
        "not_attempted" | "passed"
    ) {
        risks.insert(FUZZ_DIVERGENCE_NEEDS_REVIEW.to_string());
        risks.insert(FUZZ_NO_TOOL_BACKED_ADAPTER.to_string());
    }
    risks.into_iter().collect()
}

fn discover_safe_functions(root: &Path) -> Result<FuzzDiscovery> {
    let mut safe_functions = Vec::new();
    let mut unsafe_functions = Vec::new();

    for path in walk_files(root)? {
        let relative = portable_relative(root, &path);
        let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        let text = fs::read_to_string(&path).unwrap_or_default();
        match extension {
            "py" => discover_python_functions(
                &relative,
                &text,
                &mut safe_functions,
                &mut unsafe_functions,
            ),
            "ts" | "tsx" | "js" | "jsx" => discover_typescript_functions(
                &relative,
                &text,
                &mut safe_functions,
                &mut unsafe_functions,
            ),
            _ => {}
        }
    }

    safe_functions.sort_by(|left, right| left.stable_id.cmp(&right.stable_id));
    unsafe_functions.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.name.cmp(&right.name))
    });
    let not_applicable_reason = safe_functions.is_empty().then(|| {
        if unsafe_functions.is_empty() {
            "no Python or TypeScript pure-function candidates were discovered".to_string()
        } else {
            "all discovered functions were unsafe for deterministic differential fuzzing"
                .to_string()
        }
    });

    Ok(FuzzDiscovery {
        root: portable_path(root),
        safe_functions,
        unsafe_functions,
        not_applicable_reason,
    })
}

fn discover_python_functions(
    path: &str,
    text: &str,
    safe: &mut Vec<PureFunctionCandidate>,
    unsafe_functions: &mut Vec<UnsafeFunctionCandidate>,
) {
    let lines = text.lines().collect::<Vec<_>>();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("def ") || trimmed.starts_with("def _") {
            continue;
        }
        let Some((name, parameters)) = parse_signature(trimmed, "def ") else {
            continue;
        };
        let body = collect_indented_body(&lines, index + 1);
        let returns = body
            .iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        if returns.len() == 1 && returns[0].starts_with("return ") {
            let expression = returns[0].trim_start_matches("return ").trim();
            push_candidate(
                FuzzLanguage::Python,
                path,
                name,
                parameters,
                expression,
                safe,
                unsafe_functions,
            );
        } else {
            unsafe_functions.push(UnsafeFunctionCandidate {
                language: FuzzLanguage::Python,
                path: path.to_string(),
                name,
                reason: "function body is not a single return expression".to_string(),
            });
        }
    }
}

fn discover_typescript_functions(
    path: &str,
    text: &str,
    safe: &mut Vec<PureFunctionCandidate>,
    unsafe_functions: &mut Vec<UnsafeFunctionCandidate>,
) {
    for line in text.lines() {
        let trimmed = line.trim();
        let Some(signature_start) = trimmed.find("function ") else {
            continue;
        };
        let signature = &trimmed[signature_start..];
        let Some((name, parameters)) = parse_signature(signature, "function ") else {
            continue;
        };
        if let Some(return_start) = trimmed.find("return ") {
            let expression = trimmed[return_start + "return ".len()..]
                .trim()
                .trim_end_matches('}')
                .trim_end_matches(';')
                .trim();
            push_candidate(
                FuzzLanguage::TypeScript,
                path,
                name,
                parameters,
                expression,
                safe,
                unsafe_functions,
            );
        } else {
            unsafe_functions.push(UnsafeFunctionCandidate {
                language: FuzzLanguage::TypeScript,
                path: path.to_string(),
                name,
                reason: "function body is not a single inline return expression".to_string(),
            });
        }
    }
}

fn push_candidate(
    language: FuzzLanguage,
    path: &str,
    name: String,
    parameters: Vec<String>,
    expression: &str,
    safe: &mut Vec<PureFunctionCandidate>,
    unsafe_functions: &mut Vec<UnsafeFunctionCandidate>,
) {
    if parameters.is_empty() {
        unsafe_functions.push(UnsafeFunctionCandidate {
            language,
            path: path.to_string(),
            name,
            reason: "function has no generated input parameters".to_string(),
        });
        return;
    }
    if let Some(reason) = unsafe_expression_reason(expression) {
        unsafe_functions.push(UnsafeFunctionCandidate {
            language,
            path: path.to_string(),
            name,
            reason,
        });
        return;
    }

    safe.push(PureFunctionCandidate {
        language,
        path: path.to_string(),
        stable_id: format!("{}::{}", path.replace('\\', "/"), name),
        name,
        parameters,
        return_expression: expression.to_string(),
        safety_notes: vec![
            "single return expression".to_string(),
            "integer arithmetic grammar only".to_string(),
        ],
    });
}

fn parse_signature(line: &str, prefix: &str) -> Option<(String, Vec<String>)> {
    let rest = line.strip_prefix(prefix)?;
    let name_end = rest.find('(')?;
    let name = rest[..name_end].trim().to_string();
    let params_end = rest[name_end + 1..].find(')')?;
    let raw_params = &rest[name_end + 1..name_end + 1 + params_end];
    let parameters = raw_params
        .split(',')
        .filter_map(|part| {
            let name = part
                .trim()
                .split(':')
                .next()
                .unwrap_or_default()
                .trim()
                .trim_start_matches('*')
                .trim_start_matches('&');
            (!name.is_empty() && name != "self").then(|| name.to_string())
        })
        .collect::<Vec<_>>();
    Some((name, parameters))
}

fn collect_indented_body<'a>(lines: &'a [&'a str], start: usize) -> Vec<&'a str> {
    let mut body = Vec::new();
    for line in &lines[start..] {
        if !line.trim().is_empty() && !line.starts_with(' ') && !line.starts_with('\t') {
            break;
        }
        body.push(*line);
    }
    body
}

fn unsafe_expression_reason(expression: &str) -> Option<String> {
    let lower = expression.to_lowercase();
    let unsafe_tokens = [
        "open(",
        "read(",
        "write(",
        "import",
        "global ",
        "nonlocal ",
        "await ",
        "yield ",
        "fetch(",
        "process.",
        "fs.",
        "require(",
        "new ",
        "=>",
        "[",
        "]",
        "{",
        "}",
    ];
    if let Some(token) = unsafe_tokens.iter().find(|token| lower.contains(**token)) {
        return Some(format!("return expression contains unsafe token `{token}`"));
    }
    if expression.contains('(') || expression.contains('.') {
        return Some("return expression contains a call or attribute access".to_string());
    }
    if expression.chars().any(|ch| {
        !(ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace() || "+-*/%()_".contains(ch))
    }) {
        return Some("return expression is outside the deterministic integer grammar".to_string());
    }
    None
}

fn generated_corpus(seed: u64) -> Vec<i64> {
    let mut values = vec![-7, -1, 0, 1, 2, 7];
    let seed_value = (seed % 19) as i64 - 9;
    if !values.contains(&seed_value) {
        values.push(seed_value);
    }
    values
}

fn compare_discoveries(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    corpus: &[i64],
    scope: &Option<ClaimScopeClassifier>,
) -> Vec<FuzzDivergence> {
    let head_by_key = head
        .safe_functions
        .iter()
        .map(|candidate| (candidate_key(candidate), candidate))
        .collect::<BTreeMap<_, _>>();
    let mut divergences = Vec::new();

    for base_candidate in &base.safe_functions {
        let Some(head_candidate) = head_by_key.get(&candidate_key(base_candidate)) else {
            continue;
        };
        for (index, seed_value) in corpus.iter().enumerate() {
            let input = build_input_case(index, &base_candidate.parameters, *seed_value);
            let base_output = eval_expression(&base_candidate.return_expression, &input.values);
            let head_output = eval_expression(&head_candidate.return_expression, &input.values);
            if base_output != head_output {
                let (classification, rationale) =
                    classify_divergence(head_candidate, scope.as_ref());
                divergences.push(FuzzDivergence {
                    stable_id: base_candidate.stable_id.clone(),
                    function_name: base_candidate.name.clone(),
                    path: base_candidate.path.clone(),
                    input,
                    base_output,
                    head_output,
                    classification,
                    rationale,
                });
            }
        }
    }

    divergences
}

fn dedupe_and_sort_divergences(divergences: &mut Vec<FuzzDivergence>) {
    let mut seen = BTreeSet::new();
    divergences.retain(|item| {
        let key = format!("{}#{}", item.stable_id, item.input.index);
        seen.insert(key)
    });
    divergences.sort_by(|left, right| {
        left.stable_id
            .cmp(&right.stable_id)
            .then_with(|| left.input.index.cmp(&right.input.index))
    });
}

fn candidate_key(candidate: &PureFunctionCandidate) -> String {
    format!("{}::{}", candidate.path, candidate.name)
}

fn build_input_case(index: usize, parameters: &[String], seed_value: i64) -> FuzzInputCase {
    let values = parameters
        .iter()
        .enumerate()
        .map(|(offset, parameter)| (parameter.clone(), seed_value + offset as i64))
        .collect::<BTreeMap<_, _>>();
    FuzzInputCase { index, values }
}

fn eval_expression(expression: &str, values: &BTreeMap<String, i64>) -> String {
    match ArithmeticParser::new(expression, values).parse() {
        Some(value) => value.to_string(),
        None => format!("eval_error:{expression}"),
    }
}

struct ArithmeticParser<'a> {
    input: Vec<char>,
    index: usize,
    values: &'a BTreeMap<String, i64>,
}

impl<'a> ArithmeticParser<'a> {
    fn new(expression: &str, values: &'a BTreeMap<String, i64>) -> Self {
        Self {
            input: expression.chars().collect(),
            index: 0,
            values,
        }
    }

    fn parse(mut self) -> Option<i64> {
        let value = self.parse_expr()?;
        self.skip_ws();
        (self.index == self.input.len()).then_some(value)
    }

    fn parse_expr(&mut self) -> Option<i64> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_ws();
            if self.consume('+') {
                value += self.parse_term()?;
            } else if self.consume('-') {
                value -= self.parse_term()?;
            } else {
                return Some(value);
            }
        }
    }

    fn parse_term(&mut self) -> Option<i64> {
        let mut value = self.parse_factor()?;
        loop {
            self.skip_ws();
            if self.consume('*') {
                value *= self.parse_factor()?;
            } else if self.consume('/') {
                let rhs = self.parse_factor()?;
                if rhs == 0 {
                    return None;
                }
                value /= rhs;
            } else if self.consume('%') {
                let rhs = self.parse_factor()?;
                if rhs == 0 {
                    return None;
                }
                value %= rhs;
            } else {
                return Some(value);
            }
        }
    }

    fn parse_factor(&mut self) -> Option<i64> {
        self.skip_ws();
        if self.consume('-') {
            return Some(-self.parse_factor()?);
        }
        if self.consume('(') {
            let value = self.parse_expr()?;
            self.skip_ws();
            return self.consume(')').then_some(value);
        }
        if self.peek()?.is_ascii_digit() {
            return self.parse_number();
        }
        self.parse_identifier()
            .and_then(|name| self.values.get(&name).copied())
    }

    fn parse_number(&mut self) -> Option<i64> {
        let start = self.index;
        while self.peek().map(|ch| ch.is_ascii_digit()).unwrap_or(false) {
            self.index += 1;
        }
        self.input[start..self.index]
            .iter()
            .collect::<String>()
            .parse()
            .ok()
    }

    fn parse_identifier(&mut self) -> Option<String> {
        let start = self.index;
        while self
            .peek()
            .map(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            .unwrap_or(false)
        {
            self.index += 1;
        }
        (self.index > start).then(|| self.input[start..self.index].iter().collect())
    }

    fn skip_ws(&mut self) {
        while self.peek().map(|ch| ch.is_whitespace()).unwrap_or(false) {
            self.index += 1;
        }
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.index).copied()
    }
}

#[derive(Debug, Clone)]
struct ClaimScopeClassifier {
    expected_text: String,
    out_of_scope_text: String,
    public_api_symbols: Vec<String>,
}

fn read_claim_scope(path: &PathBuf) -> Result<ClaimScopeClassifier> {
    let value: Value = serde_json::from_slice(
        &fs::read(path).with_context(|| format!("reading {}", path.display()))?,
    )
    .with_context(|| format!("parsing {}", path.display()))?;
    Ok(ClaimScopeClassifier {
        expected_text: collect_descriptions(&value["expected_behavior"]).to_lowercase(),
        out_of_scope_text: collect_descriptions(&value["out_of_scope_behavior"]).to_lowercase(),
        public_api_symbols: value["touched_public_apis"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| item["symbol"].as_str().map(|symbol| symbol.to_lowercase()))
            .collect(),
    })
}

fn collect_descriptions(value: &Value) -> String {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|item| item["description"].as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

fn classify_divergence(
    candidate: &PureFunctionCandidate,
    scope: Option<&ClaimScopeClassifier>,
) -> (DivergenceClassification, String) {
    let Some(scope) = scope else {
        return (
            DivergenceClassification::NeedsReview,
            "no claim scope was provided for divergence classification".to_string(),
        );
    };
    let name = candidate.name.to_lowercase();
    let path = candidate.path.to_lowercase();
    let symbol_match = scope
        .public_api_symbols
        .iter()
        .any(|symbol| symbol.ends_with(&name) || symbol.contains(&name) || path.contains(symbol));
    let expected_match = scope.expected_text.contains(&name);
    let out_of_scope_match = scope.out_of_scope_text.contains(&name);

    if expected_match || symbol_match {
        (
            DivergenceClassification::Expected,
            "function matched expected behavior or touched public API in claim scope".to_string(),
        )
    } else if out_of_scope_match || !scope.public_api_symbols.is_empty() {
        (
            DivergenceClassification::Unexpected,
            "function divergence was outside the claimed expected behavior scope".to_string(),
        )
    } else {
        (
            DivergenceClassification::NeedsReview,
            "claim scope did not name enough public API detail to classify the divergence"
                .to_string(),
        )
    }
}

fn adapter_availability(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    repo: &Path,
    tool_run: &Option<ToolHarnessRun>,
) -> FuzzAdapterAvailability {
    let hypothesis_available = has_language(base, FuzzLanguage::Python)
        && has_language(head, FuzzLanguage::Python)
        && hypothesis_available();
    let fast_check_available = has_language(base, FuzzLanguage::TypeScript)
        && has_language(head, FuzzLanguage::TypeScript)
        && fast_check_available(repo);
    if let Some(run) = tool_run {
        let tool_backed = run.execution_status == "passed";
        return FuzzAdapterAvailability {
            hypothesis_available,
            fast_check_available,
            selected_mode: run.mode,
            tool_backed,
            reason: format!(
                "safe generated harness executed; status={}; tool_version={}; generated_cases={}; raw_output_digest={}; harness_path={}; raw_output_path={}",
                run.execution_status,
                run.tool_version,
                run.generated_cases,
                run.raw_output_digest,
                run.harness_path,
                run.raw_output_path
            ),
            tool_version: Some(run.tool_version.clone()),
            tool_generated_case_count: run.generated_cases,
            raw_output_digest: Some(run.raw_output_digest.clone()),
            harness_path: Some(run.harness_path.clone()),
            raw_output_path: Some(run.raw_output_path.clone()),
            execution_status: Some(run.execution_status.clone()),
            timeout_ms: Some(run.timeout_ms),
        };
    }

    FuzzAdapterAvailability {
        hypothesis_available,
        fast_check_available,
        selected_mode: FuzzAdapterMode::DeterministicSimulated,
        tool_backed: false,
        reason: if hypothesis_available || fast_check_available {
            "external property-testing tool was detected, but safe generated harness execution is not enabled in this build; deterministic replay evidence was selected"
                .to_string()
        } else {
            "no supported external property-testing adapter was available for the discovered pure-function candidates; deterministic replay evidence was selected"
                .to_string()
        },
        tool_version: None,
        tool_generated_case_count: 0,
        raw_output_digest: None,
        harness_path: None,
        raw_output_path: None,
        execution_status: Some("not_attempted".to_string()),
        timeout_ms: None,
    }
}

#[derive(Debug, Clone)]
struct ToolHarnessRun {
    mode: FuzzAdapterMode,
    tool_version: String,
    generated_cases: usize,
    harness_path: String,
    raw_output_path: String,
    raw_output_digest: String,
    execution_status: String,
    timeout_ms: u64,
    divergences: Vec<FuzzDivergence>,
}

fn run_tool_backed_harness_if_available(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    repo: &Path,
    out: &Path,
    seed: u64,
    scope: Option<&ClaimScopeClassifier>,
    deterministic_count: usize,
) -> Result<Option<ToolHarnessRun>> {
    if has_language(base, FuzzLanguage::Python)
        && has_language(head, FuzzLanguage::Python)
        && hypothesis_available()
    {
        return run_hypothesis_harness(base, head, out, seed, scope, deterministic_count).map(Some);
    }
    if has_language(base, FuzzLanguage::TypeScript)
        && has_language(head, FuzzLanguage::TypeScript)
        && fast_check_available(repo)
    {
        return run_fast_check_harness(base, head, repo, out, seed, scope, deterministic_count)
            .map(Some);
    }
    Ok(None)
}

fn run_hypothesis_harness(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    out: &Path,
    seed: u64,
    scope: Option<&ClaimScopeClassifier>,
    deterministic_count: usize,
) -> Result<ToolHarnessRun> {
    const TIMEOUT: Duration = Duration::from_secs(10);
    let harness_dir = out.join("tool-harness");
    fs::create_dir_all(&harness_dir)
        .with_context(|| format!("creating {}", harness_dir.display()))?;
    let harness_path = harness_dir.join("hypothesis_harness.py");
    let output_path = harness_dir.join("hypothesis-output.json");
    let cases_path = harness_dir.join("hypothesis-cases.json");
    write_json(
        &cases_path,
        &tool_harness_cases(base, head, FuzzLanguage::Python),
    )?;
    fs::write(
        &harness_path,
        hypothesis_harness_source(&cases_path, &output_path, seed),
    )
    .with_context(|| format!("writing {}", harness_path.display()))?;

    let output = run_with_timeout(Command::new("python").arg(&harness_path), TIMEOUT)
        .context("running Hypothesis harness")?;

    let value = read_or_write_harness_output(&output_path, "hypothesis", "unknown", seed, &output)?;
    let tool_version = value["tool_version"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let generated_cases = value["generated_cases"].as_u64().unwrap_or(0) as usize;
    let raw_output_digest = digest_file(&output_path)?;
    let execution_status = harness_execution_status(&output);
    let divergences = if output.status_success && !output.timed_out {
        tool_failures_to_divergences(
            &value,
            base,
            head,
            FuzzLanguage::Python,
            scope,
            deterministic_count,
        )
    } else {
        Vec::new()
    };
    Ok(ToolHarnessRun {
        mode: FuzzAdapterMode::Hypothesis,
        tool_version,
        generated_cases,
        harness_path: portable_path(&harness_path),
        raw_output_path: portable_path(&output_path),
        raw_output_digest,
        execution_status,
        timeout_ms: TIMEOUT.as_millis() as u64,
        divergences,
    })
}

fn run_fast_check_harness(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    repo: &Path,
    out: &Path,
    seed: u64,
    scope: Option<&ClaimScopeClassifier>,
    deterministic_count: usize,
) -> Result<ToolHarnessRun> {
    const TIMEOUT: Duration = Duration::from_secs(10);
    let harness_dir = out.join("tool-harness");
    fs::create_dir_all(&harness_dir)
        .with_context(|| format!("creating {}", harness_dir.display()))?;
    let harness_path = harness_dir.join("fast-check-harness.cjs");
    let output_path = harness_dir.join("fast-check-output.json");
    let cases_path = harness_dir.join("fast-check-cases.json");
    write_json(
        &cases_path,
        &tool_harness_cases(base, head, FuzzLanguage::TypeScript),
    )?;
    fs::write(
        &harness_path,
        fast_check_harness_source(&cases_path, &output_path, seed),
    )
    .with_context(|| format!("writing {}", harness_path.display()))?;

    let output = run_with_timeout(
        Command::new("node").current_dir(repo).arg(&harness_path),
        TIMEOUT,
    )
    .context("running fast-check harness")?;

    let value = read_or_write_harness_output(&output_path, "fast-check", "unknown", seed, &output)?;
    let tool_version = value["tool_version"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let generated_cases = value["generated_cases"].as_u64().unwrap_or(0) as usize;
    let raw_output_digest = digest_file(&output_path)?;
    let execution_status = harness_execution_status(&output);
    let divergences = if output.status_success && !output.timed_out {
        tool_failures_to_divergences(
            &value,
            base,
            head,
            FuzzLanguage::TypeScript,
            scope,
            deterministic_count,
        )
    } else {
        Vec::new()
    };
    Ok(ToolHarnessRun {
        mode: FuzzAdapterMode::FastCheck,
        tool_version,
        generated_cases,
        harness_path: portable_path(&harness_path),
        raw_output_path: portable_path(&output_path),
        raw_output_digest,
        execution_status,
        timeout_ms: TIMEOUT.as_millis() as u64,
        divergences,
    })
}

#[derive(Debug)]
struct HarnessOutput {
    status_success: bool,
    timed_out: bool,
    exit_code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

fn harness_execution_status(output: &HarnessOutput) -> String {
    if output.timed_out {
        "timeout".to_string()
    } else if output.status_success {
        "passed".to_string()
    } else {
        "failed".to_string()
    }
}

fn read_or_write_harness_output(
    output_path: &Path,
    tool: &str,
    tool_version: &str,
    seed: u64,
    output: &HarnessOutput,
) -> Result<Value> {
    if output_path.exists() {
        return serde_json::from_slice(
            &fs::read(output_path).with_context(|| format!("reading {}", output_path.display()))?,
        )
        .with_context(|| format!("parsing {}", output_path.display()));
    }
    let value = serde_json::json!({
        "tool": tool,
        "tool_version": tool_version,
        "seed": seed,
        "generated_cases": 0,
        "failures": [],
        "execution_status": harness_execution_status(output),
        "exit_code": output.exit_code,
        "timed_out": output.timed_out,
        "stdout_digest": sha256_hex(&output.stdout),
        "stderr_digest": sha256_hex(&output.stderr)
    });
    write_json(output_path, &value)?;
    Ok(value)
}

fn tool_failures_to_divergences(
    value: &Value,
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    language: FuzzLanguage,
    scope: Option<&ClaimScopeClassifier>,
    start_index: usize,
) -> Vec<FuzzDivergence> {
    let head_by_key = head
        .safe_functions
        .iter()
        .filter(|candidate| candidate.language == language)
        .map(|candidate| (candidate_key(candidate), candidate))
        .collect::<BTreeMap<_, _>>();
    let base_by_stable = base
        .safe_functions
        .iter()
        .filter(|candidate| candidate.language == language)
        .map(|candidate| (candidate.stable_id.clone(), candidate))
        .collect::<BTreeMap<_, _>>();

    value["failures"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(offset, failure)| {
            let stable_id = failure["stable_id"].as_str()?.to_string();
            let base_candidate = base_by_stable.get(&stable_id)?;
            let head_candidate = head_by_key
                .get(&candidate_key(base_candidate))
                .copied()
                .unwrap_or(base_candidate);
            let mut values = BTreeMap::new();
            if let Some(input) = failure["input"].as_object() {
                for (key, value) in input {
                    if let Some(number) = value.as_i64() {
                        values.insert(key.clone(), number);
                    }
                }
            }
            let (classification, mut rationale) = classify_divergence(head_candidate, scope);
            let classification = if classification == DivergenceClassification::Expected {
                classification
            } else {
                rationale =
                    format!("tool-backed harness found a generated counterexample; {rationale}");
                DivergenceClassification::Unexpected
            };
            Some(FuzzDivergence {
                stable_id,
                function_name: base_candidate.name.clone(),
                path: base_candidate.path.clone(),
                input: FuzzInputCase {
                    index: start_index + offset,
                    values,
                },
                base_output: failure["base"].as_str().unwrap_or("unknown").to_string(),
                head_output: failure["head"].as_str().unwrap_or("unknown").to_string(),
                classification,
                rationale,
            })
        })
        .collect()
}

fn tool_harness_cases(
    base: &FuzzDiscovery,
    head: &FuzzDiscovery,
    language: FuzzLanguage,
) -> Vec<Value> {
    let head_by_key = head
        .safe_functions
        .iter()
        .filter(|candidate| candidate.language == language)
        .map(|candidate| (candidate_key(candidate), candidate))
        .collect::<BTreeMap<_, _>>();
    base.safe_functions
        .iter()
        .filter(|candidate| candidate.language == language)
        .filter_map(|base_candidate| {
            let head_candidate = head_by_key.get(&candidate_key(base_candidate))?;
            Some(serde_json::json!({
                "stable_id": base_candidate.stable_id,
                "name": base_candidate.name,
                "parameters": base_candidate.parameters,
                "base_expression": base_candidate.return_expression,
                "head_expression": head_candidate.return_expression
            }))
        })
        .collect()
}

fn hypothesis_harness_source(cases_path: &Path, output_path: &Path, seed: u64) -> String {
    format!(
        r#"import json
import os
from hypothesis import given, settings, strategies as st, HealthCheck, __version__ as HYPOTHESIS_VERSION

CASES_PATH = r"{cases_path}"
OUTPUT_PATH = r"{output_path}"
SEED = {seed}
cases = json.load(open(CASES_PATH, "r", encoding="utf-8"))
generated = set()
failures = []

def eval_expr(expr, env):
    return eval(expr, {{"__builtins__": {{}}}}, dict(env))

@settings(max_examples=25, deadline=200, derandomize=True, database=None, suppress_health_check=[HealthCheck.too_slow])
@given(st.integers(min_value=-10, max_value=10))
def run_case(seed_value):
    generated.add(seed_value)
    for case in cases:
        env = {{name: seed_value + index for index, name in enumerate(case["parameters"])}}
        base = eval_expr(case["base_expression"], env)
        head = eval_expr(case["head_expression"], env)
        if base != head:
            failures.append({{"stable_id": case["stable_id"], "input": env, "base": str(base), "head": str(head)}})

run_case()
os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)
json.dump({{
    "tool": "hypothesis",
    "tool_version": HYPOTHESIS_VERSION,
    "seed": SEED,
    "generated_cases": len(generated),
    "failures": failures[:20]
}}, open(OUTPUT_PATH, "w", encoding="utf-8"), indent=2, sort_keys=True)
"#,
        cases_path = portable_path(cases_path),
        output_path = portable_path(output_path),
        seed = seed
    )
}

fn fast_check_harness_source(cases_path: &Path, output_path: &Path, seed: u64) -> String {
    format!(
        r#"const fs = require("fs");
const fc = require("fast-check");
const cases = JSON.parse(fs.readFileSync("{cases_path}", "utf8"));
const generated = new Set();
const failures = [];

function evalExpr(expr, env) {{
  const tokens = expr.match(/[A-Za-z_][A-Za-z0-9_]*|-?\\d+|[()+\\-*/%]/g) || [];
  let index = 0;
  function peek() {{ return tokens[index]; }}
  function take() {{ return tokens[index++]; }}
  function factor() {{
    const token = take();
    if (token === "(") {{
      const value = expression();
      if (take() !== ")") throw new Error("unbalanced expression");
      return value;
    }}
    if (/^-?\\d+$/.test(token)) return Number(token);
    if (Object.prototype.hasOwnProperty.call(env, token)) return env[token];
    throw new Error(`unknown identifier ${{token}}`);
  }}
  function term() {{
    let value = factor();
    while (["*", "/", "%"].includes(peek())) {{
      const op = take();
      const rhs = factor();
      if ((op === "/" || op === "%") && rhs === 0) throw new Error("division by zero");
      if (op === "*") value *= rhs;
      if (op === "/") value = Math.trunc(value / rhs);
      if (op === "%") value %= rhs;
    }}
    return value;
  }}
  function expression() {{
    let value = term();
    while (["+", "-"].includes(peek())) {{
      const op = take();
      const rhs = term();
      value = op === "+" ? value + rhs : value - rhs;
    }}
    return value;
  }}
  const value = expression();
  if (index !== tokens.length) throw new Error("trailing tokens");
  return value;
}}

fc.assert(
  fc.property(fc.integer({{ min: -10, max: 10 }}), (seedValue) => {{
    generated.add(seedValue);
    for (const item of cases) {{
      const env = Object.fromEntries(item.parameters.map((name, index) => [name, seedValue + index]));
      const base = evalExpr(item.base_expression, env);
      const head = evalExpr(item.head_expression, env);
      if (base !== head) failures.push({{ stable_id: item.stable_id, input: env, base: String(base), head: String(head) }});
    }}
    return true;
  }}),
  {{ numRuns: 25, seed: {seed}, endOnFailure: false }}
);

fs.writeFileSync("{output_path}", JSON.stringify({{
  tool: "fast-check",
  tool_version: require("fast-check/package.json").version,
  seed: {seed},
  generated_cases: generated.size,
  failures: failures.slice(0, 20)
}}, null, 2));
"#,
        cases_path = portable_path(cases_path).replace('\\', "\\\\"),
        output_path = portable_path(output_path).replace('\\', "\\\\"),
        seed = seed
    )
}

fn run_with_timeout(command: &mut Command, timeout: Duration) -> Result<HarnessOutput> {
    command
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    let started = Instant::now();
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return Ok(HarnessOutput {
                status_success: false,
                timed_out: false,
                exit_code: None,
                stdout: Vec::new(),
                stderr: format!("spawn_error:{error}").into_bytes(),
            });
        }
    };
    loop {
        if let Some(status) = child.try_wait().context("polling generated harness")? {
            let output = child
                .wait_with_output()
                .context("collecting generated harness output")?;
            return Ok(HarnessOutput {
                status_success: status.success(),
                timed_out: false,
                exit_code: status.code(),
                stdout: output.stdout,
                stderr: output.stderr,
            });
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let output = child
                .wait_with_output()
                .context("collecting timed-out generated harness output")?;
            return Ok(HarnessOutput {
                status_success: false,
                timed_out: true,
                exit_code: output.status.code(),
                stdout: output.stdout,
                stderr: output.stderr,
            });
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn has_language(discovery: &FuzzDiscovery, language: FuzzLanguage) -> bool {
    discovery
        .safe_functions
        .iter()
        .any(|candidate| candidate.language == language)
}

fn hypothesis_available() -> bool {
    Command::new("python")
        .args(["-c", "import hypothesis"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn fast_check_available(repo: &Path) -> bool {
    repo.join("node_modules").join("fast-check").exists()
        || fs::read_to_string(repo.join("package.json"))
            .map(|package_json| package_json.contains("\"fast-check\""))
            .unwrap_or(false)
}

fn walk_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        for entry in fs::read_dir(&path).with_context(|| format!("reading {}", path.display()))? {
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

fn portable_relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn render_fuzz_summary(
    base_repo: &Path,
    head_repo: &Path,
    out: &Path,
    receipt_path: &Path,
    evidence: &FuzzRunEvidence,
) {
    println!("Pramaan differential fuzz complete");
    println!("base_repo: {}", base_repo.display());
    println!("head_repo: {}", head_repo.display());
    println!("bundle: {}", out.display());
    println!("receipt: {}", receipt_path.display());
    println!(
        "adapter: {} seed: {} corpus: {} inputs: {}",
        evidence.adapter.as_str(),
        evidence.seed,
        evidence.corpus_hash,
        evidence.generated_input_count
    );
    println!("replay: {}", evidence.replay_path);
    println!("divergences: {}", evidence.divergences.len());

    if let Some(reason) = evidence
        .base_discovery
        .not_applicable_reason
        .as_ref()
        .or(evidence.head_discovery.not_applicable_reason.as_ref())
    {
        println!("not_applicable: {reason}");
    }

    if !evidence.divergences.is_empty() {
        println!();
        println!("{:<16} {:<28} path", "classification", "function");
        for divergence in &evidence.divergences {
            println!(
                "{:<16} {:<28} {}",
                divergence.classification.as_str(),
                divergence.function_name,
                divergence.path
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(language: FuzzLanguage, expression: &str) -> PureFunctionCandidate {
        PureFunctionCandidate {
            language,
            path: match language {
                FuzzLanguage::Python => "math_ops.py".to_string(),
                FuzzLanguage::TypeScript => "math_ops.ts".to_string(),
            },
            name: "adjust".to_string(),
            stable_id: match language {
                FuzzLanguage::Python => "math_ops.py::adjust".to_string(),
                FuzzLanguage::TypeScript => "math_ops.ts::adjust".to_string(),
            },
            parameters: vec!["x".to_string()],
            return_expression: expression.to_string(),
            safety_notes: vec!["test".to_string()],
        }
    }

    #[test]
    fn tool_failures_become_unexpected_divergences() {
        let base_candidate = candidate(FuzzLanguage::Python, "x + 1");
        let head_candidate = candidate(FuzzLanguage::Python, "x + 2");
        let base = FuzzDiscovery {
            root: "base".to_string(),
            safe_functions: vec![base_candidate],
            unsafe_functions: vec![],
            not_applicable_reason: None,
        };
        let head = FuzzDiscovery {
            root: "head".to_string(),
            safe_functions: vec![head_candidate],
            unsafe_functions: vec![],
            not_applicable_reason: None,
        };
        let value = serde_json::json!({
            "failures": [{
                "stable_id": "math_ops.py::adjust",
                "input": { "x": 4 },
                "base": "5",
                "head": "6"
            }]
        });

        let divergences =
            tool_failures_to_divergences(&value, &base, &head, FuzzLanguage::Python, None, 7);

        assert_eq!(divergences.len(), 1);
        assert_eq!(
            divergences[0].classification,
            DivergenceClassification::Unexpected
        );
        assert_eq!(divergences[0].input.index, 7);
        assert!(divergences[0]
            .rationale
            .contains("tool-backed harness found"));
    }

    #[test]
    fn run_with_timeout_kills_long_running_processes() {
        let mut command = if cfg!(windows) {
            let mut command = Command::new("powershell");
            command.args(["-NoProfile", "-Command", "Start-Sleep -Seconds 5"]);
            command
        } else {
            let mut command = Command::new("sh");
            command.args(["-c", "sleep 5"]);
            command
        };
        let output = run_with_timeout(&mut command, Duration::from_millis(50))
            .expect("timeout command should produce structured output");
        assert!(output.timed_out);
        assert!(!output.status_success);
    }

    #[test]
    fn harness_failure_writes_structured_output() {
        let path = std::env::temp_dir().join(format!(
            "pramaan-fuzz-harness-failure-{}.json",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);
        let output = HarnessOutput {
            status_success: false,
            timed_out: false,
            exit_code: Some(2),
            stdout: b"hello".to_vec(),
            stderr: b"boom".to_vec(),
        };
        let value = read_or_write_harness_output(&path, "fixture", "0", 1, &output)
            .expect("structured failure output");
        assert_eq!(value["execution_status"], "failed");
        assert_eq!(value["generated_cases"], 0);
        assert_eq!(value["exit_code"], 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn fast_check_harness_uses_safe_evaluator_not_dynamic_function() {
        let source = fast_check_harness_source(Path::new("cases.json"), Path::new("out.json"), 7);
        assert!(!source.contains("Function("));
        assert!(source.contains("function evalExpr"));
        assert!(source.contains("unknown identifier"));
    }
}
