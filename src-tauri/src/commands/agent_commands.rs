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
pub fn get_current_model(
    agent: State<'_, AgentService>,
) -> Result<String, String> {
    Ok(agent.get_current_model())
}

#[tauri::command]
pub async fn set_current_model(
    model: String,
    agent: State<'_, AgentService>,
) -> Result<(), String> {
    // 設定をデータベースに保存（実際のモデル変更は次回起動時に反映）
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
        VALUES ('current_model', ?1, datetime('now'))
        "#
    )
    .bind(&model)
    .execute(&agent.db)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
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
pub async fn set_ai_personality(
    personality_id: String,
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<(), String> {
    // ロックを取得して即座にクローンを作成
    let db = {
        let manager = personality_manager.read().map_err(|e| e.to_string())?;
        manager.db.clone()
    };
    
    // データベースへの保存
    if let Some(db) = db {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
            VALUES ('current_personality', ?1, datetime('now'))
            "#
        )
        .bind(&personality_id)
        .execute(&db)
        .await
        .map_err(|e| format!("Failed to save personality to database: {}", e))?;
    }
    
    // メモリ内の状態を更新（データベース処理は既に完了しているので同期的に処理）
    {
        let mut manager = personality_manager.write().map_err(|e| e.to_string())?;
        // PersonalityManagerの既存メソッドを使用（データベース処理はスキップ）
        if manager.get_personality(&personality_id).is_none() {
            return Err(format!("Personality '{}' not found", personality_id));
        }
        manager.set_current_personality_memory_only(personality_id)?;
    }
    
    Ok(())
}

#[tauri::command]
pub fn get_current_personality(
    personality_manager: State<'_, Arc<RwLock<PersonalityManager>>>,
) -> Result<Option<(String, String)>, String> {
    let manager = personality_manager.read().map_err(|e| e.to_string())?;
    Ok(manager.get_current_personality_info())
}

