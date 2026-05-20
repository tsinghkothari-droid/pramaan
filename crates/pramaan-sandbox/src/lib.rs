use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SANDBOX_EVIDENCE_SCHEMA_VERSION: &str = "pramaan.sandbox_evidence.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxPlan {
    pub base_ref: String,
    pub head_ref: String,
    pub mode: SandboxMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxMode {
    SyntheticOnly,
    IsolatedWorktree,
}

impl SandboxPlan {
    pub fn synthetic(base_ref: impl Into<String>, head_ref: impl Into<String>) -> Self {
        Self {
            base_ref: base_ref.into(),
            head_ref: head_ref.into(),
            mode: SandboxMode::SyntheticOnly,
        }
    }

    pub fn isolated_worktree(base_ref: impl Into<String>, head_ref: impl Into<String>) -> Self {
        Self {
            base_ref: base_ref.into(),
            head_ref: head_ref.into(),
            mode: SandboxMode::IsolatedWorktree,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxEvidence {
    pub schema_version: String,
    pub mode: SandboxMode,
    pub base: WorktreeEvidence,
    pub head: WorktreeEvidence,
    pub source_dirty_state: DirtyState,
    pub source_after_setup_dirty_state: DirtyState,
    pub source_changed_after_setup: bool,
    pub repository: RepositoryEvidence,
    pub environment: EnvironmentEvidence,
    pub network_policy: NetworkPolicyEvidence,
    pub hermetic: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_identity_source: Option<String>,
    pub limitations: Vec<String>,
    pub mitigated_risks: Vec<String>,
    pub residual_risks: Vec<String>,
    pub not_applicable_risks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeEvidence {
    pub requested_ref: String,
    pub commit_sha: String,
    pub path: String,
    pub dirty_state: DirtyState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirtyState {
    pub is_dirty: bool,
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryEvidence {
    pub lockfiles: Vec<FileHash>,
    pub missing_lockfiles: Vec<String>,
    pub lockfile_drift: Vec<LockfileDrift>,
    pub config_files: Vec<FileHash>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileDrift {
    pub path: String,
    pub base_digest: Option<String>,
    pub head_digest: Option<String>,
    pub changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileHash {
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentEvidence {
    pub os: String,
    pub arch: String,
    pub family: String,
    pub shell: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub git_version: String,
    pub rustc_version: Option<String>,
    pub cargo_version: Option<String>,
    pub node_version: Option<String>,
    pub npm_version: Option<String>,
    pub python_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkPolicyEvidence {
    pub policy: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerIdentityEvidence {
    pub image_name: Option<String>,
    pub image_digest: Option<String>,
    pub source: String,
}

pub struct SandboxRun {
    pub evidence: SandboxEvidence,
    base_guard: WorktreeGuard,
    head_guard: WorktreeGuard,
}

impl SandboxRun {
    pub fn worktree_paths(&self) -> (&Path, &Path) {
        (&self.base_guard.path, &self.head_guard.path)
    }
}

pub struct SandboxRunner {
    repo_path: PathBuf,
    out_dir: PathBuf,
    image_name: Option<String>,
    image_digest: Option<String>,
    network_policy: Option<String>,
}

impl SandboxRunner {
    pub fn new(repo_path: impl Into<PathBuf>, out_dir: impl Into<PathBuf>) -> Self {
        Self {
            repo_path: repo_path.into(),
            out_dir: out_dir.into(),
            image_name: None,
            image_digest: None,
            network_policy: None,
        }
    }

    pub fn with_image_name(mut self, image_name: impl Into<String>) -> Self {
        self.image_name = Some(image_name.into());
        self
    }

    pub fn with_image_digest(mut self, image_digest: impl Into<String>) -> Self {
        self.image_digest = Some(image_digest.into());
        self
    }

    pub fn with_network_policy(mut self, network_policy: impl Into<String>) -> Self {
        self.network_policy = Some(network_policy.into());
        self
    }

    pub fn prepare(&self, plan: &SandboxPlan) -> Result<SandboxRun, SandboxError> {
        if plan.mode != SandboxMode::IsolatedWorktree {
            return Err(SandboxError::UnsupportedMode);
        }

        let source_dirty_state = dirty_state(&self.repo_path)?;
        let base_sha = resolve_commit(&self.repo_path, &plan.base_ref)?;
        let head_sha = resolve_commit(&self.repo_path, &plan.head_ref)?;

        fs::create_dir_all(&self.out_dir).map_err(|source| SandboxError::Io {
            action: "create sandbox output directory".to_string(),
            path: self.out_dir.clone(),
            source,
        })?;

        let worktree_root = self.out_dir.join(unique_dir_name("worktrees"));
        fs::create_dir_all(&worktree_root).map_err(|source| SandboxError::Io {
            action: "create worktree root".to_string(),
            path: worktree_root.clone(),
            source,
        })?;

        let base_guard = add_worktree(&self.repo_path, worktree_root.join("base"), &plan.base_ref)?;
        let head_guard = add_worktree(&self.repo_path, worktree_root.join("head"), &plan.head_ref)?;

        let repository = repository_evidence(&self.repo_path, &base_guard.path, &head_guard.path)?;
        let environment = environment_evidence()?;
        let network_policy = network_policy_evidence(self.network_policy.as_deref());
        let detected_identity = detect_container_identity_from_env();
        let image_name = self.image_name.clone().or_else(|| {
            detected_identity
                .as_ref()
                .and_then(|identity| identity.image_name.clone())
        });
        let image_digest = self.image_digest.clone().or_else(|| {
            detected_identity
                .as_ref()
                .and_then(|identity| identity.image_digest.clone())
        });
        let image_identity_source = if self.image_name.is_some() || self.image_digest.is_some() {
            Some("explicit_runner_config".to_string())
        } else {
            detected_identity.map(|identity| identity.source)
        };
        let source_after_setup_dirty_state = dirty_state(&self.repo_path)?;
        let source_changed_after_setup =
            relevant_dirty_entries(
                &source_after_setup_dirty_state,
                &self.repo_path,
                &self.out_dir,
            ) != relevant_dirty_entries(&source_dirty_state, &self.repo_path, &self.out_dir);
        let hermetic = image_digest.is_some() && !source_dirty_state.is_dirty;
        let limitations = limitations(
            hermetic,
            image_digest.is_some(),
            &source_dirty_state,
            &repository,
            source_changed_after_setup,
        );
        let (mitigated_risks, residual_risks, not_applicable_risks) = sandbox_risks(
            hermetic,
            image_digest.is_some(),
            &repository,
            source_changed_after_setup,
        );

        let evidence = SandboxEvidence {
            schema_version: SANDBOX_EVIDENCE_SCHEMA_VERSION.to_string(),
            mode: plan.mode.clone(),
            base: WorktreeEvidence {
                requested_ref: plan.base_ref.clone(),
                commit_sha: base_sha,
                path: portable_path(&base_guard.path),
                dirty_state: dirty_state(&base_guard.path)?,
            },
            head: WorktreeEvidence {
                requested_ref: plan.head_ref.clone(),
                commit_sha: head_sha,
                path: portable_path(&head_guard.path),
                dirty_state: dirty_state(&head_guard.path)?,
            },
            source_dirty_state,
            source_after_setup_dirty_state,
            source_changed_after_setup,
            repository,
            environment,
            network_policy,
            hermetic,
            image_name,
            image_digest,
            image_identity_source,
            limitations,
            mitigated_risks,
            residual_risks,
            not_applicable_risks,
        };

        Ok(SandboxRun {
            evidence,
            base_guard,
            head_guard,
        })
    }
}

struct WorktreeGuard {
    repo_path: PathBuf,
    path: PathBuf,
}

impl Drop for WorktreeGuard {
    fn drop(&mut self) {
        let _ = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["worktree", "remove", "--force"])
            .arg(&self.path)
            .output();

        let _ = fs::remove_dir_all(&self.path);
        if let Some(parent) = self.path.parent() {
            let _ = fs::remove_dir_all(parent);
        }
    }
}

#[derive(Debug)]
pub enum SandboxError {
    UnsupportedMode,
    Io {
        action: String,
        path: PathBuf,
        source: std::io::Error,
    },
    Command {
        program: String,
        args: Vec<String>,
        status: Option<i32>,
        stderr: String,
    },
}

impl fmt::Display for SandboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMode => {
                write!(formatter, "sandbox runner requires isolated_worktree mode")
            }
            Self::Io {
                action,
                path,
                source,
            } => write!(formatter, "{action} at {}: {source}", path.display()),
            Self::Command {
                program,
                args,
                status,
                stderr,
            } => write!(
                formatter,
                "{program} {} failed with status {:?}: {}",
                args.join(" "),
                status,
                stderr.trim()
            ),
        }
    }
}

impl std::error::Error for SandboxError {}

fn add_worktree(
    repo_path: &Path,
    path: PathBuf,
    git_ref: &str,
) -> Result<WorktreeGuard, SandboxError> {
    run_command(
        repo_path,
        "git",
        &[
            "worktree".to_string(),
            "add".to_string(),
            "--detach".to_string(),
            path.to_string_lossy().to_string(),
            git_ref.to_string(),
        ],
    )?;

    Ok(WorktreeGuard {
        repo_path: repo_path.to_path_buf(),
        path,
    })
}

fn resolve_commit(repo_path: &Path, git_ref: &str) -> Result<String, SandboxError> {
    run_command(
        repo_path,
        "git",
        &[
            "rev-parse".to_string(),
            "--verify".to_string(),
            format!("{git_ref}^{{commit}}"),
        ],
    )
}

fn dirty_state(repo_path: &Path) -> Result<DirtyState, SandboxError> {
    let output = run_command(
        repo_path,
        "git",
        &[
            "status".to_string(),
            "--porcelain=v1".to_string(),
            "--untracked-files=all".to_string(),
        ],
    )?;
    let entries = output
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    Ok(DirtyState {
        is_dirty: !entries.is_empty(),
        entries,
    })
}

fn relevant_dirty_entries(
    state: &DirtyState,
    repo_path: &Path,
    ignored_path: &Path,
) -> Vec<String> {
    let ignored_prefix = ignored_path
        .strip_prefix(repo_path)
        .ok()
        .map(portable_path)
        .map(|path| path.trim_end_matches('/').to_string());

    state
        .entries
        .iter()
        .filter(|entry| {
            let Some(prefix) = ignored_prefix.as_deref() else {
                return true;
            };
            let normalized = entry.replace('\\', "/");
            !normalized.contains(prefix)
        })
        .cloned()
        .collect()
}

fn repository_evidence(
    repo_path: &Path,
    base_path: &Path,
    head_path: &Path,
) -> Result<RepositoryEvidence, SandboxError> {
    let lockfile_candidates = [
        "Cargo.lock",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "bun.lockb",
        "poetry.lock",
        "uv.lock",
    ];
    let config_candidates = [
        "Cargo.toml",
        "rust-toolchain.toml",
        ".cargo/config.toml",
        ".cargo/config",
        "package.json",
        "pyproject.toml",
    ];

    let lockfiles = existing_hashes(repo_path, &lockfile_candidates)?;
    let missing_lockfiles = if lockfiles.is_empty() {
        vec!["no recognized lockfile found".to_string()]
    } else {
        Vec::new()
    };

    Ok(RepositoryEvidence {
        lockfiles,
        missing_lockfiles,
        lockfile_drift: lockfile_drift(base_path, head_path, &lockfile_candidates)?,
        config_files: existing_hashes(repo_path, &config_candidates)?,
    })
}

fn lockfile_drift(
    base_path: &Path,
    head_path: &Path,
    candidates: &[&str],
) -> Result<Vec<LockfileDrift>, SandboxError> {
    let mut drift = Vec::new();
    for candidate in candidates {
        let base_digest = file_digest(base_path, candidate)?;
        let head_digest = file_digest(head_path, candidate)?;
        if base_digest.is_some() || head_digest.is_some() {
            let changed = base_digest != head_digest;
            drift.push(LockfileDrift {
                path: candidate.replace('\\', "/"),
                base_digest,
                head_digest,
                changed,
            });
        }
    }
    Ok(drift)
}

fn file_digest(repo_path: &Path, relative_path: &str) -> Result<Option<String>, SandboxError> {
    let path = repo_path.join(relative_path);
    if !path.exists() {
        return Ok(None);
    }
    let digest = run_command(
        repo_path,
        "git",
        &[
            "hash-object".to_string(),
            "--".to_string(),
            relative_path.to_string(),
        ],
    )?;
    Ok(Some(format!("git-blob:{digest}")))
}

fn existing_hashes(repo_path: &Path, candidates: &[&str]) -> Result<Vec<FileHash>, SandboxError> {
    let mut hashes = Vec::new();

    for candidate in candidates {
        let path = repo_path.join(candidate);
        if path.exists() {
            let digest = run_command(
                repo_path,
                "git",
                &[
                    "hash-object".to_string(),
                    "--".to_string(),
                    candidate.to_string(),
                ],
            )?;
            hashes.push(FileHash {
                path: candidate.replace('\\', "/"),
                digest: format!("git-blob:{digest}"),
            });
        }
    }

    Ok(hashes)
}

fn environment_evidence() -> Result<EnvironmentEvidence, SandboxError> {
    Ok(EnvironmentEvidence {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        family: std::env::consts::FAMILY.to_string(),
        shell: std::env::var("SHELL")
            .ok()
            .or_else(|| std::env::var("ComSpec").ok()),
        timezone: std::env::var("TZ").ok(),
        locale: std::env::var("LC_ALL")
            .ok()
            .or_else(|| std::env::var("LANG").ok()),
        git_version: run_command(Path::new("."), "git", &["--version".to_string()])?,
        rustc_version: optional_command("rustc", &["--version".to_string()]),
        cargo_version: optional_command("cargo", &["--version".to_string()]),
        node_version: optional_command("node", &["--version".to_string()]),
        npm_version: optional_command("npm", &["--version".to_string()]),
        python_version: optional_command("python", &["--version".to_string()]),
    })
}

fn network_policy_evidence(policy: Option<&str>) -> NetworkPolicyEvidence {
    let policy = policy
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("unknown");
    NetworkPolicyEvidence {
        policy: policy.to_string(),
        source: if policy == "unknown" {
            "default_unknown".to_string()
        } else {
            "PRAMAAN_NETWORK_POLICY".to_string()
        },
    }
}

fn detect_container_identity_from_env() -> Option<ContainerIdentityEvidence> {
    detect_container_identity_from_pairs(std::env::vars())
}

pub fn detect_container_identity_from_pairs<I, K, V>(pairs: I) -> Option<ContainerIdentityEvidence>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let mut image_name = None;
    let mut image_digest = None;
    let mut sources = Vec::new();

    for (key, value) in pairs {
        let key = key.as_ref().to_ascii_uppercase();
        let value = value.as_ref().trim();
        if value.is_empty() {
            continue;
        }

        match key.as_str() {
            "CONTAINER_IMAGE"
            | "OCI_IMAGE_NAME"
            | "IMAGE_NAME"
            | "DEVCONTAINER_IMAGE"
            | "GITHUB_ACTIONS_RUNNER_CONTAINER_IMAGE" => {
                if image_name.is_none() {
                    image_name = Some(value.to_string());
                    sources.push(format!("env:{key}"));
                }
            }
            "CONTAINER_IMAGE_DIGEST"
            | "OCI_IMAGE_DIGEST"
            | "IMAGE_DIGEST"
            | "GITHUB_ACTIONS_RUNNER_CONTAINER_DIGEST" => {
                if image_digest.is_none() {
                    image_digest = Some(value.to_string());
                    sources.push(format!("env:{key}"));
                }
            }
            _ => {}
        }
    }

    if image_name.is_some() || image_digest.is_some() {
        Some(ContainerIdentityEvidence {
            image_name,
            image_digest,
            source: sources.join(","),
        })
    } else {
        None
    }
}

fn optional_command(program: &str, args: &[String]) -> Option<String> {
    run_command(Path::new("."), program, args).ok()
}

fn run_command(cwd: &Path, program: &str, args: &[String]) -> Result<String, SandboxError> {
    let output = Command::new(program)
        .current_dir(cwd)
        .args(args)
        .output()
        .map_err(|source| SandboxError::Io {
            action: format!("run {program}"),
            path: cwd.to_path_buf(),
            source,
        })?;

    if !output.status.success() {
        return Err(SandboxError::Command {
            program: program.to_string(),
            args: args.to_vec(),
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn limitations(
    hermetic: bool,
    has_image_digest: bool,
    source_dirty_state: &DirtyState,
    repository: &RepositoryEvidence,
    source_changed_after_setup: bool,
) -> Vec<String> {
    let mut limitations = Vec::new();

    if !has_image_digest {
        limitations.push(
            "No container or VM image digest was supplied; host runtime may affect results."
                .to_string(),
        );
    }

    if source_dirty_state.is_dirty {
        limitations.push(
            "Source checkout had uncommitted or untracked files when sandbox evidence was captured."
                .to_string(),
        );
    }

    if source_changed_after_setup {
        limitations.push(
            "Source checkout changed while sandbox setup was running; verify stage isolation before trusting downstream evidence."
                .to_string(),
        );
    }

    if !hermetic {
        limitations.push(
            "Sandbox worktrees isolate Git refs, but this run is not fully hermetic.".to_string(),
        );
    }

    if repository.lockfile_drift.iter().any(|entry| entry.changed) {
        limitations.push(
            "One or more dependency lockfiles changed between base and head; dependency drift may affect verification results."
                .to_string(),
        );
    }

    limitations
}

fn sandbox_risks(
    hermetic: bool,
    has_image_digest: bool,
    repository: &RepositoryEvidence,
    source_changed_after_setup: bool,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut mitigated = vec![
        "R-021".to_string(),
        "R-022".to_string(),
        "R-023".to_string(),
        "R-024".to_string(),
        "R-025".to_string(),
        "R-026".to_string(),
        "R-027".to_string(),
    ];
    let mut residual = Vec::new();
    let not_applicable = Vec::new();

    if hermetic {
        mitigated.extend([
            "R-028".to_string(),
            "R-029".to_string(),
            "R-033".to_string(),
        ]);
    } else {
        residual.extend([
            "R-028".to_string(),
            "R-029".to_string(),
            "R-033".to_string(),
        ]);
    }

    let has_lockfile_drift = repository.lockfile_drift.iter().any(|entry| entry.changed);
    if repository.missing_lockfiles.is_empty() && !has_lockfile_drift {
        mitigated.push("R-030".to_string());
    } else {
        residual.push("R-030".to_string());
    }

    if has_image_digest {
        mitigated.extend(["R-031".to_string(), "R-032".to_string()]);
    } else {
        residual.extend(["R-031".to_string(), "R-032".to_string()]);
    }

    if source_changed_after_setup {
        residual.push("R-034".to_string());
    }

    (mitigated, residual, not_applicable)
}

fn unique_dir_name(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{prefix}-{}-{nanos}", std::process::id())
}

fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_repo_creates_base_and_head_worktrees_then_cleans_them() {
        let repo = FixtureRepo::new("clean");
        repo.write(
            "Cargo.toml",
            "[package]\nname = \"clean\"\nversion = \"0.1.0\"\n",
        );
        repo.write("Cargo.lock", "# lock\n");
        repo.commit("initial");
        repo.write("src/lib.rs", "pub fn answer() -> i32 { 42 }\n");
        repo.commit("head");

        let run = SandboxRunner::new(repo.path(), repo.path().join("target/pramaan-sandbox"))
            .with_image_name("ghcr.io/pramaan/test:latest")
            .with_image_digest("sha256:test-image")
            .with_network_policy("disabled")
            .prepare(&SandboxPlan::isolated_worktree("HEAD~1", "HEAD"))
            .expect("prepare sandbox");
        let (base_path, head_path) = run.worktree_paths();
        let base_path = base_path.to_path_buf();
        let head_path = head_path.to_path_buf();

        assert!(base_path.exists());
        assert!(head_path.exists());
        assert_eq!(run.evidence.base.commit_sha.len(), 40);
        assert_eq!(run.evidence.head.commit_sha.len(), 40);
        assert!(!run.evidence.source_dirty_state.is_dirty);
        assert!(run
            .evidence
            .repository
            .lockfiles
            .iter()
            .any(|file| file.path == "Cargo.lock"));
        assert!(run.evidence.hermetic);
        assert_eq!(
            run.evidence.image_name.as_deref(),
            Some("ghcr.io/pramaan/test:latest")
        );
        assert_eq!(
            run.evidence.image_identity_source.as_deref(),
            Some("explicit_runner_config")
        );
        assert!(!run.evidence.source_changed_after_setup);
        assert_eq!(run.evidence.network_policy.policy, "disabled");
        assert!(run.evidence.environment.shell.is_some());
        assert!(run.evidence.residual_risks.is_empty());

        drop(run);

        assert!(!base_path.exists());
        assert!(!head_path.exists());
    }

    #[test]
    fn dirty_repo_records_source_dirty_state() {
        let repo = FixtureRepo::new("dirty");
        repo.write(
            "Cargo.toml",
            "[package]\nname = \"dirty\"\nversion = \"0.1.0\"\n",
        );
        repo.write("Cargo.lock", "# lock\n");
        repo.commit("initial");
        repo.write("untracked.txt", "dirty\n");

        let run = SandboxRunner::new(repo.path(), repo.path().join("target/pramaan-sandbox"))
            .with_image_digest("sha256:test-image")
            .prepare(&SandboxPlan::isolated_worktree("HEAD", "HEAD"))
            .expect("prepare sandbox");

        assert!(run.evidence.source_dirty_state.is_dirty);
        assert!(run
            .evidence
            .source_dirty_state
            .entries
            .iter()
            .any(|entry| entry.contains("untracked.txt")));
        assert!(!run.evidence.hermetic);
        assert!(run.evidence.residual_risks.contains(&"R-028".to_string()));
    }

    #[test]
    fn missing_lockfile_is_captured_as_residual_risk() {
        let repo = FixtureRepo::new("missing-lockfile");
        repo.write(
            "Cargo.toml",
            "[package]\nname = \"missing\"\nversion = \"0.1.0\"\n",
        );
        repo.commit("initial");

        let run = SandboxRunner::new(repo.path(), repo.path().join("target/pramaan-sandbox"))
            .with_image_digest("sha256:test-image")
            .prepare(&SandboxPlan::isolated_worktree("HEAD", "HEAD"))
            .expect("prepare sandbox");

        assert!(run.evidence.repository.lockfiles.is_empty());
        assert_eq!(
            run.evidence.repository.missing_lockfiles,
            vec!["no recognized lockfile found".to_string()]
        );
        assert!(run.evidence.residual_risks.contains(&"R-030".to_string()));
    }

    #[test]
    fn lockfile_drift_is_captured_between_base_and_head() {
        let repo = FixtureRepo::new("lockfile-drift");
        repo.write(
            "Cargo.toml",
            "[package]\nname = \"lockfile_drift\"\nversion = \"0.1.0\"\n",
        );
        repo.write("Cargo.lock", "# lock v1\n");
        repo.commit("initial");
        repo.write("Cargo.lock", "# lock v2\n");
        repo.commit("head");

        let run = SandboxRunner::new(repo.path(), repo.path().join("target/pramaan-sandbox"))
            .with_image_digest("sha256:test-image")
            .prepare(&SandboxPlan::isolated_worktree("HEAD~1", "HEAD"))
            .expect("prepare sandbox");

        let drift = run
            .evidence
            .repository
            .lockfile_drift
            .iter()
            .find(|entry| entry.path == "Cargo.lock")
            .expect("Cargo.lock drift");
        assert!(drift.changed);
        assert_ne!(drift.base_digest, drift.head_digest);
    }

    #[test]
    fn missing_image_digest_marks_run_non_hermetic() {
        let repo = FixtureRepo::new("non-hermetic");
        repo.write(
            "Cargo.toml",
            "[package]\nname = \"non_hermetic\"\nversion = \"0.1.0\"\n",
        );
        repo.write("Cargo.lock", "# lock\n");
        repo.commit("initial");

        let run = SandboxRunner::new(repo.path(), repo.path().join("target/pramaan-sandbox"))
            .prepare(&SandboxPlan::isolated_worktree("HEAD", "HEAD"))
            .expect("prepare sandbox");

        assert!(!run.evidence.hermetic);
        assert!(run
            .evidence
            .limitations
            .iter()
            .any(|limitation| limitation.contains("No container or VM image digest")));
        assert!(run.evidence.residual_risks.contains(&"R-031".to_string()));
        assert!(run.evidence.residual_risks.contains(&"R-032".to_string()));
    }

    #[test]
    fn container_identity_can_be_detected_from_environment_pairs() {
        let identity = detect_container_identity_from_pairs([
            ("OCI_IMAGE_NAME", "ghcr.io/pramaan/runner:latest"),
            ("OCI_IMAGE_DIGEST", "sha256:abc123"),
        ])
        .expect("container identity");

        assert_eq!(
            identity.image_name.as_deref(),
            Some("ghcr.io/pramaan/runner:latest")
        );
        assert_eq!(identity.image_digest.as_deref(), Some("sha256:abc123"));
        assert!(identity.source.contains("env:OCI_IMAGE_NAME"));
        assert!(identity.source.contains("env:OCI_IMAGE_DIGEST"));
    }

    #[test]
    fn sandbox_example_fixtures_cover_required_cases() {
        let fixtures = [
            include_str!("../../../examples/fixtures/sandbox/clean.json"),
            include_str!("../../../examples/fixtures/sandbox/dirty.json"),
            include_str!("../../../examples/fixtures/sandbox/missing-lockfile.json"),
            include_str!("../../../examples/fixtures/sandbox/non-hermetic.json"),
        ];

        for fixture in fixtures {
            assert!(fixture.contains(SANDBOX_EVIDENCE_SCHEMA_VERSION));
            assert!(fixture.contains("\"R-021\""));
        }

        assert!(fixtures[0].contains("\"hermetic\": true"));
        assert!(fixtures[1].contains("\"source_dirty_state\""));
        assert!(fixtures[2].contains("no recognized lockfile found"));
        assert!(fixtures[3].contains("No container or VM image digest"));
    }

    struct FixtureRepo {
        path: PathBuf,
    }

    impl FixtureRepo {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(unique_dir_name(&format!("pramaan-{name}")));
            fs::create_dir_all(&path).expect("create fixture repo");
            run_fixture_git(&path, &["init"]);
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write(&self, relative: &str, content: &str) {
            let path = self.path.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create fixture parent");
            }
            fs::write(path, content).expect("write fixture file");
        }

        fn commit(&self, message: &str) {
            run_fixture_git(&self.path, &["add", "."]);
            run_fixture_git(
                &self.path,
                &[
                    "-c",
                    "user.name=Pramaan Test",
                    "-c",
                    "user.email=pramaan@example.invalid",
                    "commit",
                    "-m",
                    message,
                ],
            );
        }
    }

    impl Drop for FixtureRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn run_fixture_git(cwd: &Path, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(cwd)
            .args(args)
            .output()
            .expect("run fixture git");
        assert!(
            output.status.success(),
            "git {} failed\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
