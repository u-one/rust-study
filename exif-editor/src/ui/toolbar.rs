use egui::Ui;

pub struct ToolbarAction {
    pub open_folder: bool,
    pub save_all: bool,
}

pub fn show_toolbar(ui: &mut Ui, has_modified: bool) -> ToolbarAction {
    let mut action = ToolbarAction {
        open_folder: false,
        save_all: false,
    };

    ui.horizontal(|ui| {
        if ui.button("フォルダ選択").clicked() {
            action.open_folder = true;
        }

        ui.add_enabled_ui(has_modified, |ui| {
            if ui.button("保存").clicked() {
                action.save_all = true;
            }
        });
    });

    action
}

pub fn pick_folder() -> Option<std::path::PathBuf> {
    rfd::FileDialog::new().pick_folder()
}
