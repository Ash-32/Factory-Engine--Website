use std::collections::{HashMap, HashSet};

use crate::classify::{group_by_part_revision, ClassifiedFile};
use crate::security::part_group_categories;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreeNodeKind {
    Category,
    PartBranch,
    LooseBucket,
    File,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub label: String,
    pub kind: TreeNodeKind,
    pub children: Vec<TreeNode>,
    /// Index into the classified slice when kind == File.
    pub file_index: Option<usize>,
    pub file_count: usize,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct BranchTree {
    pub roots: Vec<TreeNode>,
}

impl BranchTree {
    pub fn count_branches(&self) -> usize {
        self.roots
            .iter()
            .map(count_branches_recursive)
            .sum()
    }
}

fn count_branches_recursive(node: &TreeNode) -> usize {
    let here = if node.kind == TreeNodeKind::PartBranch {
        1
    } else {
        0
    };
    here + node
        .children
        .iter()
        .map(count_branches_recursive)
        .sum::<usize>()
}

pub fn build_branch_tree(classified: &[ClassifiedFile]) -> BranchTree {
    let groupable: HashSet<&str> = part_group_categories().iter().copied().collect();

    let mut by_category: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, cf) in classified.iter().enumerate() {
        by_category
            .entry(cf.result.category.clone())
            .or_default()
            .push(idx);
    }

    let mut category_names: Vec<_> = by_category.keys().cloned().collect();
    category_names.sort_by(|a, b| {
        if a.contains("Unclassified") {
            std::cmp::Ordering::Greater
        } else if b.contains("Unclassified") {
            std::cmp::Ordering::Less
        } else {
            a.cmp(b)
        }
    });

    let mut roots = Vec::new();

    for cat in category_names {
        let indices = &by_category[&cat];
        let cat_bytes: u64 = indices.iter().map(|&i| classified[i].entry.file_size).sum();

        if groupable.contains(cat.as_str()) {
            let subset: Vec<ClassifiedFile> =
                indices.iter().map(|&i| classified[i].clone()).collect();
            let groups = group_by_part_revision(&subset);

            let mut branch_children = Vec::new();
            let mut grouped_indices = HashSet::new();

            for group in &groups {
                if group.files.len() <= 1 && group.revision.is_none() {
                    continue;
                }
                let label = match &group.revision {
                    Some(rev) => format!("{}  ·  Rev {}", group.part_key, rev),
                    None => format!("{}  ·  (no rev)", group.part_key),
                };
                let mut file_nodes = Vec::new();
                for gf in &group.files {
                    if let Some(&global_idx) = indices
                        .iter()
                        .find(|&&i| classified[i].entry.path == gf.entry.path)
                    {
                        grouped_indices.insert(global_idx);
                        file_nodes.push(file_node(classified, global_idx));
                    }
                }
                let (fc, tb) = aggregate(&file_nodes);
                branch_children.push(TreeNode {
                    label,
                    kind: TreeNodeKind::PartBranch,
                    children: file_nodes,
                    file_index: None,
                    file_count: fc,
                    total_bytes: tb,
                });
            }

            let loose: Vec<usize> = indices
                .iter()
                .copied()
                .filter(|i| !grouped_indices.contains(i))
                .collect();

            if !loose.is_empty() {
                let file_nodes: Vec<_> = loose.iter().map(|&i| file_node(classified, i)).collect();
                let (fc, tb) = aggregate(&file_nodes);
                branch_children.push(TreeNode {
                    label: "Other files".to_string(),
                    kind: TreeNodeKind::LooseBucket,
                    children: file_nodes,
                    file_index: None,
                    file_count: fc,
                    total_bytes: tb,
                });
            }

            roots.push(TreeNode {
                label: format!("{}  ({})", cat, indices.len()),
                kind: TreeNodeKind::Category,
                children: branch_children,
                file_index: None,
                file_count: indices.len(),
                total_bytes: cat_bytes,
            });
        } else {
            let file_nodes: Vec<_> = indices.iter().map(|&i| file_node(classified, i)).collect();
            roots.push(TreeNode {
                label: format!("{}  ({})", cat, indices.len()),
                kind: TreeNodeKind::Category,
                children: file_nodes,
                file_index: None,
                file_count: indices.len(),
                total_bytes: cat_bytes,
            });
        }
    }

    BranchTree { roots }
}

fn file_node(classified: &[ClassifiedFile], idx: usize) -> TreeNode {
    let cf = &classified[idx];
    TreeNode {
        label: cf.entry.filename.clone(),
        kind: TreeNodeKind::File,
        children: vec![],
        file_index: Some(idx),
        file_count: 1,
        total_bytes: cf.entry.file_size,
    }
}

fn aggregate(nodes: &[TreeNode]) -> (usize, u64) {
    (
        nodes.iter().map(|n| n.file_count).sum(),
        nodes.iter().map(|n| n.total_bytes).sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::mft::FileTimestamps;
    use crate::catalog::FileEntry;
    use crate::classify::ClassificationResult;

    fn classified(name: &str, path: &str, category: &str) -> ClassifiedFile {
        ClassifiedFile {
            entry: FileEntry {
                record_number: 1,
                sequence_number: 1,
                parent_record: 5,
                parent_sequence: 1,
                filename: name.to_string(),
                path: path.to_string(),
                file_size: 1024,
                timestamps: FileTimestamps::default(),
                parent_valid: true,
                is_active: true,
            },
            result: ClassificationResult {
                category: category.to_string(),
                confidence: 0.9,
                matched_layers: vec![],
            },
        }
    }

    #[test]
    fn branch_tree_has_part_branches_for_drawings() {
        let files = vec![
            classified("ABC-100_REV-A.pdf", r"\d\ABC-100_REV-A.pdf", "Drawing"),
            classified("ABC-100_REV-A.dwg", r"\d\ABC-100_REV-A.dwg", "Drawing"),
            classified("ABC-100_REV-B.pdf", r"\d\ABC-100_REV-B.pdf", "Drawing"),
        ];
        let tree = build_branch_tree(&files);
        assert!(!tree.roots.is_empty());
        assert!(tree.count_branches() >= 2);
    }
}
