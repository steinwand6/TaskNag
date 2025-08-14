use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::tag::Tag;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "TEXT", rename_all = "snake_case")]
pub enum TaskStatus {
    Inbox,
    Todo,
    InProgress,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Inbox => write!(f, "inbox"),
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inbox" => Ok(TaskStatus::Inbox),
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

// Priority enum REMOVED as per .kiro/specs/notification-system-redesign
// Individual notification settings replace the priority system

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskNotificationSettings {
    pub notification_type: String,           // 'none', 'due_date_based', 'recurring'
    pub days_before: Option<i32>,            // 期日何日前から
    pub notification_time: Option<String>,   // HH:MM形式
    pub days_of_week: Option<Vec<i32>>,      // 0=日曜, 1=月曜...
    pub level: i32,                          // 1, 2, 3
}

impl Default for TaskNotificationSettings {
    fn default() -> Self {
        Self {
            notification_type: "none".to_string(),
            days_before: None,
            notification_time: None,
            days_of_week: None,
            level: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskNotification {
    pub task_id: String,
    pub title: String,
    pub level: i32,
    pub days_until_due: Option<i64>,
    pub notification_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    // priority field REMOVED as per .kiro/specs/notification-system-redesign
    pub parent_id: Option<String>,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub progress: Option<i32>,
    // Notification settings fields (as per .kiro spec)
    pub notification_type: Option<String>,        // 'none', 'due_date_based', 'recurring'
    pub notification_days_before: Option<i32>,   // 期日何日前から
    pub notification_time: Option<String>,       // HH:MM形式
    pub notification_days_of_week: Option<String>, // JSON配列 "[0,1,2]"
    pub notification_level: Option<i32>,         // 1, 2, 3
    // Tag system
    #[sqlx(skip)]
    pub tags: Option<Vec<Tag>>,
}

impl Task {
    pub fn new(title: String, description: Option<String>, status: TaskStatus) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            status: status.to_string(),
            // priority field removed
            parent_id: None,
            due_date: None,
            completed_at: None,
            created_at: now.clone(),
            updated_at: now,
            progress: Some(0),
            // Default notification settings (as per .kiro spec)
            notification_type: Some("none".to_string()),
            notification_days_before: None,
            notification_time: None,
            notification_days_of_week: None,
            notification_level: Some(1),
            // Tag system
            tags: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    // priority field removed as per .kiro/specs/notification-system-redesign
    pub parent_id: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    // Notification settings (replaces priority system)
    pub notification_settings: Option<TaskNotificationSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    // priority field removed as per .kiro/specs/notification-system-redesign
    pub parent_id: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    // Notification settings (replaces priority system)
    pub notification_settings: Option<TaskNotificationSettings>,
    pub tags: Option<Vec<Tag>>,
}