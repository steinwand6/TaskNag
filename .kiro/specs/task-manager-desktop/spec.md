# TaskNag - Desktop Task Manager Specification

## Overview
**TaskNag** - 口うるさくて世話焼きなWindowsデスクトップタスク管理アプリケーション。Rust、Tauri、SQLiteを使用した高性能でセキュアなネイティブアプリケーション。ユーザーが忘れがちなタスクを積極的にリマインドし、生産性向上をサポートします。

## Project Details

### Platform
- **Target OS**: Windows (10/11)
- **Application Type**: Desktop Application (System Tray常駐型)
- **Architecture**: Native Application with Web Technologies UI

### Technology Stack
- **Backend**: Rust
- **Frontend Framework**: Tauri
- **Database**: SQLite (ローカルストレージ)
- **UI Technologies**: HTML/CSS/JavaScript (TypeScript推奨)
- **Build System**: Cargo + Tauri CLI

### Core Features

#### Task Management
- **Task States**:
  - `inbox`: 新規タスク（未分類）
  - `todo`: 実行予定タスク
  - `in_progress`: 実行中タスク
  - `done`: 完了タスク

#### Notification System
- **個別通知設定**: タスクごとに通知タイミングをカスタマイズ可能
- **期日ベース通知**: 「期日N日前」から期日まで連続通知
- **定期通知**: 曜日・時刻指定での定期リマインド
- **通知タイミング**: 分単位での精密な時刻設定

#### System Integration
- **System Tray**: バックグラウンド常駐
- **Quick Access**: ホットキーによる高速アクセス
- **Auto Start**: Windows起動時の自動起動オプション

### Data Model

#### Task Entity
```rust
struct Task {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: TaskStatus,
    created_at: DateTime,
    updated_at: DateTime,
    due_date: Option<DateTime>,
    notification_settings: NotificationSettings, // 個別通知設定
    tags: Vec<String>,
    completed_at: Option<DateTime>,
    parent_task_id: Option<Uuid>,  // 親タスクのID
    subtask_ids: Vec<Uuid>,        // 子タスクのIDリスト
    progress: f32,                  // 進捗率 (0.0 - 1.0)
    recurrence: Option<RecurrenceRule>, // 定期タスク設定
}

enum TaskStatus {
    Inbox,
    Todo,
    InProgress,
    Done,
}

enum NotificationSettings {
    None,                              // 通知なし
    DueDateBased {
        days_before: u8,               // 期日何日前から通知開始
        notification_time: NaiveTime,  // 通知時刻
    },
    Recurring {
        days_of_week: Vec<DayOfWeek>,  // 通知する曜日
        notification_time: NaiveTime,  // 通知時刻
    },
}

struct RecurrenceRule {
    pattern: RecurrencePattern,
    interval: u32,              // 間隔（パターンに応じて日/週/月/年）
    days_of_week: Option<Vec<DayOfWeek>>, // 週次の場合の曜日
    day_of_month: Option<u8>,  // 月次の場合の日付
    time_of_day: NaiveTime,    // 実行時刻
    end_date: Option<DateTime>, // 終了日
    max_occurrences: Option<u32>, // 最大実行回数
    next_occurrence: DateTime,  // 次回実行日時
    is_active: bool,           // 有効/無効
}

enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Custom,
}

enum DayOfWeek {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday,
}
```

### Database Schema
```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('inbox', 'todo', 'in_progress', 'done')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    due_date DATETIME,
    notification_type TEXT DEFAULT 'none' CHECK (notification_type IN ('none', 'due_date_based', 'recurring')),
    notification_days_before INTEGER DEFAULT NULL, -- 期日何日前から通知
    notification_time TIME DEFAULT NULL,           -- 通知時刻
    notification_days_of_week TEXT DEFAULT NULL,   -- 定期通知の曜日（JSON配列）
    completed_at DATETIME,
    parent_task_id TEXT,
    progress INTEGER DEFAULT 0 CHECK (progress >= 0 AND progress <= 100),
    FOREIGN KEY (parent_task_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE task_tags (
    task_id TEXT,
    tag_id INTEGER,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, tag_id)
);

CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_parent ON tasks(parent_task_id);

-- 定期タスク設定テーブル
CREATE TABLE recurring_tasks (
    id TEXT PRIMARY KEY,
    task_template_id TEXT NOT NULL,
    pattern TEXT NOT NULL CHECK (pattern IN ('daily', 'weekly', 'monthly', 'yearly', 'custom')),
    interval_value INTEGER NOT NULL DEFAULT 1,
    days_of_week TEXT, -- JSON配列形式 ["Monday", "Wednesday", "Friday"]
    day_of_month INTEGER CHECK (day_of_month >= 1 AND day_of_month <= 31),
    time_of_day TIME NOT NULL,
    end_date DATETIME,
    max_occurrences INTEGER,
    current_occurrence_count INTEGER DEFAULT 0,
    next_occurrence DATETIME NOT NULL,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_template_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE INDEX idx_recurring_next ON recurring_tasks(next_occurrence);
CREATE INDEX idx_recurring_active ON recurring_tasks(is_active);
```

### Future Enhancements

#### LLM Integration
- **Task Assistance**: タスク作成・編集の支援
- **Smart Suggestions**: コンテキストに基づくタスク提案
- **Natural Language Processing**: 自然言語でのタスク入力
- **Interactive Features**:
  - タスクの自動分類
  - 優先度の自動判定
  - 期限の推定
  - 関連タスクのグループ化

#### Planned Features
- Cloud Sync (オプション)
- Team Collaboration
- Analytics Dashboard
- Plugin System
- Mobile Companion App

## Development Phases

### Phase 1: Core Application (MVP)
- Basic Tauri application setup
- SQLite database integration
- Task CRUD operations
- System tray implementation
- Basic UI with task list view

### Phase 2: Notification System
- Windows notification API integration
- Sound notification support
- App maximize functionality
- Notification scheduling

### Phase 3: Enhanced UX
- Hotkey support
- Quick add functionality
- Search and filtering
- Tag management
- Keyboard shortcuts

### Phase 4: LLM Integration
- LLM API integration
- Natural language input
- Smart suggestions
- Task automation

## Success Criteria
- アプリケーション起動時間: < 1秒
- メモリ使用量: < 50MB (アイドル時)
- データベース応答時間: < 10ms
- 通知遅延: < 100ms
- システムトレイからの復帰: < 500ms

## Security Requirements
- ローカルデータベースの暗号化オプション
- セキュアな設定保存
- LLM API キーの安全な管理
- 自動バックアップ機能

## Testing Strategy
- Unit Tests (Rust backend)
- Integration Tests (Tauri API)
- E2E Tests (UI interactions)
- Performance Tests
- Security Audits

## Approval Status
- [ ] Requirements Phase
- [ ] Design Phase
- [ ] Implementation Phase