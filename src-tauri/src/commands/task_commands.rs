use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::services::TaskService;
use tauri::{AppHandle, State, Emitter};

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

#[tauri::command]
pub async fn delete_task(id: String, service: State<'_, TaskService>) -> Result<(), String> {
    service
        .delete_task(&id)
        .await
        .map_err(|e| e.to_string())
}

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

#[tauri::command]
pub async fn get_incomplete_task_count(service: State<'_, TaskService>) -> Result<usize, String> {
    service
        .get_incomplete_task_count()
        .await
        .map_err(|e| e.to_string())
}

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
        
        // 通知情報を記録
        result.push(serde_json::json!({
            "taskId": notification.task_id,
            "title": notification.title,
            "level": notification.level,
            "daysUntilDue": notification.days_until_due,
            "notificationType": notification.notification_type
        }));
    }
    
    Ok(result)
}

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
        parent_id: None,
        due_date: None,
        notification_settings: Some(notification_settings),
    };
    
    service
        .update_task(&id, update_request)
        .await
        .map_err(|e| e.to_string())
}

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

#[tauri::command]
pub async fn get_root_tasks(service: State<'_, TaskService>) -> Result<Vec<Task>, String> {
    service.get_root_tasks().await.map_err(|e| e.to_string())
}