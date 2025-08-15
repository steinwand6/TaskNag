use tauri::State;
use crate::services::agent_service::{AgentService, TaskAnalysis};
use crate::services::prompt_manager::GeneratedPrompt;
use crate::services::context_service::ContextData;

#[tauri::command]
pub async fn chat_with_task_consultation(
    message: String,
    agent_service: State<'_, AgentService>,
) -> Result<String, String> {
    agent_service.chat_with_task_consultation(&message)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn chat_with_planning_assistance(
    message: String,
    agent_service: State<'_, AgentService>,
) -> Result<String, String> {
    agent_service.chat_with_planning_assistance(&message)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_motivation_boost(
    agent_service: State<'_, AgentService>,
) -> Result<String, String> {
    agent_service.generate_motivation_boost()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_context(
    agent_service: State<'_, AgentService>,
) -> Result<Vec<ContextData>, String> {
    agent_service.get_current_context()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_context_aware_prompt(
    template_id: String,
    agent_service: State<'_, AgentService>,
) -> Result<GeneratedPrompt, String> {
    agent_service.generate_context_aware_prompt(&template_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn analyze_task_with_context(
    description: String,
    agent_service: State<'_, AgentService>,
) -> Result<TaskAnalysis, String> {
    agent_service.analyze_task_with_context(&description)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_task_consultation_prompt(
    agent_service: State<'_, AgentService>,
) -> Result<GeneratedPrompt, String> {
    agent_service.generate_context_aware_prompt("task_consultation")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_planning_prompt(
    agent_service: State<'_, AgentService>,
) -> Result<GeneratedPrompt, String> {
    agent_service.generate_context_aware_prompt("planning_assistant")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_motivation_prompt(
    agent_service: State<'_, AgentService>,
) -> Result<GeneratedPrompt, String> {
    agent_service.generate_context_aware_prompt("motivation_boost")
        .await
        .map_err(|e| e.to_string())
}