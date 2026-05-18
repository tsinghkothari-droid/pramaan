use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestArtifact {
    pub name: String,
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleManifest {
    pub schema_version: String,
    pub receipts: Vec<ManifestArtifact>,
    pub artifacts: Vec<ManifestArtifact>,
    pub signing: SigningPlaceholder,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SigningPlaceholder {
    pub mode: String,
    pub note: String,
}

impl BundleManifest {
    pub fn placeholder() -> Self {
        Self {
            schema_version: "pramaan.bundle.v1".to_string(),
            receipts: Vec::new(),
            artifacts: Vec::new(),
            signing: SigningPlaceholder {
                mode: "unsigned_phase_1_placeholder".to_string(),
                note: "Phase 1 records signable structure only; real signing is deferred."
                    .to_string(),
            },
        }
    }
}

pub fn sha256_hex(bytes: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(bytes.as_ref());
    format!("sha256:{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_with_sha256_prefix() {
        assert_eq!(
            sha256_hex("pramaan"),
            "sha256:8c5f42527c4bdf03d8d19b4d8e25f530367751d7ea75ccdaf760b50482449cc0"
        );
    }
}
