use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::services::TaskService;
use tauri::State;

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