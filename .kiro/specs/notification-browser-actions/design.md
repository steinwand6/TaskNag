# System Design - Notification Browser Actions

## Architecture Overview

### Component Architecture
```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (React)                    │
├─────────────────┬─────────────────┬─────────────────────┤
│  Task Form UI   │  URL Config UI  │  Notification UI    │
│  - URL Input    │  - Validation   │  - Status Display   │
│  - Test Button  │  - Preview      │  - Error Messages   │
└─────────────────┴─────────────────┴─────────────────────┘
                           │
                    ┌──────┴──────┐
                    │ Tauri Core  │
                    │   Commands  │
                    └──────┬──────┘
┌─────────────────────────────────────────────────────────┐
│                   Backend (Rust)                       │
├─────────────────┬─────────────────┬─────────────────────┤
│ Notification    │   URL Service   │   Browser Service   │
│    Service      │  - Validation   │   - Shell Command   │
│  - Scheduler    │  - Storage      │   - Error Handling  │
│  - Trigger      │  - History      │   - Async Execution │
└─────────────────┴─────────────────┴─────────────────────┘
                           │
                    ┌──────┴──────┐
                    │  Database   │
                    │  (SQLite)   │
                    └─────────────┘
```

## Data Model Design

### Database Schema Changes
```sql
-- TaskテーブルにJSONカラム追加
ALTER TABLE tasks ADD COLUMN browser_actions TEXT DEFAULT NULL;

-- Example JSON structure:
-- {
--   "enabled": true,
--   "actions": [
--     {
--       "id": "uuid",
--       "label": "会議室予約",
--       "url": "https://calendar.google.com/calendar/u/0/r",
--       "enabled": true,
--       "order": 1
--     }
--   ]
-- }
```

### Type Definitions
```typescript
// Frontend Types
interface BrowserAction {
  id: string;                    // UUID v4
  label: string;                 // ユーザー定義名称
  url: string;                   // 完全なURL
  enabled: boolean;              // 個別有効/無効
  order: number;                 // 実行順序 (1-5)
  createdAt: Date;               // 作成日時
}

interface BrowserActionSettings {
  enabled: boolean;              // 全体有効/無効
  actions: BrowserAction[];      // アクション配列
}

interface TaskNotificationSettings {
  // 既存フィールド
  notificationType: 'none' | 'due_date_based' | 'recurring';
  daysBefore?: number;
  notificationTime?: string;
  daysOfWeek?: number[];
  level: 1 | 2 | 3;
  
  // 新規フィールド
  browserActions?: BrowserActionSettings;
}

// Validation Types
interface URLValidationResult {
  isValid: boolean;
  protocol: 'http' | 'https' | 'invalid';
  host: string;
  error?: string;
}

interface URLPreviewInfo {
  title?: string;
  favicon?: string;
  description?: string;
  status: 'success' | 'error' | 'loading';
}
```

```rust
// Backend Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserAction {
    pub id: String,
    pub label: String,
    pub url: String,
    pub enabled: bool,
    pub order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserActionSettings {
    pub enabled: bool,
    pub actions: Vec<BrowserAction>,
}

#[derive(Debug)]
pub enum BrowserActionError {
    InvalidUrl(String),
    CommandFailed(String),
    Timeout,
    SecurityViolation(String),
}
```

## Component Design

### 1. URL Configuration Component (`URLActionConfig.tsx`)
```typescript
interface URLActionConfigProps {
  settings: BrowserActionSettings;
  onChange: (settings: BrowserActionSettings) => void;
  maxActions?: number; // default: 5
}

// Features:
// - Add/Remove URL actions
// - Drag & drop reordering
// - Individual enable/disable toggles
// - URL validation with real-time feedback
// - Test button for immediate URL opening
// - URL preview with favicon and title
```

### 2. URL Input Component (`URLInput.tsx`)
```typescript
interface URLInputProps {
  url: string;
  label: string;
  onUrlChange: (url: string) => void;
  onLabelChange: (label: string) => void;
  onRemove: () => void;
  onTest: () => void;
  isValidating: boolean;
  validationResult?: URLValidationResult;
  previewInfo?: URLPreviewInfo;
}

// Features:
// - Real-time URL validation
// - Auto-protocol completion (http:// prefix)
// - Common URL suggestions
// - Visual validation status indicators
```

### 3. Browser Action Service (`browser_action_service.rs`)
```rust
pub struct BrowserActionService {
    shell: Arc<dyn ShellExecutor>,
    url_validator: URLValidator,
}

impl BrowserActionService {
    // Main execution method
    pub async fn execute_actions(
        &self, 
        actions: &[BrowserAction]
    ) -> Result<(), BrowserActionError> {
        for action in actions.iter().filter(|a| a.enabled) {
            self.open_url_async(&action.url).await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        Ok(())
    }
    
    // URL validation
    pub fn validate_url(&self, url: &str) -> URLValidationResult {
        // Protocol validation
        // Host validation  
        // Security checks
    }
    
    // Async browser opening
    async fn open_url_async(&self, url: &str) -> Result<(), BrowserActionError> {
        let timeout = Duration::from_secs(3);
        tokio::time::timeout(timeout, self.shell.open_url(url)).await
    }
}
```

## User Interface Design

### Task Creation/Edit Form Integration
```
┌─────────────────────────────────────────────────────────┐
│ タスク作成フォーム                                        │
├─────────────────────────────────────────────────────────┤
│ タイトル: [____________________]                        │
│ 説明:     [____________________]                        │
│ 期限:     [____________________]                        │
│                                                         │
│ ┌─── 📅 通知設定 ─────────────────────────────────┐    │
│ │ 通知タイプ: (○) なし (○) 期日ベース (○) 定期     │    │
│ │ 通知レベル: (○) 1 (○) 2 (○) 3                  │    │
│ └─────────────────────────────────────────────────┘    │
│                                                         │
│ ┌─── 🌐 ブラウザアクション ────────────────────────┐    │
│ │ □ 通知時にWebページを開く                          │    │
│ │                                                   │    │
│ │ ┌─ URL 1 ─────────────────────────────────────┐ │    │
│ │ │ 名前: [会議室確認___________] [×]              │ │    │
│ │ │ URL:  [https://calendar.google.com/______] 🔗│ │    │
│ │ │       ✅ 有効なURL                            │ │    │
│ │ └─────────────────────────────────────────────┘ │    │
│ │                                                   │    │
│ │ [+ URLを追加] [💡 候補] [📝 ヘルプ]              │    │
│ └─────────────────────────────────────────────────┘    │
│                                                         │
│ [キャンセル] [保存]                                      │
└─────────────────────────────────────────────────────────┘
```

### URL Input Detail View
```
┌─── URL設定 ─────────────────────────────────────────────┐
│ 名前: [プロジェクト資料_______________] [必須]            │
│                                                         │
│ URL:  [https://docs.google.com/document/d/abc...]       │
│       🔍 プレビュー: "Q3 Project Plan - Google Docs"   │
│       📄 docs.google.com • ✅ 有効                      │
│                                                         │
│ [🔗 テスト開く] [📋 履歴] [💾 保存]                    │
└─────────────────────────────────────────────────────────┘
```

## Notification Flow Integration

### Enhanced Notification Process
```
1. Notification Trigger (Timer Check)
   ↓
2. Should Notify? (existing logic)
   ↓ YES
3. Prepare Notification Data
   ↓
4. Show Desktop Notification
   ↓
5. Play Audio Notification  
   ↓
6. Execute Browser Actions (NEW)
   ├─ Validate URLs
   ├─ Open URLs sequentially  
   └─ Log execution results
   ↓
7. Log Notification Event
```

### Rust Implementation Flow
```rust
// In notification_service.rs
pub async fn send_notification(&self, task: &Task) -> Result<(), NotificationError> {
    // Existing notification logic
    self.show_desktop_notification(task)?;
    self.play_audio_notification(task)?;
    
    // NEW: Browser actions
    if let Some(browser_settings) = &task.notification_settings.browser_actions {
        if browser_settings.enabled && task.notification_settings.level >= 2 {
            if let Err(e) = self.browser_service.execute_actions(&browser_settings.actions).await {
                log::warn!("Browser action failed for task {}: {}", task.id, e);
                // Continue with normal notification flow
            }
        }
    }
    
    self.log_notification_event(task)?;
    Ok(())
}
```

## Security Design

### URL Validation Pipeline
```
Input URL → Protocol Check → Host Validation → Security Filter → Storage
    ↓              ↓               ↓                ↓            ↓
"example.com" → "https://" → "valid host" → "safe scheme" → Database
```

### Security Rules Implementation
```typescript
const URL_SECURITY_RULES = {
  allowedProtocols: ['http:', 'https:'],
  blockedProtocols: ['javascript:', 'data:', 'file:', 'ftp:'],
  maxLength: 2048,
  allowedHosts: /^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/,
  blockedPatterns: [
    /javascript:/i,
    /data:/i,
    /vbscript:/i,
    /<script/i,
  ]
};
```

## Error Handling Strategy

### Frontend Error States
```typescript
type URLValidationState = 
  | { status: 'valid'; url: string }
  | { status: 'invalid'; error: string; suggestions?: string[] }
  | { status: 'validating' }
  | { status: 'unknown' };

type BrowserActionState =
  | { status: 'ready' }
  | { status: 'executing'; progress: number }
  | { status: 'success'; openedUrls: string[] }
  | { status: 'partial'; failures: Array<{url: string; error: string}> }
  | { status: 'failed'; error: string };
```

### Backend Error Recovery
```rust
// Graceful degradation strategy
impl BrowserActionService {
    async fn execute_with_fallback(&self, actions: &[BrowserAction]) {
        for action in actions {
            match self.open_url_async(&action.url).await {
                Ok(_) => log::info!("Opened URL: {}", action.url),
                Err(e) => {
                    log::warn!("Failed to open URL {}: {}. Continuing...", action.url, e);
                    // Continue with next URL instead of failing completely
                }
            }
        }
    }
}
```

## Performance Considerations

### Async Execution Strategy
- Browser action execution runs in background thread
- Does not block notification display
- 500ms delay between multiple URL openings
- 3-second timeout per URL opening
- Failed URLs are logged but don't affect other actions

### Caching Strategy
- URL validation results cached for 1 hour
- URL preview info cached for 24 hours
- Common URL suggestions stored in local storage
- Browser action history kept for 30 days

---
*Created: 2025-08-15*
*Status: 🎨 System Design Complete*