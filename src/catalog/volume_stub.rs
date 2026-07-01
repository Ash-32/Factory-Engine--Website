use anyhow::{anyhow, Result};

use super::columnar::Catalog;

pub struct VolumeInfo {
    pub drive_letter: char,
    pub catalog: Catalog,
    pub record_count: u64,
}

pub fn scan_volume(_drive: char) -> Result<VolumeInfo> {
    Err(anyhow!(
        "NTFS MFT direct read requires Windows — use pre-built catalog files for tests"
    ))
}
