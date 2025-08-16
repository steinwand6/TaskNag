use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::services::TaskService;
use tauri::{AppHandle, State, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;

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
    log::info!("Command: get_task_by_id called with id: {}", id);
    
    match service.get_task_by_id(&id).await {
        Ok(task) => {
            log::info!("Command: get_task_by_id succeeded for id: {}", id);
            Ok(task)
        }
        Err(e) => {
            log::error!("Command: get_task_by_id failed for id {}: {}", id, e);
            Err(e.to_string())
        }
    }
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
        let title = match notification.notification_type.as_str() {
            "due_date_based" => {
                let days_text = match notification.days_until_due.unwrap_or(0) {
                    0 => "【期限当日】",
                    1 => "【期限明日】",
                    d if d <= 3 => "【期限間近】",
                    _ => "【期限通知】",
                };
                format!("📅 {}", days_text)
            },
            "recurring" => "🔔 定期リマインド".to_string(),
            _ => "📋 タスク通知".to_string()
        };
        
        // Windows通知を送信
        send_windows_notification(
            app.clone(),
            title,
            notification.title.clone(),
            notification.level as u32,
        ).await?;
        
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
        browser_actions: None,
        tags: None,
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

#[tauri::command]
pub async fn send_windows_notification(
    app: AppHandle,
    title: String,
    body: String,
    level: u32,
) -> Result<(), String> {
    // Windows通知を送信
    app.notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| e.to_string())?;
    
    // レベル2以上で音を鳴らす
    if level >= 2 {
        let _ = app.emit("play_notification_sound", serde_json::json!({ "level": level }));
    }
    
    // レベル3でアプリを最大化
    if level >= 3 {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn test_notification_immediate(
    app: AppHandle,
    service: State<'_, TaskService>,
) -> Result<Vec<serde_json::Value>, String> {
    // 現在の通知設定を持つタスクをすべて取得して即座に通知を送信
    let _notifications = service.check_notifications().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    
    // 通知チェックロジックを無視して、設定のあるすべてのタスクを通知
    let all_tasks = service.get_tasks().await.map_err(|e| e.to_string())?;
    
    for task in all_tasks {
        if let Some(notification_type) = &task.notification_type {
            if notification_type != "none" {
                let level = task.notification_level.unwrap_or(1);
                
                // 通知タイプに応じた表示
                let (title_prefix, test_suffix) = match notification_type.as_str() {
                    "due_date_based" => ("📅 期日通知", "（テスト）"),
                    "recurring" => ("🔔 定期通知", "（テスト）"),
                    _ => ("📋 通知", "（テスト）"),
                };
                
                let title = format!("{}{}", title_prefix, test_suffix);
                
                // Windows通知を送信
                send_windows_notification(
                    app.clone(),
                    title.clone(),
                    task.title.clone(),
                    level as u32,
                ).await?;
                
                result.push(serde_json::json!({
                    "taskId": task.id,
                    "title": task.title,
                    "level": level,
                    "notificationType": notification_type,
                    "testMode": true
                }));
                
                println!("TestNotification: Sent immediate test notification for task: {} (Level {})", task.title, level);
            }
        }
    }
    
    if result.is_empty() {
        println!("TestNotification: No tasks with notification settings found");
    } else {
        println!("TestNotification: Sent {} immediate test notifications", result.len());
    }
    
    Ok(result)
}