use std::path::Path;

use crate::catalog::FileEntry;
use crate::classify::corrections::load_corrections;
use crate::classify::rules::{CategoryRule, RulesConfig};

#[derive(Debug, Clone)]
pub struct ClassificationResult {
    pub category: String,
    pub confidence: f64,
    pub matched_layers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClassifiedFile {
    pub entry: FileEntry,
    pub result: ClassificationResult,
}

pub struct ClassificationEngine {
    config: RulesConfig,
    correction_patterns: Vec<(String, String)>,
}

impl ClassificationEngine {
    pub fn new(config: RulesConfig) -> Self {
        let corrections = load_corrections();
        let correction_patterns = corrections
            .corrections
            .into_iter()
            .map(|c| (c.path_pattern.to_ascii_lowercase(), c.category))
            .collect();

        Self {
            config,
            correction_patterns,
        }
    }

    pub fn classify_path(&self, path: &str, filename: &str) -> ClassificationResult {
        let path_lower = path.to_ascii_lowercase();
        let filename_lower = filename.to_ascii_lowercase();

        for (pattern, category) in &self.correction_patterns {
            if path_lower.contains(pattern) || pattern.contains(&path_lower) {
                return ClassificationResult {
                    category: category.clone(),
                    confidence: 1.0,
                    matched_layers: vec!["user_correction".to_string()],
                };
            }
        }

        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .unwrap_or_default();

        let mut best: Option<(String, f64, Vec<String>)> = None;

        for rule in &self.config.categories {
            let (score, layers) = score_rule(rule, &ext, &path_lower, &filename_lower);
            if score <= 0.0 {
                continue;
            }
            let replace = best.as_ref().map(|(_, s, _)| score > *s).unwrap_or(true);
            if replace {
                best = Some((rule.name.clone(), score, layers));
            }
        }

        if let Some((ref category, confidence, ref layers)) = best {
            if confidence >= self.config.settings.confidence_threshold {
                return ClassificationResult {
                    category: category.clone(),
                    confidence,
                    matched_layers: layers.clone(),
                };
            }
        }

        ClassificationResult {
            category: self.config.settings.unclassified_label.clone(),
            confidence: best.as_ref().map(|(_, s, _)| *s).unwrap_or(0.0),
            matched_layers: best.map(|(_, _, l)| l).unwrap_or_default(),
        }
    }

    pub fn classify_entry(&self, entry: &FileEntry) -> ClassificationResult {
        self.classify_path(&entry.path, &entry.filename)
    }

    pub fn classify_all<'a>(
        &self,
        entries: impl Iterator<Item = &'a FileEntry>,
    ) -> Vec<ClassifiedFile> {
        entries
            .map(|entry| ClassifiedFile {
                result: self.classify_entry(entry),
                entry: entry.clone(),
            })
            .collect()
    }
}

fn score_rule(
    rule: &CategoryRule,
    ext: &str,
    path_lower: &str,
    filename_lower: &str,
) -> (f64, Vec<String>) {
    let mut score = 0.0f64;
    let mut layers = Vec::new();

    if !ext.is_empty() && rule.extensions.iter().any(|e| e.to_ascii_lowercase() == ext) {
        score += 0.45;
        layers.push("extension".to_string());
    }

    for token in &rule.folder_tokens {
        let token_lower = token.to_ascii_lowercase();
        if path_lower.contains(&token_lower) {
            score += 0.30;
            layers.push(format!("folder:{}", token));
            break;
        }
    }

    for kw in &rule.filename_keywords {
        let kw_lower = kw.to_ascii_lowercase();
        if filename_lower.contains(&kw_lower) {
            score += 0.25;
            layers.push(format!("keyword:{}", kw));
            break;
        }
    }

    (score.min(1.0), layers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classify::rules::{ClassificationSettings, RulesConfig};

    fn test_config() -> RulesConfig {
        RulesConfig {
            settings: ClassificationSettings {
                confidence_threshold: 0.55,
                unclassified_label: "Unclassified — Needs Review".to_string(),
            },
            categories: vec![
                CategoryRule {
                    name: "Drawing".to_string(),
                    extensions: vec!["pdf".to_string(), "dwg".to_string()],
                    folder_tokens: vec!["drawings".to_string()],
                    filename_keywords: vec!["DRW".to_string()],
                },
                CategoryRule {
                    name: "Quality".to_string(),
                    extensions: vec!["xlsx".to_string()],
                    folder_tokens: vec!["quality".to_string(), "fmea".to_string()],
                    filename_keywords: vec!["FMEA".to_string()],
                },
            ],
        }
    }

    #[test]
    fn extension_and_folder_scoring() {
        let engine = ClassificationEngine::new(test_config());
        let result = engine.classify_path(
            r"\projects\drawings\part001.pdf",
            "part001.pdf",
        );
        assert_eq!(result.category, "Drawing");
        assert!(result.confidence >= 0.55);
        assert!(result.matched_layers.iter().any(|l| l == "extension"));
    }

    #[test]
    fn keyword_match_boosts_quality() {
        let engine = ClassificationEngine::new(test_config());
        let result = engine.classify_path(
            r"\docs\report-FMEA.xlsx",
            "report-FMEA.xlsx",
        );
        assert_eq!(result.category, "Quality");
        assert!(result.confidence >= 0.55);
    }

    #[test]
    fn below_threshold_goes_unclassified() {
        let engine = ClassificationEngine::new(test_config());
        let result = engine.classify_path(r"\temp\data.bin", "data.bin");
        assert_eq!(result.category, "Unclassified — Needs Review");
        assert!(result.confidence < 0.55);
    }
}
