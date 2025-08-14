# Notification System Redesign - TaskNag

## Overview
タスクごとの個別通知設定システムの実装。優先度システムを廃止し、より柔軟で詳細な通知制御を提供する。

## Feature Scope

### 変更される機能
- **優先度システム**: 完全廃止（required, high, medium, low）
- **固定通知ロジック**: 期日ベースの固定ルールを廃止
- **通知タイミング**: 5分間隔→1分間隔に変更

### 新規機能
- **個別通知設定**: タスクごとに通知タイミング・レベルを設定
- **期日ベース通知**: N日前から期日まで連続通知
- **定期通知**: 曜日・時刻指定での定期リマインド

## Data Model Changes

### Database Schema Changes
```sql
-- 削除するカラム
ALTER TABLE tasks DROP COLUMN priority;

-- 追加するカラム
ALTER TABLE tasks ADD COLUMN notification_type TEXT DEFAULT 'none' 
  CHECK (notification_type IN ('none', 'due_date_based', 'recurring'));
ALTER TABLE tasks ADD COLUMN notification_days_before INTEGER DEFAULT NULL;
ALTER TABLE tasks ADD COLUMN notification_time TIME DEFAULT NULL;
ALTER TABLE tasks ADD COLUMN notification_days_of_week TEXT DEFAULT NULL; -- JSON配列
ALTER TABLE tasks ADD COLUMN notification_level INTEGER DEFAULT 1 
  CHECK (notification_level IN (1, 2, 3));
```

### Type Definitions
```typescript
interface TaskNotificationSettings {
  type: 'none' | 'due_date_based' | 'recurring';
  daysBefore?: number;        // 期日何日前から（1-30）
  notificationTime?: string;  // HH:MM形式
  daysOfWeek?: number[];      // 0=日曜, 1=月曜...6=土曜
  level: 1 | 2 | 3;          // 通知レベル
}

interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  dueDate?: Date;
  notificationSettings: TaskNotificationSettings;
  // priority フィールドを削除
  parentId?: string;
  progress: number;
  createdAt: Date;
  updatedAt: Date;
  completedAt?: Date;
}
```

## Implementation Plan

### Phase 1: Database Migration ✅ COMPLETED
1. ✅ 既存priorityカラムの削除
2. ✅ 新しい通知関連カラムの追加
3. ✅ 既存データの通知設定デフォルト値設定

### Phase 2: Backend Logic Update ✅ COMPLETED
1. ✅ 通知チェックロジックの完全書き換え
2. ✅ 新しいTaskNotificationSettings処理
3. ✅ 曜日・時刻ベースの通知判定

### Phase 3: Frontend UI Update 🔄 IN PROGRESS
1. ✅ 優先度関連UI要素の削除（型定義）
2. ⏳ 通知設定UIコンポーネントの作成
3. ⏳ タスク作成・編集フォームの更新

### Phase 4: Testing & Polish ✅ COMPLETED
1. ✅ 各通知パターンのテスト
2. ⏳ UI/UX調整
3. ✅ エラーハンドリング強化

## Technical Details

### Notification Check Logic
```rust
// 期日ベース通知の判定
fn should_notify_due_date_based(
    task: &Task, 
    now: DateTime<Utc>
) -> bool {
    let settings = &task.notification_settings;
    
    if let Some(due_date) = task.due_date {
        let days_until_due = (due_date - now).num_days();
        let notification_time = settings.notification_time.unwrap_or(time!(09:00));
        let current_time = now.time();
        
        // 設定した日数前〜当日かつ、指定時刻
        days_until_due <= settings.days_before && 
        days_until_due >= 0 &&
        current_time >= notification_time &&
        current_time < (notification_time + duration!(hours: 1))
    } else {
        false
    }
}

// 定期通知の判定
fn should_notify_recurring(
    task: &Task,
    now: DateTime<Utc>
) -> bool {
    let settings = &task.notification_settings;
    let current_weekday = now.weekday().num_days_from_sunday() as usize;
    let current_time = now.time();
    let notification_time = settings.notification_time.unwrap_or(time!(09:00));
    
    settings.days_of_week.contains(&current_weekday) &&
    current_time >= notification_time &&
    current_time < (notification_time + duration!(hours: 1))
}
```

### UI Components
```typescript
// NotificationSettingsComponent
interface NotificationSettingsProps {
  settings: TaskNotificationSettings;
  onChange: (settings: TaskNotificationSettings) => void;
  hasDueDate: boolean;
}

// DaysOfWeekSelector
interface DaysOfWeekSelectorProps {
  selected: number[];
  onChange: (days: number[]) => void;
}

// TimeSelector  
interface TimeSelectorProps {
  time: string; // HH:MM
  onChange: (time: string) => void;
}
```

## Success Criteria
- [x] 既存の優先度システムが完全に削除される
- [x] 期日ベース通知が正確に動作する
- [x] 定期通知が指定曜日・時刻に動作する
- [x] 通知レベル（1-3）が正しく機能する
- [ ] UI上で直感的に通知設定が可能
- [x] 期限なしタスクで定期通知が機能する
- [x] 既存タスクの移行が正常に完了する

## Migration Strategy
1. **データ保護**: 既存タスクデータのバックアップ
2. **段階的移行**: priorityフィールドを通知設定にマッピング
3. **デフォルト設定**: 既存タスクには通知なしを設定
4. **ユーザー通知**: 機能変更の説明とガイド

---
*Created: 2025-01-14*
*Updated: 2025-01-14*
*Status: ✅ Mostly Completed (UI pending)*