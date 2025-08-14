use crate::database::Database;
use crate::error::AppError;
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use chrono::Utc;
use uuid::Uuid;

pub struct TaskService {
    db: Database,
}

impl TaskService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
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
            // priority field removed as per .kiro/specs/notification-system-redesign
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
        
        sqlx::query(
            r#"
            INSERT INTO tasks (
                id, title, description, status, parent_id, due_date, completed_at, 
                created_at, updated_at, progress, notification_type, notification_days_before, 
                notification_time, notification_days_of_week, notification_level
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.parent_id)
        .bind(&task.due_date)
        .bind(&task.completed_at)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .bind(&task.progress)
        .bind(&task.notification_type)
        .bind(&task.notification_days_before)
        .bind(&task.notification_time)
        .bind(&task.notification_days_of_week)
        .bind(&task.notification_level)
        .execute(&self.db.pool)
        .await?;
        
        Ok(task)
    }
    
    pub async fn get_tasks(&self) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
            FROM tasks
            ORDER BY 
                CASE status 
                    WHEN 'inbox' THEN 1
                    WHEN 'todo' THEN 2
                    WHEN 'in_progress' THEN 3
                    WHEN 'done' THEN 4
                END,
                CASE notification_level
                    WHEN 3 THEN 1
                    WHEN 2 THEN 2
                    WHEN 1 THEN 3
                    ELSE 4
                END,
                created_at DESC
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(tasks)
    }
    
    pub async fn get_task_by_id(&self, id: &str) -> Result<Task, AppError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;
        
        task.ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))
    }
    
    pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Task, AppError> {
        // Get existing task first
        let mut task = self.get_task_by_id(id).await?;
        
        // Update fields if provided
        if let Some(title) = request.title {
            task.title = title;
        }
        if let Some(description) = request.description {
            task.description = Some(description);
        }
        if let Some(status) = request.status {
            task.status = status.to_string();
            // Set completed_at if status is Done
            if task.status == "done" {
                task.completed_at = Some(Utc::now().to_rfc3339());
            } else {
                task.completed_at = None;
            }
        }
        // priority field removed as per .kiro/specs/notification-system-redesign
        if request.parent_id.is_some() {
            task.parent_id = request.parent_id;
        }
        if let Some(due_date) = request.due_date {
            task.due_date = Some(due_date.to_rfc3339());
        }
        
        // 通知設定の更新
        if let Some(notification_settings) = request.notification_settings {
            task.notification_type = Some(notification_settings.notification_type);
            task.notification_days_before = notification_settings.days_before;
            task.notification_time = notification_settings.notification_time;
            task.notification_days_of_week = notification_settings.days_of_week.map(|days| 
                serde_json::to_string(&days).unwrap_or_default()
            );
            task.notification_level = Some(notification_settings.level);
        }
        
        task.updated_at = Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            UPDATE tasks
            SET title = ?2, description = ?3, status = ?4, 
                parent_id = ?5, due_date = ?6, completed_at = ?7, updated_at = ?8, progress = ?9,
                notification_type = ?10, notification_days_before = ?11, notification_time = ?12,
                notification_days_of_week = ?13, notification_level = ?14
            WHERE id = ?1
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.parent_id)
        .bind(&task.due_date)
        .bind(&task.completed_at)
        .bind(&task.updated_at)
        .bind(&task.progress)
        .bind(&task.notification_type)
        .bind(&task.notification_days_before)
        .bind(&task.notification_time)
        .bind(&task.notification_days_of_week)
        .bind(&task.notification_level)
        .execute(&self.db.pool)
        .await?;
        
        Ok(task)
    }
    
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
    
    pub async fn get_tasks_by_status(&self, status: &str) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
            FROM tasks
            WHERE status = ?1
            ORDER BY 
                CASE notification_level
                    WHEN 3 THEN 1
                    WHEN 2 THEN 2
                    WHEN 1 THEN 3
                    ELSE 4
                END,
                created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(tasks)
    }
    
    pub async fn move_task(&self, id: &str, new_status: &str) -> Result<Task, AppError> {
        use std::str::FromStr;
        use crate::models::TaskStatus;
        
        let status = TaskStatus::from_str(new_status)
            .map_err(|e| AppError::InvalidInput(e))?;
        
        self.update_task(id, UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(status),
            parent_id: None,
            due_date: None,
            notification_settings: None,
        }).await
    }
    
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
    
    // 子タスク管理機能
    pub async fn get_children(&self, parent_id: &str) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
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
    
    pub async fn get_task_with_children(&self, id: &str) -> Result<Task, AppError> {
        let mut task = self.get_task_by_id(id).await?;
        let children = self.get_children(id).await?;
        
        // 子タスクがある場合は進捗率を計算
        if !children.is_empty() {
            task.progress = Some(self.calculate_progress(&children));
        }
        
        Ok(task)
    }
    
    // 進捗率計算機能
    pub async fn calculate_and_update_progress(&self, parent_id: &str) -> Result<i32, AppError> {
        let children = self.get_children(parent_id).await?;
        
        if children.is_empty() {
            return Ok(0);
        }
        
        let progress = self.calculate_progress(&children);
        
        // 親タスクの進捗率を更新
        sqlx::query(
            r#"
            UPDATE tasks 
            SET progress = ?2, updated_at = ?3
            WHERE id = ?1
            "#,
        )
        .bind(parent_id)
        .bind(progress)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.db.pool)
        .await?;
        
        Ok(progress)
    }
    
    fn calculate_progress(&self, children: &[Task]) -> i32 {
        if children.is_empty() {
            return 0;
        }
        
        let total_progress: i32 = children.iter()
            .map(|child| {
                if child.status == "done" {
                    100
                } else {
                    child.progress.unwrap_or(0)
                }
            })
            .sum();
        
        total_progress / children.len() as i32
    }
    
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
        
        sqlx::query(
            r#"
            UPDATE tasks 
            SET progress = ?2, status = ?3, completed_at = ?4, updated_at = ?5
            WHERE id = ?1
            "#,
        )
        .bind(&task.id)
        .bind(&task.progress)
        .bind(&task.status)
        .bind(&task.completed_at)
        .bind(&task.updated_at)
        .execute(&self.db.pool)
        .await?;
        
        // 親タスクがある場合は親の進捗率も更新
        if let Some(parent_id) = &task.parent_id {
            self.calculate_and_update_progress(parent_id).await?;
        }
        
        Ok(task)
    }
    
    pub async fn get_root_tasks(&self) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
            FROM tasks
            WHERE parent_id IS NULL
            ORDER BY 
                CASE status 
                    WHEN 'inbox' THEN 1
                    WHEN 'todo' THEN 2
                    WHEN 'in_progress' THEN 3
                    WHEN 'done' THEN 4
                END,
                CASE notification_level
                    WHEN 3 THEN 1
                    WHEN 2 THEN 2
                    WHEN 1 THEN 3
                    ELSE 4
                END,
                created_at DESC
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(tasks)
    }
    
    // 新しい通知システム
    pub async fn check_notifications(&self) -> Result<Vec<crate::models::TaskNotification>, AppError> {
        use chrono::{DateTime, Utc, Weekday, Datelike};
        
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level
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
                },
                "recurring" => {
                    // 定期通知の判定
                    if let (Some(days_str), Some(time_str)) = (&task.notification_days_of_week, &task.notification_time) {
                        if let Ok(days_of_week) = serde_json::from_str::<Vec<i32>>(days_str) {
                            let current_weekday = match now.weekday() {
                                Weekday::Sun => 0,
                                Weekday::Mon => 1,
                                Weekday::Tue => 2,
                                Weekday::Wed => 3,
                                Weekday::Thu => 4,
                                Weekday::Fri => 5,
                                Weekday::Sat => 6,
                            };
                            
                            if days_of_week.contains(&current_weekday) && should_notify_at_time(&now, time_str) {
                                notifications.push(crate::models::TaskNotification {
                                    task_id: task.id,
                                    title: task.title,
                                    level: task.notification_level.unwrap_or(1),
                                    days_until_due: None,
                                    notification_type: "recurring".to_string(),
                                });
                            }
                        }
                    }
                },
                _ => {} // 'none' or unknown type
            }
        }
        
        Ok(notifications)
    }
}

// 指定時刻での通知判定（±30秒の範囲）
fn should_notify_at_time(now: &chrono::DateTime<chrono::Utc>, time_str: &str) -> bool {
    use chrono::{NaiveTime, Timelike};
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