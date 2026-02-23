# EXIF Editor

JPG画像のEXIF情報（撮影日時、GPS座標）を一覧編集するWindowsアプリ。

> **Note**: このプロジェクトはAI（Claude）により生成されたコードです。

## 機能

- フォルダ内のJPG/JPEG画像を一覧表示
- EXIF情報の表示・編集
  - 撮影日時
  - GPS座標（緯度・経度）
- ファイルの作成日時・更新日時の表示
- 画像プレビュー
- 変更の一括保存

## 使い方

### ビルド・実行

```bash
cd exif-editor
cargo run
```

### 操作

1. 「フォルダ選択」でJPG画像が含まれるフォルダを選択
2. 一覧からファイルを選択
3. 下部のフォームで日時・GPS座標を編集
4. 「適用」で変更を反映（`*`マークが付く）
5. 「保存」で全ての変更をファイルに書き込み

## 技術構成

| 用途 | クレート |
|-----|---------|
| GUI | eframe, egui, egui_extras |
| EXIF読み書き | little_exif |
| 画像プレビュー | image |
| ファイルダイアログ | rfd |
| 日時処理 | chrono |

## ドキュメント

- [ARCHITECTURE.md](ARCHITECTURE.md) - アプリ構造
- [EGUI_BASICS.md](EGUI_BASICS.md) - egui基礎
