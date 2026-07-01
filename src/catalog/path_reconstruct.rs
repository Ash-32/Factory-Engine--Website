use std::collections::HashMap;

use crate::catalog::columnar::Catalog;
use crate::catalog::mft::ROOT_RECORD;

/// Reconstruct full paths from MFT parent references with memoization.
pub fn reconstruct_paths(catalog: &mut Catalog) {
    let count = catalog.len();
    if count == 0 {
        return;
    }

    let mut memo: HashMap<u64, String> = HashMap::new();
    memo.insert(ROOT_RECORD, String::new());

    let records: Vec<u64> = catalog.record_numbers_ref().to_vec();
    let parents: Vec<u64> = catalog.parent_records_ref().to_vec();
    let parent_seqs: Vec<u16> = catalog.parent_sequences_ref().to_vec();
    let filenames: Vec<String> = catalog
        .entries()
        .map(|e| e.filename)
        .collect();

    let mut paths = Vec::with_capacity(count);
    let mut parent_valid = Vec::with_capacity(count);

    for idx in 0..count {
        let rec = records[idx];
        let filename = &filenames[idx];
        let parent = parents[idx];
        let expected_parent_seq = parent_seqs[idx];

        if rec == ROOT_RECORD {
            paths.push(catalog.intern_string("\\"));
            parent_valid.push(true);
            continue;
        }

        let (path, valid) = build_path(
            catalog,
            parent,
            expected_parent_seq,
            filename,
            &mut memo,
        );
        paths.push(catalog.intern_string(&path));
        parent_valid.push(valid);
    }

    let drive = catalog.drive_letter();
    *catalog = Catalog::set_from_columns(
        drive,
        catalog.record_numbers_ref().to_vec(),
        catalog.sequence_numbers_ref().to_vec(),
        catalog.parent_records_ref().to_vec(),
        catalog.parent_sequences_ref().to_vec(),
        catalog.file_sizes_ref().to_vec(),
        catalog.created_times_ref().to_vec(),
        catalog.modified_times_ref().to_vec(),
        catalog.mft_modified_times_ref().to_vec(),
        catalog.accessed_times_ref().to_vec(),
        catalog.filename_ids_ref().to_vec(),
        paths,
        parent_valid,
        catalog.is_active_ref().to_vec(),
        catalog.string_pool_ref().to_vec(),
    );
}

fn build_path(
    catalog: &Catalog,
    parent_record: u64,
    expected_parent_seq: u16,
    filename: &str,
    memo: &mut HashMap<u64, String>,
) -> (String, bool) {
    if filename.is_empty() {
        return (String::new(), false);
    }

    if parent_record == 0 {
        return (filename.to_string(), false);
    }

    let mut valid = true;
    if let Some(actual_seq) = catalog.get_sequence(parent_record) {
        if actual_seq != expected_parent_seq {
            valid = false;
        }
    } else if parent_record != ROOT_RECORD {
        valid = false;
    }

    let parent_path = if let Some(cached) = memo.get(&parent_record) {
        cached.clone()
    } else {
        let built = if parent_record == ROOT_RECORD {
            String::new()
        } else if let Some(entry) = catalog.get(parent_record) {
            let (p, v) = build_path(
                catalog,
                entry.parent_record,
                entry.parent_sequence,
                &entry.filename,
                memo,
            );
            valid &= v;
            p
        } else {
            valid = false;
            String::new()
        };
        memo.insert(parent_record, built.clone());
        built
    };

    let path = if parent_path.is_empty() {
        format!("\\{}", filename)
    } else {
        format!("{}\\{}", parent_path.trim_end_matches('\\'), filename)
    };

    (path, valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::columnar::Catalog;
    use crate::catalog::mft::FileTimestamps;

    #[test]
    fn path_reconstruction_to_root() {
        let mut cat = Catalog::new('C');
        cat.upsert_entry(5, 1, 5, 1, "", "\\", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(100, 1, 5, 1, "Windows", "", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(200, 1, 100, 1, "System32", "", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(300, 1, 200, 1, "kernel32.dll", "", 4096, FileTimestamps::default(), true, true);

        reconstruct_paths(&mut cat);

        let entry = cat.get(300).unwrap();
        assert_eq!(entry.path, "\\Windows\\System32\\kernel32.dll");
        assert!(entry.parent_valid);
    }

    #[test]
    fn parent_sequence_mismatch_flags_invalid() {
        let mut cat = Catalog::new('C');
        cat.upsert_entry(5, 1, 5, 1, "", "\\", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(100, 2, 5, 1, "folder", "", 0, FileTimestamps::default(), true, true);
        cat.upsert_entry(200, 1, 100, 1, "file.txt", "", 0, FileTimestamps::default(), true, true);

        reconstruct_paths(&mut cat);

        let entry = cat.get(200).unwrap();
        assert!(!entry.parent_valid);
    }
}
