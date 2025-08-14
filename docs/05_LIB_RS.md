# lib.rs - Tauri アプリケーションエントリーポイント解説

## 📋 概要

`lib.rs`はTaskNagアプリケーションのRustバックエンドの中核となるエントリーポイントファイルです。Tauriフレームワークを使用してデスクトップアプリケーションを構築し、システムトレイ統合、ウィンドウ管理、データベース初期化、コマンドハンドラーの登録を行います。

---

## 🏗️ ファイル構造

### モジュール構成
```rust
pub mod commands;     // Tauri コマンドハンドラー
pub mod database;     // データベース接続・管理
pub mod error;        // エラー型定義
pub mod models;       // データモデル
pub mod services;     // ビジネスロジック

pub mod tests;        // テストモジュール
```

**アーキテクチャ設計:**
- **commands**: フロントエンド-バックエンド間のAPI層
- **database**: データ永続化レイヤー
- **services**: ビジネスロジック抽象化
- **models**: データ構造定義
- **error**: 統一エラーハンドリング

### 主要インポート
```rust
use database::Database;
use services::TaskService;
use tauri::{
  AppHandle, Manager, WindowEvent, 
  tray::{TrayIconBuilder, TrayIconEvent, MouseButton},
  menu::{Menu, MenuItem, MenuEvent}
};
```

**依存関係:**
- **Database**: SQLite データベース管理
- **TaskService**: タスク関連ビジネスロジック
- **Tauri API**: ウィンドウ・トレイ・メニュー制御

---

## 🖥️ システムトレイ機能

### トレイアイコンイベント処理
```rust
fn handle_tray_event(app: &AppHandle, event: TrayIconEvent) {
  match event {
    TrayIconEvent::Click { button, .. } => {
      match button {
        MouseButton::Left => {
          if let Some(window) = app.get_webview_window("main") {
            // シングルクリックでは表示のみ（非表示にはしない）
            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.unminimize();
          }
        }
        _ => {}
      }
    }
    TrayIconEvent::DoubleClick { .. } => {
      if let Some(window) = app.get_webview_window("main") {
        // ダブルクリックでは確実に表示・フォーカス・最大化
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
      }
    }
    _ => {}
  }
}
```

**インタラクション設計:**
- **左シングルクリック**: ウィンドウ表示・フォーカス・最小化解除
- **ダブルクリック**: 確実な表示・フォーカス・最大化
- **エラーハンドリング**: `let _` で意図的に結果を無視

### コンテキストメニュー処理
```rust
fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
  match event.id().as_ref() {
    "show" => {
      if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
      }
    }
    "hide" => {
      if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
      }
    }
    "quit" => {
      std::process::exit(0);
    }
    _ => {}
  }
}
```

**メニュー項目:**
- **"show"**: ウィンドウ表示・復元・フォーカス
- **"hide"**: ウィンドウ非表示
- **"quit"**: アプリケーション終了 (`exit(0)`)

---

## 🚪 ウィンドウ管理

### ウィンドウ閉じるボタンの挙動制御
```rust
.on_window_event(|window, event| {
  match event {
    WindowEvent::CloseRequested { api, .. } => {
      // ウィンドウを閉じる代わりに最小化
      let _ = window.hide();
      api.prevent_close();
    }
    _ => {}
  }
})
```

**システムトレイ常駐の実装:**
- **通常の閉じる動作を阻止**: `api.prevent_close()`
- **非表示に変更**: `window.hide()`
- **バックグラウンド実行継続**: プロセスは終了しない

**UX設計理念:**
```
従来: [X]ボタン → アプリ終了
TaskNag: [X]ボタン → システムトレイに最小化
```

---

## 🔧 アプリケーション初期化

### セットアップシーケンス
```rust
.setup(|app| {
  // 1. ログプラグイン初期化 (デバッグビルドのみ)
  if cfg!(debug_assertions) {
    app.handle().plugin(
      tauri_plugin_log::Builder::default()
        .level(log::LevelFilter::Info)
        .build(),
    )?;
  }
  
  // 2. 通知プラグイン初期化
  app.handle().plugin(tauri_plugin_notification::init())?;
  
  // 3. データベース・サービス初期化
  let handle = app.handle().clone();
  let icon = app.default_window_icon().unwrap().clone();
  
  tauri::async_runtime::block_on(async move {
    let db = Database::new(&handle)
      .await
      .expect("Failed to initialize database");
    
    let task_service = TaskService::new(db);
    handle.manage(task_service);
  });
  
  // 4. システムトレイ構築
  // ... (メニュー・トレイアイコン作成)
  
  Ok(())
})
```

**初期化順序の重要性:**
1. **プラグイン**: ログ・通知機能の有効化
2. **データベース**: 非同期初期化とエラーハンドリング
3. **サービス**: 状態管理への登録
4. **システムトレイ**: UI統合の完了

### 非同期データベース初期化
```rust
tauri::async_runtime::block_on(async move {
  let db = Database::new(&handle)
    .await
    .expect("Failed to initialize database");
  
  let task_service = TaskService::new(db);
  handle.manage(task_service);
});
```

**設計パターン:**
- **`block_on`**: セットアップ時の同期的な非同期処理
- **`expect`**: 初期化失敗時のクラッシュ（設計として正しい）
- **`handle.manage`**: サービスをアプリ状態に登録

---

## 🎛️ システムトレイ構築

### メニュー作成
```rust
// メニューアイテム作成
let show_item = MenuItem::with_id(app, "show", "表示", true, None::<&str>)?;
let hide_item = MenuItem::with_id(app, "hide", "非表示", true, None::<&str>)?;
let quit_item = MenuItem::with_id(app, "quit", "終了", true, None::<&str>)?;
let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;
```

### トレイアイコン構築
```rust
let _tray = TrayIconBuilder::new()
  .icon(icon)                    // アプリのデフォルトアイコン使用
  .title("TaskNag")              // ツールチップテキスト
  .menu(&menu)                   // 右クリックメニュー
  .on_tray_icon_event(|tray, event| handle_tray_event(tray.app_handle(), event))
  .on_menu_event(|app, event| handle_menu_event(app, event))
  .build(app)?;
```

**Builder パターンの活用:**
- **fluent interface**: メソッドチェーンによる設定
- **イベントハンドラー**: クロージャでのコールバック登録
- **エラー処理**: `?` 演算子による早期リターン

---

## 📡 コマンドハンドラー登録

### Tauri コマンド定義
```rust
.invoke_handler(tauri::generate_handler![
  // タスク管理コマンド
  commands::task_commands::create_task,
  commands::task_commands::get_tasks,
  commands::task_commands::get_task_by_id,
  commands::task_commands::update_task,
  commands::task_commands::delete_task,
  commands::task_commands::get_tasks_by_status,
  commands::task_commands::move_task,
  commands::task_commands::get_incomplete_task_count,
  commands::task_commands::update_tray_title,
  
  // 通知機能コマンド
  commands::task_commands::check_notifications,
  commands::task_commands::update_task_notification_settings,
  
  // 階層タスク機能
  commands::task_commands::get_children,
  commands::task_commands::get_task_with_children,
  commands::task_commands::update_progress,
  commands::task_commands::calculate_and_update_progress,
  commands::task_commands::get_root_tasks,
  
  // ログ機能コマンド
  commands::log_commands::write_log,
  commands::log_commands::get_log_file_path,
  commands::log_commands::read_recent_logs,
])
```

**API設計の特徴:**
- **CRUD操作**: create, get, update, delete の基本操作
- **フィルタリング**: status, children による絞り込み
- **通知統合**: check_notifications, update_notification_settings
- **階層管理**: 親子関係とプログレス計算
- **ログ機能**: デバッグ・監視用途

---

## 🔄 エラーハンドリング戦略

### Result型の使用パターン
```rust
// セットアップ時のエラー処理
app.handle().plugin(tauri_plugin_notification::init())?;

// データベース初期化の厳格なエラー処理
let db = Database::new(&handle)
  .await
  .expect("Failed to initialize database");

// ウィンドウ操作の寛容なエラー処理
let _ = window.show();
let _ = window.set_focus();
```

**エラー処理方針:**
- **初期化**: `?` または `expect` で早期終了
- **UI操作**: `let _` で意図的な無視
- **ビジネスロジック**: Result型で適切な伝播

### 非同期処理との統合
```rust
tauri::async_runtime::block_on(async move {
  // 非同期データベース操作
});
```

**`block_on` の使用理由:**
- セットアップは同期的である必要
- データベース初期化は非同期API
- ブロッキング実行で順序保証

---

## 🧪 テスト戦略

### テストモジュール
```rust
pub mod tests;
```

**テスト構成 (参照):**
- task_crud_tests.rs
- hierarchical_task_tests.rs  
- notification_system_tests.rs
- error_handling_tests.rs

### 統合テストの考慮事項
```rust
// lib.rs は統合テストでのエントリーポイント
#[cfg(test)]
mod integration_tests {
  use super::*;
  
  // Tauri アプリ全体の統合テスト
  // システムトレイ機能のテスト
  // ウィンドウ管理のテスト
}
```

---

## 🚀 パフォーマンス特性

### 起動時間最適化
```rust
// 条件付きログプラグイン (リリースビルドでは無効)
if cfg!(debug_assertions) {
  app.handle().plugin(/* ... */)?;
}

// 非同期初期化のブロッキング実行
tauri::async_runtime::block_on(/* ... */);
```

### メモリ効率
- **アイコン複製**: `app.default_window_icon().unwrap().clone()`
- **ハンドル複製**: `app.handle().clone()`
- **適切なライフタイム管理**: 所有権の明示的な移動

---

## 🔧 設定・カスタマイゼーション

### ビルド設定による分岐
```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // モバイル対応の条件付きコンパイル
}

if cfg!(debug_assertions) {
  // デバッグビルド限定の機能
}
```

### 多言語対応の基盤
```rust
// 現在: ハードコード
let show_item = MenuItem::with_id(app, "show", "表示", true, None::<&str>)?;

// 改善案: 国際化対応
let show_item = MenuItem::with_id(app, "show", &t!("menu.show"), true, None::<&str>)?;
```

---

## 🛡️ セキュリティ考慮事項

### プロセス終了の処理
```rust
"quit" => {
  std::process::exit(0);  // 強制終了
}
```

**リスク評価:**
- **データ損失**: 保存されていないデータは失われる
- **改善案**: グレースフルシャットダウンの実装

### ウィンドウアクセス制御
```rust
if let Some(window) = app.get_webview_window("main") {
  // ウィンドウが存在する場合のみ操作
}
```

**防御的プログラミング:**
- ウィンドウの存在確認
- Option型による安全なアクセス

---

## 📝 開発者向けノート

### アーキテクチャパターン
- **Service Layer**: TaskService による抽象化
- **Command Pattern**: Tauri コマンドによるAPI設計
- **State Management**: `handle.manage()` による依存注入

### 拡張ポイント
```rust
// 新しいサービスの追加
let notification_service = NotificationService::new();
handle.manage(notification_service);

// 新しいコマンドの登録
commands::notification_commands::schedule_notification,
```

### デバッグ支援
```rust
// ログレベルの調整
.level(log::LevelFilter::Debug)  // より詳細なログ

// 追加のプラグイン
app.handle().plugin(tauri_plugin_devtools::init())?;
```