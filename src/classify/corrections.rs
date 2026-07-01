use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCorrection {
    pub path_pattern: String,
    pub category: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CorrectionsFile {
    pub corrections: Vec<UserCorrection>,
}

pub fn corrections_path() -> std::path::PathBuf {
    directories::ProjectDirs::from("", "", "EngineVault")
        .map(|d| d.data_local_dir().join("rules").join("user_corrections.json"))
        .unwrap_or_else(|| std::path::PathBuf::from("rules/user_corrections.json"))
}

pub fn load_corrections() -> CorrectionsFile {
    let path = corrections_path();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        CorrectionsFile::default()
    }
}

pub fn save_correction(path_pattern: &str, category: &str) -> anyhow::Result<()> {
    let mut file = load_corrections();
    let now = chrono_lite_now();
    file.corrections.retain(|c| c.path_pattern != path_pattern);
    file.corrections.push(UserCorrection {
        path_pattern: path_pattern.to_string(),
        category: category.to_string(),
        created_at: now,
    });

    let path = corrections_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&file)?;
    std::fs::write(&path, json)?;
    Ok(())
}

pub fn apply_correction(path: &Path, category: &str) -> anyhow::Result<()> {
    let pattern = path.to_string_lossy().to_string();
    save_correction(&pattern, category)
}

fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{}", secs)
}
