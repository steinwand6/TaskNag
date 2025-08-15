use crate::services::{AgentService, PersonalityManager};
use crate::services::personality_manager::AIPersonality;
use tauri::State;
use serde_json::Value;
use std::sync::{Arc, RwLock};

#[tauri::command]
pub async fn test_ollama_connection(
    agent: State<'_, AgentService>,
) -> Result<bool, String> {
    log::info!("Ollama接続テスト開始");
    
    let result = agent
        .test_connection()
        .await
        .map_err(|e| {
            log::error!("Ollama接続テストエラー: {}", e);
            format!("Ollama接続失敗: {}", e)
        })?;
    
    log::info!("Ollama接続テスト結果: {}", result);
    Ok(result)
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
    log::info!("AI分析リクエスト開始: {}", description);
    
    let analysis = agent
        .analyze_task(&description)
        .await
        .map_err(|e| {
            log::error!("AI分析エラー: {}", e);
            format!("AI分析に失敗しました: {}", e)
        })?;
    
    log::info!("AI分析成功");
    serde_json::to_value(analysis)
        .map_err(|e| {
            log::error!("JSON変換エラー: {}", e);
            format!("結果の変換に失敗しました: {}", e)
        })
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
    log::info!("自然言語タスク解析リクエスト開始: {}", request);
    
    let result = agent
        .parse_natural_language_task(&request)
        .await
        .map_err(|e| {
            log::error!("自然言語タスク解析エラー: {}", e);
            format!("自然言語解析に失敗しました: {}", e)
        })?;
    
    log::info!("自然言語タスク解析成功");
    serde_json::to_value(result)
        .map_err(|e| {
            log::error!("JSON変換エラー: {}", e);
            format!("結果の変換に失敗しました: {}", e)
        })
}

#[tauri::command]
pub async fn chat_with_agent(
    message: String,
    context: Option<String>,
    agent: State<'_, AgentService>,
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<String, String> {
    // 基本プロンプトを構築
    let base_prompt = if let Some(ctx) = context {
        format!("Context: {}\n\nユーザー: {}", ctx, message)
    } else {
        format!("ユーザー: {}", message)
    };
    
    // 現在の性格でプロンプトを拡張
    let enhanced_prompt = {
        let manager = personality_manager.read().map_err(|e| e.to_string())?;
        manager.enhance_prompt(&base_prompt)
    };
    
    // 性格が適用されたプロンプトでチャット実行
    agent
        .chat_with_personality(&enhanced_prompt, true)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_available_personalities(
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<Vec<AIPersonality>, String> {
    let manager = personality_manager.read().map_err(|e| e.to_string())?;
    let personalities = manager.get_personalities()
        .into_iter()
        .cloned()
        .collect();
    Ok(personalities)
}

#[tauri::command]
pub fn set_ai_personality(
    personality_id: String,
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<(), String> {
    let mut manager = personality_manager.write().map_err(|e| e.to_string())?;
    manager.set_current_personality(personality_id)
}

#[tauri::command]
pub fn get_current_personality(
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<Option<(String, String)>, String> {
    let manager = personality_manager.read().map_err(|e| e.to_string())?;
    Ok(manager.get_current_personality_info())
}

