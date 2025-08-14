use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::{Task, TaskStatus, Priority};
use crate::error::AppError;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct MockDatabase {
    pub tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl MockDatabase {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_task(&self, mut task: Task) -> Result<Task, AppError> {
        if task.id.is_empty() {
            task.id = Uuid::new_v4().to_string();
        }
        
        let now = Utc::now().to_rfc3339();
        if task.created_at.is_empty() {
            task.created_at = now.clone();
        }
        task.updated_at = now;

        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task.id.clone(), task.clone());
        Ok(task)
    }

    pub fn get_task_by_id(&self, id: &str) -> Result<Task, AppError> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))
    }

    pub fn update_task(&self, id: &str, updated_task: Task) -> Result<Task, AppError> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(id) {
            let mut task = updated_task;
            task.updated_at = Utc::now().to_rfc3339();
            tasks.insert(id.to_string(), task.clone());
            Ok(task)
        } else {
            Err(AppError::NotFound(format!("Task with id {} not found", id)))
        }
    }

    pub fn delete_task(&self, id: &str) -> Result<(), AppError> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.remove(id).is_some() {
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Task with id {} not found", id)))
        }
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.values().cloned().collect()
    }

    pub fn clear(&self) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.clear();
    }
}

pub fn create_test_task_with_notifications() -> Task {
    Task {
        id: Uuid::new_v4().to_string(),
        title: "Test Notification Task".to_string(),
        description: Some("Testing notification settings".to_string()),
        status: "todo".to_string(),
        priority: "medium".to_string(),
        parent_id: None,
        due_date: None,
        completed_at: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        progress: Some(0),
        // Notification settings
        notification_type: Some("recurring".to_string()),
        notification_days_before: None,
        notification_time: Some("09:00".to_string()),
        notification_days_of_week: Some("[1,2,3,4,5]".to_string()),
        notification_level: Some(2),
    }
}

pub fn create_test_task_due_date_based() -> Task {
    Task {
        id: Uuid::new_v4().to_string(),
        title: "Test Due Date Task".to_string(),
        description: Some("Testing due date notification".to_string()),
        status: "todo".to_string(),
        priority: "high".to_string(),
        parent_id: None,
        due_date: Some("2025-12-31T23:59:59Z".to_string()),
        completed_at: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        progress: Some(0),
        // Notification settings
        notification_type: Some("due_date_based".to_string()),
        notification_days_before: Some(3),
        notification_time: Some("10:30".to_string()),
        notification_days_of_week: None,
        notification_level: Some(3),
    }
}