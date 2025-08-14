# task_commands.rs - Tauri コマンドハンドラー解説

## 📋 概要

`task_commands.rs`はTaskNagアプリケーションのフロントエンド-バックエンド間のAPIインターフェースを提供するTauriコマンドハンドラー群です。TaskServiceの全機能へのアクセス、エラーハンドリングの統一、システム通知の制御、そしてリアルタイムイベント送信を担当します。

---

## 🏗️ ファイル構造

### インポート構成
```rust
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::services::TaskService;
use tauri::{AppHandle, State, Emitter};
```

**依存関係:**
- **models**: データ構造とリクエスト型
- **TaskService**: ビジネスロジック層
- **Tauri API**: アプリハンドル、状態管理、イベント送信

### 設計パターン
```rust
#[tauri::command]
pub async fn function_name(
    parameters: Type,
    service: State<'_, TaskService>,
) -> Result<ReturnType, String> {
    service
        .service_method(parameters)
        .await
        .map_err(|e| e.to_string())
}
```

**統一パターンの特徴:**
- **`#[tauri::command]`**: フロントエンドからの呼び出し可能
- **`State<'_, TaskService>`**: 依存注入による Service アクセス
- **`Result<T, String>`**: 統一されたエラーハンドリング
- **`.map_err(|e| e.to_string())`**: AppError → String 変換

---

## 🔨 基本CRUD操作コマンド

### 1. タスク作成
```rust
#[tauri::command]
pub async fn create_task(
    request: CreateTaskRequest,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    service
        .create_task(request)
        .await
        .map_err(|e| e.to_string())
}
```

**フロントエンド呼び出し例:**
```typescript
import { invoke } from '@tauri-apps/api/core';

const newTask = await invoke<Task>('create_task', {
  request: {
    title: "新しいタスク",
    description: "説明",
    status: "todo",
    priority: "medium"
  }
});
```

### 2. タスク取得
```rust
#[tauri::command]
pub async fn get_tasks(service: State<'_, TaskService>) -> Result<Vec<Task>, String> {
    service.get_tasks().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_by_id(id: String, service: State<'_, TaskService>) -> Result<Task, String> {
    service
        .get_task_by_id(&id)
        .await
        .map_err(|e| e.to_string())
}
```

**取得パターン:**
- **全タスク**: `get_tasks()` - ソート済み全件取得
- **個別タスク**: `get_task_by_id(id)` - ID指定での単件取得

### 3. タスク更新
```rust
#[tauri::command]
pub async fn update_task(
    id: String,
    request: UpdateTaskRequest,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    service
        .update_task(&id, request)
        .await
        .map_err(|e| e.to_string())
}
```

### 4. タスク削除
```rust
#[tauri::command]
pub async fn delete_task(id: String, service: State<'_, TaskService>) -> Result<(), String> {
    service
        .delete_task(&id)
        .await
        .map_err(|e| e.to_string())
}
```

**削除の特徴:**
- **戻り値**: `()` - 削除成功の確認のみ
- **エラー**: 存在しないIDの場合はエラーレスポンス

---

## 🎯 特殊操作コマンド

### ステータス関連操作
```rust
#[tauri::command]
pub async fn get_tasks_by_status(
    status: String,
    service: State<'_, TaskService>,
) -> Result<Vec<Task>, String> {
    service
        .get_tasks_by_status(&status)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn move_task(
    id: String,
    new_status: String,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    service
        .move_task(&id, &new_status)
        .await
        .map_err(|e| e.to_string())
}
```

**使用例 - ドラッグ&ドロップ:**
```typescript
// カンバンボードでのタスク移動
await invoke('move_task', {
  id: taskId,
  newStatus: 'in_progress'
});
```

### 統計情報取得
```rust
#[tauri::command]
pub async fn get_incomplete_task_count(service: State<'_, TaskService>) -> Result<usize, String> {
    service
        .get_incomplete_task_count()
        .await
        .map_err(|e| e.to_string())
}
```

**活用場面:**
- システムトレイのタスク数表示
- ダッシュボードの統計情報
- バッジ表示の数値

---

## 🖥️ システムトレイ統合

### トレイタイトル更新
```rust
#[tauri::command]
pub async fn update_tray_title(
    _app: AppHandle,
    service: State<'_, TaskService>,
) -> Result<(), String> {
    let count = service
        .get_incomplete_task_count()
        .await
        .map_err(|e| e.to_string())?;
    
    let title = if count > 0 {
        format!("TaskNag ({} 件)", count)
    } else {
        "TaskNag".to_string()
    };
    
    // Tauri v2では直接トレイアイコンのタイトルを更新する方法が異なります
    // 現在のところ、動的更新はサポートされていない可能性があります
    println!("Would update tray title to: {}", title);
    
    Ok(())
}
```

**タイトル生成ロジック:**
- **タスクあり**: "TaskNag (7 件)" 形式
- **タスクなし**: "TaskNag" のみ
- **動的更新**: 未完了タスク数の変更に応じて自動更新

**Tauri v2 制限事項:**
現在のTauri v2では、システムトレイタイトルの動的更新に制限があります。将来のバージョンでの改善が期待されます。

---

## 🔔 通知システム

### 通知チェック・送信
```rust
#[tauri::command]
pub async fn check_notifications(
    app: AppHandle,
    service: State<'_, TaskService>,
) -> Result<Vec<serde_json::Value>, String> {
    let notifications = service.check_notifications().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    
    for notification in notifications {
        // 通知レベルに応じて通知を送信
        let (title, body) = match notification.notification_type.as_str() {
            "due_date_based" => {
                let days_text = match notification.days_until_due.unwrap_or(0) {
                    0 => "【期限当日】",
                    1 => "【期限明日】",
                    d if d <= 3 => "【期限間近】",
                    _ => "【期限通知】",
                };
                (
                    format!("📅 {}", days_text),
                    notification.title.clone()
                )
            },
            "recurring" => {
                (
                    "🔔 定期リマインド".to_string(),
                    notification.title.clone()
                )
            },
            _ => (
                "📋 タスク通知".to_string(),
                notification.title.clone()
            )
        };
        
        // ... (通知レベル処理)
    }
    
    Ok(result)
}
```

### 通知レベル別処理
```rust
// 通知レベルに応じた処理（Level 1-3）
match notification.level {
    1 => {
        // Level 1: システム通知のみ
        let _ = app.emit("notification", serde_json::json!({
            "title": title,
            "body": body
        }));
    },
    2 => {
        // Level 2: システム通知 + 音声通知
        let _ = app.emit("notification", serde_json::json!({
            "title": title,
            "body": body
        }));
        let _ = app.emit("sound_notification", serde_json::json!({}));
    },
    3 => {
        // Level 3: アプリ最大化 + 通知
        let _ = app.emit("notification", serde_json::json!({
            "title": title,
            "body": body
        }));
        let _ = app.emit("sound_notification", serde_json::json!({}));
        let _ = app.emit("maximize_app", serde_json::json!({}));
    },
    _ => {} // Invalid level
}
```

**通知段階設計:**
- **Level 1**: 静かな通知（システム通知のみ）
- **Level 2**: 音声付き通知（通知 + サウンド）
- **Level 3**: 強制的な通知（アプリ最大化 + 全効果）

**期日通知の表現:**
- **0日**: 【期限当日】
- **1日**: 【期限明日】  
- **2-3日**: 【期限間近】
- **4日以上**: 【期限通知】

### 通知設定更新
```rust
#[tauri::command]
pub async fn update_task_notification_settings(
    id: String,
    notification_settings: crate::models::TaskNotificationSettings,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    let update_request = crate::models::UpdateTaskRequest {
        title: None,
        description: None,
        status: None,
        priority: None,
        parent_id: None,
        due_date: None,
        notification_settings: Some(notification_settings),
    };
    
    service
        .update_task(&id, update_request)
        .await
        .map_err(|e| e.to_string())
}
```

**部分更新パターン:**
- 通知設定のみの更新に特化
- 他のフィールドは`None`で変更せず
- UpdateTaskRequestの活用による統一的処理

---

## 🌳 階層タスク管理コマンド

### 子タスク関連
```rust
#[tauri::command]
pub async fn get_children(
    parent_id: String,
    service: State<'_, TaskService>,
) -> Result<Vec<Task>, String> {
    service
        .get_children(&parent_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_with_children(
    id: String,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    service
        .get_task_with_children(&id)
        .await
        .map_err(|e| e.to_string())
}
```

### 進捗管理
```rust
#[tauri::command]
pub async fn update_progress(
    id: String,
    progress: i32,
    service: State<'_, TaskService>,
) -> Result<Task, String> {
    service
        .update_progress(&id, progress)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn calculate_and_update_progress(
    parent_id: String,
    service: State<'_, TaskService>,
) -> Result<i32, String> {
    service
        .calculate_and_update_progress(&parent_id)
        .await
        .map_err(|e| e.to_string())
}
```

**進捗管理の特徴:**
- **個別更新**: 手動での進捗率設定
- **自動計算**: 子タスクからの進捗率計算
- **連鎖更新**: 親タスクへの自動反映

### ルートタスク取得
```rust
#[tauri::command]
pub async fn get_root_tasks(service: State<'_, TaskService>) -> Result<Vec<Task>, String> {
    service.get_root_tasks().await.map_err(|e| e.to_string())
}
```

**活用例:**
- メインビューでの表示タスク
- 階層構造のトップレベル
- パフォーマンス最適化（必要な分のみ取得）

---

## 🎛️ イベント駆動アーキテクチャ

### Tauri イベント送信
```rust
// システム通知
let _ = app.emit("notification", serde_json::json!({
    "title": title,
    "body": body
}));

// 音声通知
let _ = app.emit("sound_notification", serde_json::json!({}));

// アプリ最大化
let _ = app.emit("maximize_app", serde_json::json!({}));
```

**フロントエンド受信例:**
```typescript
import { listen } from '@tauri-apps/api/event';

// 通知イベントの受信
await listen('notification', (event) => {
  console.log('Notification received:', event.payload);
  // デスクトップ通知の表示
  new Notification(event.payload.title, {
    body: event.payload.body
  });
});

// 音声通知の受信
await listen('sound_notification', () => {
  // 音声ファイルの再生
  playNotificationSound();
});

// アプリ最大化の受信
await listen('maximize_app', () => {
  // ウィンドウの最大化・フォーカス
  window.focus();
});
```

### エラー処理の寛容性
```rust
let _ = app.emit("notification", /* ... */);
```

**`let _` パターンの理由:**
- 通知送信失敗でもアプリ機能は継続
- ログ記録は別途実装
- UI操作の継続性を優先

---

## 🧪 エラーハンドリング戦略

### 統一エラー変換
```rust
.map_err(|e| e.to_string())
```

**変換の流れ:**
```
AppError → String → JSON → Frontend Error
```

**エラータイプ別の処理:**
```rust
// カスタムエラーメッセージの例
match service.operation().await {
    Ok(result) => Ok(result),
    Err(AppError::NotFound(msg)) => Err(format!("見つかりません: {}", msg)),
    Err(AppError::InvalidInput(msg)) => Err(format!("入力エラー: {}", msg)),
    Err(AppError::Database(msg)) => Err(format!("データベースエラー: {}", msg)),
}
```

### フロントエンド側のエラー処理
```typescript
try {
  const result = await invoke('create_task', { request });
  // 成功処理
} catch (error) {
  console.error('Task creation failed:', error);
  // エラー表示・ログ記録
}
```

---

## 🚀 パフォーマンス考慮事項

### 非同期処理の最適化
```rust
// 全て非同期関数として実装
pub async fn command_name(/* ... */) -> Result<T, String>
```

**非同期の利点:**
- UI ブロッキングの回避
- データベース操作の効率化
- 複数操作の並行実行可能性

### State インジェクションの効率性
```rust
service: State<'_, TaskService>
```

**パフォーマンス特性:**
- **シングルトン**: アプリ全体で単一インスタンス
- **ライフタイム**: アプリケーション生存期間中有効
- **ゼロコスト**: コンパイル時最適化

---

## 🧪 テスト戦略

### 単体テスト例
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::mock_database::get_mock_service;

    #[tokio::test]
    async fn test_create_task_command() {
        let service = get_mock_service().await;
        let request = CreateTaskRequest {
            title: "Test Task".to_string(),
            // ... 他のフィールド
        };

        let result = create_task(request, State::from(&service)).await;
        assert!(result.is_ok());
    }
}
```

### 統合テスト項目
- [ ] 全コマンドの正常動作
- [ ] エラーハンドリングの確認
- [ ] 通知システムの動作
- [ ] システムトレイとの連携

---

## 📝 開発者向けノート

### 新しいコマンドの追加パターン
```rust
#[tauri::command]
pub async fn new_command(
    parameter: ParameterType,
    service: State<'_, TaskService>,
) -> Result<ReturnType, String> {
    service
        .new_service_method(parameter)
        .await
        .map_err(|e| e.to_string())
}
```

### lib.rs での登録
```rust
.invoke_handler(tauri::generate_handler![
    // ... 既存のコマンド
    commands::task_commands::new_command,
])
```

### TypeScript型定義の生成
Tauriは自動的にTypeScript型定義を生成しないため、手動での型定義維持が必要です：

```typescript
// types/tauri.d.ts
declare module '@tauri-apps/api/core' {
  function invoke<T>(cmd: 'create_task', args: { request: CreateTaskRequest }): Promise<T>;
  function invoke<T>(cmd: 'get_tasks'): Promise<T>;
  // ... 他のコマンド
}
```

### デバッグ支援
```rust
// デバッグ用ログ追加例
println!("Command executed: {} with params: {:?}", "create_task", request);
```