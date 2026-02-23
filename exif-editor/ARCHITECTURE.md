# EXIF Editor アーキテクチャ

## 概要

Rust + egui で作成したJPG画像のEXIF情報編集アプリ。

## ディレクトリ構成

```
src/
├── main.rs        # エントリポイント、ウィンドウ設定
├── app.rs         # アプリ状態管理、メインUI制御
├── ui/
│   ├── toolbar.rs   # ツールバー（フォルダ選択・保存）
│   ├── file_list.rs # ファイル一覧テーブル
│   └── editor.rs    # EXIF編集フォーム
├── exif/
│   ├── types.rs     # データ構造定義
│   ├── reader.rs    # EXIF/ファイル情報読み取り
│   └── writer.rs    # EXIF書き込み
└── gps/
    └── converter.rs # GPS座標変換ユーティリティ
```

## データフロー

```
[フォルダ選択] → reader.rs → [JpgFileEntry] → file_list.rs（表示）
                                    ↓
                              editor.rs（編集）
                                    ↓
[保存ボタン] → writer.rs → [ファイル書き込み]
```

## 主要構造体

### ExifEditorApp (app.rs)
アプリ全体の状態を保持。`eframe::App`を実装。

| フィールド | 説明 |
|-----------|------|
| files | 読み込んだJPGファイル一覧 |
| selected_index | 選択中のファイルインデックス |
| editor_state | 編集フォームの入力状態 |
| preview_texture | プレビュー画像のテクスチャ |

### JpgFileEntry (exif/types.rs)
1つのJPGファイルの情報。

| フィールド | 説明 |
|-----------|------|
| path | ファイルパス |
| original_exif | 読み込み時のEXIF |
| edited_exif | 編集後のEXIF |
| is_modified | 変更フラグ |
| file_created/modified | ファイルシステムの日時 |

### ExifData (exif/types.rs)
EXIF情報。

| フィールド | 説明 |
|-----------|------|
| date, time | 撮影日時 |
| latitude, longitude | GPS座標 |

## UI構成

```
┌─────────────────────────────────────────────────┐
│ TopBottomPanel (top): toolbar.rs                │
├───────────────────────────────┬─────────────────┤
│ CentralPanel                  │ SidePanel       │
│ ┌───────────────────────────┐ │ (right)         │
│ │ file_list.rs              │ │                 │
│ │ TableBuilder              │ │ プレビュー      │
│ └───────────────────────────┘ │ 画像表示        │
│ ┌───────────────────────────┐ │                 │
│ │ editor.rs                 │ │                 │
│ │ 編集フォーム              │ │                 │
│ └───────────────────────────┘ │                 │
├───────────────────────────────┴─────────────────┤
│ TopBottomPanel (bottom): ステータスバー         │
└─────────────────────────────────────────────────┘
```

## 依存クレート

| クレート | 用途 |
|---------|------|
| eframe/egui | GUI フレームワーク |
| egui_extras | テーブル表示 |
| little_exif | EXIF読み書き |
| image | 画像プレビュー読み込み |
| rfd | ファイルダイアログ |
| chrono | 日時処理 |
