use crate::services::ollama_client::{OllamaClient, OllamaError, GenerateOptions};
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
    db: SqlitePool,
}

impl AgentService {
    pub fn new(db: SqlitePool) -> Self {
        Self {
            ollama: OllamaClient::new(
                "http://localhost:11434".to_string(),
                "gemma3:12b".to_string(),
                60  // タイムアウトも60秒に延長
            ),
            prompt_manager: PromptManager::new(),
            db,
        }
    }
    
    pub fn with_custom_ollama(db: SqlitePool, base_url: String, model: String) -> Self {
        Self {
            ollama: OllamaClient::new(base_url, model, 30),
            prompt_manager: PromptManager::new(),
            db,
        }
    }
    
    /// Test Ollama connection
    pub async fn test_connection(&self) -> Result<bool, AgentError> {
        Ok(self.ollama.test_connection().await?)
    }
    
    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>, AgentError> {
        let models = self.ollama.list_models().await?;
        Ok(models.into_iter().map(|m| m.name).collect())
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
}