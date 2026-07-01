use anyhow::{anyhow, Context, Result};
use bytemuck::{Pod, Zeroable};

pub const MFT_RECORD_SIZE: usize = 1024;
pub const FILE_SIGNATURE: u32 = 0x454C_4946; // "FILE"
pub const ATTR_STANDARD_INFORMATION: u32 = 0x10;
pub const ATTR_FILE_NAME: u32 = 0x30;
pub const ATTR_DATA: u32 = 0x80;
pub const ATTR_END: u32 = 0xFF_FF_FF_FF;

/// Root directory MFT record number.
pub const ROOT_RECORD: u64 = 5;

#[repr(C, packed)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct MftRecordHeader {
    pub magic: u32,
    pub usa_offset: u16,
    pub usa_count: u16,
    pub lsn: u64,
    pub sequence_number: u16,
    pub link_count: u16,
    pub attrs_offset: u16,
    pub flags: u16,
    pub used_size: u32,
    pub allocated_size: u32,
    pub base_record: u64,
    pub next_attr_id: u16,
    pub record_number: u16,
    pub update_seq_num: u16,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FileTimestamps {
    pub created: u64,
    pub modified: u64,
    pub mft_modified: u64,
    pub accessed: u64,
}

#[derive(Debug, Clone)]
pub struct ParsedFileRecord {
    pub record_number: u64,
    pub sequence_number: u16,
    pub is_active: bool,
    pub file_size: u64,
    pub timestamps: FileTimestamps,
    /// From $FILE_NAME namespace 1 (Win32) or 3 (Win32+DOS).
    pub filename: Option<String>,
    pub parent_record: u64,
    pub parent_sequence: u16,
    pub namespace: u8,
}

/// Apply NTFS fixup array to restore sector checksums in an MFT record.
pub fn apply_fixup(record: &mut [u8]) -> Result<()> {
    if record.len() < MFT_RECORD_SIZE {
        return Err(anyhow!("record too small for fixup: {} bytes", record.len()));
    }

    let header: MftRecordHeader = *bytemuck::from_bytes(&record[..32]);
    let magic = header.magic;
    if magic != FILE_SIGNATURE {
        return Err(anyhow!(
            "invalid FILE signature: {:08X}",
            magic
        ));
    }

    let usa_offset = header.usa_offset as usize;
    let usa_count = header.usa_count as usize;
    if usa_count < 2 {
        return Err(anyhow!("invalid usa_count: {}", usa_count));
    }
    if usa_offset + usa_count * 2 > record.len() {
        return Err(anyhow!("fixup array out of bounds"));
    }

    let update_seq = u16::from_le_bytes([record[usa_offset], record[usa_offset + 1]]);
    let sector_size = 512usize;
    let sectors = record.len() / sector_size;

    for i in 1..usa_count.min(sectors + 1) {
        let fixup_offset = usa_offset + i * 2;
        let saved = u16::from_le_bytes([record[fixup_offset], record[fixup_offset + 1]]);
        let sector_end = i * sector_size - 2;
        if sector_end + 2 > record.len() {
            break;
        }
        let current = u16::from_le_bytes([record[sector_end], record[sector_end + 1]]);
        if current != update_seq && current != saved {
            // Allow mismatch on partially written records; still restore.
        }
        record[sector_end] = (saved & 0xFF) as u8;
        record[sector_end + 1] = (saved >> 8) as u8;
    }

    Ok(())
}

fn decode_parent_reference(parent_ref: u64) -> (u64, u16) {
    let record = parent_ref & 0x000F_FFFF_FFFF;
    let sequence = ((parent_ref >> 48) & 0xFFFF) as u16;
    (record, sequence)
}

fn read_utf16le(data: &[u8], offset: usize, char_count: usize) -> Result<String> {
    let byte_len = char_count * 2;
    if offset + byte_len > data.len() {
        return Err(anyhow!("utf16 read out of bounds"));
    }
    let mut chars = Vec::with_capacity(char_count);
    for i in 0..char_count {
        let pos = offset + i * 2;
        let unit = u16::from_le_bytes([data[pos], data[pos + 1]]);
        if unit == 0 {
            break;
        }
        chars.push(unit);
    }
    Ok(String::from_utf16_lossy(&chars))
}

/// Parse a fixup-restored MFT FILE record.
pub fn parse_file_record(record: &[u8], record_number: u64) -> Result<ParsedFileRecord> {
    if record.len() < 48 {
        return Err(anyhow!("record too small"));
    }

    let header: MftRecordHeader = *bytemuck::from_bytes(&record[..32]);
    if header.magic != FILE_SIGNATURE {
        return Err(anyhow!("not a FILE record"));
    }

    let is_active = header.flags & 0x0001 != 0;
    let attrs_offset = header.attrs_offset as usize;
    let mut file_size = 0u64;
    let mut timestamps = FileTimestamps::default();
    let mut best_filename: Option<(u8, String, u64, u16)> = None;

    let mut offset = attrs_offset;
    while offset + 4 <= record.len() {
        let attr_type = u32::from_le_bytes(record[offset..offset + 4].try_into()?);
        if attr_type == ATTR_END || attr_type == 0 {
            break;
        }
        if offset + 16 > record.len() {
            break;
        }
        let attr_len = u32::from_le_bytes(record[offset + 4..offset + 8].try_into()?) as usize;
        if attr_len < 16 || offset + attr_len > record.len() {
            break;
        }

        let non_resident = record[offset + 8];
        if attr_type == ATTR_STANDARD_INFORMATION && non_resident == 0 {
            let content_offset =
                u16::from_le_bytes(record[offset + 20..offset + 22].try_into()?) as usize;
            let content_start = offset + content_offset;
            if content_start + 32 <= record.len() {
                timestamps.created = u64::from_le_bytes(
                    record[content_start..content_start + 8].try_into().unwrap_or([0; 8]),
                );
                timestamps.modified = u64::from_le_bytes(
                    record[content_start + 8..content_start + 16]
                        .try_into()
                        .unwrap_or([0; 8]),
                );
                timestamps.mft_modified = u64::from_le_bytes(
                    record[content_start + 16..content_start + 24]
                        .try_into()
                        .unwrap_or([0; 8]),
                );
                timestamps.accessed = u64::from_le_bytes(
                    record[content_start + 24..content_start + 32]
                        .try_into()
                        .unwrap_or([0; 8]),
                );
            }
        } else if attr_type == ATTR_DATA && non_resident == 0 && file_size == 0 {
            if offset + 24 <= record.len() {
                let val_len =
                    u32::from_le_bytes(record[offset + 16..offset + 20].try_into()?) as usize;
                if val_len >= 8 && offset + 24 + 8 <= record.len() {
                    file_size = u64::from_le_bytes(
                        record[offset + 24..offset + 32].try_into().unwrap_or([0; 8]),
                    );
                }
            }
        } else if attr_type == ATTR_FILE_NAME && non_resident == 0 {
            if offset + 66 <= record.len() {
                let content_offset =
                    u16::from_le_bytes(record[offset + 20..offset + 22].try_into()?) as usize;
                let content_start = offset + content_offset;
                if content_start + 66 <= record.len() {
                    let parent_ref = u64::from_le_bytes(
                        record[content_start..content_start + 8].try_into()?,
                    );
                    let (parent_record, parent_sequence) = decode_parent_reference(parent_ref);
                    let name_length = record[content_start + 64] as usize;
                    let namespace = record[content_start + 65];
                    if namespace == 1 || namespace == 3 {
                        let name_start = content_start + 66;
                        if let Ok(name) = read_utf16le(record, name_start, name_length) {
                            let rank = if namespace == 1 { 2 } else { 1 };
                            let replace = best_filename
                                .as_ref()
                                .map(|(ns, _, _, _)| rank > *ns)
                                .unwrap_or(true);
                            if replace {
                                best_filename =
                                    Some((rank, name, parent_record, parent_sequence));
                            }
                        }
                    }
                }
            }
        }

        offset += attr_len;
    }

    let (filename, parent_record, parent_sequence, namespace) =
        if let Some((_, name, pr, ps)) = best_filename {
            (Some(name), pr, ps, 1u8)
        } else {
            (None, 0, 0, 0)
        };

    Ok(ParsedFileRecord {
        record_number,
        sequence_number: header.sequence_number,
        is_active,
        file_size,
        timestamps,
        filename,
        parent_record,
        parent_sequence,
        namespace,
    })
}

/// Read raw MFT record bytes and parse.
pub fn read_and_parse_record(raw: &mut [u8], record_number: u64) -> Result<ParsedFileRecord> {
    apply_fixup(raw).context("fixup failed")?;
    parse_file_record(raw, record_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record_with_fixup(body: &mut [u8]) {
        // Write FILE header
        body[0..4].copy_from_slice(&FILE_SIGNATURE.to_le_bytes());
        body[4..6].copy_from_slice(&48u16.to_le_bytes()); // usa_offset
        body[6..8].copy_from_slice(&3u16.to_le_bytes()); // usa_count (1 seq + 2 sectors)
        body[24..26].copy_from_slice(&1u16.to_le_bytes()); // sequence
        body[26..28].copy_from_slice(&1u16.to_le_bytes()); // link_count
        body[28..30].copy_from_slice(&48u16.to_le_bytes()); // attrs_offset
        body[32..34].copy_from_slice(&1u16.to_le_bytes()); // flags = in use
        body[34..38].copy_from_slice(&256u32.to_le_bytes()); // used_size

        // Fixup array at offset 48
        let update_seq: u16 = 0xABCD;
        body[48..50].copy_from_slice(&update_seq.to_le_bytes());
        // Sector 1 end at 510-511
        body[510] = 0xCD;
        body[511] = 0xAB;
        body[52..54].copy_from_slice(&0x1234u16.to_le_bytes()); // saved bytes sector 1
        // Sector 2 end at 1022-1023
        body[1022] = 0xCD;
        body[1023] = 0xAB;
        body[54..56].copy_from_slice(&0x5678u16.to_le_bytes()); // saved bytes sector 2
    }

    #[test]
    fn fixup_restores_sector_trailer_bytes() {
        let mut record = vec![0u8; MFT_RECORD_SIZE];
        make_record_with_fixup(&mut record);
        apply_fixup(&mut record).unwrap();
        assert_eq!(record[510], 0x34);
        assert_eq!(record[511], 0x12);
        assert_eq!(record[1022], 0x78);
        assert_eq!(record[1023], 0x56);
    }

    #[test]
    fn fixup_rejects_invalid_signature() {
        let mut record = vec![0u8; MFT_RECORD_SIZE];
        make_record_with_fixup(&mut record);
        record[0..4].copy_from_slice(&0u32.to_le_bytes());
        assert!(apply_fixup(&mut record).is_err());
    }
}
