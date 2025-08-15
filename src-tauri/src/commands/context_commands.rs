use crate::services::ContextService;
use tauri::State;
use serde_json::Value;

#[tauri::command]
pub async fn get_temporal_context(
    context_service: State<'_, ContextService>,
) -> Result<Value, String> {
    let temporal = context_service.get_temporal_context();
    serde_json::to_value(temporal).map_err(|e| format!("Serialization error: {}", e))
}

#[tauri::command]
pub async fn get_task_context(
    context_service: State<'_, ContextService>,
) -> Result<Value, String> {
    let task_context = context_service.get_task_context().await
        .map_err(|e| format!("Failed to get task context: {}", e))?;
    serde_json::to_value(task_context).map_err(|e| format!("Serialization error: {}", e))
}

#[tauri::command]
pub async fn get_basic_context(
    context_service: State<'_, ContextService>,
) -> Result<Value, String> {
    let contexts = context_service.collect_basic_context().await
        .map_err(|e| format!("Failed to collect basic context: {}", e))?;
    serde_json::to_value(contexts).map_err(|e| format!("Serialization error: {}", e))
}

#[tauri::command]
pub async fn get_context_for_scope(
    context_service: State<'_, ContextService>,
    scope: Vec<String>,
) -> Result<Value, String> {
    let scope_refs: Vec<&str> = scope.iter().map(|s| s.as_str()).collect();
    let contexts = context_service.collect_context_for_scope(&scope_refs).await
        .map_err(|e| format!("Failed to collect context for scope: {}", e))?;
    serde_json::to_value(contexts).map_err(|e| format!("Serialization error: {}", e))
}

#[tauri::command]
pub async fn get_context_as_prompt_variables(
    context_service: State<'_, ContextService>,
    scope: Vec<String>,
) -> Result<Value, String> {
    let scope_refs: Vec<&str> = scope.iter().map(|s| s.as_str()).collect();
    let contexts = context_service.collect_context_for_scope(&scope_refs).await
        .map_err(|e| format!("Failed to collect context: {}", e))?;
    
    let variables = context_service.context_to_prompt_variables(&contexts);
    serde_json::to_value(variables).map_err(|e| format!("Serialization error: {}", e))
}