use chrono::Utc;
use sqlx::{Pool, Sqlite};

use crate::error::AppError;
use crate::models::tag::{Tag, CreateTagRequest, UpdateTagRequest};

pub struct TagService;

impl TagService {
    /// すべてのタグを取得
    pub async fn get_all_tags(pool: &Pool<Sqlite>) -> Result<Vec<Tag>, AppError> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT id, name, color, created_at, updated_at FROM tags ORDER BY created_at ASC"
        )
        .fetch_all(pool)
        .await?;

        Ok(tags)
    }

    /// IDでタグを取得
    pub async fn get_tag_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Tag, AppError> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT id, name, color, created_at, updated_at FROM tags WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Tag with id {} not found", id)))?;

        Ok(tag)
    }

    /// 新しいタグを作成
    pub async fn create_tag(pool: &Pool<Sqlite>, request: CreateTagRequest) -> Result<Tag, AppError> {
        // 同じ名前のタグが存在するかチェック
        let existing_tag = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tags WHERE name = ?"
        )
        .bind(&request.name)
        .fetch_one(pool)
        .await?;

        if existing_tag > 0 {
            return Err(AppError::Validation(format!("Tag with name '{}' already exists", request.name)));
        }

        let tag = Tag::new(request.name, request.color);

        sqlx::query(
            "INSERT INTO tags (id, name, color, created_at, updated_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&tag.id)
        .bind(&tag.name)
        .bind(&tag.color)
        .bind(&tag.created_at)
        .bind(&tag.updated_at)
        .execute(pool)
        .await?;

        Ok(tag)
    }

    /// タグを更新
    pub async fn update_tag(
        pool: &Pool<Sqlite>, 
        id: &str, 
        request: UpdateTagRequest
    ) -> Result<Tag, AppError> {
        // タグが存在するかチェック
        let mut tag = Self::get_tag_by_id(pool, id).await?;

        // 更新するフィールドがあるかチェック
        if request.name.is_none() && request.color.is_none() {
            return Ok(tag); // 何も更新する必要がない
        }

        // 名前の重複チェック（名前を変更する場合）
        if let Some(ref new_name) = request.name {
            if new_name != &tag.name {
                let existing_tag = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM tags WHERE name = ? AND id != ?"
                )
                .bind(new_name)
                .bind(id)
                .fetch_one(pool)
                .await?;

                if existing_tag > 0 {
                    return Err(AppError::Validation(format!("Tag with name '{}' already exists", new_name)));
                }
            }
        }

        // フィールドを更新
        if let Some(name) = request.name {
            tag.name = name;
        }
        if let Some(color) = request.color {
            tag.color = color;
        }
        tag.updated_at = Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE tags SET name = ?, color = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&tag.name)
        .bind(&tag.color)
        .bind(&tag.updated_at)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(tag)
    }

    /// タグを削除
    pub async fn delete_tag(pool: &Pool<Sqlite>, id: &str) -> Result<(), AppError> {
        // タグが存在するかチェック
        let _ = Self::get_tag_by_id(pool, id).await?;

        // 関連するtask_tagsも自動削除される（CASCADE設定済み）
        let result = sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Tag with id {} not found", id)));
        }

        Ok(())
    }

    /// タスクにタグを追加
    pub async fn add_tag_to_task(pool: &Pool<Sqlite>, task_id: &str, tag_id: &str) -> Result<(), AppError> {
        // タスクとタグが存在するかチェック
        let task_exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM tasks WHERE id = ?"
        )
        .bind(task_id)
        .fetch_one(pool)
        .await?;

        if task_exists == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        let _ = Self::get_tag_by_id(pool, tag_id).await?; // タグの存在チェック

        // 既に関連付けられているかチェック
        let existing_relation = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM task_tags WHERE task_id = ? AND tag_id = ?"
        )
        .bind(task_id)
        .bind(tag_id)
        .fetch_one(pool)
        .await?;

        if existing_relation > 0 {
            return Ok(()); // 既に関連付けられている場合は何もしない
        }

        // 関連付けを作成
        sqlx::query(
            "INSERT INTO task_tags (task_id, tag_id, created_at) VALUES (?, ?, ?)"
        )
        .bind(task_id)
        .bind(tag_id)
        .bind(Utc::now().to_rfc3339())
        .execute(pool)
        .await?;

        Ok(())
    }

    /// タスクからタグを削除
    pub async fn remove_tag_from_task(pool: &Pool<Sqlite>, task_id: &str, tag_id: &str) -> Result<(), AppError> {
        let result = sqlx::query(
            "DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?"
        )
        .bind(task_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task-tag relation not found")));
        }

        Ok(())
    }

    /// タスクに付与されているタグを取得
    pub async fn get_tags_for_task(pool: &Pool<Sqlite>, task_id: &str) -> Result<Vec<Tag>, AppError> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT t.id, t.name, t.color, t.created_at, t.updated_at 
             FROM tags t 
             INNER JOIN task_tags tt ON t.id = tt.tag_id 
             WHERE tt.task_id = ? 
             ORDER BY t.created_at ASC"
        )
        .bind(task_id)
        .fetch_all(pool)
        .await?;

        Ok(tags)
    }
}