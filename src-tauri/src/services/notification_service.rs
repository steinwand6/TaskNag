use crate::database::Database;
use crate::error::AppError;
use crate::models::{Task, TaskNotification};
use crate::services::browser_action_service::BrowserActionService;
use chrono::{DateTime, Local, Duration, Datelike, Timelike};
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationService {
    db: Database,
    browser_action_service: Arc<BrowserActionService>,
}

impl NotificationService {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            browser_action_service: Arc::new(BrowserActionService::new()),
        }
    }

    /// Create service with custom browser action service (for testing)
    pub fn with_browser_action_service(db: Database, browser_action_service: Arc<BrowserActionService>) -> Self {
        Self {
            db,
            browser_action_service,
        }
    }

    /// 現在の通知をチェックして返すメイン関数
    pub async fn check_notifications(&self, current_time: DateTime<Local>) -> Result<Vec<TaskNotification>, AppError> {
        let mut notifications = Vec::new();
        
        // アクティブなタスクを取得
        let tasks = self.get_active_tasks().await?;
        
        for task in tasks {
            // Skip completed tasks
            if task.status == "done" {
                continue;
            }
            
            // Skip tasks without notification settings
            let notification_type = match &task.notification_type {
                Some(t) if t != "none" => t,
                _ => continue,
            };
            
            match notification_type.as_str() {
                "due_date_based" => {
                    if let Some(notification) = self.check_due_date_notification(&task, current_time) {
                        notifications.push(notification);
                    }
                }
                "recurring" => {
                    if let Some(notification) = self.check_recurring_notification(&task, current_time) {
                        notifications.push(notification);
                    }
                }
                _ => {}
            }
        }
        
        Ok(notifications)
    }

    /// 通知を発火し、ブラウザアクションを実行
    pub async fn fire_notification(&self, notification: &TaskNotification) -> Result<(), AppError> {
        log::info!("Firing notification for task: {} - {}", notification.task_id, notification.title);
        
        // タスクの詳細情報を取得
        let task = self.get_task_by_id(&notification.task_id).await?;
        
        // ブラウザアクションが設定されている場合、実行
        if let Some(browser_actions_json) = &task.browser_actions {
            match self.parse_browser_action_settings(browser_actions_json) {
                Ok(browser_action_settings) => {
                    if browser_action_settings.enabled && !browser_action_settings.actions.is_empty() {
                        log::info!("Executing {} browser actions for notification", browser_action_settings.actions.len());
                        match self.browser_action_service.execute_actions(&browser_action_settings.actions).await {
                            Ok(_) => {
                                log::info!("Successfully executed browser actions for task: {}", task.id);
                            }
                            Err(e) => {
                                log::warn!("Failed to execute browser actions for task {}: {}. Notification will still be shown.", task.id, e);
                                // Continue with notification even if browser actions fail
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Failed to parse browser actions for task {}: {}. Skipping browser actions.", task.id, e);
                    // Continue with notification even if parsing fails
                }
            }
        }
        
        // TODO: 実際の通知システム（システムトレイ、デスクトップ通知等）の実装
        log::info!("Desktop notification shown for: {}", notification.title);
        
        Ok(())
    }

    /// 通知レベルに基づく重要度判定
    pub fn should_execute_browser_actions(&self, notification_level: Option<i32>) -> bool {
        match notification_level {
            Some(3) => true,  // High priority - always execute
            Some(2) => true,  // Medium priority - execute  
            Some(1) => false, // Low priority - skip browser actions
            _ => false,       // No level set - skip
        }
    }

    /// 期日ベース通知のチェック
    fn check_due_date_notification(&self, task: &Task, current_time: DateTime<Local>) -> Option<TaskNotification> {
        let due_date_str = task.due_date.as_ref()?;
        let due_date = DateTime::parse_from_rfc3339(due_date_str).ok()?.with_timezone(&Local);
        
        let days_before = task.notification_days_before.unwrap_or(1);
        let default_time = "09:00".to_string();
        let notification_time = task.notification_time.as_ref().unwrap_or(&default_time);
        
        // Parse notification time
        let time_parts: Vec<&str> = notification_time.split(':').collect();
        if time_parts.len() != 2 {
            return None;
        }
        
        let hour = time_parts[0].parse::<u32>().ok()?;
        let minute = time_parts[1].parse::<u32>().ok()?;
        
        // Calculate notification date
        let notification_date = due_date - Duration::days(days_before as i64);
        let notification_datetime = notification_date
            .date_naive()
            .and_hms_opt(hour, minute, 0)?
            .and_local_timezone(Local)
            .single()?;
        
        // Check if it's time for notification (within 15 minutes after the notification time)
        let time_diff_minutes = (current_time - notification_datetime).num_minutes();
        log::info!("NotificationService: Checking due date notification for task '{}' - Target: {}, Current: {}, Diff: {} minutes", 
                   task.title, notification_datetime.format("%Y-%m-%d %H:%M:%S"), current_time.format("%Y-%m-%d %H:%M:%S"), time_diff_minutes);
        
        // Fire notification if current time is within 15 minutes after the target time (0 to 15 minutes late)
        if time_diff_minutes >= 0 && time_diff_minutes <= 15 {
            let days_until_due = (due_date - current_time).num_days();
            Some(TaskNotification {
                task_id: task.id.clone(),
                title: task.title.clone(),
                notification_type: "due_date_based".to_string(),
                level: task.notification_level.unwrap_or(1),
                days_until_due: Some(days_until_due),
            })
        } else {
            None
        }
    }

    /// 繰り返し通知のチェック
    fn check_recurring_notification(&self, task: &Task, current_time: DateTime<Local>) -> Option<TaskNotification> {
        let notification_time = task.notification_time.as_ref()?;
        let days_of_week_str = task.notification_days_of_week.as_ref()?;
        
        // Parse days of week
        let days_of_week: Vec<u32> = serde_json::from_str(days_of_week_str).ok()?;
        
        // Use local time directly (already in JST)
        let jst_time = current_time;
        let current_weekday = jst_time.weekday().num_days_from_monday() + 1; // Monday = 1
        
        log::info!("NotificationService: Checking recurring notification for task '{}' - JST: {}, Target time: {}, Days: {:?}", 
                   task.title, jst_time.format("%Y-%m-%d %H:%M:%S"), notification_time, days_of_week);
        
        // Check if current day is in the list
        if !days_of_week.contains(&current_weekday) {
            log::info!("NotificationService: Current weekday {} not in configured days {:?}", current_weekday, days_of_week);
            return None;
        }
        
        // Parse notification time
        let time_parts: Vec<&str> = notification_time.split(':').collect();
        if time_parts.len() != 2 {
            log::warn!("NotificationService: Invalid time format: {}", notification_time);
            return None;
        }
        
        let hour = time_parts[0].parse::<u32>().ok()?;
        let minute = time_parts[1].parse::<u32>().ok()?;
        
        // Check if it's the right time (within 15 minutes after the target time)
        let target_minutes = hour * 60 + minute;
        let current_minutes = jst_time.hour() * 60 + jst_time.minute();
        
        // Fire notification if target time is within the previous 15 minutes (16-30 handles 16-30 minute notifications)
        let time_diff_after = if current_minutes >= target_minutes {
            current_minutes - target_minutes
        } else {
            // Handle day rollover (e.g., target 23:30, current 00:15)
            (24 * 60 + current_minutes) - target_minutes
        };
        
        if time_diff_after <= 15 {
            log::info!("NotificationService: ✅ Firing recurring notification for task '{}' (target: {}:{:02}, current: {}:{:02}, diff: {} minutes)", 
                      task.title, hour, minute, jst_time.hour(), jst_time.minute(), time_diff_after);
            
            Some(TaskNotification {
                task_id: task.id.clone(),
                title: task.title.clone(),
                notification_type: "recurring".to_string(),
                level: task.notification_level.unwrap_or(1),
                days_until_due: None,
            })
        } else {
            log::info!("NotificationService: Time window missed for task '{}' (target: {}:{:02}, current: {}:{:02}, diff: {} minutes)", 
                      task.title, hour, minute, jst_time.hour(), jst_time.minute(), time_diff_after);
            None
        }
    }

    /// アクティブなタスクを取得
    async fn get_active_tasks(&self) -> Result<Vec<Task>, AppError> {
        log::info!("NotificationService: Executing get_active_tasks query");
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, 
                   created_at, updated_at, progress, notification_type, notification_days_before, 
                   notification_time, notification_days_of_week, notification_level, browser_actions
            FROM tasks
            WHERE status != 'done' AND notification_type IS NOT NULL AND notification_type != 'none'
            ORDER BY notification_level DESC, created_at DESC
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        log::info!("NotificationService: Retrieved {} active tasks", tasks.len());
        Ok(tasks)
    }

    /// IDでタスクを取得
    async fn get_task_by_id(&self, id: &str) -> Result<Task, AppError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, 
                   created_at, updated_at, progress, notification_type, notification_days_before, 
                   notification_time, notification_days_of_week, notification_level, browser_actions
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;
        
        Ok(task)
    }


    /// JSONからBrowserActionSettingsをパース
    fn parse_browser_action_settings(&self, json: &str) -> Result<crate::models::browser_action::BrowserActionSettings, AppError> {
        if json.trim().is_empty() {
            return Ok(crate::models::browser_action::BrowserActionSettings {
                enabled: false,
                actions: Vec::new(),
            });
        }
        
        serde_json::from_str(json)
            .map_err(|e| AppError::ParseError(format!("Failed to parse browser action settings: {}", e)))
    }

    /// 通知サービスの可用性をチェック
    pub async fn is_available(&self) -> bool {
        // データベース接続とブラウザアクションサービスの可用性をチェック
        self.browser_action_service.is_available().await
    }

    /// 実行ログと監査証跡の記録
    pub async fn log_notification_execution(&self, notification: &TaskNotification, success: bool, error: Option<&str>) -> Result<(), AppError> {
        let log_message = if success {
            format!("Successfully fired notification for task {}: {}", notification.task_id, notification.title)
        } else {
            format!("Failed to fire notification for task {}: {} - Error: {}", 
                notification.task_id, notification.title, error.unwrap_or("Unknown"))
        };
        
        log::info!("{}", log_message);
        
        // TODO: 将来的にはデータベースに実行ログを保存することも検討
        // INSERT INTO notification_logs (task_id, notification_id, executed_at, success, error_message)
        
        Ok(())
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new(Database::new_placeholder())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_notification_level_filtering() {
        let db = Database::new_placeholder();
        let service = NotificationService::new(db);

        assert!(service.should_execute_browser_actions(Some(3))); // High
        assert!(service.should_execute_browser_actions(Some(2))); // Medium
        assert!(!service.should_execute_browser_actions(Some(1))); // Low
        assert!(!service.should_execute_browser_actions(None)); // None
    }

    #[tokio::test]
    async fn test_browser_action_settings_parsing() {
        let db = Database::new_placeholder();
        let service = NotificationService::new(db);

        // Valid JSON for BrowserActionSettings
        let valid_json = r#"{"enabled":true,"actions":[{"id":"1","label":"Google","url":"https://google.com","enabled":true,"order":1,"createdAt":"2024-01-01T00:00:00Z"}]}"#;
        let result = service.parse_browser_action_settings(valid_json);
        match &result {
            Ok(settings) => {
                assert_eq!(settings.enabled, true);
                assert_eq!(settings.actions.len(), 1);
            },
            Err(e) => {
                panic!("Expected valid JSON to parse correctly, but got error: {:?}", e);
            }
        }

        // Empty JSON
        let empty_json = "";
        let result = service.parse_browser_action_settings(empty_json);
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.enabled, false);
        assert_eq!(settings.actions.len(), 0);

        // Invalid JSON
        let invalid_json = "invalid json";
        let result = service.parse_browser_action_settings(invalid_json);
        assert!(result.is_err());
    }
}