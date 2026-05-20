use anyhow::{Context, Result};
use chrono::Utc;
use pramaan_bundle::sha256_hex;
use pramaan_core::{
    classify_static_hallucinations, timestamp, ArtifactRef, InputRef, OutputRef, Receipt,
    ReceiptSummary, StageStatus, ToolIdentity, RECEIPT_SCHEMA_VERSION,
};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

pub fn run_static_checks(repo: PathBuf, out: PathBuf) -> Result<()> {
    let repo = repo
        .canonicalize()
        .with_context(|| format!("resolving repository path {}", repo.display()))?;
    let receipt_dir = out.join("receipts").join("static");
    fs::create_dir_all(&receipt_dir).with_context(|| {
        format!(
            "creating static receipt directory {}",
            receipt_dir.display()
        )
    })?;

    let plans = discover_checks(&repo)?;
    let mut receipts = Vec::new();

    for plan in plans {
        let receipt = run_or_skip_check(&repo, &plan)?;
        let receipt_path = receipt_dir.join(format!("{}.receipt.json", plan.id));
        write_json(&receipt_path, &receipt)?;
        receipts.push((receipt, receipt_path));
    }

    render_static_summary(&repo, &out, &receipts);
    Ok(())
}

#[derive(Debug, Clone)]
struct StaticCheckPlan {
    id: String,
    language: &'static str,
    title: String,
    tool: String,
    command: Vec<String>,
    configured: bool,
    applicable: bool,
    skip_reason: Option<String>,
    config_path: Option<PathBuf>,
}

fn discover_checks(repo: &Path) -> Result<Vec<StaticCheckPlan>> {
    let mut plans = Vec::new();
    plans.extend(discover_python(repo)?);
    plans.extend(discover_typescript(repo)?);
    plans.extend(discover_rust(repo)?);
    Ok(plans)
}

fn discover_python(repo: &Path) -> Result<Vec<StaticCheckPlan>> {
    let has_python = has_extension(repo, "py")?;
    let pyproject = repo.join("pyproject.toml");
    let ruff_config = first_existing(repo, &["ruff.toml", ".ruff.toml"])
        .or_else(|| contains_text(&pyproject, "[tool.ruff]").then_some(pyproject.clone()));
    let mypy_config = first_existing(repo, &["mypy.ini", ".mypy.ini"])
        .or_else(|| contains_text(&pyproject, "[tool.mypy]").then_some(pyproject.clone()));
    let pyright_config = first_existing(repo, &["pyrightconfig.json"])
        .or_else(|| contains_text(&pyproject, "[tool.pyright]").then_some(pyproject));

    Ok(vec![
        StaticCheckPlan {
            id: "python-compileall".to_string(),
            language: "python",
            title: "Python compileall".to_string(),
            tool: "python".to_string(),
            command: vec![
                "python".to_string(),
                "-m".to_string(),
                "compileall".to_string(),
                "-q".to_string(),
                ".".to_string(),
            ],
            configured: has_python,
            applicable: has_python,
            skip_reason: (!has_python).then(|| "no Python files were discovered".to_string()),
            config_path: None,
        },
        StaticCheckPlan {
            id: "python-ruff".to_string(),
            language: "python",
            title: "Python ruff".to_string(),
            tool: "ruff".to_string(),
            command: vec!["ruff".to_string(), "check".to_string(), ".".to_string()],
            configured: ruff_config.is_some(),
            applicable: has_python && ruff_config.is_some(),
            skip_reason: if has_python {
                ruff_config
                    .is_none()
                    .then(|| "ruff is not configured".to_string())
            } else {
                Some("no Python files were discovered".to_string())
            },
            config_path: ruff_config,
        },
        StaticCheckPlan {
            id: "python-mypy".to_string(),
            language: "python",
            title: "Python mypy".to_string(),
            tool: "mypy".to_string(),
            command: vec!["mypy".to_string(), ".".to_string()],
            configured: mypy_config.is_some(),
            applicable: has_python && mypy_config.is_some(),
            skip_reason: if has_python {
                mypy_config
                    .is_none()
                    .then(|| "mypy is not configured".to_string())
            } else {
                Some("no Python files were discovered".to_string())
            },
            config_path: mypy_config,
        },
        StaticCheckPlan {
            id: "python-pyright".to_string(),
            language: "python",
            title: "Python pyright".to_string(),
            tool: "pyright".to_string(),
            command: vec!["pyright".to_string(), ".".to_string()],
            configured: pyright_config.is_some(),
            applicable: has_python && pyright_config.is_some(),
            skip_reason: if has_python {
                pyright_config
                    .is_none()
                    .then(|| "pyright is not configured".to_string())
            } else {
                Some("no Python files were discovered".to_string())
            },
            config_path: pyright_config,
        },
    ])
}

fn discover_typescript(repo: &Path) -> Result<Vec<StaticCheckPlan>> {
    let package_json = repo.join("package.json");
    let tsconfig = repo.join("tsconfig.json");
    let has_package = package_json.exists();
    let has_tsconfig = tsconfig.exists();
    let has_ts_files = has_extension(repo, "ts")? || has_extension(repo, "tsx")?;
    let lint_script = has_package && contains_text(&package_json, "\"lint\"");
    let package_manager = package_manager(repo);
    let tsc_command = typescript_tsc_command(&package_manager);

    Ok(vec![
        StaticCheckPlan {
            id: "typescript-tsc".to_string(),
            language: "typescript",
            title: "TypeScript tsc --noEmit".to_string(),
            tool: package_manager.clone(),
            command: tsc_command,
            configured: has_tsconfig,
            applicable: has_ts_files && has_package && has_tsconfig,
            skip_reason: if !has_ts_files {
                Some("no TypeScript files were discovered".to_string())
            } else if !has_package {
                Some("package.json was not found".to_string())
            } else {
                (!has_tsconfig).then(|| "tsconfig.json was not found".to_string())
            },
            config_path: has_tsconfig.then_some(tsconfig),
        },
        StaticCheckPlan {
            id: "typescript-lint".to_string(),
            language: "typescript",
            title: "TypeScript lint script".to_string(),
            tool: package_manager.clone(),
            command: vec![package_manager, "run".to_string(), "lint".to_string()],
            configured: lint_script,
            applicable: has_ts_files && has_package && lint_script,
            skip_reason: if !has_ts_files {
                Some("no TypeScript files were discovered".to_string())
            } else if !has_package {
                Some("package.json was not found".to_string())
            } else {
                (!lint_script).then(|| "package.json has no lint script".to_string())
            },
            config_path: has_package.then_some(package_json),
        },
    ])
}

fn discover_rust(repo: &Path) -> Result<Vec<StaticCheckPlan>> {
    let cargo_toml = repo.join("Cargo.toml");
    let has_rust = has_extension(repo, "rs")?;
    let has_manifest = cargo_toml.exists();

    Ok(vec![
        StaticCheckPlan {
            id: "rust-cargo-check".to_string(),
            language: "rust",
            title: "Rust cargo check".to_string(),
            tool: "cargo".to_string(),
            command: vec!["cargo".to_string(), "check".to_string()],
            configured: has_manifest,
            applicable: has_rust && has_manifest,
            skip_reason: if has_rust {
                (!has_manifest).then(|| "Cargo.toml was not found".to_string())
            } else {
                Some("no Rust files were discovered".to_string())
            },
            config_path: has_manifest.then_some(cargo_toml.clone()),
        },
        StaticCheckPlan {
            id: "rust-cargo-test-no-run".to_string(),
            language: "rust",
            title: "Rust cargo test --no-run".to_string(),
            tool: "cargo".to_string(),
            command: vec![
                "cargo".to_string(),
                "test".to_string(),
                "--no-run".to_string(),
            ],
            configured: has_manifest,
            applicable: has_rust && has_manifest,
            skip_reason: if has_rust {
                (!has_manifest).then(|| "Cargo.toml was not found".to_string())
            } else {
                Some("no Rust files were discovered".to_string())
            },
            config_path: has_manifest.then_some(cargo_toml),
        },
        StaticCheckPlan {
            id: "rust-cargo-clippy".to_string(),
            language: "rust",
            title: "Rust cargo clippy".to_string(),
            tool: "cargo".to_string(),
            command: vec![
                "cargo".to_string(),
                "clippy".to_string(),
                "--all-targets".to_string(),
                "--no-deps".to_string(),
            ],
            configured: has_manifest,
            applicable: has_rust && has_manifest,
            skip_reason: if has_rust {
                (!has_manifest).then(|| "Cargo.toml was not found".to_string())
            } else {
                Some("no Rust files were discovered".to_string())
            },
            config_path: has_manifest.then_some(repo.join("Cargo.toml")),
        },
    ])
}

fn run_or_skip_check(repo: &Path, plan: &StaticCheckPlan) -> Result<Receipt> {
    let started_at = Utc::now();
    let timer = Instant::now();
    let mut metadata = BTreeMap::new();
    metadata.insert("language".to_string(), plan.language.to_string());
    metadata.insert("command".to_string(), plan.command.join(" "));
    metadata.insert("configured".to_string(), plan.configured.to_string());

    if let Some(config_path) = &plan.config_path {
        metadata.insert("config_path".to_string(), portable_path(config_path));
    }

    if !plan.applicable {
        return Ok(receipt(
            plan,
            StageStatus::NotApplicable,
            started_at,
            timer,
            None,
            ReceiptSummary {
                title: format!("{} not applicable", plan.title),
                details: plan
                    .skip_reason
                    .clone()
                    .unwrap_or_else(|| "check is not applicable to this repository".to_string()),
            },
            vec![],
            vec!["R-031".to_string(), "R-032".to_string()],
            vec!["R-038".to_string()],
            vec![format!(
                "No static evidence was produced because {}.",
                plan.skip_reason
                    .as_deref()
                    .unwrap_or("the check was not applicable")
            )],
            metadata,
        ));
    }

    if !tool_available(&plan.tool) {
        metadata.insert("missing_tool".to_string(), plan.tool.clone());
        return Ok(receipt(
            plan,
            StageStatus::Skipped,
            started_at,
            timer,
            None,
            ReceiptSummary {
                title: format!("{} skipped", plan.title),
                details: format!("tool `{}` was not available on PATH", plan.tool),
            },
            vec![],
            vec!["R-031".to_string(), "R-032".to_string()],
            vec![],
            vec![format!(
                "Configured static check could not run because `{}` was unavailable.",
                plan.tool
            )],
            metadata,
        ));
    }

    let output = Command::new(&plan.command[0])
        .args(&plan.command[1..])
        .current_dir(repo)
        .output()
        .with_context(|| format!("running {}", plan.command.join(" ")))?;
    let combined_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let categories = classify_static_hallucinations(&combined_output);
    if !categories.is_empty() {
        metadata.insert(
            "hallucination_categories".to_string(),
            categories
                .iter()
                .map(|category| category.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );
    }

    let status = if output.status.success() {
        StageStatus::Passed
    } else {
        StageStatus::Failed
    };
    let mut residual_risks = Vec::new();
    if status == StageStatus::Failed {
        residual_risks.push("R-038".to_string());
    }

    Ok(receipt(
        plan,
        status,
        started_at,
        timer,
        output.status.code(),
        ReceiptSummary {
            title: if output.status.success() {
                format!("{} passed", plan.title)
            } else {
                format!("{} failed", plan.title)
            },
            details: summarize_output(&combined_output),
        },
        vec![
            "R-031".to_string(),
            "R-032".to_string(),
            "R-038".to_string(),
        ],
        residual_risks,
        vec![],
        vec![
            "Static checks are command evidence only; they do not prove semantic correctness."
                .to_string(),
        ],
        metadata,
    ))
}

#[allow(clippy::too_many_arguments)]
fn receipt(
    plan: &StaticCheckPlan,
    status: StageStatus,
    started_at: chrono::DateTime<Utc>,
    timer: Instant,
    exit_code: Option<i32>,
    summary: ReceiptSummary,
    mitigated_risks: Vec<String>,
    residual_risks: Vec<String>,
    not_applicable_risks: Vec<String>,
    limitations: Vec<String>,
    metadata: BTreeMap<String, String>,
) -> Receipt {
    Receipt {
        schema_version: RECEIPT_SCHEMA_VERSION.to_string(),
        stage: format!("static_{}", plan.id.replace('-', "_")),
        status,
        tool: ToolIdentity::new(
            format!("pramaan-{}-static", plan.language),
            env!("CARGO_PKG_VERSION"),
        ),
        started_at: timestamp(started_at),
        ended_at: timestamp(Utc::now()),
        exit_code,
        inputs: vec![InputRef {
            name: "repository".to_string(),
            value: ".".to_string(),
            digest: None,
        }],
        outputs: vec![OutputRef {
            name: "static_receipt".to_string(),
            path: format!("receipts/static/{}.receipt.json", plan.id),
            digest: None,
        }],
        artifacts: vec![ArtifactRef {
            name: "static_command".to_string(),
            path: plan.command.join(" "),
            media_type: Some("text/x-shellscript".to_string()),
            digest: None,
        }],
        summary,
        limitations,
        mitigated_risks,
        residual_risks,
        not_applicable_risks,
        agent_author: None,
        reviewer_override: None,
        multi_agent_provenance: Vec::new(),
        plugin_identity: None,
        plugin_permissions: None,
        evidence_sensitivity: None,
        redaction_manifest: None,
        policy_decision: None,
        stage_budget: None,
        metadata: metadata
            .into_iter()
            .chain([(
                "duration_ms".to_string(),
                timer.elapsed().as_millis().to_string(),
            )])
            .collect(),
    }
}

fn tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn has_extension(repo: &Path, extension: &str) -> Result<bool> {
    Ok(walk_files(repo)?.iter().any(|path| {
        path.extension()
            .and_then(OsStr::to_str)
            .map(|actual| actual.eq_ignore_ascii_case(extension))
            .unwrap_or(false)
    }))
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

fn first_existing(repo: &Path, names: &[&str]) -> Option<PathBuf> {
    names
        .iter()
        .map(|name| repo.join(name))
        .find(|path| path.exists())
}

fn contains_text(path: &Path, needle: &str) -> bool {
    fs::read_to_string(path)
        .map(|text| text.contains(needle))
        .unwrap_or(false)
}

fn package_manager(repo: &Path) -> String {
    if repo.join("pnpm-lock.yaml").exists() {
        "pnpm".to_string()
    } else if repo.join("yarn.lock").exists() {
        "yarn".to_string()
    } else {
        "npm".to_string()
    }
}

fn typescript_tsc_command(package_manager: &str) -> Vec<String> {
    match package_manager {
        "npm" => vec![
            "npm".to_string(),
            "exec".to_string(),
            "--offline".to_string(),
            "--".to_string(),
            "tsc".to_string(),
            "--noEmit".to_string(),
        ],
        "yarn" => vec![
            "yarn".to_string(),
            "tsc".to_string(),
            "--noEmit".to_string(),
        ],
        _ => vec![
            package_manager.to_string(),
            "exec".to_string(),
            "tsc".to_string(),
            "--noEmit".to_string(),
        ],
    }
}

fn summarize_output(output: &str) -> String {
    let lines = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .take(8)
        .collect::<Vec<_>>();

    if lines.is_empty() {
        "command produced no output".to_string()
    } else {
        lines.join("\n")
    }
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("creating parent directory {}", parent.display()))?;
    }

    let bytes = serde_json::to_vec_pretty(value).context("serializing JSON artifact")?;
    fs::write(path, bytes).with_context(|| format!("writing {}", path.display()))
}

fn digest_receipt(path: &Path) -> Option<String> {
    fs::read(path).ok().map(sha256_hex)
}

fn render_static_summary(repo: &Path, out: &Path, receipts: &[(Receipt, PathBuf)]) {
    println!("Pramaan static checks complete");
    println!("repo: {}", repo.display());
    println!("bundle: {}", out.display());
    println!();
    println!("Stages");
    println!("{:<32} {:<16} receipt", "stage", "status");

    for (receipt, path) in receipts {
        let digest = digest_receipt(path)
            .map(|value| value.replace("sha256:", "").chars().take(12).collect())
            .unwrap_or_else(|| "unavailable".to_string());
        println!(
            "{:<32} {:<16} {} sha256:{}",
            receipt.stage,
            receipt.status.as_str(),
            path.display(),
            digest
        );
    }

    let categories = receipts
        .iter()
        .filter_map(|(receipt, _)| receipt.metadata.get("hallucination_categories"))
        .flat_map(|raw| raw.split(','))
        .collect::<BTreeSet<_>>();

    if !categories.is_empty() {
        println!();
        println!(
            "Hallucination categories: {}",
            categories.into_iter().collect::<Vec<_>>().join(", ")
        );
    }
}

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
