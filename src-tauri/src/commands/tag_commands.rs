use tauri::State;
use crate::models::{Tag, CreateTagRequest, UpdateTagRequest};
use crate::services::TaskService;

#[tauri::command]
pub async fn get_all_tags(service: State<'_, TaskService>) -> Result<Vec<Tag>, String> {
    service.get_all_tags().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tag_by_id(id: String, service: State<'_, TaskService>) -> Result<Tag, String> {
    service.get_tag_by_id(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_tag(request: CreateTagRequest, service: State<'_, TaskService>) -> Result<Tag, String> {
    service.create_tag(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_tag(id: String, request: UpdateTagRequest, service: State<'_, TaskService>) -> Result<Tag, String> {
    service.update_tag(&id, request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_tag(id: String, service: State<'_, TaskService>) -> Result<(), String> {
    service.delete_tag(&id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_tag_to_task(task_id: String, tag_id: String, service: State<'_, TaskService>) -> Result<(), String> {
    service.add_tag_to_task(&task_id, &tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_tag_from_task(task_id: String, tag_id: String, service: State<'_, TaskService>) -> Result<(), String> {
    service.remove_tag_from_task(&task_id, &tag_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tags_for_task(task_id: String, service: State<'_, TaskService>) -> Result<Vec<Tag>, String> {
    service.get_tags_for_task(&task_id).await.map_err(|e| e.to_string())
}