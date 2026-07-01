#[cfg(windows)]
mod imp {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;

    use anyhow::{anyhow, Result};
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;
    use windows_sys::Win32::System::Ioctl::{
        FSCTL_QUERY_USN_JOURNAL, FSCTL_READ_USN_JOURNAL, READ_USN_JOURNAL_DATA_V0,
        USN_JOURNAL_DATA_V0,
    };

    use crate::catalog::persistence::{load_catalog, save_catalog};
    use crate::catalog::volume::scan_volume;

    const ERROR_HANDLE_EOF: u32 = 38;
    const ERROR_JOURNAL_ENTRY_DELETED: u32 = 1301;

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    fn open_volume(drive: char) -> Result<HANDLE> {
        let path = format!(r"\\.\{}:", drive.to_ascii_uppercase());
        let wide = to_wide(&path);
        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                0,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS,
                std::ptr::null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(anyhow!("failed to open volume {}", path));
        }
        Ok(handle)
    }

    fn query_usn_journal(vol: HANDLE) -> Result<USN_JOURNAL_DATA_V0> {
        let mut journal_data: USN_JOURNAL_DATA_V0 = unsafe { std::mem::zeroed() };
        let mut bytes_returned = 0u32;
        let ok = unsafe {
            DeviceIoControl(
                vol,
                FSCTL_QUERY_USN_JOURNAL,
                std::ptr::null(),
                0,
                &mut journal_data as *mut _ as *mut _,
                std::mem::size_of::<USN_JOURNAL_DATA_V0>() as u32,
                &mut bytes_returned,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(anyhow!("FSCTL_QUERY_USN_JOURNAL failed"));
        }
        Ok(journal_data)
    }

    fn read_usn_journal_blocking(
        vol: HANDLE,
        journal_id: u64,
        start_usn: i64,
    ) -> Result<(i64, bool)> {
        let input = READ_USN_JOURNAL_DATA_V0 {
            StartUsn: start_usn,
            ReasonMask: 0xFFFF_FFFF,
            ReturnOnlyOnClose: 0,
            Timeout: u64::MAX,
            BytesToWaitFor: 1,
            UsnJournalID: journal_id,
        };

        let mut output = vec![0u8; 64 * 1024];
        let mut bytes_returned = 0u32;
        let ok = unsafe {
            DeviceIoControl(
                vol,
                FSCTL_READ_USN_JOURNAL,
                &input as *const _ as *const _,
                std::mem::size_of::<READ_USN_JOURNAL_DATA_V0>() as u32,
                output.as_mut_ptr() as *mut _,
                output.len() as u32,
                &mut bytes_returned,
                std::ptr::null_mut(),
            )
        };

        if ok == 0 {
            let err = unsafe { windows_sys::Win32::Foundation::GetLastError() };
            if err == ERROR_HANDLE_EOF || err == ERROR_JOURNAL_ENTRY_DELETED {
                return Ok((start_usn, true));
            }
            return Err(anyhow!("FSCTL_READ_USN_JOURNAL failed (error {err})"));
        }

        if bytes_returned < 8 {
            return Ok((start_usn, false));
        }

        let next_usn = i64::from_le_bytes(output[0..8].try_into().unwrap());
        let changed = bytes_returned > 8;
        Ok((next_usn, changed))
    }

    fn full_rescan(drive: char, catalog_path: &Path) -> Result<()> {
        eprintln!("Performing full MFT rescan on {}:", drive);
        let info = scan_volume(drive)?;
        save_catalog(&info.catalog, catalog_path)?;
        eprintln!("Rescan complete — {} records", info.record_count);
        Ok(())
    }

    /// Blocking USN journal watch via FSCTL_READ_USN_JOURNAL; rescans on change, wrap, or mismatch.
    pub fn watch_volume(drive: char, catalog_path: &Path) -> Result<()> {
        let vol = open_volume(drive)?;
        let journal = query_usn_journal(vol)?;
        let journal_id = journal.UsnJournalID;

        let _catalog = load_catalog(catalog_path).unwrap_or_else(|_| {
            crate::catalog::columnar::Catalog::new(drive)
        });

        eprintln!(
            "Watching USN journal on {}: (journal_id={:#X}, next_usn={})",
            drive, journal.UsnJournalID, journal.NextUsn
        );

        let mut start_usn = journal.FirstUsn;

        loop {
            let current = query_usn_journal(vol)?;
            if current.UsnJournalID != journal_id {
                eprintln!("USN journal ID changed — full rescan");
                full_rescan(drive, catalog_path)?;
                unsafe { CloseHandle(vol) };
                return watch_volume(drive, catalog_path);
            }

            if current.NextUsn < start_usn {
                eprintln!("USN journal wrapped — full rescan");
                full_rescan(drive, catalog_path)?;
                start_usn = current.FirstUsn;
                continue;
            }

            let (next_usn, changed) = read_usn_journal_blocking(vol, journal_id, start_usn)?;

            if changed {
                eprintln!("USN changes detected — full rescan");
                full_rescan(drive, catalog_path)?;
            }

            start_usn = next_usn;
        }
    }
}

#[cfg(windows)]
pub use imp::watch_volume;

#[cfg(not(windows))]
pub fn watch_volume(_drive: char, _catalog_path: &std::path::Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!("USN journal watch requires Windows"))
}
