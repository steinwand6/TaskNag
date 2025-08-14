use crate::services::AgentService;
use tauri::State;
use serde_json::Value;

#[tauri::command]
pub async fn test_ollama_connection(
    agent: State<'_, AgentService>,
) -> Result<bool, String> {
    agent
        .test_connection()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_ollama_models(
    agent: State<'_, AgentService>,
) -> Result<Vec<String>, String> {
    agent
        .list_models()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_task_with_ai(
    description: String,
    agent: State<'_, AgentService>,
) -> Result<Value, String> {
    let analysis = agent
        .analyze_task(&description)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(analysis)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_project_plan(
    description: String,
    agent: State<'_, AgentService>,
) -> Result<Value, String> {
    let plan = agent
        .create_project_plan(&description)
        .await
        .map_err(|e| e.to_string())?;
    
    serde_json::to_value(plan)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn parse_natural_language_task(
    request: String,
    agent: State<'_, AgentService>,
) -> Result<Value, String> {
    agent
        .parse_natural_language_task(&request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn chat_with_agent(
    message: String,
    context: Option<String>,
    agent: State<'_, AgentService>,
) -> Result<String, String> {
    agent
        .chat(&message, context)
        .await
        .map_err(|e| e.to_string())
}