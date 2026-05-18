use serde::{Deserialize, Serialize};

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
}
