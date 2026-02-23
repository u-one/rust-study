use chrono::{NaiveDate, NaiveTime};
use egui::Ui;

use crate::exif::types::{GpsCoordinate, JpgFileEntry};

pub struct EditorState {
    pub date_str: String,
    pub time_str: String,
    pub lat_str: String,
    pub lat_ref: bool, // true = N, false = S
    pub lon_str: String,
    pub lon_ref: bool, // true = E, false = W
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            date_str: String::new(),
            time_str: String::new(),
            lat_str: String::new(),
            lat_ref: true,
            lon_str: String::new(),
            lon_ref: true,
        }
    }
}

impl EditorState {
    pub fn load_from_entry(&mut self, entry: &JpgFileEntry) {
        let exif = &entry.edited_exif;

        self.date_str = exif
            .date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_default();

        self.time_str = exif
            .time
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or_default();

        if let Some(lat) = &exif.latitude {
            self.lat_str = format!("{:.6}", lat.degrees);
            self.lat_ref = lat.is_positive;
        } else {
            self.lat_str.clear();
            self.lat_ref = true;
        }

        if let Some(lon) = &exif.longitude {
            self.lon_str = format!("{:.6}", lon.degrees);
            self.lon_ref = lon.is_positive;
        } else {
            self.lon_str.clear();
            self.lon_ref = true;
        }
    }

    pub fn apply_to_entry(&self, entry: &mut JpgFileEntry) {
        // 日付
        entry.edited_exif.date = NaiveDate::parse_from_str(&self.date_str, "%Y-%m-%d").ok();

        // 時刻
        entry.edited_exif.time = NaiveTime::parse_from_str(&self.time_str, "%H:%M:%S").ok();

        // 緯度
        entry.edited_exif.latitude = self.lat_str.parse::<f64>().ok().map(|d| GpsCoordinate {
            degrees: d.abs(),
            is_positive: self.lat_ref,
        });

        // 経度
        entry.edited_exif.longitude = self.lon_str.parse::<f64>().ok().map(|d| GpsCoordinate {
            degrees: d.abs(),
            is_positive: self.lon_ref,
        });

        entry.mark_modified();
    }
}

pub struct EditorAction {
    pub apply: bool,
    pub reset: bool,
}

pub fn show_editor(
    ui: &mut Ui,
    entry: Option<&JpgFileEntry>,
    state: &mut EditorState,
) -> EditorAction {
    let mut action = EditorAction {
        apply: false,
        reset: false,
    };

    ui.group(|ui| {
        if let Some(entry) = entry {
            ui.label(format!("編集: {}", entry.filename));
            ui.add_space(8.0);

            egui::Grid::new("editor_grid")
                .num_columns(2)
                .spacing([10.0, 6.0])
                .show(ui, |ui| {
                    // 日付
                    ui.label("日付:");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.date_str).desired_width(100.0));
                        ui.label("(YYYY-MM-DD)");
                    });
                    ui.end_row();

                    // 時刻
                    ui.label("時刻:");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.time_str).desired_width(100.0));
                        ui.label("(HH:MM:SS)");
                    });
                    ui.end_row();

                    // 緯度
                    ui.label("緯度:");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.lat_str).desired_width(120.0));
                        egui::ComboBox::from_id_salt("lat_ref")
                            .selected_text(if state.lat_ref { "N" } else { "S" })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut state.lat_ref, true, "N");
                                ui.selectable_value(&mut state.lat_ref, false, "S");
                            });
                    });
                    ui.end_row();

                    // 経度
                    ui.label("経度:");
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut state.lon_str).desired_width(120.0));
                        egui::ComboBox::from_id_salt("lon_ref")
                            .selected_text(if state.lon_ref { "E" } else { "W" })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut state.lon_ref, true, "E");
                                ui.selectable_value(&mut state.lon_ref, false, "W");
                            });
                    });
                    ui.end_row();
                });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("適用").clicked() {
                    action.apply = true;
                }
                if ui.button("リセット").clicked() {
                    action.reset = true;
                }
            });
        } else {
            ui.label("ファイルを選択してください");
        }
    });

    action
}
