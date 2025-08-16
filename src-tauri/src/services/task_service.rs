use crate::database::Database;
use crate::error::AppError;
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest, Tag, CreateTagRequest, UpdateTagRequest};
use crate::services::TagService;
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
            // Browser actions
            browser_actions: request.browser_actions.map(|ba| 
                serde_json::to_string(&ba).unwrap_or_default()
            ),
            // Tag system
            tags: None,
        };
        
        sqlx::query(
            r#"
            INSERT INTO tasks (
                id, title, description, status, parent_id, due_date, completed_at, 
                created_at, updated_at, progress, notification_type, notification_days_before, 
                notification_time, notification_days_of_week, notification_level, browser_actions
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
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
        .bind(task.progress)
        .bind(&task.notification_type)
        .bind(task.notification_days_before)
        .bind(&task.notification_time)
        .bind(&task.notification_days_of_week)
        .bind(task.notification_level)
        .bind(&task.browser_actions)
        .execute(&self.db.pool)
        .await?;
        
        Ok(task)
    }
    
    pub async fn get_tasks(&self) -> Result<Vec<Task>, AppError> {
        let mut tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
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
        
        // 各タスクにタグ情報を追加
        for task in &mut tasks {
            task.tags = self.get_tags_for_task(&task.id).await.ok();
        }
        
        Ok(tasks)
    }
    
    pub async fn get_task_by_id(&self, id: &str) -> Result<Task, AppError> {
        log::info!("Getting task by id: {}", id);
        
        let mut task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| {
            log::error!("Database error in get_task_by_id for id {}: {}", id, e);
            AppError::Database(e)
        })?
        .ok_or_else(|| {
            log::warn!("Task not found with id: {}", id);
            AppError::NotFound(format!("Task with id {} not found", id))
        })?;
        
        log::info!("Successfully retrieved task: {} (title: {})", task.id, task.title);
        
        // タグ情報を追加
        match self.get_tags_for_task(&task.id).await {
            Ok(tags) => {
                task.tags = Some(tags);
                log::debug!("Added {} tags to task {}", task.tags.as_ref().map(|t| t.len()).unwrap_or(0), task.id);
            }
            Err(e) => {
                log::warn!("Failed to get tags for task {}: {}", task.id, e);
                task.tags = None;
            }
        }
        
        Ok(task)
    }
    
    pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Task, AppError> {
        // トランザクションを開始
        let mut tx = self.db.pool.begin().await?;
        
        // Get existing task first (トランザクション内で実行)
        let mut task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))?;
        
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
        
        // ブラウザアクションの更新
        if let Some(browser_actions) = request.browser_actions {
            task.browser_actions = Some(serde_json::to_string(&browser_actions).unwrap_or_default());
        }
        
        task.updated_at = Utc::now().to_rfc3339();
        
        // メインのタスクレコードを先に更新
        println!("UpdateTask: About to update main task record for task {}", task.id);
        match sqlx::query(
            r#"
            UPDATE tasks
            SET title = ?2, description = ?3, status = ?4, 
                parent_id = ?5, due_date = ?6, completed_at = ?7, updated_at = ?8, progress = ?9,
                notification_type = ?10, notification_days_before = ?11, notification_time = ?12,
                notification_days_of_week = ?13, notification_level = ?14, browser_actions = ?15
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
        .bind(task.progress)
        .bind(&task.notification_type)
        .bind(task.notification_days_before)
        .bind(&task.notification_time)
        .bind(&task.notification_days_of_week)
        .bind(task.notification_level)
        .bind(&task.browser_actions)
        .execute(&mut *tx)
        .await {
            Ok(result) => {
                println!("UpdateTask: Successfully updated main task record for task {}, rows_affected: {}", task.id, result.rows_affected());
            },
            Err(e) => {
                println!("UpdateTask: FAILED to update main task record for task {}: {:?}", task.id, e);
                return Err(e.into());
            }
        }
        
        // タグの更新処理（メインタスク更新後に実行）
        if let Some(tags) = request.tags {
            println!("UpdateTask: Processing {} tags for task {}", tags.len(), task.id);
            for tag in &tags {
                println!("UpdateTask: Tag ID: {}, Name: {}", tag.id, tag.name);
            }
            
            // 既存のタグ関連付けを削除
            println!("UpdateTask: Deleting existing tag relations for task {}", task.id);
            let delete_result = sqlx::query("DELETE FROM task_tags WHERE task_id = ?1")
                .bind(&task.id)
                .execute(&mut *tx)
                .await?;
            println!("UpdateTask: Deleted {} existing tag relations", delete_result.rows_affected());
            
            // 新しいタグ関連付けを追加（存在するタグのみ）
            for tag in tags {
                // タスクが存在するかチェック（念のため）
                let task_exists: Option<(String,)> = sqlx::query_as(
                    "SELECT id FROM tasks WHERE id = ?1"
                )
                .bind(&task.id)
                .fetch_optional(&mut *tx)
                .await?;
                
                println!("UpdateTask: Task {} exists: {}", task.id, task_exists.is_some());
                
                // タグが存在するかチェック
                let tag_exists: Option<(String, String, String)> = sqlx::query_as(
                    "SELECT id, name, color FROM tags WHERE id = ?1"
                )
                .bind(&tag.id)
                .fetch_optional(&mut *tx)
                .await?;
                
                let tag_found = if let Some((found_id, found_name, found_color)) = &tag_exists {
                    println!("UpdateTask: Tag found - ID: {}, Name: {}, Color: {}", found_id, found_name, found_color);
                    true
                } else {
                    println!("UpdateTask: Tag {} does not exist", tag.id);
                    false
                };
                
                if task_exists.is_some() && tag_found {
                    println!("UpdateTask: About to insert task_tag relation: task_id={}, tag_id={}", task.id, tag.id);
                    
                    let current_time = Utc::now().to_rfc3339();
                    match sqlx::query(
                        r#"
                        INSERT INTO task_tags (task_id, tag_id, created_at)
                        VALUES (?1, ?2, ?3)
                        "#,
                    )
                    .bind(&task.id)
                    .bind(&tag.id)
                    .bind(&current_time)
                    .execute(&mut *tx)
                    .await {
                        Ok(result) => {
                            println!("UpdateTask: Successfully added tag {} to task {}, rows_affected: {}", tag.id, task.id, result.rows_affected());
                        },
                        Err(e) => {
                            println!("UpdateTask: FAILED to add tag {} to task {}: {:?}", tag.id, task.id, e);
                            
                            // FOREIGN KEY制約の詳細なデバッグ情報を取得
                            let fk_check: Result<Vec<(String, String, String, String)>, _> = sqlx::query_as(
                                "PRAGMA foreign_key_check"
                            )
                            .fetch_all(&mut *tx)
                            .await;
                            
                            match fk_check {
                                Ok(violations) => {
                                    if !violations.is_empty() {
                                        println!("UpdateTask: FOREIGN KEY violations found:");
                                        for (table, rowid, parent, fkid) in violations {
                                            println!("  - Table: {}, RowID: {}, Parent: {}, ForeignKeyID: {}", table, rowid, parent, fkid);
                                        }
                                    } else {
                                        println!("UpdateTask: No FOREIGN KEY violations found in entire database");
                                    }
                                },
                                Err(fk_err) => {
                                    println!("UpdateTask: Failed to check FOREIGN KEY constraints: {:?}", fk_err);
                                }
                            }
                            
                            // FOREIGN KEY設定を確認
                            let fk_status: Result<(i64,), _> = sqlx::query_as(
                                "PRAGMA foreign_keys"
                            )
                            .fetch_one(&mut *tx)
                            .await;
                            
                            match fk_status {
                                Ok((enabled,)) => {
                                    println!("UpdateTask: FOREIGN KEY constraints enabled: {}", enabled == 1);
                                },
                                Err(status_err) => {
                                    println!("UpdateTask: Failed to check FOREIGN KEY status: {:?}", status_err);
                                }
                            }
                            
                            // 手動でINSERTを試行して詳細エラーを取得
                            println!("UpdateTask: Attempting manual INSERT to identify specific constraint failure");
                            let manual_insert_result = sqlx::query(
                                "INSERT INTO task_tags (task_id, tag_id, created_at) VALUES (?1, ?2, ?3)"
                            )
                            .bind(&task.id)
                            .bind(&tag.id)  
                            .bind(&current_time)
                            .execute(&mut *tx)
                            .await;
                            
                            match manual_insert_result {
                                Ok(result) => {
                                    println!("UpdateTask: Manual INSERT succeeded, rows_affected: {}", result.rows_affected());
                                    // 成功したので重複を避けるためにロールバック要素を削除
                                    sqlx::query("DELETE FROM task_tags WHERE task_id = ?1 AND tag_id = ?2")
                                        .bind(&task.id)
                                        .bind(&tag.id)
                                        .execute(&mut *tx)
                                        .await
                                        .ok();
                                },
                                Err(manual_err) => {
                                    println!("UpdateTask: Manual INSERT also failed: {:?}", manual_err);
                                }
                            }
                            
                            return Err(e.into());
                        }
                    }
                } else {
                    println!("UpdateTask: Tag {} does not exist, skipping", tag.id);
                }
            }
        }
        
        // トランザクションをコミット
        tx.commit().await?;
        println!("UpdateTask: Transaction committed successfully for task {}", task.id);
        
        // 更新後のタスクを最新のタグ情報と一緒に返す
        self.get_task_by_id(id).await
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
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
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
            .map_err(AppError::InvalidInput)?;
        
        self.update_task(id, UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(status),
            parent_id: None,
            due_date: None,
            notification_settings: None,
            browser_actions: None,
            tags: None,
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
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
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
        if !(0..=100).contains(&progress) {
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
        .bind(task.progress)
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
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
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
        use chrono::{DateTime, Utc, Local, Weekday, Datelike};
        
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, parent_id, due_date, completed_at, created_at, updated_at, progress, notification_type, notification_days_before, notification_time, notification_days_of_week, notification_level, browser_actions
            FROM tasks
            WHERE status != 'done' 
              AND notification_type IS NOT NULL 
              AND notification_type != 'none'
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        if !tasks.is_empty() {
            println!("NotificationCheck: Found {} tasks with notifications at {} (Local: {})", 
                     tasks.len(), 
                     Utc::now().format("%H:%M:%S UTC"),
                     Local::now().format("%H:%M:%S JST"));
        }
        
        let mut notifications = Vec::new();
        let now_local = Local::now();
        let now = now_local.naive_local().and_utc(); // ローカル時刻をnaive形式でUTCとして扱う
        
        for task in &tasks {
            let notification_type = task.notification_type.as_deref().unwrap_or("none");
            
            match notification_type {
                "due_date_based" => {
                    if let Some(due_date_str) = &task.due_date {
                        if let Ok(due_date) = DateTime::parse_from_rfc3339(due_date_str) {
                            // 期日もローカル時刻として解釈
                            let due_date_local = due_date.naive_utc().and_local_timezone(chrono::Local).unwrap();
                            
                            // notification_timeが設定されている場合は、期限時刻として使用
                            let target_due_time = if let Some(time_str) = &task.notification_time {
                                if let Ok(target_time) = chrono::NaiveTime::parse_from_str(time_str, "%H:%M") {
                                    // 期日の日付 + 指定された時刻
                                    due_date_local.date_naive().and_time(target_time).and_local_timezone(chrono::Local).unwrap()
                                } else {
                                    due_date_local
                                }
                            } else {
                                due_date_local
                            };
                            
                            let target_due_as_utc = target_due_time.naive_local().and_utc();
                            let hours_until_due = (target_due_as_utc - now).num_hours();
                            let days_before = task.notification_days_before.unwrap_or(1);
                            let notification_start_hours = days_before as i64 * 24;
                            
                            println!("NotificationCheck: Task '{}' - Target Due: {} JST, Current: {} JST, Hours until: {}", 
                                     task.title, 
                                     target_due_time.format("%m/%d %H:%M"),
                                     now_local.format("%m/%d %H:%M"),
                                     hours_until_due);
                            
                            // 期日ベース通知の判定：指定日数前から毎時0分に通知
                            if hours_until_due <= notification_start_hours && hours_until_due >= 0 {
                                // 毎時0分±1分（0分、1分）で通知
                                use chrono::Timelike;
                                let minutes = now_local.minute();
                                let is_notification_time = minutes <= 1;
                                
                                if is_notification_time {
                                    println!("NotificationCheck: ✅ Creating due-date notification for task: {} ({}h until target due time {}) at {}:{:02}", 
                                             task.title, hours_until_due, target_due_time.format("%H:%M"), now_local.hour(), minutes);
                                    notifications.push(crate::models::TaskNotification {
                                        task_id: task.id.clone(),
                                        title: task.title.clone(),
                                        level: task.notification_level.unwrap_or(1),
                                        days_until_due: Some(hours_until_due / 24),
                                        notification_type: "due_date_based".to_string(),
                                    });
                                }
                            }
                        }
                    }
                },
                "recurring" => {
                    // 定期通知の判定
                    if let (Some(days_str), Some(time_str)) = (&task.notification_days_of_week, &task.notification_time) {
                        if let Ok(days_of_week) = serde_json::from_str::<Vec<i32>>(days_str) {
                            let current_weekday = match now_local.weekday() {
                                Weekday::Sun => 0,
                                Weekday::Mon => 1,
                                Weekday::Tue => 2,
                                Weekday::Wed => 3,
                                Weekday::Thu => 4,
                                Weekday::Fri => 5,
                                Weekday::Sat => 6,
                            };
                            
                            if days_of_week.contains(&current_weekday) && should_notify_at_time(&now_local, time_str) {
                                notifications.push(crate::models::TaskNotification {
                                    task_id: task.id.clone(),
                                    title: task.title.clone(),
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
        
        if !notifications.is_empty() {
            println!("NotificationCheck: Generated {} notifications:", notifications.len());
            for notification in &notifications {
                println!("  - {} (Level {}, {})", notification.title, notification.level, notification.notification_type);
            }
        }
        
        Ok(notifications)
    }
}

// 指定時刻での通知判定（±30秒の範囲）
fn should_notify_at_time<T>(now: &chrono::DateTime<T>, time_str: &str) -> bool 
where T: chrono::TimeZone {
    use chrono::{NaiveTime, Timelike};
    
    if let Ok(target_time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        let current_time = now.time();
        let target_seconds = target_time.num_seconds_from_midnight();
        let current_seconds = current_time.num_seconds_from_midnight();
        
        let time_diff = (current_seconds as i32 - target_seconds as i32).abs();
        
        // ±30秒の範囲
        time_diff <= 30
    } else {
        false
    }
}


impl TaskService {
    // タグ関連メソッド
    pub async fn get_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        TagService::get_all_tags(&self.db.pool).await
    }
    
    pub async fn get_tag_by_id(&self, id: &str) -> Result<Tag, AppError> {
        TagService::get_tag_by_id(&self.db.pool, id).await
    }
    
    pub async fn create_tag(&self, request: CreateTagRequest) -> Result<Tag, AppError> {
        TagService::create_tag(&self.db.pool, request).await
    }
    
    pub async fn update_tag(&self, id: &str, request: UpdateTagRequest) -> Result<Tag, AppError> {
        TagService::update_tag(&self.db.pool, id, request).await
    }
    
    pub async fn delete_tag(&self, id: &str) -> Result<(), AppError> {
        TagService::delete_tag(&self.db.pool, id).await
    }
    
    pub async fn add_tag_to_task(&self, task_id: &str, tag_id: &str) -> Result<(), AppError> {
        TagService::add_tag_to_task(&self.db.pool, task_id, tag_id).await
    }
    
    pub async fn remove_tag_from_task(&self, task_id: &str, tag_id: &str) -> Result<(), AppError> {
        TagService::remove_tag_from_task(&self.db.pool, task_id, tag_id).await
    }
    
    pub async fn get_tags_for_task(&self, task_id: &str) -> Result<Vec<Tag>, AppError> {
        TagService::get_tags_for_task(&self.db.pool, task_id).await
    }
}