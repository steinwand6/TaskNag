use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::services::context_service::{ContextService, ContextData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub template: String,
    pub required_context: Vec<String>,
    pub optional_context: Vec<String>,
    pub category: PromptCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptCategory {
    TaskManagement,
    Planning,
    Analysis,
    Motivation,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPrompt {
    pub template_id: String,
    pub final_prompt: String,
    pub used_context: Vec<String>,
    pub missing_context: Vec<String>,
}

pub struct EnhancedPromptManager {
    context_service: ContextService,
    templates: HashMap<String, PromptTemplate>,
}

impl EnhancedPromptManager {
    pub fn new(db: SqlitePool) -> Self {
        let context_service = ContextService::new(db);
        let mut manager = Self {
            context_service,
            templates: HashMap::new(),
        };
        
        manager.initialize_default_templates();
        manager
    }
    
    fn initialize_default_templates(&mut self) {
        // TaskNag基本タスク相談テンプレート
        self.add_template(PromptTemplate {
            id: "task_consultation".to_string(),
            name: "タスク相談".to_string(),
            template: r#"あなたはTaskNagAI、口うるさくて世話焼きなタスク管理アシスタントです。

## 現在の状況
- 時刻: {{current_time}}
- 日付: {{current_date}} ({{day_of_week}})
{{#if is_business_day}}
- 今日は営業日です
{{else}}
- 今日は休日です
{{/if}}

## タスク状況
{{#if task_count}}
- 総タスク数: {{task_count}}個
- 完了済み: {{completed_tasks}}個
- 進行中: {{in_progress_tasks}}個
- 未着手: {{pending_tasks}}個
{{#if overdue_tasks}}
- ⚠️ 期限切れ: {{overdue_tasks}}個
{{/if}}
{{#if urgent_tasks}}
- 🔥 緊急: {{urgent_tasks}}個
{{/if}}
{{else}}
- まだタスクが登録されていません
{{/if}}

ユーザーのタスクについて親身になって相談に乗り、具体的で実行可能なアドバイスを提供してください。"#.to_string(),
            required_context: vec![
                "current_time".to_string(),
                "current_date".to_string(),
                "day_of_week".to_string(),
                "is_business_day".to_string(),
            ],
            optional_context: vec![
                "task_count".to_string(),
                "completed_tasks".to_string(),
                "in_progress_tasks".to_string(),
                "pending_tasks".to_string(),
                "overdue_tasks".to_string(),
                "urgent_tasks".to_string(),
            ],
            category: PromptCategory::TaskManagement,
        });

        // 計画立案テンプレート
        self.add_template(PromptTemplate {
            id: "planning_assistant".to_string(),
            name: "計画立案アシスタント".to_string(),
            template: r#"あなたはTaskNagAI、効率的な計画立案をサポートするアシスタントです。

## 現在の時間状況
- 現在: {{current_time}} {{time_period}}
- {{current_date}} ({{day_of_week}})
{{#if is_business_day}}
- 営業日のため、通常の作業時間を想定します
{{else}}
- 休日のため、リラックスした計画を提案します
{{/if}}

## 既存のワークロード
{{#if task_count}}
- 現在{{task_count}}個のタスクを管理中
{{#if workload_level}}
- ワークロード: {{workload_level}}
{{/if}}
{{#if overdue_tasks}}
- 注意: {{overdue_tasks}}個の期限切れタスクがあります
{{/if}}
{{else}}
- 新しいプロジェクトを始める絶好の機会です
{{/if}}

効率的で実現可能な計画を一緒に立てましょう。具体的な時間配分と優先順位を提案します。"#.to_string(),
            required_context: vec![
                "current_time".to_string(),
                "current_date".to_string(),
                "day_of_week".to_string(),
                "is_business_day".to_string(),
                "time_period".to_string(),
            ],
            optional_context: vec![
                "task_count".to_string(),
                "workload_level".to_string(),
                "overdue_tasks".to_string(),
            ],
            category: PromptCategory::Planning,
        });

        // モチベーション向上テンプレート
        self.add_template(PromptTemplate {
            id: "motivation_boost".to_string(),
            name: "モチベーション向上".to_string(),
            template: r#"あなたはTaskNagAI、ユーザーのやる気を引き出す応援団長です！

## 現在の状況
{{current_time}} {{time_period}}、{{day_of_week}}の{{current_date}}

{{#if completed_tasks}}
🎉 素晴らしい！{{completed_tasks}}個のタスクを完了済み！
{{/if}}

{{#if in_progress_tasks}}
💪 {{in_progress_tasks}}個のタスクに取り組み中、頑張ってますね！
{{/if}}

{{#if pending_tasks}}
{{#if overdue_tasks}}
⚠️ {{overdue_tasks}}個の期限切れタスクがありますが、大丈夫！一歩ずつ進めばOK！
{{/if}}
📝 {{pending_tasks}}個の新しいタスクが待ってます。チャンスです！
{{/if}}

{{#unless task_count}}
✨ 今日は新しいことを始める最高の日です！
{{/unless}}

あなたの頑張りを全力でサポートします！一緒に目標を達成しましょう！"#.to_string(),
            required_context: vec![
                "current_time".to_string(),
                "current_date".to_string(),
                "day_of_week".to_string(),
                "time_period".to_string(),
            ],
            optional_context: vec![
                "task_count".to_string(),
                "completed_tasks".to_string(),
                "in_progress_tasks".to_string(),
                "pending_tasks".to_string(),
                "overdue_tasks".to_string(),
            ],
            category: PromptCategory::Motivation,
        });
    }
    
    pub fn add_template(&mut self, template: PromptTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    
    pub fn get_templates(&self) -> Vec<&PromptTemplate> {
        self.templates.values().collect()
    }
    
    pub fn get_template(&self, template_id: &str) -> Option<&PromptTemplate> {
        self.templates.get(template_id)
    }
    
    pub async fn generate_prompt(&self, template_id: &str) -> Result<GeneratedPrompt, PromptError> {
        let template = self.templates.get(template_id)
            .ok_or(PromptError::TemplateNotFound(template_id.to_string()))?;
            
        // コンテキストデータを収集
        let context_data = self.context_service.collect_basic_context().await?;
        let context_map = self.context_data_to_map(context_data);
        
        // テンプレートを処理
        let (final_prompt, used_context, missing_context) = 
            self.process_template(template, &context_map)?;
            
        Ok(GeneratedPrompt {
            template_id: template_id.to_string(),
            final_prompt,
            used_context,
            missing_context,
        })
    }
    
    fn context_data_to_map(&self, context_data: Vec<ContextData>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for context in context_data {
            for (key, value) in context.data {
                result.insert(key, value);
            }
        }
        result
    }
    
    fn process_template(
        &self,
        template: &PromptTemplate,
        context_map: &HashMap<String, String>,
    ) -> Result<(String, Vec<String>, Vec<String>), PromptError> {
        let mut final_prompt = template.template.clone();
        let mut used_context = Vec::new();
        let mut missing_context = Vec::new();
        
        // すべてのコンテキストキーを収集
        let mut all_keys = template.required_context.clone();
        all_keys.extend(template.optional_context.clone());
        
        // 各キーを処理
        for key in &all_keys {
            if let Some(value) = context_map.get(key) {
                final_prompt = final_prompt.replace(&format!("{{{{{}}}}}", key), value);
                used_context.push(key.clone());
                final_prompt = self.process_conditional_blocks(&final_prompt, key, true);
            } else {
                // 存在しない変数は空文字に置換
                final_prompt = final_prompt.replace(&format!("{{{{{}}}}}", key), "");
                final_prompt = self.process_conditional_blocks(&final_prompt, key, false);
                
                // 必須コンテキストが不足している場合は記録
                if template.required_context.contains(key) {
                    missing_context.push(key.clone());
                }
            }
        }
        
        // 未処理の条件付きブロックをクリーンアップ
        final_prompt = self.cleanup_conditional_blocks(final_prompt);
        
        Ok((final_prompt, used_context, missing_context))
    }
    
    fn process_conditional_blocks(&self, template: &str, key: &str, value_exists: bool) -> String {
        let mut result = template.to_string();
        
        // {{#if key}} ... {{/if}} パターンを処理
        let if_start = format!("{{{{#if {}}}}}", key);
        let if_end = "{{/if}}";
        
        while let Some(start_pos) = result.find(&if_start) {
            if let Some(end_pos) = result[start_pos..].find(if_end) {
                let full_end_pos = start_pos + end_pos + if_end.len();
                let content = result[start_pos + if_start.len()..start_pos + end_pos].to_string();
                
                let replacement = if value_exists { content } else { String::new() };
                result.replace_range(start_pos..full_end_pos, &replacement);
            } else {
                break;
            }
        }
        
        // {{#unless key}} ... {{/unless}} パターンを処理
        let unless_start = format!("{{{{#unless {}}}}}", key);
        let unless_end = "{{/unless}}";
        
        while let Some(start_pos) = result.find(&unless_start) {
            if let Some(end_pos) = result[start_pos..].find(unless_end) {
                let full_end_pos = start_pos + end_pos + unless_end.len();
                let content = result[start_pos + unless_start.len()..start_pos + end_pos].to_string();
                
                let replacement = if !value_exists { content } else { String::new() };
                result.replace_range(start_pos..full_end_pos, &replacement);
            } else {
                break;
            }
        }
        
        result
    }
    
    fn cleanup_conditional_blocks(&self, mut template: String) -> String {
        // 未処理の条件付きブロックを削除
        while let Some(start) = template.find("{{#") {
            if let Some(relative_end) = template[start..].find("{{/") {
                let end_start = start + relative_end;
                if let Some(relative_close) = template[end_start..].find("}}") {
                    let full_end = end_start + relative_close + 2;
                    if full_end <= template.len() {
                        template.drain(start..full_end);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        // 余分な空行を削除
        template = template.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n");
            
        // 3行以上の連続した空行を2行に制限
        while template.contains("\n\n\n") {
            template = template.replace("\n\n\n", "\n\n");
        }
        
        template.trim().to_string()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    #[error("Context service error: {0}")]
    ContextError(#[from] crate::services::context_service::ContextError),
    #[error("Template processing error: {0}")]
    ProcessingError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn create_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Create test tables
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
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }

    #[tokio::test]
    async fn test_template_initialization() {
        let pool = create_test_pool().await;
        let manager = EnhancedPromptManager::new(pool);
        
        let templates = manager.get_templates();
        assert!(!templates.is_empty());
        
        let task_template = manager.get_template("task_consultation");
        assert!(task_template.is_some());
    }

    #[tokio::test]
    async fn test_prompt_generation() {
        let pool = create_test_pool().await;
        let manager = EnhancedPromptManager::new(pool);
        
        let result = manager.generate_prompt("task_consultation").await;
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
        
        let generated = result.unwrap();
        assert_eq!(generated.template_id, "task_consultation");
        assert!(!generated.final_prompt.is_empty());
    }

    #[tokio::test]
    async fn test_conditional_block_processing() {
        let pool = create_test_pool().await;
        let manager = EnhancedPromptManager::new(pool);
        
        let template = "{{#if test_key}}Found{{/if}}{{#unless test_key}}Not found{{/unless}}";
        let _context_map: HashMap<String, String> = [("test_key".to_string(), "value".to_string())].iter().cloned().collect();
        
        let result = manager.process_conditional_blocks(template, "test_key", true);
        assert!(result.contains("Found"));
        assert!(!result.contains("Not found"));
    }

    #[tokio::test]
    async fn test_template_with_missing_context() {
        let pool = create_test_pool().await;
        let manager = EnhancedPromptManager::new(pool);
        
        let result = manager.generate_prompt("motivation_boost").await;
        if let Err(e) = &result {
            println!("Error in motivation test: {:?}", e);
        }
        assert!(result.is_ok());
        
        let generated = result.unwrap();
        println!("Generated prompt: {}", generated.final_prompt);
        assert!(!generated.final_prompt.contains("{{"));
        assert!(!generated.final_prompt.contains("}}"));
    }
}