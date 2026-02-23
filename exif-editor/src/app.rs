use std::path::PathBuf;

use crate::exif::reader::{read_exif, read_file_timestamps, scan_jpg_files};
use crate::exif::types::JpgFileEntry;
use crate::exif::writer::write_exif;
use crate::ui::editor::{show_editor, EditorState};
use crate::ui::file_list::show_file_list;
use crate::ui::toolbar::{pick_folder, show_toolbar};

fn setup_japanese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Windowsの日本語フォントを読み込み
    let font_paths = [
        "C:\\Windows\\Fonts\\YuGothM.ttc",  // Yu Gothic Medium
        "C:\\Windows\\Fonts\\meiryo.ttc",   // Meiryo
        "C:\\Windows\\Fonts\\msgothic.ttc", // MS Gothic
    ];

    for path in font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "japanese".to_owned(),
                egui::FontData::from_owned(font_data).into(),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "japanese".to_owned());

            break;
        }
    }

    ctx.set_fonts(fonts);
}

pub struct ExifEditorApp {
    files: Vec<JpgFileEntry>,
    selected_index: Option<usize>,
    editor_state: EditorState,
    status_message: String,
    current_folder: Option<std::path::PathBuf>,
    preview_texture: Option<egui::TextureHandle>,
    preview_path: Option<PathBuf>,
}

impl Default for ExifEditorApp {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_index: None,
            editor_state: EditorState::default(),
            status_message: String::new(),
            current_folder: None,
            preview_texture: None,
            preview_path: None,
        }
    }
}

impl ExifEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_japanese_fonts(&cc.egui_ctx);
        Self::default()
    }

    fn load_preview(&mut self, ctx: &egui::Context, path: &PathBuf) {
        if self.preview_path.as_ref() == Some(path) {
            return; // 既にロード済み
        }

        self.preview_texture = None;
        self.preview_path = None;

        if let Ok(img) = image::open(path) {
            let rgba = img.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let pixels = rgba.into_raw();

            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            let texture = ctx.load_texture(
                "preview",
                color_image,
                egui::TextureOptions::LINEAR,
            );

            self.preview_texture = Some(texture);
            self.preview_path = Some(path.clone());
        }
    }

    fn load_folder(&mut self, path: std::path::PathBuf) {
        self.files.clear();
        self.selected_index = None;

        let jpg_paths = scan_jpg_files(&path);

        for jpg_path in jpg_paths {
            let exif_data = read_exif(&jpg_path);
            let timestamps = read_file_timestamps(&jpg_path);
            let entry = JpgFileEntry::new(
                jpg_path,
                exif_data,
                timestamps.created,
                timestamps.modified,
            );
            self.files.push(entry);
        }

        self.current_folder = Some(path);
        self.status_message = format!("{}ファイルを読み込みました", self.files.len());
    }

    fn has_modified_files(&self) -> bool {
        self.files.iter().any(|f| f.is_modified)
    }

    fn save_modified_files(&mut self) {
        let mut saved_count = 0;
        let mut errors = Vec::new();

        for file in &mut self.files {
            if file.is_modified {
                match write_exif(&file.path, &file.edited_exif) {
                    Ok(()) => {
                        file.original_exif = file.edited_exif.clone();
                        file.is_modified = false;
                        saved_count += 1;
                    }
                    Err(e) => {
                        errors.push(format!("{}: {}", file.filename, e));
                    }
                }
            }
        }

        if errors.is_empty() {
            self.status_message = format!("{}ファイルを保存しました", saved_count);
        } else {
            self.status_message = format!(
                "{}ファイル保存、{}エラー: {}",
                saved_count,
                errors.len(),
                errors.join(", ")
            );
        }
    }

    fn sync_editor_state(&mut self) {
        if let Some(idx) = self.selected_index {
            if let Some(entry) = self.files.get(idx) {
                self.editor_state.load_from_entry(entry);
            }
        }
    }
}

impl eframe::App for ExifEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let action = show_toolbar(ui, self.has_modified_files());

            if action.open_folder {
                if let Some(path) = pick_folder() {
                    self.load_folder(path);
                }
            }

            if action.save_all {
                self.save_modified_files();
            }
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(folder) = &self.current_folder {
                    ui.label(format!("フォルダ: {}", folder.display()));
                    ui.separator();
                }
                ui.label(&self.status_message);
            });
        });

        // 右側: 画像プレビュー
        // 選択されたファイルのパスを取得
        let selected_path = self
            .selected_index
            .and_then(|idx| self.files.get(idx))
            .map(|e| e.path.clone());

        // プレビュー画像をロード
        if let Some(path) = &selected_path {
            self.load_preview(ctx, path);
        } else {
            self.preview_texture = None;
            self.preview_path = None;
        }

        egui::SidePanel::right("preview_panel")
            .default_width(300.0)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("プレビュー");
                ui.separator();

                if let Some(texture) = &self.preview_texture {
                    let available_size = ui.available_size();
                    let tex_size = texture.size_vec2();

                    // アスペクト比を維持してサイズを計算
                    let scale = (available_size.x / tex_size.x)
                        .min(available_size.y / tex_size.y)
                        .min(1.0);
                    let display_size = tex_size * scale;

                    ui.image((texture.id(), display_size));
                } else if self.selected_index.is_some() {
                    ui.label("読み込み中...");
                } else {
                    ui.label("ファイルを選択してください");
                }
            });

        // 左側: ファイル一覧と編集フォーム
        egui::CentralPanel::default().show(ctx, |ui| {
            let prev_selected = self.selected_index;

            show_file_list(ui, &self.files, &mut self.selected_index);

            // 選択が変わったらエディタの状態を更新
            if self.selected_index != prev_selected {
                self.sync_editor_state();
            }

            ui.separator();

            let selected_entry = self.selected_index.and_then(|idx| self.files.get(idx));
            let action = show_editor(ui, selected_entry, &mut self.editor_state);

            if action.apply {
                if let Some(idx) = self.selected_index {
                    if let Some(entry) = self.files.get_mut(idx) {
                        self.editor_state.apply_to_entry(entry);
                    }
                }
            }

            if action.reset {
                if let Some(idx) = self.selected_index {
                    if let Some(entry) = self.files.get_mut(idx) {
                        entry.reset();
                        self.editor_state.load_from_entry(entry);
                    }
                }
            }
        });
    }
}
