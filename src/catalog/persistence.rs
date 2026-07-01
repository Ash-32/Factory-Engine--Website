use std::fs::{File, OpenOptions};
use std::io::{Seek, Write};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use bytemuck::{Pod, Zeroable};
use memmap2::Mmap;

use super::columnar::Catalog;

pub const CATALOG_MAGIC: &[u8; 8] = b"NTFSCAT1";
pub const HEADER_SIZE: usize = 256;
pub const FORMAT_VERSION: u32 = 2;

#[repr(C, align(8))]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CatalogHeader {
    magic: [u8; 8],
    version: u32,
    drive_letter: u16,
    _pad0: u16,
    record_count: u64,
    string_pool_size: u64,
}

const HEADER_RESERVED: usize = HEADER_SIZE - std::mem::size_of::<CatalogHeader>();

struct ColumnDescriptor {
    column_id: u32,
    offset: u64,
    elem_size: u32,
    count: u64,
}

fn desc_to_bytes(d: &ColumnDescriptor) -> [u8; DESCRIPTOR_SIZE] {
    let mut buf = [0u8; DESCRIPTOR_SIZE];
    buf[0..4].copy_from_slice(&d.column_id.to_le_bytes());
    buf[8..16].copy_from_slice(&d.offset.to_le_bytes());
    buf[16..20].copy_from_slice(&d.elem_size.to_le_bytes());
    buf[24..32].copy_from_slice(&d.count.to_le_bytes());
    buf
}

fn desc_from_bytes(buf: &[u8]) -> ColumnDescriptor {
    ColumnDescriptor {
        column_id: u32::from_le_bytes(buf[0..4].try_into().unwrap()),
        offset: u64::from_le_bytes(buf[8..16].try_into().unwrap()),
        elem_size: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
        count: u64::from_le_bytes(buf[24..32].try_into().unwrap()),
    }
}

const COL_RECORD_NUMBERS: u32 = 1;
const COL_SEQUENCE: u32 = 2;
const COL_PARENT_RECORD: u32 = 3;
const COL_PARENT_SEQ: u32 = 4;
const COL_FILE_SIZE: u32 = 5;
const COL_FILENAME_ID: u32 = 6;
const COL_PATH_ID: u32 = 7;
const COL_PARENT_VALID: u32 = 8;
const COL_IS_ACTIVE: u32 = 9;
const COL_CREATED: u32 = 10;
const COL_MODIFIED: u32 = 11;
const COL_MFT_MODIFIED: u32 = 12;
const COL_ACCESSED: u32 = 13;

const NUM_COLUMNS: usize = 13;
const DESCRIPTOR_SIZE: usize = 32;
const DESCRIPTOR_TABLE_SIZE: usize = NUM_COLUMNS * DESCRIPTOR_SIZE;

fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

pub fn save_catalog(catalog: &Catalog, path: &Path) -> Result<()> {
    let count = catalog.len() as u64;
    let string_pool = catalog.string_pool_ref();

    let mut data_offset = HEADER_SIZE + DESCRIPTOR_TABLE_SIZE;
    let align = 8usize;

    let descriptors = [
        (COL_RECORD_NUMBERS, 8usize),
        (COL_SEQUENCE, 2),
        (COL_PARENT_RECORD, 8),
        (COL_PARENT_SEQ, 2),
        (COL_FILE_SIZE, 8),
        (COL_FILENAME_ID, 4),
        (COL_PATH_ID, 4),
        (COL_PARENT_VALID, 1),
        (COL_IS_ACTIVE, 1),
        (COL_CREATED, 8),
        (COL_MODIFIED, 8),
        (COL_MFT_MODIFIED, 8),
        (COL_ACCESSED, 8),
    ];

    let mut desc_vec = Vec::with_capacity(NUM_COLUMNS);
    for (col_id, elem_size) in &descriptors {
        desc_vec.push(ColumnDescriptor {
            column_id: *col_id,
            offset: data_offset as u64,
            elem_size: *elem_size as u32,
            count,
        });
        data_offset = align_up(data_offset + (*elem_size) * (count as usize), align);
    }

    let header = CatalogHeader {
        magic: *CATALOG_MAGIC,
        version: FORMAT_VERSION,
        drive_letter: catalog.drive_letter() as u16,
        _pad0: 0,
        record_count: count,
        string_pool_size: string_pool.len() as u64,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("create catalog file {}", path.display()))?;

    file.write_all(bytemuck::bytes_of(&header))?;
    file.write_all(&vec![0u8; HEADER_RESERVED])?;
    for desc in &desc_vec {
        file.write_all(&desc_to_bytes(desc))?;
    }

    write_padded(&mut file, bytemuck::cast_slice(catalog.record_numbers_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.sequence_numbers_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.parent_records_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.parent_sequences_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.file_sizes_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.filename_ids_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.path_ids_ref()), align)?;

    let mut bool_buf: Vec<u8> = catalog.parent_valid_ref().iter().map(|&b| b as u8).collect();
    pad_vec(&mut bool_buf, align);
    file.write_all(&bool_buf)?;

    bool_buf = catalog.is_active_ref().iter().map(|&b| b as u8).collect();
    pad_vec(&mut bool_buf, align);
    file.write_all(&bool_buf)?;

    write_padded(&mut file, bytemuck::cast_slice(catalog.created_times_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.modified_times_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.mft_modified_times_ref()), align)?;
    write_padded(&mut file, bytemuck::cast_slice(catalog.accessed_times_ref()), align)?;

    file.write_all(string_pool)?;

    let current = file.stream_position()?;
    let padded = align_up(current as usize, 4096);
    if padded > current as usize {
        file.write_all(&vec![0u8; padded - current as usize])?;
    }

    file.sync_all()?;
    Ok(())
}

fn pad_vec(buf: &mut Vec<u8>, align: usize) {
    let padded = align_up(buf.len(), align);
    buf.resize(padded, 0);
}

fn write_padded(file: &mut File, data: &[u8], align: usize) -> Result<()> {
    file.write_all(data)?;
    let padded = align_up(data.len(), align);
    if padded > data.len() {
        file.write_all(&vec![0u8; padded - data.len()])?;
    }
    Ok(())
}

pub fn load_catalog(path: &Path) -> Result<Catalog> {
    let file = OpenOptions::new().read(true).open(path)
        .with_context(|| format!("open catalog {}", path.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    load_catalog_from_bytes(&mmap)
}

fn load_catalog_from_bytes(data: &[u8]) -> Result<Catalog> {
    if data.len() < HEADER_SIZE + DESCRIPTOR_TABLE_SIZE {
        return Err(anyhow!("catalog file too small"));
    }

    let header: CatalogHeader = *bytemuck::from_bytes(&data[..std::mem::size_of::<CatalogHeader>()]);

    if &header.magic != CATALOG_MAGIC {
        return Err(anyhow!(
            "invalid catalog magic: {:?}",
            String::from_utf8_lossy(&header.magic)
        ));
    }
    if header.version != FORMAT_VERSION {
        return Err(anyhow!("unsupported catalog version: {}", header.version));
    }

    let drive_letter = char::from_u32(header.drive_letter as u32).unwrap_or('?');
    let count = header.record_count as usize;

    let desc_buf = &data[HEADER_SIZE..HEADER_SIZE + DESCRIPTOR_TABLE_SIZE];

    let mut record_numbers = Vec::new();
    let mut sequence_numbers = Vec::new();
    let mut parent_records = Vec::new();
    let mut parent_sequences = Vec::new();
    let mut file_sizes = Vec::new();
    let mut filename_ids = Vec::new();
    let mut path_ids = Vec::new();
    let mut parent_valid = Vec::new();
    let mut is_active = Vec::new();
    let mut created_times = Vec::new();
    let mut modified_times = Vec::new();
    let mut mft_modified_times = Vec::new();
    let mut accessed_times = Vec::new();

    for i in 0..NUM_COLUMNS {
        let desc = desc_from_bytes(
            &desc_buf[i * DESCRIPTOR_SIZE..(i + 1) * DESCRIPTOR_SIZE],
        );
        let offset = desc.offset as usize;
        let byte_len = (desc.elem_size as usize) * (desc.count as usize);
        let aligned_len = align_up(byte_len, 8);
        if offset + aligned_len > data.len() {
            return Err(anyhow!("column {} extends past file end", desc.column_id));
        }
        let col_buf = &data[offset..offset + aligned_len];

        match desc.column_id {
            COL_RECORD_NUMBERS => record_numbers = read_pod_vec(&col_buf, count),
            COL_SEQUENCE => sequence_numbers = read_pod_vec(&col_buf, count),
            COL_PARENT_RECORD => parent_records = read_pod_vec(&col_buf, count),
            COL_PARENT_SEQ => parent_sequences = read_pod_vec(&col_buf, count),
            COL_FILE_SIZE => file_sizes = read_pod_vec(&col_buf, count),
            COL_FILENAME_ID => filename_ids = read_pod_vec(&col_buf, count),
            COL_PATH_ID => path_ids = read_pod_vec(&col_buf, count),
            COL_PARENT_VALID => {
                parent_valid = col_buf[..count.min(col_buf.len())]
                    .iter()
                    .map(|&b| b != 0)
                    .collect();
            }
            COL_IS_ACTIVE => {
                is_active = col_buf[..count.min(col_buf.len())]
                    .iter()
                    .map(|&b| b != 0)
                    .collect();
            }
            COL_CREATED => created_times = read_pod_vec(&col_buf, count),
            COL_MODIFIED => modified_times = read_pod_vec(&col_buf, count),
            COL_MFT_MODIFIED => mft_modified_times = read_pod_vec(&col_buf, count),
            COL_ACCESSED => accessed_times = read_pod_vec(&col_buf, count),
            _ => {}
        }
    }

    let pool_size = header.string_pool_size as usize;
    let mut string_pool = vec![0u8; pool_size];
    if pool_size > 0 {
        let last_desc = desc_from_bytes(
            &desc_buf[(NUM_COLUMNS - 1) * DESCRIPTOR_SIZE..NUM_COLUMNS * DESCRIPTOR_SIZE],
        );
        let last_end = last_desc.offset as usize
            + align_up((last_desc.elem_size as usize) * (last_desc.count as usize), 8);
        if last_end + pool_size <= data.len() {
            string_pool = data[last_end..last_end + pool_size].to_vec();
        }
    }

    Ok(Catalog::set_from_columns(
        drive_letter,
        record_numbers,
        sequence_numbers,
        parent_records,
        parent_sequences,
        file_sizes,
        created_times,
        modified_times,
        mft_modified_times,
        accessed_times,
        filename_ids,
        path_ids,
        parent_valid,
        is_active,
        string_pool,
    ))
}

fn read_pod_vec<T: Pod>(buf: &[u8], count: usize) -> Vec<T> {
    let size = std::mem::size_of::<T>();
    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let start = i * size;
        if start + size <= buf.len() {
            result.push(*bytemuck::from_bytes(&buf[start..start + size]));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::columnar::Catalog;
    use crate::catalog::mft::FileTimestamps;
    use tempfile::NamedTempFile;

    #[test]
    fn mmap_roundtrip_preserves_entries() {
        let mut cat = Catalog::new('D');
        let ts = FileTimestamps {
            created: 100,
            modified: 200,
            mft_modified: 300,
            accessed: 400,
        };
        cat.upsert_entry(5, 1, 5, 1, "", "\\", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(
            42,
            3,
            5,
            1,
            "readme.txt",
            "\\readme.txt",
            1024,
            ts,
            true,
            true,
        );
        cat.upsert_entry(
            99,
            1,
            42,
            3,
            "nested.pdf",
            "\\readme.txt\\nested.pdf",
            2048,
            FileTimestamps::default(),
            false,
            true,
        );

        let tmp = NamedTempFile::new().unwrap();
        save_catalog(&cat, tmp.path()).unwrap();

        let loaded = load_catalog(tmp.path()).unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded.drive_letter(), 'D');

        let e42 = loaded.get(42).unwrap();
        assert_eq!(e42.filename, "readme.txt");
        assert_eq!(e42.file_size, 1024);
        assert_eq!(e42.timestamps.modified, 200);

        let e99 = loaded.get(99).unwrap();
        assert_eq!(e99.path, "\\readme.txt\\nested.pdf");
        assert!(!e99.parent_valid);
    }

    #[test]
    fn rejects_bad_magic() {
        let tmp = NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), b"NOTCAT01").unwrap();
        assert!(load_catalog(tmp.path()).is_err());
    }
}
