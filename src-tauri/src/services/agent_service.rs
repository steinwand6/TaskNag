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

impl PromptManager {
    pub fn new() -> Self {
        let mut templates = std::collections::HashMap::new();
        
        // Task Analysis Prompt
        templates.insert(
            "task_analysis".to_string(),
            r#"You are a task management expert. Analyze the following task description and provide structured suggestions.

Task Description: {description}

Please respond with a JSON object containing:
- improved_title: A clear, action-oriented title (max 50 chars)
- improved_description: An enhanced description with clear objectives
- suggested_tags: Array of relevant tags (max 5)
- complexity: Either "simple", "medium", or "complex"
- estimated_hours: Realistic time estimate in hours
- subtasks: Array of subtask objects with title, description, and order
- priority_reasoning: Brief explanation of priority level

Focus on making the task actionable and measurable. Be concise but comprehensive."#.to_string()
        );
        
        // Project Planning Prompt
        templates.insert(
            "project_planning".to_string(),
            r#"You are a project planning expert. Create a detailed project plan for the following request.

Project Description: {description}

Please respond with a JSON object containing:
- phases: Array of project phases, each with name, description, tasks, estimated_days, and order
- total_estimated_days: Total project duration
- dependencies: Array of task dependencies
- milestones: Key project milestones

Break down the project into logical phases with clear deliverables. Be realistic with time estimates."#.to_string()
        );
        
        // Natural Language Task Creation
        templates.insert(
            "natural_language_task".to_string(),
            r#"Convert the following natural language request into structured task data.

Request: {request}

Extract and return a JSON object with:
- title: Brief task title
- description: Detailed description
- suggested_status: "todo", "in_progress", or "in_review"
- due_date_suggestion: If mentioned, format as "YYYY-MM-DD"
- tags: Relevant category tags
- notification_needed: true/false based on urgency

Be precise and extract all relevant information from the request."#.to_string()
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
            ollama: OllamaClient::default(),
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
        let mut prompt = message.to_string();
        
        if let Some(ctx) = context {
            prompt = format!("Context: {}\n\nUser: {}", ctx, message);
        }
        
        let options = GenerateOptions {
            temperature: Some(0.8),
            num_predict: Some(1000),
            top_k: None,
            top_p: None,
        };
        
        let response = self.ollama.generate(&prompt, Some(options)).await?;
        Ok(response.response)
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