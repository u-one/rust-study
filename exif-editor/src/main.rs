#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod exif;
mod gps;
mod ui;

use app::ExifEditorApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "EXIF Editor",
        options,
        Box::new(|cc| Ok(Box::new(ExifEditorApp::new(cc)))),
    )
}
