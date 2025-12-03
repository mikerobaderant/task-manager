use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    pub title: String,
}

impl Task {
    /// Fetch all tasks from the database
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT id, title, completed, created_at FROM tasks ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Create a new task
    pub async fn create(pool: &SqlitePool, title: &str) -> Result<Task, sqlx::Error> {
        let result = sqlx::query("INSERT INTO tasks (title) VALUES (?)")
            .bind(title)
            .execute(pool)
            .await?;

        let id = result.last_insert_rowid();

        sqlx::query_as::<_, Task>("SELECT id, title, completed, created_at FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    /// Mark a task as completed
    pub async fn complete(pool: &SqlitePool, id: i64) -> Result<Option<Task>, sqlx::Error> {
        let rows_affected = sqlx::query("UPDATE tasks SET completed = TRUE WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Ok(None);
        }

        sqlx::query_as::<_, Task>("SELECT id, title, completed, created_at FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Toggle the completed state of a task
    pub async fn toggle(pool: &SqlitePool, id: i64) -> Result<Option<Task>, sqlx::Error> {
        let rows_affected = sqlx::query("UPDATE tasks SET completed = NOT completed WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Ok(None);
        }

        sqlx::query_as::<_, Task>("SELECT id, title, completed, created_at FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Update a task's title
    pub async fn update(pool: &SqlitePool, id: i64, title: &str) -> Result<Option<Task>, sqlx::Error> {
        let rows_affected = sqlx::query("UPDATE tasks SET title = ? WHERE id = ?")
            .bind(title)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Ok(None);
        }

        sqlx::query_as::<_, Task>("SELECT id, title, completed, created_at FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// Delete a task
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }
}

