use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use eframe::egui::{self, Color32, RichText, ScrollArea, Ui};
use egui_extras::{Column, TableBuilder};

use crate::catalog::{load_catalog, save_catalog, FileEntry, FileTimestamps};
use crate::classify::{apply_correction, load_rules, ClassificationEngine, ClassifiedFile};
use crate::dashboard::{build_branch_tree, compute_stats, BranchTree, DashboardStats};
use crate::gui::demo::build_demo_catalog;
use crate::security::AppPaths;

pub struct EngineVaultApp {
    paths: AppPaths,
    rules_path: PathBuf,
    drive: char,
    scan_rx: Option<Receiver<ScanEvent>>,
    scan_status: ScanStatus,
    classified: Vec<ClassifiedFile>,
    tree: BranchTree,
    stats: DashboardStats,
    selected_file: Option<usize>,
    search: String,
    expanded: std::collections::HashSet<String>,
    relabel_category: String,
    status_message: String,
    is_demo: bool,
}

enum ScanEvent {
    Progress(String),
    Finished(Result<ScanOutcome, String>),
}

struct ScanOutcome {
    record_count: u64,
}

enum ScanStatus {
    Idle,
    Scanning(String),
    Ready,
    Error(String),
}

impl EngineVaultApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let paths = AppPaths::resolve().expect("resolve app paths");
        paths.audit("application started");

        let bundled = AppPaths::bundled_rules_next_to_exe();
        let rules_path = paths
            .ensure_rules(&bundled)
            .unwrap_or_else(|_| paths.classification_rules());

        let mut app = Self {
            paths,
            rules_path,
            drive: 'C',
            scan_rx: None,
            scan_status: ScanStatus::Idle,
            classified: Vec::new(),
            tree: BranchTree::default(),
            stats: DashboardStats::default(),
            selected_file: None,
            search: String::new(),
            expanded: Self::initial_expanded_nodes(),
            relabel_category: String::new(),
            status_message: "Welcome — scan a drive or load the demo catalog.".to_string(),
            is_demo: false,
        };

        if app.paths.catalog_path.exists() {
            if app.reload_from_disk().is_ok() {
                app.status_message = "Loaded cached catalog from previous scan.".to_string();
                app.scan_status = ScanStatus::Ready;
            }
        }

        if app.classified.is_empty() {
            app.load_demo();
        }

        app
    }

    fn initial_expanded_nodes() -> std::collections::HashSet<String> {
        let mut s = std::collections::HashSet::new();
        s.insert("cat:Drawing".to_string());
        s.insert("cat:CAD Model".to_string());
        s.insert("cat:Quality".to_string());
        s
    }

    fn reload_from_disk(&mut self) -> anyhow::Result<()> {
        let catalog = load_catalog(&self.paths.catalog_path)?;
        self.rebuild_dashboard(&catalog);
        self.is_demo = false;
        Ok(())
    }

    fn rebuild_dashboard(&mut self, catalog: &crate::Catalog) {
        let rules = load_rules(&self.rules_path).unwrap_or_default();
        let engine = ClassificationEngine::new(rules);
        let entries: Vec<FileEntry> = catalog.active_entries().collect();
        self.classified = engine.classify_all(entries.iter());
        self.tree = build_branch_tree(&self.classified);
        self.stats = compute_stats(&self.classified, &self.tree);
    }

    fn start_scan(&mut self) {
        if matches!(self.scan_status, ScanStatus::Scanning(_)) {
            return;
        }

        let drive = self.drive;
        let catalog_path = self.paths.catalog_path.clone();
        let (tx, rx) = mpsc::channel();
        self.scan_rx = Some(rx);
        self.scan_status = ScanStatus::Scanning(format!("Opening \\\\.\\{}:", drive));
        self.paths.audit(&format!("scan started drive={drive}"));

        thread::spawn(move || {
            let _ = tx.send(ScanEvent::Progress("Reading Master File Table…".into()));
            #[cfg(windows)]
            {
                use crate::catalog::scan_volume;
                match scan_volume(drive) {
                    Ok(info) => {
                        let _ = tx.send(ScanEvent::Progress(format!(
                            "Indexed {} records — saving…",
                            info.record_count
                        )));
                        match save_catalog(&info.catalog, &catalog_path) {
                            Ok(()) => {
                                let _ = tx.send(ScanEvent::Finished(Ok(ScanOutcome {
                                    record_count: info.record_count,
                                })));
                            }
                            Err(e) => {
                                let _ = tx.send(ScanEvent::Finished(Err(e.to_string())));
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(ScanEvent::Finished(Err(e.to_string())));
                    }
                }
            }
            #[cfg(not(windows))]
            {
                let _ = tx.send(ScanEvent::Finished(Err(
                    "NTFS scan requires Windows".into(),
                )));
            }
        });
    }

    fn load_demo(&mut self) {
        let catalog = build_demo_catalog();
        self.rebuild_dashboard(&catalog);
        self.is_demo = true;
        self.scan_status = ScanStatus::Ready;
        self.status_message =
            "Demo catalog loaded — branch tree shows sample engineering files.".to_string();
        self.paths.audit("demo catalog loaded");
    }

    fn poll_scan(&mut self) {
        let events: Vec<ScanEvent> = if let Some(rx) = &self.scan_rx {
            rx.try_iter().collect()
        } else {
            return;
        };

        for ev in events {
            match ev {
                ScanEvent::Progress(msg) => {
                    self.scan_status = ScanStatus::Scanning(msg);
                }
                ScanEvent::Finished(Ok(out)) => {
                    self.scan_rx = None;
                    self.scan_status = ScanStatus::Ready;
                    self.status_message = format!(
                        "Scan complete — {} MFT records indexed.",
                        out.record_count
                    );
                    let _ = self.reload_from_disk();
                    self.paths
                        .audit(&format!("scan complete records={}", out.record_count));
                }
                ScanEvent::Finished(Err(e)) => {
                    self.scan_rx = None;
                    self.scan_status = ScanStatus::Error(e.clone());
                    self.status_message = format!(
                        "Scan failed: {e} — try Run as Administrator or use Demo."
                    );
                    self.paths.audit(&format!("scan failed: {e}"));
                }
            }
        }
    }

    fn apply_relabel(&mut self) {
        let Some(idx) = self.selected_file else {
            return;
        };
        let path = self.classified[idx].entry.path.clone();
        let category = self.relabel_category.trim().to_string();
        if category.is_empty() {
            return;
        }
        if apply_correction(&PathBuf::from(&path), &category).is_ok() {
            if self.is_demo {
                let catalog = build_demo_catalog();
                self.rebuild_dashboard(&catalog);
            } else {
                let _ = self.reload_from_disk();
            }
            self.status_message = format!("Saved correction: {path} → {category}");
            self.paths.audit(&format!("correction {path} -> {category}"));
        }
    }
}

impl eframe::App for EngineVaultApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_scan();
        if matches!(self.scan_status, ScanStatus::Scanning(_)) {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            self.render_top_bar(ui);
        });

        egui::SidePanel::left("stats_panel")
            .min_width(220.0)
            .show(ctx, |ui| {
                self.render_stats(ui);
            });

        egui::SidePanel::right("detail_panel")
            .min_width(280.0)
            .show(ctx, |ui| {
                self.render_detail(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_tree(ui);
        });
    }
}

impl EngineVaultApp {
    fn render_top_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new("EngineVault").color(Color32::from_rgb(56, 189, 248)).size(22.0));
            ui.label(
                RichText::new("Engineering File Intelligence")
                    .color(Color32::GRAY)
                    .size(13.0),
            );
            ui.separator();

            ui.label("Drive:");
            egui::ComboBox::from_id_salt("drive")
                .selected_text(format!("{}:", self.drive))
                .show_ui(ui, |ui| {
                    for d in 'C'..='Z' {
                        ui.selectable_value(&mut self.drive, d, format!("{d}:"));
                    }
                });

            let scanning = matches!(self.scan_status, ScanStatus::Scanning(_));
            if ui
                .add_enabled(!scanning, egui::Button::new("▶  Scan Drive"))
                .clicked()
            {
                self.start_scan();
            }
            if ui.button("📂  Load Demo").clicked() {
                self.load_demo();
            }

            ui.separator();
            ui.label(RichText::new(&self.status_message).size(12.0));
        });
    }

    fn render_stats(&self, ui: &mut Ui) {
        ui.heading("Overview");
        ui.add_space(8.0);

        stat_card(ui, "Total files", &self.stats.total_files.to_string());
        stat_card(ui, "Total size", &format_bytes(self.stats.total_bytes));
        stat_card(ui, "Part branches", &self.stats.part_branches.to_string());
        stat_card(
            ui,
            "Needs review",
            &self.stats.unclassified.to_string(),
        );
        stat_card(ui, "Orphan paths", &self.stats.orphan_paths.to_string());

        ui.add_space(12.0);
        ui.heading("By category");
        ui.add_space(4.0);

        let mut cats: Vec<_> = self.stats.by_category.iter().collect();
        cats.sort_by(|a, b| b.1.cmp(a.1));

        ScrollArea::vertical().show(ui, |ui| {
            for (cat, count) in cats {
                ui.horizontal(|ui| {
                    ui.label(cat.as_str());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(count.to_string()).strong());
                    });
                });
            }
        });

        ui.add_space(16.0);
        ui.separator();
        ui.label(
            RichText::new("🔒 Local-only · No cloud · Data stays on this PC")
                .size(11.0)
                .color(Color32::from_rgb(74, 222, 128)),
        );
    }

    fn render_tree(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Branch Explorer");
            ui.add(
                egui::TextEdit::singleline(&mut self.search)
                    .hint_text("Filter files…")
                    .desired_width(240.0),
            );
        });
        ui.add_space(4.0);

        if self.classified.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(80.0);
                ui.heading("No catalog loaded");
                ui.label("Click Scan Drive (requires Administrator) or Load Demo.");
            });
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            let roots = self.tree.roots.clone();
            for (ci, root) in roots.iter().enumerate() {
                let cat_name = root.label.split("  (").next().unwrap_or("").to_string();
                let cat_key = format!("cat:{cat_name}");
                let default_open = self.expanded.contains(&cat_key);
                egui::CollapsingHeader::new(format!("📁  {}", root.label))
                    .default_open(default_open)
                    .show(ui, |ui| {
                        self.expanded.insert(cat_key);
                        for (bi, child) in root.children.iter().enumerate() {
                            self.render_tree_node(ui, child, &format!("{ci}-{bi}"), 1);
                        }
                    });
            }
        });
    }

    fn render_tree_node(
        &mut self,
        ui: &mut Ui,
        node: &crate::dashboard::TreeNode,
        path: &str,
        depth: usize,
    ) {
        let indent = depth as f32 * 16.0;
        match node.kind {
            crate::dashboard::TreeNodeKind::PartBranch | crate::dashboard::TreeNodeKind::LooseBucket => {
                let icon = if node.kind == crate::dashboard::TreeNodeKind::PartBranch {
                    "🌿"
                } else {
                    "📄"
                };
                let label = format!(
                    "{icon}  {}  ·  {} files · {}",
                    node.label,
                    node.file_count,
                    format_bytes(node.total_bytes)
                );
                egui::CollapsingHeader::new(label)
                    .default_open(node.kind == crate::dashboard::TreeNodeKind::PartBranch)
                    .show(ui, |ui| {
                        for (i, child) in node.children.iter().enumerate() {
                            self.render_tree_node(ui, child, &format!("{path}-{i}"), depth + 1);
                        }
                    });
            }
            crate::dashboard::TreeNodeKind::File => {
                if let Some(idx) = node.file_index {
                    if !self.search.is_empty()
                        && !node
                            .label
                            .to_ascii_lowercase()
                            .contains(&self.search.to_ascii_lowercase())
                    {
                        return;
                    }
                    ui.horizontal(|ui| {
                        ui.add_space(indent);
                        let selected = self.selected_file == Some(idx);
                        if ui
                            .selectable_label(selected, format!("📎  {}", node.label))
                            .clicked()
                        {
                            self.selected_file = Some(idx);
                            let cf = &self.classified[idx];
                            self.relabel_category = cf.result.category.clone();
                        }
                    });
                }
            }
            crate::dashboard::TreeNodeKind::Category => {
                for (i, child) in node.children.iter().enumerate() {
                    self.render_tree_node(ui, child, &format!("{path}-{i}"), depth);
                }
            }
        }
    }

    fn render_detail(&mut self, ui: &mut Ui) {
        ui.heading("File detail");
        ui.add_space(8.0);

        let Some(idx) = self.selected_file else {
            ui.label("Select a file in the branch tree.");
            return;
        };

        let cf = self.classified[idx].clone();

        detail_row(ui, "Filename", &cf.entry.filename);
        detail_row(ui, "Path", &cf.entry.path);
        detail_row(ui, "Category", &cf.result.category);
        detail_row(ui, "Confidence", &format!("{:.0}%", cf.result.confidence * 100.0));
        detail_row(ui, "Size", &format_bytes(cf.entry.file_size));
        detail_row(
            ui,
            "Modified",
            &format_filetime(cf.entry.timestamps.modified),
        );
        detail_row(
            ui,
            "Matched via",
            &cf.result.matched_layers.join(", "),
        );
        detail_row(
            ui,
            "Parent valid",
            if cf.entry.parent_valid { "Yes" } else { "Orphan" },
        );

        ui.add_space(12.0);
        ui.heading("Re-label");
        ui.label("Teach EngineVault — future matches auto-classify:");
        ui.text_edit_singleline(&mut self.relabel_category);

        let categories = [
            "Drawing",
            "CAD Model",
            "Quality",
            "Test/Simulation Report",
            "Supplier/Quote",
            "Manufacturing/Production Data",
            "Correspondence",
            "Other",
        ];
        ui.horizontal_wrapped(|ui| {
            for cat in categories {
                if ui.small_button(cat).clicked() {
                    self.relabel_category = cat.to_string();
                }
            }
        });

        if ui.button("Save correction rule").clicked() {
            self.apply_relabel();
        }

        ui.add_space(16.0);
        ui.separator();
        ui.heading("Recent in category");
        let cat = cf.result.category.clone();
        let peers: Vec<_> = self
            .classified
            .iter()
            .filter(|c| c.result.category == cat && c.entry.path != cf.entry.path)
            .take(8)
            .collect();

        TableBuilder::new(ui)
            .column(Column::remainder())
            .column(Column::auto())
            .header(20.0, |mut row| {
                row.col(|ui| {
                    ui.label(RichText::new("File").strong());
                });
                row.col(|ui| {
                    ui.label(RichText::new("Size").strong());
                });
            })
            .body(|mut body| {
                for peer in peers {
                    body.row(18.0, |mut row| {
                        row.col(|ui| {
                            ui.label(&peer.entry.filename);
                        });
                        row.col(|ui| {
                            ui.label(format_bytes(peer.entry.file_size));
                        });
                    });
                }
            });
    }
}

fn stat_card(ui: &mut Ui, label: &str, value: &str) {
    egui::Frame::none()
        .fill(Color32::from_rgb(30, 41, 59))
        .inner_margin(10.0)
        .rounding(6.0)
        .show(ui, |ui| {
            ui.label(RichText::new(label).size(11.0).color(Color32::GRAY));
            ui.label(RichText::new(value).size(20.0).strong());
        });
    ui.add_space(6.0);
}

fn detail_row(ui: &mut Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{key}:")).strong());
        ui.label(value);
    });
}

fn format_bytes(n: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if n >= GB {
        format!("{:.2} GB", n as f64 / GB as f64)
    } else if n >= MB {
        format!("{:.2} MB", n as f64 / MB as f64)
    } else if n >= KB {
        format!("{:.1} KB", n as f64 / KB as f64)
    } else {
        format!("{n} B")
    }
}

fn format_filetime(ft: u64) -> String {
    if ft == 0 {
        return "—".to_string();
    }
    // FILETIME → approximate ISO date for display
    let unix = (ft / 10_000_000).saturating_sub(11_644_473_600);
    format!("{unix} (unix)")
}
