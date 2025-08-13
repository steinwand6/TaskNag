use crate::database::Database;
use crate::error::AppError;
use crate::models::{CreateTaskRequest, Task, UpdateTaskRequest};
use chrono::Utc;
use uuid::Uuid;

pub struct TaskService {
    db: Database,
}

impl TaskService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task, AppError> {
        let now = Utc::now().to_rfc3339();
        let id = Uuid::new_v4().to_string();
        
        let task = Task {
            id: id.clone(),
            title: request.title,
            description: request.description,
            status: request.status.to_string(),
            priority: request.priority.to_string(),
            parent_id: request.parent_id,
            due_date: request.due_date.map(|d| d.to_rfc3339()),
            completed_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        
        sqlx::query(
            r#"
            INSERT INTO tasks (id, title, description, status, priority, parent_id, due_date, completed_at, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.parent_id)
        .bind(&task.due_date)
        .bind(&task.completed_at)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .execute(&self.db.pool)
        .await?;
        
        Ok(task)
    }
    
    pub async fn get_tasks(&self) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, priority, parent_id, due_date, completed_at, created_at, updated_at
            FROM tasks
            ORDER BY 
                CASE status 
                    WHEN 'inbox' THEN 1
                    WHEN 'todo' THEN 2
                    WHEN 'in_progress' THEN 3
                    WHEN 'done' THEN 4
                END,
                CASE priority
                    WHEN 'urgent' THEN 1
                    WHEN 'high' THEN 2
                    WHEN 'medium' THEN 3
                    WHEN 'low' THEN 4
                END,
                created_at DESC
            "#,
        )
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(tasks)
    }
    
    pub async fn get_task_by_id(&self, id: &str) -> Result<Task, AppError> {
        let task = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, priority, parent_id, due_date, completed_at, created_at, updated_at
            FROM tasks
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;
        
        task.ok_or_else(|| AppError::NotFound(format!("Task with id {} not found", id)))
    }
    
    pub async fn update_task(&self, id: &str, request: UpdateTaskRequest) -> Result<Task, AppError> {
        // Get existing task first
        let mut task = self.get_task_by_id(id).await?;
        
        // Update fields if provided
        if let Some(title) = request.title {
            task.title = title;
        }
        if let Some(description) = request.description {
            task.description = Some(description);
        }
        if let Some(status) = request.status {
            task.status = status.to_string();
            // Set completed_at if status is Done
            if task.status == "done" {
                task.completed_at = Some(Utc::now().to_rfc3339());
            } else {
                task.completed_at = None;
            }
        }
        if let Some(priority) = request.priority {
            task.priority = priority.to_string();
        }
        if request.parent_id.is_some() {
            task.parent_id = request.parent_id;
        }
        if let Some(due_date) = request.due_date {
            task.due_date = Some(due_date.to_rfc3339());
        }
        
        task.updated_at = Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            UPDATE tasks
            SET title = ?2, description = ?3, status = ?4, priority = ?5, 
                parent_id = ?6, due_date = ?7, completed_at = ?8, updated_at = ?9
            WHERE id = ?1
            "#,
        )
        .bind(&task.id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(&task.priority)
        .bind(&task.parent_id)
        .bind(&task.due_date)
        .bind(&task.completed_at)
        .bind(&task.updated_at)
        .execute(&self.db.pool)
        .await?;
        
        Ok(task)
    }
    
    pub async fn delete_task(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(id)
            .execute(&self.db.pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", id)));
        }
        
        Ok(())
    }
    
    pub async fn get_tasks_by_status(&self, status: &str) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, title, description, status, priority, parent_id, due_date, completed_at, created_at, updated_at
            FROM tasks
            WHERE status = ?1
            ORDER BY 
                CASE priority
                    WHEN 'urgent' THEN 1
                    WHEN 'high' THEN 2
                    WHEN 'medium' THEN 3
                    WHEN 'low' THEN 4
                END,
                created_at DESC
            "#,
        )
        .bind(status)
        .fetch_all(&self.db.pool)
        .await?;
        
        Ok(tasks)
    }
    
    pub async fn move_task(&self, id: &str, new_status: &str) -> Result<Task, AppError> {
        use std::str::FromStr;
        use crate::models::TaskStatus;
        
        let status = TaskStatus::from_str(new_status)
            .map_err(|e| AppError::InvalidInput(e))?;
        
        self.update_task(id, UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(status),
            priority: None,
            parent_id: None,
            due_date: None,
        }).await
    }
}