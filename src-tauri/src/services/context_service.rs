use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use chrono::{DateTime, Utc, Local, Weekday, Duration, Datelike, Timelike};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Context collection failed: {0}")]
    CollectionError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextData {
    pub context_type: String,
    pub data: HashMap<String, String>,
}

impl ContextData {
    pub fn new(context_type: &str) -> Self {
        Self {
            context_type: context_type.to_string(),
            data: HashMap::new(),
        }
    }
    
    pub fn with(mut self, key: &str, value: String) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalContext {
    pub current_datetime: DateTime<Local>,
    pub utc_datetime: DateTime<Utc>,
    pub weekday: Weekday,
    pub is_business_day: bool,
    pub time_of_day: String,
    pub hour: u32,
    pub formatted_date: String,
    pub formatted_time: String,
    pub season: String,
}

impl TemporalContext {
    pub fn new() -> Self {
        let now_local = Local::now();
        let now_utc = now_local.with_timezone(&Utc);
        let weekday = now_local.weekday();
        
        Self {
            current_datetime: now_local,
            utc_datetime: now_utc,
            weekday,
            is_business_day: Self::is_business_day(weekday),
            time_of_day: Self::get_time_period(now_local.hour()),
            hour: now_local.hour(),
            formatted_date: now_local.format("%Y-%m-%d").to_string(),
            formatted_time: now_local.format("%H:%M:%S").to_string(),
            season: Self::get_season(now_local.month()),
        }
    }
    
    fn is_business_day(weekday: Weekday) -> bool {
        !matches!(weekday, Weekday::Sat | Weekday::Sun)
    }
    
    fn get_time_period(hour: u32) -> String {
        match hour {
            5..=11 => "morning".to_string(),
            12..=17 => "afternoon".to_string(),
            18..=22 => "evening".to_string(),
            _ => "night".to_string(),
        }
    }
    
    fn get_season(month: u32) -> String {
        match month {
            3..=5 => "spring".to_string(),
            6..=8 => "summer".to_string(),
            9..=11 => "autumn".to_string(),
            _ => "winter".to_string(),
        }
    }
    
    pub fn to_context_data(&self) -> ContextData {
        ContextData::new("temporal")
            .with("current_datetime", self.current_datetime.format("%Y-%m-%d %H:%M:%S").to_string())
            .with("weekday", format!("{:?}", self.weekday))
            .with("is_business_day", self.is_business_day.to_string())
            .with("time_of_day", self.time_of_day.clone())
            .with("hour", self.hour.to_string())
            .with("formatted_date", self.formatted_date.clone())
            .with("formatted_time", self.formatted_time.clone())
            .with("season", self.season.clone())
    }
    
    pub fn calculate_relative_date(&self, days_offset: i64) -> String {
        let target_date = self.current_datetime + Duration::days(days_offset);
        target_date.format("%Y-%m-%d").to_string()
    }
    
    pub fn calculate_business_days_ahead(&self, days: u32) -> String {
        let mut current = self.current_datetime;
        let mut business_days_count = 0;
        
        while business_days_count < days {
            current = current + Duration::days(1);
            if Self::is_business_day(current.weekday()) {
                business_days_count += 1;
            }
        }
        
        current.format("%Y-%m-%d").to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    pub total_tasks: i32,
    pub completed_today: i32,
    pub pending_tasks: i32,
    pub overdue_tasks: i32,
    pub completed_this_week: i32,
    pub average_completion_time: Option<f32>, // days
    pub most_common_tags: Vec<String>,
    pub current_workload_level: String, // "low", "medium", "high"
    pub tasks_due_today: i32,
    pub tasks_due_this_week: i32,
}

impl TaskContext {
    pub async fn build(db: &SqlitePool) -> Result<Self, ContextError> {
        let now = Utc::now();
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let week_start = today_start - Duration::days(now.weekday().num_days_from_monday() as i64);
        
        // 総タスク数
        let total_tasks: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM tasks")
            .fetch_one(db)
            .await?;
        
        // 今日完了したタスク数
        let completed_today: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE status = 'completed' AND DATE(updated_at) = DATE('now')"
        )
        .fetch_one(db)
        .await?;
        
        // ペンディングタスク数
        let pending_tasks: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE status IN ('todo', 'in_progress')"
        )
        .fetch_one(db)
        .await?;
        
        // 期限切れタスク数
        let overdue_tasks: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE due_date < DATE('now') AND status != 'completed'"
        )
        .fetch_one(db)
        .await?;
        
        // 今週完了したタスク数
        let completed_this_week: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE status = 'completed' AND updated_at >= ?"
        )
        .bind(week_start.to_rfc3339())
        .fetch_one(db)
        .await?;
        
        // 今日が期限のタスク数
        let tasks_due_today: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE DATE(due_date) = DATE('now') AND status != 'completed'"
        )
        .fetch_one(db)
        .await?;
        
        // 今週期限のタスク数
        let tasks_due_this_week: i32 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tasks WHERE due_date BETWEEN DATE('now') AND DATE('now', '+7 days') AND status != 'completed'"
        )
        .fetch_one(db)
        .await?;
        
        // よく使われるタグ (上位5つ)
        let most_common_tags: Vec<String> = sqlx::query_scalar::<_, String>(
            "SELECT tag FROM task_tags GROUP BY tag ORDER BY COUNT(*) DESC LIMIT 5"
        )
        .fetch_all(db)
        .await
        .unwrap_or_default();
        
        // ワークロードレベルを判定
        let current_workload_level = Self::calculate_workload_level(pending_tasks, tasks_due_this_week);
        
        Ok(Self {
            total_tasks,
            completed_today,
            pending_tasks,
            overdue_tasks,
            completed_this_week,
            average_completion_time: None, // TODO: 実装
            most_common_tags,
            current_workload_level,
            tasks_due_today,
            tasks_due_this_week,
        })
    }
    
    fn calculate_workload_level(pending_tasks: i32, due_this_week: i32) -> String {
        let workload_score = pending_tasks + (due_this_week * 2); // 今週期限は重み2倍
        
        match workload_score {
            0..=5 => "low".to_string(),
            6..=15 => "medium".to_string(),
            _ => "high".to_string(),
        }
    }
    
    pub fn to_context_data(&self) -> ContextData {
        ContextData::new("task")
            .with("total_tasks", self.total_tasks.to_string())
            .with("completed_today", self.completed_today.to_string())
            .with("pending_tasks", self.pending_tasks.to_string())
            .with("overdue_tasks", self.overdue_tasks.to_string())
            .with("completed_this_week", self.completed_this_week.to_string())
            .with("current_workload_level", self.current_workload_level.clone())
            .with("tasks_due_today", self.tasks_due_today.to_string())
            .with("tasks_due_this_week", self.tasks_due_this_week.to_string())
            .with("most_common_tags", self.most_common_tags.join(", "))
    }
}

pub struct ContextService {
    db: SqlitePool,
}

impl ContextService {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
    
    pub fn get_temporal_context(&self) -> TemporalContext {
        TemporalContext::new()
    }
    
    pub async fn get_task_context(&self) -> Result<TaskContext, ContextError> {
        TaskContext::build(&self.db).await
    }
    
    pub async fn collect_basic_context(&self) -> Result<Vec<ContextData>, ContextError> {
        let temporal = self.get_temporal_context();
        let task = self.get_task_context().await?;
        
        Ok(vec![
            temporal.to_context_data(),
            task.to_context_data(),
        ])
    }
    
    pub async fn collect_context_for_scope(&self, scope: &[&str]) -> Result<Vec<ContextData>, ContextError> {
        let mut contexts = Vec::new();
        
        for context_type in scope {
            match *context_type {
                "temporal" => {
                    let temporal = self.get_temporal_context();
                    contexts.push(temporal.to_context_data());
                },
                "task" => {
                    let task = self.get_task_context().await?;
                    contexts.push(task.to_context_data());
                },
                _ => {
                    // 未知のコンテキストタイプは無視
                    continue;
                }
            }
        }
        
        Ok(contexts)
    }
    
    pub fn context_to_prompt_variables(&self, contexts: &[ContextData]) -> HashMap<String, String> {
        let mut variables = HashMap::new();
        
        for context in contexts {
            for (key, value) in &context.data {
                let prefixed_key = format!("{}_{}", context.context_type, key);
                variables.insert(prefixed_key, value.clone());
            }
        }
        
        variables
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_temporal_context_creation() {
        let temporal = TemporalContext::new();
        assert!(!temporal.formatted_date.is_empty());
        assert!(!temporal.formatted_time.is_empty());
        assert!(temporal.hour < 24);
    }
    
    #[test]
    fn test_temporal_context_business_day() {
        assert!(TemporalContext::is_business_day(Weekday::Mon));
        assert!(TemporalContext::is_business_day(Weekday::Fri));
        assert!(!TemporalContext::is_business_day(Weekday::Sat));
        assert!(!TemporalContext::is_business_day(Weekday::Sun));
    }
    
    #[test]
    fn test_time_period_calculation() {
        assert_eq!(TemporalContext::get_time_period(9), "morning");
        assert_eq!(TemporalContext::get_time_period(14), "afternoon");
        assert_eq!(TemporalContext::get_time_period(19), "evening");
        assert_eq!(TemporalContext::get_time_period(2), "night");
    }
    
    #[test]
    fn test_context_data_creation() {
        let context = ContextData::new("test")
            .with("key1", "value1".to_string())
            .with("key2", "value2".to_string());
            
        assert_eq!(context.context_type, "test");
        assert_eq!(context.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.get("key2"), Some(&"value2".to_string()));
    }
    
    #[test]
    fn test_temporal_context_to_context_data() {
        let temporal = TemporalContext::new();
        let context_data = temporal.to_context_data();
        
        assert_eq!(context_data.context_type, "temporal");
        assert!(context_data.get("current_datetime").is_some());
        assert!(context_data.get("weekday").is_some());
        assert!(context_data.get("is_business_day").is_some());
        assert!(context_data.get("time_of_day").is_some());
    }
    
    #[tokio::test]
    async fn test_context_service_basic_functionality() {
        // メモリ内データベースでのテスト
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
        
        // テスト用のテーブルを作成
        sqlx::query(r#"
            CREATE TABLE tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                due_date TEXT,
                updated_at TEXT NOT NULL
            )
        "#).execute(&pool).await.unwrap();
        
        sqlx::query(r#"
            CREATE TABLE task_tags (
                task_id TEXT,
                tag TEXT,
                PRIMARY KEY (task_id, tag)
            )
        "#).execute(&pool).await.unwrap();
        
        // テストデータを挿入
        sqlx::query(r#"
            INSERT INTO tasks (id, title, status, due_date, updated_at) 
            VALUES 
                ('1', 'Test Task 1', 'todo', date('now', '+1 day'), datetime('now')),
                ('2', 'Test Task 2', 'completed', date('now'), datetime('now')),
                ('3', 'Test Task 3', 'todo', date('now', '-1 day'), datetime('now'))
        "#).execute(&pool).await.unwrap();
        
        sqlx::query(r#"
            INSERT INTO task_tags (task_id, tag) 
            VALUES 
                ('1', 'work'),
                ('2', 'personal'),
                ('3', 'urgent')
        "#).execute(&pool).await.unwrap();
        
        let service = ContextService::new(pool);
        
        // TemporalContextのテスト
        let temporal = service.get_temporal_context();
        assert!(!temporal.formatted_date.is_empty());
        
        // TaskContextのテスト
        let task_context = service.get_task_context().await.unwrap();
        assert_eq!(task_context.total_tasks, 3);
        assert_eq!(task_context.completed_today, 1);
        assert_eq!(task_context.pending_tasks, 2);
        assert_eq!(task_context.overdue_tasks, 1);
        
        // 基本コンテキスト収集のテスト
        let contexts = service.collect_basic_context().await.unwrap();
        assert_eq!(contexts.len(), 2); // temporal + task
        
        // スコープ指定コンテキスト収集のテスト
        let scoped_contexts = service.collect_context_for_scope(&["temporal"]).await.unwrap();
        assert_eq!(scoped_contexts.len(), 1);
        assert_eq!(scoped_contexts[0].context_type, "temporal");
        
        // プロンプト変数変換のテスト
        let variables = service.context_to_prompt_variables(&contexts);
        assert!(variables.contains_key("temporal_current_datetime"));
        assert!(variables.contains_key("temporal_weekday"));
        assert!(variables.contains_key("task_total_tasks"));
        assert_eq!(variables.get("task_total_tasks"), Some(&"3".to_string()));
    }
}