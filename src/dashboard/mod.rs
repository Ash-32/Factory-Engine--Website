mod tree;

pub use tree::{build_branch_tree, BranchTree, TreeNode, TreeNodeKind};

use std::collections::HashMap;

use crate::classify::ClassifiedFile;

#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub total_files: usize,
    pub total_bytes: u64,
    pub by_category: HashMap<String, usize>,
    pub unclassified: usize,
    pub part_branches: usize,
    pub orphan_paths: usize,
}

pub fn compute_stats(classified: &[ClassifiedFile], tree: &BranchTree) -> DashboardStats {
    let mut stats = DashboardStats {
        total_files: classified.len(),
        ..Default::default()
    };

    for cf in classified {
        stats.total_bytes += cf.entry.file_size;
        *stats.by_category.entry(cf.result.category.clone()).or_insert(0) += 1;
        if cf.result.category.contains("Unclassified") {
            stats.unclassified += 1;
        }
        if !cf.entry.parent_valid {
            stats.orphan_paths += 1;
        }
    }

    stats.part_branches = tree.count_branches();
    stats
}
