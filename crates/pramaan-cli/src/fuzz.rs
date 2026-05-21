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
use std::time::Instant;

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
    divergences.sort_by(|left, right| {
        left.stable_id
            .cmp(&right.stable_id)
            .then_with(|| left.input.index.cmp(&right.input.index))
    });

    write_json(&replay_path, &divergences)?;
    let counterexample_path = if divergences.is_empty() {
        None
    } else {
        let path = out.join("counterexamples.json");
        write_json(&path, &divergences)?;
        Some(portable_path(&path))
    };

    let adapter_availability = adapter_availability(&base_discovery, &head_discovery, &head_repo);
    let adapter = adapter_availability.selected_mode;
    let not_applicable =
        base_discovery.safe_functions.is_empty() || head_discovery.safe_functions.is_empty();
    let evidence = FuzzRunEvidence {
        schema_version: "pramaan.differential_fuzz.v1".to_string(),
        adapter,
        adapter_availability,
        seed,
        generated_input_count: corpus.len(),
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
    let status = if not_applicable {
        StageStatus::NotApplicable
    } else if unexpected > 0 {
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
                "Adapter={}, seed={}, generated_inputs={}, corpus_hash={}, divergences={}, unexpected={}, needs_review={}.",
                evidence.adapter.as_str(),
                evidence.seed,
                evidence.generated_input_count,
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
            ("corpus_hash".to_string(), evidence.corpus_hash.clone()),
            ("replay_path".to_string(), evidence.replay_path.clone()),
            ("divergences".to_string(), evidence.divergences.len().to_string()),
            ("unexpected".to_string(), unexpected.to_string()),
            ("needs_review".to_string(), needs_review.to_string()),
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
) -> FuzzAdapterAvailability {
    let hypothesis_available = has_language(base, FuzzLanguage::Python)
        && has_language(head, FuzzLanguage::Python)
        && hypothesis_available();
    let fast_check_available = has_language(base, FuzzLanguage::TypeScript)
        && has_language(head, FuzzLanguage::TypeScript)
        && fast_check_available(repo);

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
