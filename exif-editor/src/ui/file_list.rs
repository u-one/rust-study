use chrono::{DateTime, Local};
use egui::Ui;
use egui_extras::{Column, TableBuilder};

use crate::exif::types::JpgFileEntry;

pub fn show_file_list(ui: &mut Ui, files: &[JpgFileEntry], selected_index: &mut Option<usize>) {
    let available_height = ui.available_height() * 0.6;

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::initial(180.0).at_least(100.0)) // ファイル名
        .column(Column::initial(130.0).at_least(100.0)) // 撮影日時
        .column(Column::initial(130.0).at_least(100.0)) // 作成日時
        .column(Column::initial(130.0).at_least(100.0)) // 更新日時
        .column(Column::initial(80.0).at_least(60.0))   // 緯度
        .column(Column::initial(80.0).at_least(60.0))   // 経度
        .min_scrolled_height(0.0)
        .max_scroll_height(available_height)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("ファイル名");
            });
            header.col(|ui| {
                ui.strong("撮影日時");
            });
            header.col(|ui| {
                ui.strong("作成日時");
            });
            header.col(|ui| {
                ui.strong("更新日時");
            });
            header.col(|ui| {
                ui.strong("緯度");
            });
            header.col(|ui| {
                ui.strong("経度");
            });
        })
        .body(|mut body| {
            for (idx, file) in files.iter().enumerate() {
                let is_selected = *selected_index == Some(idx);

                body.row(20.0, |mut row| {
                    row.col(|ui| {
                        let label = if file.is_modified {
                            format!("{} *", file.filename)
                        } else {
                            file.filename.clone()
                        };

                        if ui.selectable_label(is_selected, label).clicked() {
                            *selected_index = Some(idx);
                        }
                    });

                    row.col(|ui| {
                        let datetime = format_exif_datetime(&file.edited_exif);
                        if ui.selectable_label(is_selected, datetime).clicked() {
                            *selected_index = Some(idx);
                        }
                    });

                    row.col(|ui| {
                        let created = format_file_datetime(&file.file_created);
                        if ui.selectable_label(is_selected, created).clicked() {
                            *selected_index = Some(idx);
                        }
                    });

                    row.col(|ui| {
                        let modified = format_file_datetime(&file.file_modified);
                        if ui.selectable_label(is_selected, modified).clicked() {
                            *selected_index = Some(idx);
                        }
                    });

                    row.col(|ui| {
                        let lat = format_lat(&file.edited_exif);
                        if ui.selectable_label(is_selected, lat).clicked() {
                            *selected_index = Some(idx);
                        }
                    });

                    row.col(|ui| {
                        let lon = format_lon(&file.edited_exif);
                        if ui.selectable_label(is_selected, lon).clicked() {
                            *selected_index = Some(idx);
                        }
                    });
                });
            }
        });
}

fn format_exif_datetime(exif: &crate::exif::types::ExifData) -> String {
    match (&exif.date, &exif.time) {
        (Some(d), Some(t)) => format!("{} {}", d.format("%Y-%m-%d"), t.format("%H:%M")),
        (Some(d), None) => d.format("%Y-%m-%d").to_string(),
        _ => "-".to_string(),
    }
}

fn format_file_datetime(dt: &Option<DateTime<Local>>) -> String {
    dt.as_ref()
        .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_lat(exif: &crate::exif::types::ExifData) -> String {
    exif.latitude
        .as_ref()
        .map(|c| format!("{:.4}", c.to_decimal()))
        .unwrap_or_else(|| "-".to_string())
}

fn format_lon(exif: &crate::exif::types::ExifData) -> String {
    exif.longitude
        .as_ref()
        .map(|c| format!("{:.4}", c.to_decimal()))
        .unwrap_or_else(|| "-".to_string())
}
