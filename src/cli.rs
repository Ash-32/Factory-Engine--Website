use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::catalog::{load_catalog, save_catalog, watch_volume};
use crate::classify::{apply_correction, group_by_part_revision, load_rules, ClassificationEngine};

#[derive(Parser, Debug)]
#[command(name = "ntfs-catalog", about = "NTFS MFT catalog with engineering file classification")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Full MFT scan of an NTFS volume
    Scan {
        #[arg(long)]
        drive: char,
        #[arg(long, default_value = "catalog.ntfsbin")]
        output: PathBuf,
    },
    /// Blocking USN journal watch with rescan on wrap/mismatch
    Watch {
        #[arg(long)]
        drive: char,
        #[arg(long, default_value = "catalog.ntfsbin")]
        catalog: PathBuf,
    },
    /// Rule-based classification of catalog entries
    Classify {
        #[arg(long, default_value = "catalog.ntfsbin")]
        catalog: PathBuf,
        #[arg(long, default_value = "rules/classification.toml")]
        rules: PathBuf,
    },
    /// Persist a user correction rule
    ApplyCorrection {
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        category: String,
    },
    /// Export category and part/revision statistics
    ExportStats {
        #[arg(long, default_value = "catalog.ntfsbin")]
        catalog: PathBuf,
        #[arg(long, default_value = "rules/classification.toml")]
        rules: PathBuf,
    },
    /// Verify the binary has no networking imports
    VerifyNoNetwork {
        #[arg(long)]
        binary: Option<PathBuf>,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Scan { drive, output } => cmd_scan(drive, &output),
        Commands::Watch { drive, catalog } => cmd_watch(drive, &catalog),
        Commands::Classify { catalog, rules } => cmd_classify(&catalog, &rules),
        Commands::ApplyCorrection { path, category } => cmd_apply_correction(&path, &category),
        Commands::ExportStats { catalog, rules } => cmd_export_stats(&catalog, &rules),
        Commands::VerifyNoNetwork { binary } => cmd_verify_no_network(binary.as_deref()),
    }
}

fn cmd_scan(drive: char, output: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        use crate::catalog::scan_volume;
        eprintln!("Scanning MFT on drive {}:...", drive);
        let info = scan_volume(drive).context("MFT scan failed")?;
        eprintln!("Parsed {} file records", info.record_count);
        save_catalog(&info.catalog, output)?;
        eprintln!("Saved catalog to {}", output.display());
        Ok(())
    }
    #[cfg(not(windows))]
    {
        let _ = (drive, output);
        Err(anyhow::anyhow!("scan requires Windows"))
    }
}

fn cmd_watch(drive: char, catalog: &Path) -> Result<()> {
    watch_volume(drive, catalog)
}

fn cmd_classify(catalog_path: &Path, rules_path: &Path) -> Result<()> {
    let catalog = load_catalog(catalog_path)?;
    let rules = load_rules(rules_path)?;
    let engine = ClassificationEngine::new(rules);

    let entries: Vec<_> = catalog.active_entries().collect();
    let classified = engine.classify_all(entries.iter());

    let mut by_category: HashMap<String, usize> = HashMap::new();
    for cf in &classified {
        *by_category.entry(cf.result.category.clone()).or_insert(0) += 1;
    }

    let mut cats: Vec<_> = by_category.into_iter().collect();
    cats.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Classification results ({} files):", classified.len());
    for (cat, count) in cats {
        println!("  {:40} {}", cat, count);
    }

    Ok(())
}

fn cmd_apply_correction(path: &Path, category: &str) -> Result<()> {
    apply_correction(path, category)?;
    println!("Saved correction: {} -> {}", path.display(), category);
    Ok(())
}

fn cmd_export_stats(catalog_path: &Path, rules_path: &Path) -> Result<()> {
    let catalog = load_catalog(catalog_path)?;
    let rules = load_rules(rules_path)?;
    let engine = ClassificationEngine::new(rules);

    let entries: Vec<_> = catalog.active_entries().collect();
    let classified = engine.classify_all(entries.iter());
    let groups = group_by_part_revision(&classified);

    let mut by_category: HashMap<String, usize> = HashMap::new();
    for cf in &classified {
        *by_category.entry(cf.result.category.clone()).or_insert(0) += 1;
    }

    let stats = serde_json::json!({
        "total_files": classified.len(),
        "categories": by_category,
        "part_revision_groups": groups.len(),
        "groups": groups.iter().take(20).map(|g| serde_json::json!({
            "part": g.part_key,
            "revision": g.revision,
            "file_count": g.files.len(),
        })).collect::<Vec<_>>(),
    });

    println!("{}", serde_json::to_string_pretty(&stats)?);
    Ok(())
}

fn cmd_verify_no_network(binary: Option<&Path>) -> Result<()> {
    let bin_path = binary
        .map(PathBuf::from)
        .unwrap_or_else(default_binary_path);

    if !bin_path.exists() {
        return Err(anyhow::anyhow!(
            "binary not found: {} — build with `cargo build --release` first",
            bin_path.display()
        ));
    }

    let data = std::fs::read(&bin_path)?;
    let forbidden = ["WS2_32.dll", "WINHTTP.dll", "WININET.dll"];
    let mut found = Vec::new();

    for dll in &forbidden {
        if contains_ignore_case(&data, dll.as_bytes()) {
            found.push(*dll);
        }
    }

    if found.is_empty() {
        println!("OK: no networking DLL imports found in {}", bin_path.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "networking imports detected: {:?}",
            found
        ))
    }
}

fn default_binary_path() -> PathBuf {
    PathBuf::from("target/release/ntfs-catalog.exe")
}

fn contains_ignore_case(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|window| {
        window
            .iter()
            .zip(needle.iter())
            .all(|(a, b)| a.eq_ignore_ascii_case(b))
    })
}
