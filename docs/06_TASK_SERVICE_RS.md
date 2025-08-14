# task_service.rs - タスクサービス層解説

## 📋 概要

`task_service.rs`はTaskNagアプリケーションのビジネスロジック層の中核を担うサービスクラスです。データベース操作の抽象化、複雑なタスク管理機能（階層構造、進捗計算、通知システム）の実装、そしてCRUD操作の統合的な管理を提供します。

---

## 🏗️ ファイル構造

### インポート構成
```rust
use crate::database::Database;
use crate::error::AppError;
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use chrono::Utc;
use uuid::Uuid;
```

**依存関係:**
- **Database**: SQLite接続プールへのアクセス
- **AppError**: 統一されたエラーハンドリング
- **models**: データ構造とリクエスト型
- **chrono**: 日時操作（RFC3339フォーマット）
- **uuid**: ユニークID生成

### サービス構造体
```rust
pub struct TaskService {
    db: Database,
}

impl TaskService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}
```

**設計原則:**
- **Dependency Injection**: データベースを外部から注入
- **Stateless Service**: 内部状態を持たない設計
- **Single Responsibility**: タスク管理のみに特化

---

## 🔨 CRUD操作

### 1. タスク作成 (Create)
```rust
pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task, AppError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    
    // 通知設定のデフォルト値またはリクエストの値を使用
    let notification_settings = request.notification_settings.unwrap_or_default();
    
    let task = Task {
        id: id.clone(),
        title: request.title,
        description: request.description,
        status: request.status.to_string(),
        priority: request.priority.to_string(),
        parent_id: request.parent_id,
        due_date: request.due_date.map(|d| d.to_rfc3339()),
        completed_at: None,
        created_at: now.clone(),
        updated_at: now,
        progress: Some(0),
        // 新通知設定フィールド
        notification_type: Some(notification_settings.notification_type),
        notification_days_before: notification_settings.days_before,
        notification_time: notification_settings.notification_time,
        notification_days_of_week: notification_settings.days_of_week.map(|days| 
            serde_json::to_string(&days).unwrap_or_default()
        ),
        notification_level: Some(notification_settings.level),
    };
    
    // SQLite INSERT with prepared statements
    sqlx::query(/* ... */)
        .bind(&task.id)
        // ... 16 parameters
        .execute(&self.db.pool)
        .await?;
    
    Ok(task)
}
```

**特徴的な実装:**
- **UUID生成**: `Uuid::new_v4()` による衝突回避
- **RFC3339日時**: 標準化された日時フォーマット
- **通知設定の統合**: デフォルト値による柔軟性
- **JSON配列シリアライゼーション**: 曜日データの永続化

### 2. タスク取得 (Read)
```rust
pub async fn get_tasks(&self) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, title, description, status, priority, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
        FROM tasks
        ORDER BY 
            CASE status 
                WHEN 'inbox' THEN 1
                WHEN 'todo' THEN 2
                WHEN 'in_progress' THEN 3
                WHEN 'done' THEN 4
            END,
            CASE priority
                WHEN 'required' THEN 1
                WHEN 'high' THEN 2
                WHEN 'medium' THEN 3
                WHEN 'low' THEN 4
            END,
            created_at DESC
        "#,
    )
    .fetch_all(&self.db.pool)
    .await?;
    
    Ok(tasks)
}
```

**ソート戦略:**
1. **ステータス順**: inbox → todo → in_progress → done
2. **優先度順**: required → high → medium → low  
3. **作成日順**: 新しいものから（DESC）

### 3. タスク更新 (Update)
```rust
pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Task, AppError> {
    // Get existing task first
    let mut task = self.get_task_by_id(id).await?;
    
    // Update fields if provided
    if let Some(title) = request.title {
        task.title = title;
    }
    // ... 他のフィールド更新
    
    // Status special handling
    if let Some(status) = request.status {
        task.status = status.to_string();
        // Set completed_at if status is Done
        if task.status == "done" {
            task.completed_at = Some(Utc::now().to_rfc3339());
        } else {
            task.completed_at = None;
        }
    }
    
    // 通知設定の更新
    if let Some(notification_settings) = request.notification_settings {
        task.notification_type = Some(notification_settings.notification_type);
        // ... 他の通知フィールド
    }
    
    task.updated_at = Utc::now().to_rfc3339();
    
    sqlx::query(/* UPDATE SQL */)
        .bind(/* ... 15 parameters */)
        .execute(&self.db.pool)
        .await?;
    
    Ok(task)
}
```

**更新戦略:**
- **Partial Update**: Option型による部分更新対応
- **自動完了日**: ステータス変更時の completed_at 設定
- **タイムスタンプ更新**: updated_at の自動更新

### 4. タスク削除 (Delete)
```rust
pub async fn delete_task(&self, id: &str) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(id)
        .execute(&self.db.pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Task with id {} not found", id)));
    }
    
    Ok(())
}
```

**削除の検証:**
- **存在確認**: `rows_affected()` による削除対象確認
- **適切なエラー**: 存在しないIDの場合NotFoundエラー

---

## 🌳 階層タスク管理

### 子タスク取得
```rust
pub async fn get_children(&self, parent_id: &str) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT /* 全フィールド */
        FROM tasks
        WHERE parent_id = ?1
        ORDER BY created_at ASC
        "#,
    )
    .bind(parent_id)
    .fetch_all(&self.db.pool)
    .await?;
    
    Ok(tasks)
}
```

### 親子関係付きタスク取得
```rust
pub async fn get_task_with_children(&self, id: &str) -> Result<Task, AppError> {
    let mut task = self.get_task_by_id(id).await?;
    let children = self.get_children(id).await?;
    
    // 子タスクがある場合は進捗率を計算
    if !children.is_empty() {
        task.progress = Some(self.calculate_progress(&children));
    }
    
    Ok(task)
}
```

**階層機能の特徴:**
- **再帰的進捗計算**: 子タスクから親の進捗を自動算出
- **作成日ソート**: 子タスクの自然な順序付け
- **動的プログレス**: リアルタイムでの進捗率反映

---

## 📊 進捗管理システム

### 進捗率計算ロジック
```rust
fn calculate_progress(&self, children: &[Task]) -> i32 {
    if children.is_empty() {
        return 0;
    }
    
    let total_progress: i32 = children.iter()
        .map(|child| {
            if child.status == "done" {
                100  // 完了タスクは100%
            } else {
                child.progress.unwrap_or(0)  // 個別進捗率
            }
        })
        .sum();
    
    total_progress / children.len() as i32  // 平均値
}
```

**計算方式:**
- **完了タスク**: 100% として計算
- **進行中タスク**: 個別の progress 値を使用
- **未設定タスク**: 0% として扱う
- **平均計算**: 全子タスクの平均進捗率

### 進捗率更新と連鎖処理
```rust
pub async fn update_progress(&self, id: &str, progress: i32) -> Result<Task, AppError> {
    if progress < 0 || progress > 100 {
        return Err(AppError::InvalidInput("Progress must be between 0 and 100".to_string()));
    }
    
    let mut task = self.get_task_by_id(id).await?;
    task.progress = Some(progress);
    task.updated_at = Utc::now().to_rfc3339();
    
    // タスクが100%完了の場合、ステータスをdoneに変更
    if progress == 100 && task.status != "done" {
        task.status = "done".to_string();
        task.completed_at = Some(Utc::now().to_rfc3339());
    }
    
    // データベース更新
    sqlx::query(/* UPDATE SQL */).execute().await?;
    
    // 親タスクがある場合は親の進捗率も更新
    if let Some(parent_id) = &task.parent_id {
        self.calculate_and_update_progress(parent_id).await?;
    }
    
    Ok(task)
}
```

**連鎖更新の仕組み:**
1. **個別タスク更新**: 指定タスクの進捗率変更
2. **自動ステータス変更**: 100%完了時のdone変更
3. **親タスク連鎖**: 再帰的な親進捗率再計算
4. **データ整合性**: トランザクション的な更新保証

---

## 🔔 通知システム

### 通知対象タスクの検索
```rust
pub async fn check_notifications(&self) -> Result<Vec<crate::models::TaskNotification>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT /* 全フィールド */
        FROM tasks
        WHERE status != 'done' 
          AND notification_type IS NOT NULL 
          AND notification_type != 'none'
        "#,
    )
    .fetch_all(&self.db.pool)
    .await?;
    
    let mut notifications = Vec::new();
    let now = Utc::now();
    
    for task in tasks {
        let notification_type = task.notification_type.as_deref().unwrap_or("none");
        
        match notification_type {
            "due_date_based" => {
                // 期日ベース通知処理
            }
            "recurring" => {
                // 定期通知処理
            }
            _ => {} // 'none' or unknown type
        }
    }
    
    Ok(notifications)
}
```

### 期日ベース通知
```rust
"due_date_based" => {
    if let Some(due_date_str) = &task.due_date {
        if let Ok(due_date) = DateTime::parse_from_rfc3339(due_date_str) {
            let due_date_utc = due_date.with_timezone(&Utc);
            let days_until_due = (due_date_utc - now).num_days();
            let days_before = task.notification_days_before.unwrap_or(1);
            
            // 期日ベース通知の判定
            if days_until_due <= days_before as i64 && days_until_due >= 0 {
                if let Some(time_str) = &task.notification_time {
                    if should_notify_at_time(&now, time_str) {
                        notifications.push(crate::models::TaskNotification {
                            task_id: task.id,
                            title: task.title,
                            level: task.notification_level.unwrap_or(1),
                            days_until_due: Some(days_until_due),
                            notification_type: "due_date_based".to_string(),
                        });
                    }
                }
            }
        }
    }
}
```

**期日通知の特徴:**
- **日数カウントダウン**: 期日までの残り日数計算
- **時刻指定**: 特定時刻での通知発火
- **猶予期間**: 期日後は通知停止（負の日数除外）

### 定期通知
```rust
"recurring" => {
    if let (Some(days_str), Some(time_str)) = (&task.notification_days_of_week, &task.notification_time) {
        if let Ok(days_of_week) = serde_json::from_str::<Vec<i32>>(days_str) {
            let current_weekday = match now.weekday() {
                Weekday::Sun => 0,
                Weekday::Mon => 1,
                // ... 曜日マッピング
                Weekday::Sat => 6,
            };
            
            if days_of_week.contains(&current_weekday) && should_notify_at_time(&now, time_str) {
                notifications.push(/* TaskNotification */);
            }
        }
    }
}
```

**定期通知の特徴:**
- **曜日指定**: 複数曜日での反復通知
- **JSON配列**: 曜日データのシリアライゼーション
- **時刻同期**: 指定時刻での正確な発火

### 時刻判定関数
```rust
fn should_notify_at_time(now: &chrono::DateTime<chrono::Utc>, time_str: &str) -> bool {
    if let Ok(target_time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        let current_time = now.time();
        let target_seconds = target_time.num_seconds_from_midnight();
        let current_seconds = current_time.num_seconds_from_midnight();
        
        // ±30秒の範囲で通知
        (current_seconds as i32 - target_seconds as i32).abs() <= 30
    } else {
        false
    }
}
```

**時刻精度の設計:**
- **±30秒の許容範囲**: システム遅延・処理時間を考慮
- **秒単位計算**: 24時間形式での精密比較
- **エラーハンドリング**: 不正時刻フォーマットの安全な処理

---

## 🎯 特殊機能

### ステータス移動
```rust
pub async fn move_task(&self, id: &str, new_status: &str) -> Result<Task, AppError> {
    use std::str::FromStr;
    use crate::models::TaskStatus;
    
    let status = TaskStatus::from_str(new_status)
        .map_err(|e| AppError::InvalidInput(e))?;
    
    self.update_task(id, UpdateTaskRequest {
        title: None,
        description: None,
        status: Some(status),
        priority: None,
        parent_id: None,
        due_date: None,
        notification_settings: None,
    }).await
}
```

### 未完了タスク数取得
```rust
pub async fn get_incomplete_task_count(&self) -> Result<usize, AppError> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM tasks
        WHERE status != 'done'
        "#,
    )
    .fetch_one(&self.db.pool)
    .await?;
    
    Ok(count.0 as usize)
}
```

### ルートタスク取得
```rust
pub async fn get_root_tasks(&self) -> Result<Vec<Task>, AppError> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT /* 全フィールド */
        FROM tasks
        WHERE parent_id IS NULL
        ORDER BY /* 複合ソート */
        "#,
    )
    .fetch_all(&self.db.pool)
    .await?;
    
    Ok(tasks)
}
```

---

## 🎛️ エラーハンドリング

### 統一されたエラー処理
```rust
// 存在しないタスクの処理
task.ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))

// 入力値検証
if progress < 0 || progress > 100 {
    return Err(AppError::InvalidInput("Progress must be between 0 and 100".to_string()));
}

// ステータス変換エラー
let status = TaskStatus::from_str(new_status)
    .map_err(|e| AppError::InvalidInput(e))?;
```

**エラーカテゴリ:**
- **NotFound**: 存在しないリソースのアクセス
- **InvalidInput**: 不正な入力値・フォーマット
- **Database**: SQLクエリ実行エラー（自動変換）

---

## 🧪 テスト観点

### 単体テスト項目
- [ ] CRUD操作の基本動作
- [ ] 進捗率計算の正確性
- [ ] 通知判定ロジックの確認
- [ ] エラーハンドリングの網羅性

### 統合テスト項目
- [ ] 階層タスクの連鎖更新
- [ ] 通知システムの時刻精度
- [ ] データベースとの整合性
- [ ] 大量データでのパフォーマンス

### エッジケース
- [ ] 空の子タスクリストでの進捗計算
- [ ] 不正な日時フォーマットの処理
- [ ] 曜日データのJSON解析エラー
- [ ] 深い階層構造での再帰処理

---

## 🚀 パフォーマンス特性

### データベースクエリ最適化
```sql
-- インデックス活用のヒント
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_parent_id ON tasks(parent_id);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_notification ON tasks(notification_type, status);
```

### メモリ効率
- **Stream processing**: 大量タスクの場合はfetch_all回避
- **Clone回避**: 参照での処理最大化
- **JSON最小化**: 曜日配列の効率的シリアライゼーション

---

## 📝 開発者向けノート

### 拡張ポイント
```rust
// 新しい通知タイプの追加
match notification_type {
    "due_date_based" => { /* ... */ }
    "recurring" => { /* ... */ }
    "location_based" => { /* 位置ベース通知 */ }
    "custom_algorithm" => { /* カスタムアルゴリズム */ }
    _ => {}
}

// 新しい進捗計算方式
impl TaskService {
    fn calculate_weighted_progress(&self, children: &[Task]) -> i32 {
        // 重み付き進捗計算
    }
}
```

### 非同期処理のパターン
- **`await?`**: エラー早期リターン
- **トランザクション**: 複数操作の原子性保証
- **バッチ処理**: 通知チェックの効率化

### データ整合性
- **Foreign Key制約**: 親子関係の整合性
- **Check制約**: progress値の範囲制限
- **NOT NULL制約**: 必須フィールドの保証