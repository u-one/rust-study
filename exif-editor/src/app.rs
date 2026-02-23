use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

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

// バックグラウンド読み込みの結果
struct PreviewResult {
    path: PathBuf,
    pixels: Vec<u8>,
    size: [usize; 2],
}

pub struct ExifEditorApp {
    files: Vec<JpgFileEntry>,
    selected_index: Option<usize>,
    editor_state: EditorState,
    status_message: String,
    current_folder: Option<std::path::PathBuf>,
    preview_texture: Option<egui::TextureHandle>,
    preview_path: Option<PathBuf>,
    // バックグラウンド読み込み用
    preview_receiver: Option<Receiver<PreviewResult>>,
    preview_loading: Option<PathBuf>,
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
            preview_receiver: None,
            preview_loading: None,
        }
    }
}

impl ExifEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_japanese_fonts(&cc.egui_ctx);
        Self::default()
    }

    fn start_preview_load(&mut self, path: PathBuf, ctx: &egui::Context) {
        // 既に同じパスを読み込み中または読み込み済みならスキップ
        if self.preview_path.as_ref() == Some(&path) {
            return;
        }
        if self.preview_loading.as_ref() == Some(&path) {
            return;
        }

        // 前のテクスチャをクリア（ローディング表示のため）
        self.preview_texture = None;
        self.preview_path = None;

        // 新しい読み込みを開始
        let (tx, rx) = mpsc::channel();
        self.preview_receiver = Some(rx);
        self.preview_loading = Some(path.clone());

        // 再描画をリクエストするためのコンテキスト
        let ctx_clone = ctx.clone();

        thread::spawn(move || {
            if let Ok(img) = image::open(&path) {
                // プレビュー用にリサイズ（最大800px）
                let img = img.thumbnail(800, 800);
                let rgba = img.to_rgba8();
                let size = [rgba.width() as usize, rgba.height() as usize];
                let pixels = rgba.into_raw();

                let _ = tx.send(PreviewResult { path, pixels, size });
                // 読み込み完了を通知して再描画
                ctx_clone.request_repaint();
            }
        });
    }

    fn check_preview_loaded(&mut self, ctx: &egui::Context) {
        if let Some(receiver) = &self.preview_receiver {
            if let Ok(result) = receiver.try_recv() {
                // テクスチャを作成
                let color_image =
                    egui::ColorImage::from_rgba_unmultiplied(result.size, &result.pixels);
                let texture = ctx.load_texture(
                    "preview",
                    color_image,
                    egui::TextureOptions::LINEAR,
                );

                self.preview_texture = Some(texture);
                self.preview_path = Some(result.path);
                self.preview_receiver = None;
                self.preview_loading = None;
            }
        }
    }

    fn load_folder(&mut self, path: std::path::PathBuf) {
        self.files.clear();
        self.selected_index = None;
        self.preview_texture = None;
        self.preview_path = None;
        self.preview_receiver = None;
        self.preview_loading = None;

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
        // バックグラウンド読み込みの結果をチェック
        self.check_preview_loaded(ctx);

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

        // プレビュー画像の読み込みを開始
        if let Some(path) = selected_path {
            self.start_preview_load(path, ctx);
        } else {
            self.preview_texture = None;
            self.preview_path = None;
        }

        let is_loading = self.preview_loading.is_some();

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
                } else if is_loading {
                    ui.spinner();
                    ui.label("読み込み中...");
                } else if self.selected_index.is_some() {
                    ui.label("プレビューなし");
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
