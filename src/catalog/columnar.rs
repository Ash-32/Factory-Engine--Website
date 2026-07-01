use std::collections::HashMap;

use super::mft::FileTimestamps;

/// Columnar in-memory catalog indexed by MFT record number.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    /// MFT record number -> dense index in column arrays.
    record_index: HashMap<u64, usize>,
    /// Dense index -> MFT record number.
    record_numbers: Vec<u64>,
    sequence_numbers: Vec<u16>,
    parent_records: Vec<u64>,
    parent_sequences: Vec<u16>,
    file_sizes: Vec<u64>,
    created_times: Vec<u64>,
    modified_times: Vec<u64>,
    mft_modified_times: Vec<u64>,
    accessed_times: Vec<u64>,
    /// Index into string pool for filename (not full path).
    filename_ids: Vec<u32>,
    /// Index into string pool for reconstructed full path.
    path_ids: Vec<u32>,
    /// Whether parent sequence matched during path reconstruction.
    parent_valid: Vec<bool>,
    /// Whether this record represents an active (non-deleted) file.
    is_active: Vec<bool>,
    /// UTF-8 string pool; ids are byte offsets.
    string_pool: Vec<u8>,
    drive_letter: char,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub record_number: u64,
    pub sequence_number: u16,
    pub parent_record: u64,
    pub parent_sequence: u16,
    pub filename: String,
    pub path: String,
    pub file_size: u64,
    pub timestamps: FileTimestamps,
    pub parent_valid: bool,
    pub is_active: bool,
}

impl Catalog {
    pub fn new(drive_letter: char) -> Self {
        Self {
            drive_letter,
            ..Default::default()
        }
    }

    pub fn drive_letter(&self) -> char {
        self.drive_letter
    }

    pub fn len(&self) -> usize {
        self.record_numbers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.record_numbers.is_empty()
    }

    pub fn intern_string(&mut self, s: &str) -> u32 {
        let offset = self.string_pool.len() as u32;
        self.string_pool.extend_from_slice(s.as_bytes());
        self.string_pool.push(0);
        offset
    }

    fn string_at(&self, id: u32) -> String {
        if id as usize >= self.string_pool.len() {
            return String::new();
        }
        let start = id as usize;
        let end = self.string_pool[start..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| start + p)
            .unwrap_or(self.string_pool.len());
        String::from_utf8_lossy(&self.string_pool[start..end]).into_owned()
    }

    pub fn upsert_entry(
        &mut self,
        record_number: u64,
        sequence_number: u16,
        parent_record: u64,
        parent_sequence: u16,
        filename: &str,
        path: &str,
        file_size: u64,
        timestamps: FileTimestamps,
        parent_valid: bool,
        is_active: bool,
    ) {
        let filename_id = self.intern_string(filename);
        let path_id = self.intern_string(path);

        if let Some(&idx) = self.record_index.get(&record_number) {
            self.sequence_numbers[idx] = sequence_number;
            self.parent_records[idx] = parent_record;
            self.parent_sequences[idx] = parent_sequence;
            self.file_sizes[idx] = file_size;
            self.created_times[idx] = timestamps.created;
            self.modified_times[idx] = timestamps.modified;
            self.mft_modified_times[idx] = timestamps.mft_modified;
            self.accessed_times[idx] = timestamps.accessed;
            self.filename_ids[idx] = filename_id;
            self.path_ids[idx] = path_id;
            self.parent_valid[idx] = parent_valid;
            self.is_active[idx] = is_active;
        } else {
            let idx = self.record_numbers.len();
            self.record_index.insert(record_number, idx);
            self.record_numbers.push(record_number);
            self.sequence_numbers.push(sequence_number);
            self.parent_records.push(parent_record);
            self.parent_sequences.push(parent_sequence);
            self.file_sizes.push(file_size);
            self.created_times.push(timestamps.created);
            self.modified_times.push(timestamps.modified);
            self.mft_modified_times.push(timestamps.mft_modified);
            self.accessed_times.push(timestamps.accessed);
            self.filename_ids.push(filename_id);
            self.path_ids.push(path_id);
            self.parent_valid.push(parent_valid);
            self.is_active.push(is_active);
        }
    }

    pub fn get(&self, record_number: u64) -> Option<FileEntry> {
        let idx = *self.record_index.get(&record_number)?;
        Some(FileEntry {
            record_number,
            sequence_number: self.sequence_numbers[idx],
            parent_record: self.parent_records[idx],
            parent_sequence: self.parent_sequences[idx],
            filename: self.string_at(self.filename_ids[idx]),
            path: self.string_at(self.path_ids[idx]),
            file_size: self.file_sizes[idx],
            timestamps: FileTimestamps {
                created: self.created_times[idx],
                modified: self.modified_times[idx],
                mft_modified: self.mft_modified_times[idx],
                accessed: self.accessed_times[idx],
            },
            parent_valid: self.parent_valid[idx],
            is_active: self.is_active[idx],
        })
    }

    pub fn get_sequence(&self, record_number: u64) -> Option<u16> {
        self.record_index
            .get(&record_number)
            .map(|&idx| self.sequence_numbers[idx])
    }

    pub fn entries(&self) -> impl Iterator<Item = FileEntry> + '_ {
        self.record_numbers.iter().enumerate().map(|(idx, &rec)| FileEntry {
            record_number: rec,
            sequence_number: self.sequence_numbers[idx],
            parent_record: self.parent_records[idx],
            parent_sequence: self.parent_sequences[idx],
            filename: self.string_at(self.filename_ids[idx]),
            path: self.string_at(self.path_ids[idx]),
            file_size: self.file_sizes[idx],
            timestamps: FileTimestamps {
                created: self.created_times[idx],
                modified: self.modified_times[idx],
                mft_modified: self.mft_modified_times[idx],
                accessed: self.accessed_times[idx],
            },
            parent_valid: self.parent_valid[idx],
            is_active: self.is_active[idx],
        })
    }

    pub fn active_entries(&self) -> impl Iterator<Item = FileEntry> + '_ {
        self.entries().filter(|e| e.is_active)
    }

    // --- persistence accessors ---

    pub(crate) fn record_index_map(&self) -> &HashMap<u64, usize> {
        &self.record_index
    }

    pub(crate) fn record_numbers_ref(&self) -> &[u64] {
        &self.record_numbers
    }

    pub(crate) fn sequence_numbers_ref(&self) -> &[u16] {
        &self.sequence_numbers
    }

    pub(crate) fn parent_records_ref(&self) -> &[u64] {
        &self.parent_records
    }

    pub(crate) fn parent_sequences_ref(&self) -> &[u16] {
        &self.parent_sequences
    }

    pub(crate) fn file_sizes_ref(&self) -> &[u64] {
        &self.file_sizes
    }

    pub(crate) fn created_times_ref(&self) -> &[u64] {
        &self.created_times
    }

    pub(crate) fn modified_times_ref(&self) -> &[u64] {
        &self.modified_times
    }

    pub(crate) fn mft_modified_times_ref(&self) -> &[u64] {
        &self.mft_modified_times
    }

    pub(crate) fn accessed_times_ref(&self) -> &[u64] {
        &self.accessed_times
    }

    pub(crate) fn filename_ids_ref(&self) -> &[u32] {
        &self.filename_ids
    }

    pub(crate) fn path_ids_ref(&self) -> &[u32] {
        &self.path_ids
    }

    pub(crate) fn parent_valid_ref(&self) -> &[bool] {
        &self.parent_valid
    }

    pub(crate) fn is_active_ref(&self) -> &[bool] {
        &self.is_active
    }

    pub(crate) fn string_pool_ref(&self) -> &[u8] {
        &self.string_pool
    }

    pub(crate) fn rebuild_index(&mut self) {
        self.record_index.clear();
        for (idx, &rec) in self.record_numbers.iter().enumerate() {
            self.record_index.insert(rec, idx);
        }
    }

    pub(crate) fn set_from_columns(
        drive_letter: char,
        record_numbers: Vec<u64>,
        sequence_numbers: Vec<u16>,
        parent_records: Vec<u64>,
        parent_sequences: Vec<u16>,
        file_sizes: Vec<u64>,
        created_times: Vec<u64>,
        modified_times: Vec<u64>,
        mft_modified_times: Vec<u64>,
        accessed_times: Vec<u64>,
        filename_ids: Vec<u32>,
        path_ids: Vec<u32>,
        parent_valid: Vec<bool>,
        is_active: Vec<bool>,
        string_pool: Vec<u8>,
    ) -> Self {
        let mut cat = Self {
            record_index: HashMap::new(),
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
            drive_letter,
        };
        cat.rebuild_index();
        cat
    }
}
