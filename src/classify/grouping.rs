use std::collections::HashMap;

use crate::classify::ClassifiedFile;

#[derive(Debug, Clone)]
pub struct PartRevisionGroup {
    pub part_key: String,
    pub revision: Option<String>,
    pub files: Vec<ClassifiedFile>,
}

/// Group classified files by part number and revision extracted from filename.
pub fn group_by_part_revision(files: &[ClassifiedFile]) -> Vec<PartRevisionGroup> {
    let mut groups: HashMap<String, PartRevisionGroup> = HashMap::new();

    for file in files {
        let (part, rev) = extract_part_revision(&file.entry.filename);
        let key = format!("{}|{}", part, rev.as_deref().unwrap_or(""));
        groups
            .entry(key.clone())
            .or_insert_with(|| PartRevisionGroup {
                part_key: part,
                revision: rev,
                files: Vec::new(),
            })
            .files
            .push(file.clone());
    }

    let mut result: Vec<_> = groups.into_values().collect();
    result.sort_by(|a, b| a.part_key.cmp(&b.part_key));
    result
}

fn extract_part_revision(filename: &str) -> (String, Option<String>) {
    let stem = filename
        .rsplit_once('.')
        .map(|(s, _)| s)
        .unwrap_or(filename);

    let upper = stem.to_ascii_uppercase();

    if let Some(pos) = upper.find("_REV") {
        let part = stem[..pos].to_string();
        let rev = stem[pos + 4..].to_string();
        return (part, Some(rev));
    }

    if let Some(pos) = upper.find("-REV") {
        let part = stem[..pos].to_string();
        let rev = stem[pos + 4..].to_string();
        return (part, Some(rev));
    }

    (stem.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::FileEntry;
    use crate::classify::ClassificationResult;

    fn make_classified(name: &str) -> ClassifiedFile {
        ClassifiedFile {
            entry: FileEntry {
                record_number: 1,
                sequence_number: 1,
                parent_record: 5,
                parent_sequence: 1,
                filename: name.to_string(),
                path: format!(r"\x\{}", name),
                file_size: 0,
                timestamps: crate::catalog::mft::FileTimestamps::default(),
                parent_valid: true,
                is_active: true,
            },
            result: ClassificationResult {
                category: "Drawing".to_string(),
                confidence: 0.8,
                matched_layers: vec![],
            },
        }
    }

    #[test]
    fn groups_by_part_and_revision() {
        let files = vec![
            make_classified("ABC-100_REV-A.pdf"),
            make_classified("ABC-100_REV-B.pdf"),
            make_classified("ABC-100_REV-A.dwg"),
        ];
        let groups = group_by_part_revision(&files);
        assert_eq!(groups.len(), 2);
    }
}
