mod columnar;
mod mft;
mod path_reconstruct;
mod persistence;
mod usn;

#[cfg(windows)]
mod volume;

pub use columnar::{Catalog, FileEntry};
pub use mft::FileTimestamps;
pub use persistence::{load_catalog, save_catalog, CATALOG_MAGIC, HEADER_SIZE};
pub use usn::watch_volume;

#[cfg(windows)]
pub use volume::{scan_volume, VolumeInfo};

#[cfg(not(windows))]
pub use volume_stub::{scan_volume, VolumeInfo};

#[cfg(not(windows))]
mod volume_stub;
