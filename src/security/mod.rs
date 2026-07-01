//! Local-first security boundary for EngineVault.
//!
//! Phase 1 (desktop): all data stays on disk under the user's profile.
//! Phase 2 (SaaS): add TLS + OAuth2 here — never embed secrets in the binary.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

const APP_NAME: &str = "EngineVault";

/// Application directories — `%LOCALAPPDATA%\EngineVault\`.
pub struct AppPaths {
    pub data_dir: PathBuf,
    pub catalog_path: PathBuf,
    pub rules_dir: PathBuf,
    pub corrections_path: PathBuf,
    pub audit_log: PathBuf,
}

impl AppPaths {
    pub fn resolve() -> Result<Self> {
        let data_dir = directories::ProjectDirs::from("", "", APP_NAME)
            .map(|d| d.data_local_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(".").join(".enginevault"));

        std::fs::create_dir_all(&data_dir)
            .with_context(|| format!("create data dir {}", data_dir.display()))?;

        Ok(Self {
            catalog_path: data_dir.join("catalog.ntfsbin"),
            rules_dir: data_dir.join("rules"),
            corrections_path: data_dir.join("rules").join("user_corrections.json"),
            audit_log: data_dir.join("audit.log"),
            data_dir,
        })
    }

    pub fn ensure_rules(&self, bundled_rules: &Path) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.rules_dir)?;
        let dest = self.rules_dir.join("classification.toml");
        if !dest.exists() {
            let source = if bundled_rules.exists() {
                bundled_rules.to_path_buf()
            } else {
                Self::bundled_rules_next_to_exe()
            };
            if source.exists() {
                std::fs::copy(&source, &dest)?;
            }
        }
        Ok(dest)
    }

    /// Rules shipped beside the installed `.exe` (`Program Files\EngineVault\rules\`).
    pub fn bundled_rules_next_to_exe() -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("rules").join("classification.toml")))
            .unwrap_or_else(|| PathBuf::from("rules/classification.toml"))
    }

    pub fn classification_rules(&self) -> PathBuf {
        self.rules_dir.join("classification.toml")
    }

    /// Append-only local audit trail (no network).
    pub fn audit(&self, event: &str) {
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.audit_log)
        {
            let _ = writeln!(f, "[{}] {event}", chrono_lite_now());
        }
    }
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

/// Categories that participate in part/revision branch grouping.
pub fn part_group_categories() -> &'static [&'static str] {
    &["Drawing", "CAD Model", "Quality"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_data_dir() {
        let paths = AppPaths::resolve().unwrap();
        assert!(paths.data_dir.to_string_lossy().contains("EngineVault"));
    }
}
