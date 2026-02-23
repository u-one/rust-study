# egui 基礎

## egui とは

Rust用の即時モードGUIライブラリ。毎フレームUIを再構築する設計。

## 即時モード vs 保持モード

```rust
// 即時モード（egui）: 毎フレームUI記述
fn update(&mut self, ctx: &egui::Context) {
    if ui.button("Click").clicked() {
        self.count += 1;
    }
    ui.label(format!("Count: {}", self.count));
}

// 保持モード（他のGUIライブラリ）: 初期化時にUIを構築
// let button = Button::new("Click");
// button.on_click(|| ...);
```

## 基本構造

### eframe::App トレイト

```rust
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 毎フレーム呼ばれる
        // ここでUI構築
    }
}
```

### パネル

画面を分割する領域。

```rust
// 上部パネル
egui::TopBottomPanel::top("id").show(ctx, |ui| { ... });

// 下部パネル
egui::TopBottomPanel::bottom("id").show(ctx, |ui| { ... });

// 左パネル
egui::SidePanel::left("id").show(ctx, |ui| { ... });

// 右パネル
egui::SidePanel::right("id").show(ctx, |ui| { ... });

// 残り領域（最後に呼ぶ）
egui::CentralPanel::default().show(ctx, |ui| { ... });
```

### 基本ウィジェット

```rust
// ラベル
ui.label("テキスト");
ui.heading("見出し");

// ボタン
if ui.button("ボタン").clicked() {
    // クリック時の処理
}

// テキスト入力
ui.text_edit_singleline(&mut self.text);

// チェックボックス
ui.checkbox(&mut self.checked, "ラベル");

// コンボボックス
egui::ComboBox::from_label("選択")
    .selected_text(&self.selected)
    .show_ui(ui, |ui| {
        ui.selectable_value(&mut self.selected, "A", "A");
        ui.selectable_value(&mut self.selected, "B", "B");
    });
```

### レイアウト

```rust
// 水平配置
ui.horizontal(|ui| {
    ui.label("左");
    ui.label("右");
});

// 垂直配置（デフォルト）
ui.vertical(|ui| {
    ui.label("上");
    ui.label("下");
});

// グリッド
egui::Grid::new("id").show(ui, |ui| {
    ui.label("A1"); ui.label("B1"); ui.end_row();
    ui.label("A2"); ui.label("B2"); ui.end_row();
});

// グループ（枠付き）
ui.group(|ui| {
    ui.label("グループ内");
});

// セパレータ
ui.separator();

// スペース
ui.add_space(10.0);
```

### テーブル (egui_extras)

```rust
use egui_extras::{TableBuilder, Column};

TableBuilder::new(ui)
    .column(Column::initial(100.0))
    .column(Column::initial(100.0))
    .header(20.0, |mut header| {
        header.col(|ui| { ui.label("列1"); });
        header.col(|ui| { ui.label("列2"); });
    })
    .body(|mut body| {
        body.row(20.0, |mut row| {
            row.col(|ui| { ui.label("A"); });
            row.col(|ui| { ui.label("B"); });
        });
    });
```

### 画像表示

```rust
// テクスチャ読み込み
let image = image::open(path).unwrap().to_rgba8();
let size = [image.width() as usize, image.height() as usize];
let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);
let texture = ctx.load_texture("name", color_image, Default::default());

// 表示
ui.image((texture.id(), texture.size_vec2()));
```

## イベント処理

```rust
// クリック
if ui.button("btn").clicked() { ... }

// 選択可能ラベル
if ui.selectable_label(is_selected, "item").clicked() {
    selected = true;
}

// ドラッグ
let response = ui.add(egui::DragValue::new(&mut value));
if response.changed() { ... }
```

## 状態管理のポイント

1. **状態はApp構造体に保持** - UIは状態を参照して描画
2. **変更は即座に反映** - 毎フレーム再描画されるため
3. **IDの一意性** - パネルやウィジェットのIDは重複不可

## 参考

- [egui公式ドキュメント](https://docs.rs/egui)
- [egui デモ](https://www.egui.rs/)
- [eframe テンプレート](https://github.com/emilk/eframe_template)
