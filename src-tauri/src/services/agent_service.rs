use crate::services::ollama_client::{OllamaClient, OllamaError, GenerateOptions};
use crate::services::context_service::{ContextService, ContextError};
use crate::services::prompt_manager::{EnhancedPromptManager, PromptError, GeneratedPrompt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use thiserror::Error;
use chrono::{DateTime, Utc};

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Ollama error: {0}")]
    OllamaError(#[from] OllamaError),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("JSON parse error: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Agent not initialized")]
    NotInitialized,
    
    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),
    
    #[error("Context error: {0}")]
    ContextError(#[from] ContextError),
    
    #[error("Prompt error: {0}")]
    PromptError(#[from] PromptError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalysis {
    pub improved_title: String,
    pub improved_description: String,
    pub suggested_tags: Vec<String>,
    pub complexity: String, // "simple", "medium", "complex"
    pub estimated_hours: f32,
    pub subtasks: Vec<SubtaskSuggestion>,
    pub priority_reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskSuggestion {
    pub title: String,
    pub description: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPlan {
    pub phases: Vec<ProjectPhase>,
    pub total_estimated_days: i32,
    pub dependencies: Vec<TaskDependency>,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPhase {
    pub name: String,
    pub description: String,
    pub tasks: Vec<SubtaskSuggestion>,
    pub estimated_days: i32,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    pub from_task: String,
    pub to_task: String,
    pub dependency_type: String, // "blocks", "requires", "relates_to"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub description: String,
    pub target_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConversation {
    pub id: String,
    pub messages: Vec<ConversationMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub role: String, // "user" or "assistant"
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

pub struct PromptManager {
    templates: std::collections::HashMap<String, String>,
}

impl Default for PromptManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptManager {
    pub fn new() -> Self {
        let mut templates = std::collections::HashMap::new();
        
        // Task Analysis Prompt
        templates.insert(
            "task_analysis".to_string(),
            r#"あなたはタスク管理の専門家です。以下のタスクを分析して、改善提案をJSONで返してください。

タスク内容: {description}

以下の形式のJSONで応答してください:
{{
  "improved_title": "明確で行動指向のタイトル（50文字以内）",
  "improved_description": "目標が明確な詳細説明",
  "suggested_tags": ["関連するタグ（最大5個）"],
  "complexity": "simple/medium/complex のいずれか",
  "estimated_hours": 時間の見積もり（数値）,
  "subtasks": [
    {{"title": "サブタスクのタイトル", "description": "詳細", "order": 1}}
  ],
  "priority_reasoning": "優先度の根拠説明"
}}

タスクを実行可能で測定可能にすることに重点を置いて分析してください。日本語で回答してください。"#.to_string()
        );
        
        // Project Planning Prompt
        templates.insert(
            "project_planning".to_string(),
            r#"あなたはプロジェクト計画の専門家です。以下の要求に対して詳細なプロジェクト計画を作成してください。

プロジェクト概要: {description}

以下の形式のJSONで応答してください:
{{
  "phases": [
    {{
      "name": "フェーズ名",
      "description": "フェーズの説明",
      "tasks": [{"title": "タスク名", "description": "詳細", "order": 1}],
      "estimated_days": 日数見積もり,
      "order": 順序
    }}
  ],
  "total_estimated_days": 総日数見積もり,
  "dependencies": [
    {{"from_task": "前提タスク", "to_task": "依存先タスク", "dependency_type": "blocks"}}
  ],
  "milestones": [
    {{"name": "マイルストーン名", "description": "説明", "target_date": "YYYY-MM-DD"}}
  ]
}}

プロジェクトを論理的なフェーズに分解し、明確な成果物を定義してください。現実的な時間見積もりを行ってください。日本語で回答してください。"#.to_string()
        );
        
        // Natural Language Task Creation
        templates.insert(
            "natural_language_task".to_string(),
            r#"以下の自然言語の要求を構造化されたタスクデータに変換してください。

要求: {request}

以下の形式のJSONで応答してください:
{{
  "title": "簡潔なタスクタイトル",
  "description": "詳細な説明",
  "suggested_status": "todo/in_progress/in_review のいずれか",
  "due_date_suggestion": "言及されている場合 YYYY-MM-DD 形式",
  "tags": ["関連するカテゴリタグ"],
  "notification_needed": 緊急度に基づく true/false
}}

要求から関連するすべての情報を正確に抽出してください。日本語で回答してください。"#.to_string()
        );
        
        Self { templates }
    }
    
    pub fn build_prompt(&self, template_name: &str, variables: &std::collections::HashMap<String, String>) -> Result<String, AgentError> {
        let template = self.templates.get(template_name)
            .ok_or_else(|| AgentError::InvalidPrompt(format!("Template '{}' not found", template_name)))?;
        
        let mut prompt = template.clone();
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            prompt = prompt.replace(&placeholder, value);
        }
        
        Ok(prompt)
    }
}

pub struct AgentService {
    ollama: OllamaClient,
    prompt_manager: PromptManager,
    enhanced_prompt_manager: EnhancedPromptManager,
    context_service: ContextService,
    pub db: SqlitePool,
    pub config: AgentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub default_model: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub available_models: Vec<String>,
    pub model_preferences: std::collections::HashMap<String, ModelPreference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPreference {
    pub display_name: String,
    pub description: String,
    pub recommended_for: Vec<String>,
    pub performance_tier: ModelPerformanceTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelPerformanceTier {
    Fast,      // 高速だが品質は控えめ
    Balanced,  // バランス型
    Quality,   // 高品質だが時間がかかる
}

impl Default for AgentConfig {
    fn default() -> Self {
        let mut model_preferences = std::collections::HashMap::new();
        
        // 一般的なモデルの推奨設定
        model_preferences.insert(
            "gemma3:12b".to_string(),
            ModelPreference {
                display_name: "Gemma3 12B".to_string(),
                description: "高品質な日本語対応モデル、タスク分析に最適".to_string(),
                recommended_for: vec!["タスク分析".to_string(), "プロジェクト計画".to_string()],
                performance_tier: ModelPerformanceTier::Quality,
            }
        );
        
        model_preferences.insert(
            "llama3:latest".to_string(),
            ModelPreference {
                display_name: "Llama3 Latest".to_string(),
                description: "バランス型の汎用モデル".to_string(),
                recommended_for: vec!["一般的なチャット".to_string(), "タスク作成".to_string()],
                performance_tier: ModelPerformanceTier::Balanced,
            }
        );
        
        model_preferences.insert(
            "llama3:8b".to_string(),
            ModelPreference {
                display_name: "Llama3 8B".to_string(),
                description: "軽量で高速なモデル".to_string(),
                recommended_for: vec!["簡単なタスク".to_string(), "クイックチャット".to_string()],
                performance_tier: ModelPerformanceTier::Fast,
            }
        );
        
        Self {
            default_model: "gemma3:12b".to_string(),
            base_url: "http://localhost:11434".to_string(),
            timeout_seconds: 60,
            available_models: vec![],
            model_preferences,
        }
    }
}

impl AgentService {
    pub fn new(db: SqlitePool) -> Self {
        log::info!("Initializing AgentService with enhanced context support");
        let config = AgentConfig::default();
        
        let enhanced_prompt_manager = EnhancedPromptManager::new(db.clone());
        let context_service = ContextService::new(db.clone());
        
        log::info!("AgentService components initialized successfully");
        
        Self {
            ollama: OllamaClient::new(
                config.base_url.clone(),
                config.default_model.clone(),
                config.timeout_seconds
            ),
            prompt_manager: PromptManager::new(),
            enhanced_prompt_manager,
            context_service,
            db,
            config,
        }
    }
    
    pub fn with_custom_ollama(db: SqlitePool, base_url: String, model: String) -> Self {
        let config = AgentConfig {
            base_url: base_url.clone(),
            default_model: model.clone(),
            timeout_seconds: 30,
            ..Default::default()
        };
        
        Self {
            ollama: OllamaClient::new(base_url, model, 30),
            prompt_manager: PromptManager::new(),
            enhanced_prompt_manager: EnhancedPromptManager::new(db.clone()),
            context_service: ContextService::new(db.clone()),
            db,
            config,
        }
    }
    
    /// Test Ollama connection
    pub async fn test_connection(&self) -> Result<bool, AgentError> {
        Ok(self.ollama.test_connection().await?)
    }
    
    /// List available models with detailed information
    pub async fn list_models(&self) -> Result<Vec<crate::services::ollama_client::ModelInfo>, AgentError> {
        let models = self.ollama.list_models().await?;
        Ok(models)
    }
    
    /// List available model names (simple list)
    pub async fn list_model_names(&self) -> Result<Vec<String>, AgentError> {
        let models = self.ollama.list_models().await?;
        Ok(models.into_iter().map(|m| m.name).collect())
    }
    
    /// Get current model name
    pub fn get_current_model(&self) -> String {
        self.ollama.get_model().clone()
    }
    
    /// Set model (for dynamic model changing) and save to database
    pub async fn set_model(&mut self, model: String) -> Result<(), AgentError> {
        // Update the client with new model
        self.ollama = OllamaClient::new(
            self.ollama.base_url.clone(),
            model.clone(),
            self.ollama.timeout_seconds
        );
        
        // Save to database
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
            VALUES ('current_model', ?1, datetime('now'))
            "#
        )
        .bind(&model)
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    /// Load model from database
    pub async fn load_saved_model(&mut self) -> Result<(), AgentError> {
        if let Ok(Some(row)) = sqlx::query_as::<_, (String,)>(
            "SELECT value FROM agent_config WHERE key = 'current_model'"
        )
        .fetch_optional(&self.db)
        .await 
        {
            let saved_model = row.0;
            self.config.default_model = saved_model.clone();
            self.ollama = OllamaClient::new(
                self.config.base_url.clone(),
                saved_model,
                self.config.timeout_seconds
            );
        }
        Ok(())
    }
    
    /// Get agent configuration
    pub fn get_config(&self) -> &AgentConfig {
        &self.config
    }
    
    /// Update agent configuration
    pub async fn update_config(&mut self, new_config: AgentConfig) -> Result<(), AgentError> {
        // Update Ollama client with new settings
        self.ollama = OllamaClient::new(
            new_config.base_url.clone(),
            new_config.default_model.clone(),
            new_config.timeout_seconds
        );
        
        // Save default model to database
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
            VALUES ('current_model', ?1, datetime('now'))
            "#
        )
        .bind(&new_config.default_model)
        .execute(&self.db)
        .await?;
        
        // Save base URL to database
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
            VALUES ('base_url', ?1, datetime('now'))
            "#
        )
        .bind(&new_config.base_url)
        .execute(&self.db)
        .await?;
        
        // Save timeout to database
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO agent_config (key, value, updated_at) 
            VALUES ('timeout_seconds', ?1, datetime('now'))
            "#
        )
        .bind(new_config.timeout_seconds.to_string())
        .execute(&self.db)
        .await?;
        
        // Update in-memory config
        self.config = new_config;
        
        Ok(())
    }
    
    /// Load full configuration from database
    pub async fn load_saved_config(&mut self) -> Result<(), AgentError> {
        // Load saved model
        if let Ok(Some(row)) = sqlx::query_as::<_, (String,)>(
            "SELECT value FROM agent_config WHERE key = 'current_model'"
        )
        .fetch_optional(&self.db)
        .await 
        {
            self.config.default_model = row.0;
        }
        
        // Load saved base URL
        if let Ok(Some(row)) = sqlx::query_as::<_, (String,)>(
            "SELECT value FROM agent_config WHERE key = 'base_url'"
        )
        .fetch_optional(&self.db)
        .await 
        {
            self.config.base_url = row.0;
        }
        
        // Load saved timeout
        if let Ok(Some(row)) = sqlx::query_as::<_, (String,)>(
            "SELECT value FROM agent_config WHERE key = 'timeout_seconds'"
        )
        .fetch_optional(&self.db)
        .await 
        {
            if let Ok(timeout) = row.0.parse::<u64>() {
                self.config.timeout_seconds = timeout;
            }
        }
        
        // Update Ollama client with loaded config
        self.ollama = OllamaClient::new(
            self.config.base_url.clone(),
            self.config.default_model.clone(),
            self.config.timeout_seconds
        );
        
        Ok(())
    }
    
    /// Get model preferences for a specific model
    pub fn get_model_preference(&self, model_name: &str) -> Option<&ModelPreference> {
        self.config.model_preferences.get(model_name)
    }
    
    /// Add or update model preference
    pub fn set_model_preference(&mut self, model_name: String, preference: ModelPreference) {
        self.config.model_preferences.insert(model_name, preference);
    }
    
    /// Analyze a task description and provide suggestions
    pub async fn analyze_task(&self, description: &str) -> Result<TaskAnalysis, AgentError> {
        let mut variables = std::collections::HashMap::new();
        variables.insert("description".to_string(), description.to_string());
        
        let prompt = self.prompt_manager.build_prompt("task_analysis", &variables)?;
        
        let options = GenerateOptions {
            temperature: Some(0.7),
            num_predict: Some(1000),
            top_k: None,
            top_p: None,
        };
        
        let json_response = self.ollama.generate_json(&prompt, Some(options)).await?;
        let analysis: TaskAnalysis = serde_json::from_value(json_response)?;
        
        Ok(analysis)
    }
    
    /// Create a project plan from description
    pub async fn create_project_plan(&self, description: &str) -> Result<ProjectPlan, AgentError> {
        let mut variables = std::collections::HashMap::new();
        variables.insert("description".to_string(), description.to_string());
        
        let prompt = self.prompt_manager.build_prompt("project_planning", &variables)?;
        
        let options = GenerateOptions {
            temperature: Some(0.7),
            num_predict: Some(2000),
            top_k: None,
            top_p: None,
        };
        
        let json_response = self.ollama.generate_json(&prompt, Some(options)).await?;
        let plan: ProjectPlan = serde_json::from_value(json_response)?;
        
        Ok(plan)
    }
    
    /// Parse natural language into task data
    pub async fn parse_natural_language_task(&self, request: &str) -> Result<serde_json::Value, AgentError> {
        let mut variables = std::collections::HashMap::new();
        variables.insert("request".to_string(), request.to_string());
        
        let prompt = self.prompt_manager.build_prompt("natural_language_task", &variables)?;
        
        let options = GenerateOptions {
            temperature: Some(0.5),
            num_predict: Some(500),
            top_k: None,
            top_p: None,
        };
        
        let json_response = self.ollama.generate_json(&prompt, Some(options)).await?;
        Ok(json_response)
    }
    
    /// Chat with the agent
    pub async fn chat(&self, message: &str, context: Option<String>) -> Result<String, AgentError> {
        let mut base_prompt = format!("日本語で自然に会話してください。\n\nユーザー: {}", message);
        
        if let Some(ctx) = context {
            base_prompt = format!("Context: {}\n\n{}", ctx, base_prompt);
        }
        
        let prompt = base_prompt;
        
        let options = GenerateOptions {
            temperature: Some(0.8),
            num_predict: Some(1000),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&prompt, Some(options)).await?;
        Ok(OllamaClient::get_response_content(&response))
    }
    
    /// Chat with custom prompt (for personality-enhanced prompts)  
    pub async fn chat_with_personality(&self, message: &str, is_personality_enhanced: bool) -> Result<String, AgentError> {
        let prompt = if is_personality_enhanced {
            // 既に性格が適用されたプロンプト
            message.to_string()
        } else {
            // 通常のプロンプト
            format!("日本語で自然に会話してください。\n\n{}", message)
        };
        
        let options = GenerateOptions {
            temperature: Some(0.8),
            num_predict: Some(1000),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&prompt, Some(options)).await?;
        Ok(OllamaClient::get_response_content(&response))
    }
    
    /// Generate context-aware prompt using EnhancedPromptManager
    pub async fn generate_context_aware_prompt(&self, template_id: &str) -> Result<GeneratedPrompt, AgentError> {
        let generated_prompt = self.enhanced_prompt_manager.generate_prompt(template_id).await?;
        Ok(generated_prompt)
    }
    
    /// Chat with context-aware prompt for task consultation
    pub async fn chat_with_task_consultation(&self, user_message: &str) -> Result<String, AgentError> {
        log::info!("Starting task consultation with context awareness");
        let generated_prompt = self.enhanced_prompt_manager.generate_prompt("task_consultation").await
            .map_err(|e| {
                log::error!("Failed to generate task consultation prompt: {}", e);
                e
            })?;
        
        let full_prompt = format!(
            "{}\n\n## ユーザーの相談\n{}\n\n上記の状況を踏まえて、親身になってアドバイスしてください。",
            generated_prompt.final_prompt,
            user_message
        );
        
        let options = GenerateOptions {
            temperature: Some(0.7),
            num_predict: Some(1500),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&full_prompt, Some(options)).await
            .map_err(|e| {
                log::error!("Ollama request failed for task consultation: {}", e);
                e
            })?;
        
        log::info!("Task consultation completed successfully");
        Ok(OllamaClient::get_response_content(&response))
    }
    
    /// Chat with context-aware prompt for planning assistance
    pub async fn chat_with_planning_assistance(&self, user_message: &str) -> Result<String, AgentError> {
        let generated_prompt = self.enhanced_prompt_manager.generate_prompt("planning_assistant").await?;
        
        let full_prompt = format!(
            "{}\n\n## 計画したい内容\n{}\n\n効率的で実現可能な計画を一緒に立てましょう。",
            generated_prompt.final_prompt,
            user_message
        );
        
        let options = GenerateOptions {
            temperature: Some(0.6),
            num_predict: Some(2000),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&full_prompt, Some(options)).await?;
        Ok(OllamaClient::get_response_content(&response))
    }
    
    /// Generate motivation boost message
    pub async fn generate_motivation_boost(&self) -> Result<String, AgentError> {
        let generated_prompt = self.enhanced_prompt_manager.generate_prompt("motivation_boost").await?;
        
        let options = GenerateOptions {
            temperature: Some(0.8),
            num_predict: Some(800),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&generated_prompt.final_prompt, Some(options)).await?;
        Ok(OllamaClient::get_response_content(&response))
    }
    
    /// Get current context information
    pub async fn get_current_context(&self) -> Result<Vec<crate::services::context_service::ContextData>, AgentError> {
        let context_data = self.context_service.collect_basic_context().await?;
        Ok(context_data)
    }
    
    /// Enhanced task analysis with context awareness
    pub async fn analyze_task_with_context(&self, description: &str) -> Result<TaskAnalysis, AgentError> {
        // 基本的なコンテキストを取得
        let context_data = self.context_service.collect_basic_context().await?;
        
        // コンテキスト情報を文字列として構築
        let mut context_info = String::new();
        for data in context_data {
            context_info.push_str(&format!("## {}\n", data.context_type));
            for (key, value) in data.data {
                context_info.push_str(&format!("- {}: {}\n", key, value));
            }
            context_info.push('\n');
        }
        
        let mut vars = std::collections::HashMap::new();
        vars.insert("task_description".to_string(), description.to_string());
        vars.insert("context_info".to_string(), context_info);
        
        let prompt = self.prompt_manager.build_prompt("task_analysis", &vars)?;
        
        let options = GenerateOptions {
            temperature: Some(0.4),
            num_predict: Some(2000),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&prompt, Some(options)).await?;
        let json_response = OllamaClient::get_response_content(&response);
        
        let analysis: TaskAnalysis = serde_json::from_str(&json_response)?;
        Ok(analysis)
    }
    
    /// Save conversation to database
    pub async fn save_conversation(&self, conversation: &AgentConversation) -> Result<(), AgentError> {
        let messages_json = serde_json::to_string(&conversation.messages)?;
        
        sqlx::query(
            r#"
            INSERT INTO agent_conversations (id, messages, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(id) DO UPDATE SET
                messages = excluded.messages,
                updated_at = excluded.updated_at
            "#
        )
        .bind(&conversation.id)
        .bind(&messages_json)
        .bind(conversation.created_at.to_rfc3339())
        .bind(conversation.updated_at.to_rfc3339())
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    /// Get conversation from database
    pub async fn get_conversation(&self, id: &str) -> Result<Option<AgentConversation>, AgentError> {
        let row = sqlx::query_as::<_, (String, String, String, String)>(
            r#"
            SELECT id, messages, created_at, updated_at
            FROM agent_conversations
            WHERE id = ?1
            "#
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?;
        
        match row {
            Some((id, messages_json, created_at, updated_at)) => {
                let messages: Vec<ConversationMessage> = serde_json::from_str(&messages_json)?;
                Ok(Some(AgentConversation {
                    id,
                    messages,
                    created_at: DateTime::parse_from_rfc3339(&created_at)
                        .unwrap()
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at)
                        .unwrap()
                        .with_timezone(&Utc),
                }))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prompt_manager() {
        let manager = PromptManager::new();
        let mut vars = std::collections::HashMap::new();
        vars.insert("description".to_string(), "Test task".to_string());
        
        let prompt = manager.build_prompt("task_analysis", &vars).unwrap();
        assert!(prompt.contains("Test task"));
    }
    
    #[tokio::test]
    async fn test_model_management() {
        // テスト用のインメモリデータベース
        let db = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        
        // テスト用マイグレーション（agent_configテーブル）
        sqlx::query(
            r#"
            CREATE TABLE agent_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )
            "#
        )
        .execute(&db)
        .await
        .unwrap();
        
        // AgentServiceインスタンス作成
        let mut agent_service = AgentService::new(db.clone());
        
        // デフォルトモデル確認
        let initial_model = agent_service.get_current_model();
        assert_eq!(initial_model, "gemma3:12b");
        
        // モデル変更とデータベース保存
        let new_model = "llama3:latest".to_string();
        agent_service.set_model(new_model.clone()).await.unwrap();
        
        // モデルが変更されたことを確認
        assert_eq!(agent_service.get_current_model(), new_model);
        
        // データベースに保存されたことを確認
        let saved_model: (String,) = sqlx::query_as(
            "SELECT value FROM agent_config WHERE key = 'current_model'"
        )
        .fetch_one(&db)
        .await
        .unwrap();
        assert_eq!(saved_model.0, new_model);
        
        // 新しいAgentServiceインスタンスで保存されたモデルを読み込み
        let mut new_agent_service = AgentService::new(db.clone());
        new_agent_service.load_saved_model().await.unwrap();
        
        // 読み込まれたモデルが正しいことを確認
        assert_eq!(new_agent_service.get_current_model(), new_model);
    }
    
    #[test]
    fn test_ollama_client_model_getter() {
        let client = OllamaClient::new(
            "http://localhost:11434".to_string(),
            "test-model".to_string(),
            30
        );
        
        assert_eq!(client.get_model(), "test-model");
    }
    
    #[tokio::test]
    async fn test_enhanced_agent_service_integration() {
        // テスト用のインメモリデータベース
        let db = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        
        // テーブル作成
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'pending',
                priority TEXT NOT NULL DEFAULT 'medium',
                due_date TEXT,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                parent_id INTEGER,
                project_id TEXT,
                estimated_time INTEGER,
                actual_time INTEGER,
                difficulty INTEGER DEFAULT 1,
                progress INTEGER DEFAULT 0,
                notification_settings TEXT,
                FOREIGN KEY (parent_id) REFERENCES tasks (id)
            )
        "#)
        .execute(&db)
        .await
        .unwrap();
        
        // AgentServiceインスタンス作成
        let agent_service = AgentService::new(db.clone());
        
        // コンテキスト取得テスト
        let context_result = agent_service.get_current_context().await;
        assert!(context_result.is_ok());
        let context_data = context_result.unwrap();
        assert!(!context_data.is_empty());
        
        // プロンプト生成テスト
        let prompt_result = agent_service.generate_context_aware_prompt("task_consultation").await;
        assert!(prompt_result.is_ok());
        let generated_prompt = prompt_result.unwrap();
        assert_eq!(generated_prompt.template_id, "task_consultation");
        assert!(!generated_prompt.final_prompt.is_empty());
        
        // 統合が正しく動作していることを確認
        assert!(generated_prompt.final_prompt.contains("TaskNagAI"));
    }
}