use tauri::State;
use sqlx::SqlitePool;
use crate::services::prompt_manager::{EnhancedPromptManager, PromptTemplate, GeneratedPrompt, PromptCategory};

#[tauri::command]
pub async fn get_prompt_templates(
    db: State<'_, SqlitePool>,
) -> Result<Vec<PromptTemplate>, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    let templates = manager.get_templates()
        .into_iter()
        .cloned()
        .collect();
    Ok(templates)
}

#[tauri::command]
pub async fn get_prompt_template(
    template_id: String,
    db: State<'_, SqlitePool>,
) -> Result<Option<PromptTemplate>, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    Ok(manager.get_template(&template_id).cloned())
}

#[tauri::command]
pub async fn generate_prompt(
    template_id: String,
    db: State<'_, SqlitePool>,
) -> Result<GeneratedPrompt, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    manager.generate_prompt(&template_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_task_consultation_prompt(
    db: State<'_, SqlitePool>,
) -> Result<GeneratedPrompt, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    manager.generate_prompt("task_consultation")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_planning_prompt(
    db: State<'_, SqlitePool>,
) -> Result<GeneratedPrompt, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    manager.generate_prompt("planning_assistant")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_motivation_prompt(
    db: State<'_, SqlitePool>,
) -> Result<GeneratedPrompt, String> {
    let manager = EnhancedPromptManager::new(db.inner().clone());
    manager.generate_prompt("motivation_boost")
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_prompt_categories() -> Result<Vec<PromptCategory>, String> {
    Ok(vec![
        PromptCategory::TaskManagement,
        PromptCategory::Planning,
        PromptCategory::Analysis,
        PromptCategory::Motivation,
        PromptCategory::General,
    ])
}