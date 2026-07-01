#[cfg(windows)]
mod imp {
    use std::ffi::OsStr;
    use std::ffi::c_void;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;

    use anyhow::{anyhow, bail, Result};
    use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, ReadFile, SetFilePointerEx, FILE_BEGIN, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;
    use windows_sys::Win32::System::Ioctl::{
        FSCTL_GET_NTFS_FILE_RECORD, FSCTL_GET_NTFS_VOLUME_DATA, NTFS_FILE_RECORD_INPUT_BUFFER,
        NTFS_FILE_RECORD_OUTPUT_BUFFER, NTFS_VOLUME_DATA_BUFFER,
    };

    use crate::catalog::columnar::Catalog;
    use crate::catalog::mft::read_and_parse_record;
    use crate::catalog::path_reconstruct::reconstruct_paths;

    pub struct VolumeInfo {
        pub drive_letter: char,
        pub catalog: Catalog,
        pub record_count: u64,
    }

    struct Volume {
        handle: HANDLE,
        drive_letter: char,
        bytes_per_file_record: u32,
        bytes_per_cluster: u32,
        mft_start_lcn: i64,
    }

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    impl Volume {
        fn open(drive_letter: char) -> Result<Self> {
            let path = format!(r"\\.\{}:", drive_letter.to_ascii_uppercase());
            let wide = to_wide(&path);
            let handle = unsafe {
                CreateFileW(
                    wide.as_ptr(),
                    GENERIC_READ,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    std::ptr::null(),
                    OPEN_EXISTING,
                    FILE_FLAG_BACKUP_SEMANTICS,
                    std::ptr::null_mut(),
                )
            };
            if handle == INVALID_HANDLE_VALUE {
                bail!("failed to open volume {path} — run as Administrator");
            }

            let mut vol = Volume {
                handle,
                drive_letter,
                bytes_per_file_record: 1024,
                bytes_per_cluster: 4096,
                mft_start_lcn: 0,
            };
            vol.query_volume_data()?;
            Ok(vol)
        }

        fn query_volume_data(&mut self) -> Result<()> {
            let mut data: MaybeUninit<NTFS_VOLUME_DATA_BUFFER> = MaybeUninit::uninit();
            let mut returned = 0u32;
            let ok = unsafe {
                DeviceIoControl(
                    self.handle,
                    FSCTL_GET_NTFS_VOLUME_DATA,
                    std::ptr::null(),
                    0,
                    data.as_mut_ptr() as *mut c_void,
                    std::mem::size_of::<NTFS_VOLUME_DATA_BUFFER>() as u32,
                    &mut returned,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                bail!("FSCTL_GET_NTFS_VOLUME_DATA failed");
            }
            let data = unsafe { data.assume_init() };
            self.bytes_per_file_record = data.BytesPerFileRecordSegment;
            self.bytes_per_cluster = data.BytesPerCluster;
            self.mft_start_lcn = data.MftStartLcn;
            Ok(())
        }

        fn record_size(&self) -> usize {
            let bps = self.bytes_per_file_record as i32;
            if bps >= 0 {
                bps as usize
            } else {
                1usize << (-bps as u32)
            }
        }

        fn read_mft_chunk(&self, start_rec: u64, count: u64) -> Result<Vec<u8>> {
            let record_size = self.record_size() as u64;
            let cluster_size = self.bytes_per_cluster as u64;
            let byte_offset = self.mft_start_lcn as u64 * cluster_size + start_rec * record_size;
            let len = (count * record_size) as u32;
            let mut buf = vec![0u8; len as usize];
            let mut bytes_read = 0u32;

            let ok = unsafe {
                SetFilePointerEx(self.handle, byte_offset as i64, std::ptr::null_mut(), FILE_BEGIN)
            };
            if ok == 0 {
                bail!("SetFilePointerEx failed at MFT record {start_rec}");
            }

            let ok = unsafe {
                ReadFile(
                    self.handle,
                    buf.as_mut_ptr(),
                    len,
                    &mut bytes_read,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                bail!("ReadFile at MFT offset failed for record {start_rec}");
            }
            buf.truncate(bytes_read as usize);
            Ok(buf)
        }

        fn read_file_record_ioctl(&self, rec_num: u64) -> Result<Vec<u8>> {
            let input = NTFS_FILE_RECORD_INPUT_BUFFER {
                FileReferenceNumber: rec_num as i64,
            };
            let out_size = std::mem::size_of::<NTFS_FILE_RECORD_OUTPUT_BUFFER>()
                + self.record_size()
                + 1024;
            let mut buf = vec![0u8; out_size];
            let mut returned = 0u32;
            let ok = unsafe {
                DeviceIoControl(
                    self.handle,
                    FSCTL_GET_NTFS_FILE_RECORD,
                    &input as *const _ as *const c_void,
                    std::mem::size_of::<NTFS_FILE_RECORD_INPUT_BUFFER>() as u32,
                    buf.as_mut_ptr() as *mut c_void,
                    out_size as u32,
                    &mut returned,
                    std::ptr::null_mut(),
                )
            };
            if ok == 0 {
                bail!("FSCTL_GET_NTFS_FILE_RECORD failed for record {rec_num}");
            }
            let header_size = std::mem::size_of::<NTFS_FILE_RECORD_OUTPUT_BUFFER>();
            Ok(buf[header_size..header_size + self.record_size()].to_vec())
        }

        fn close(self) {
            unsafe { CloseHandle(self.handle) };
        }
    }

    pub fn scan_volume(drive: char) -> Result<VolumeInfo> {
        let volume = Volume::open(drive)?;
        let record_size = volume.record_size();
        if record_size == 0 || record_size > 65536 {
            return Err(anyhow!("invalid MFT record size: {}", record_size));
        }

        let mut catalog = Catalog::new(drive);
        let max_records = 4_000_000u64;
        let chunk_records = 4096u64;
        let mut rec = 0u64;
        let mut empty_run = 0u32;
        let mut bulk_ok = true;

        while rec < max_records {
            let count = chunk_records.min(max_records - rec);
            let bytes = if bulk_ok {
                match volume.read_mft_chunk(rec, count) {
                    Ok(chunk) if chunk.len() >= count as usize * record_size => chunk,
                    Ok(_) | Err(_) => {
                        if bulk_ok {
                            eprintln!("bulk MFT read failed at {rec}, falling back to IOCTL");
                            bulk_ok = false;
                        }
                        read_chunk_ioctl(&volume, rec, count, record_size)?
                    }
                }
            } else {
                read_chunk_ioctl(&volume, rec, count, record_size)?
            };

            for (idx, record) in bytes.chunks(record_size).enumerate() {
                let rec_num = rec + idx as u64;
                if record.iter().all(|&b| b == 0) {
                    empty_run += 1;
                    continue;
                }

                let magic = u32::from_le_bytes(record[0..4].try_into().unwrap_or([0; 4]));
                if magic != 0x454C_4946 {
                    empty_run += 1;
                    continue;
                }

                let mut buf = record.to_vec();
                match read_and_parse_record(&mut buf, rec_num) {
                    Ok(parsed) => {
                        empty_run = 0;
                        if let Some(ref name) = parsed.filename {
                            catalog.upsert_entry(
                                parsed.record_number,
                                parsed.sequence_number,
                                parsed.parent_record,
                                parsed.parent_sequence,
                                name,
                                "",
                                parsed.file_size,
                                parsed.timestamps,
                                true,
                                parsed.is_active,
                            );
                        }
                    }
                    Err(_) => empty_run += 1,
                }
            }

            rec += count;
            if empty_run > 8192 && rec > 10_000 {
                break;
            }
        }

        volume.close();
        reconstruct_paths(&mut catalog);

        let count = catalog.len() as u64;
        Ok(VolumeInfo {
            drive_letter: drive,
            catalog,
            record_count: count,
        })
    }

    fn read_chunk_ioctl(vol: &Volume, start: u64, count: u64, record_size: usize) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(count as usize * record_size);
        for i in 0..count {
            match vol.read_file_record_ioctl(start + i) {
                Ok(buf) => out.extend_from_slice(&buf),
                Err(_) => out.resize(out.len() + record_size, 0),
            }
        }
        Ok(out)
    }
}

#[cfg(windows)]
pub use imp::{scan_volume, VolumeInfo};
