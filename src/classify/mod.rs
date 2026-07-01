mod corrections;
mod engine;
mod grouping;
mod rules;

pub use corrections::{apply_correction, load_corrections, save_correction, UserCorrection};
pub use engine::{ClassificationEngine, ClassificationResult, ClassifiedFile};
pub use grouping::{group_by_part_revision, PartRevisionGroup};
pub use rules::{load_rules, CategoryRule, ClassificationSettings, RulesConfig};
