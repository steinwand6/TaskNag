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
    let tasks = service.get_tasks().await.map_err(|e| e.to_string())?;
    let mut notifications = Vec::new();
    
    let now = chrono::Utc::now();
    
    for task in tasks {
        // 完了済みタスクはスキップ
        if task.status == "done" {
            continue;
        }
        
        // 期限がないタスクはスキップ
        let due_date = match &task.due_date {
            Some(date_str) => {
                match chrono::DateTime::parse_from_rfc3339(date_str) {
                    Ok(date) => date.with_timezone(&chrono::Utc),
                    Err(_) => continue,
                }
            },
            None => continue,
        };
        
        let days_until_due = (due_date - now).num_days();
        
        let (level, should_notify) = match days_until_due {
            d if d <= 0 && task.priority == "required" => (1, true),  // 期限当日または過ぎている（必須のみレベル1）
            d if d <= 0 => (2, true),  // 期限当日または過ぎている（その他）
            1 => (2, true),            // 1日前（重要）
            2..=3 => (3, true),        // 2-3日前（注意）
            _ => (0, false),           // まだ通知不要
        };
        
        if should_notify {
            let priority_emoji = match task.priority.as_str() {
                "required" => "🚨",
                "high" => "⚠️",
                "medium" => "📋",
                "low" => "📝",
                _ => "📋",
            };
            
            let level_text = match level {
                1 => "【期限当日】",
                2 => "【期限明日】", 
                3 => "【期限間近】",
                _ => "",
            };
            
            let title = format!("{} {}", priority_emoji, level_text);
            let body = format!("{}\n期限: {}", task.title, due_date.format("%m/%d %H:%M"));
            
            // 通知プラグインを使用して通知を送信
            let _ = app.emit("notification", serde_json::json!({
                "title": title.clone(),
                "body": body
            }));
            
            // 通知情報を記録
            notifications.push(serde_json::json!({
                "taskId": task.id,
                "title": task.title,
                "level": level,
                "daysUntilDue": days_until_due,
                "priority": task.priority
            }));
        }
    }
    
    Ok(notifications)
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