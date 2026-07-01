#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use ntfs_catalog::gui::EngineVaultApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 600.0])
            .with_title("EngineVault — Engineering File Intelligence"),
        ..Default::default()
    };

    eframe::run_native(
        "EngineVault",
        options,
        Box::new(|cc| Ok(Box::new(EngineVaultApp::new(cc)))),
    )
}
