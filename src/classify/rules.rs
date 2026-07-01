use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRule {
    pub name: String,
    pub extensions: Vec<String>,
    pub folder_tokens: Vec<String>,
    pub filename_keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationSettings {
    pub confidence_threshold: f64,
    pub unclassified_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesConfig {
    pub settings: ClassificationSettings,
    #[serde(default)]
    pub categories: Vec<CategoryRule>,
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            settings: ClassificationSettings {
                confidence_threshold: 0.55,
                unclassified_label: "Unclassified — Needs Review".to_string(),
            },
            categories: Vec::new(),
        }
    }
}

pub fn load_rules(path: &Path) -> Result<RulesConfig> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read rules {}", path.display()))?;
    let config: RulesConfig = toml::from_str(&text)
        .with_context(|| format!("parse rules {}", path.display()))?;
    Ok(config)
}

pub fn default_rules_path() -> PathBuf {
    PathBuf::from("rules/classification.toml")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_classification_toml() {
        let path = Path::new("rules/classification.toml");
        if path.exists() {
            let cfg = load_rules(path).unwrap();
            assert!(!cfg.categories.is_empty());
            assert!(cfg.settings.confidence_threshold > 0.0);
        }
    }
}
