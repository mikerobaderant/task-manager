use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Json,
};
use sqlx::SqlitePool;

use crate::models::{CreateTask, Task, UpdateTask};

// Askama template for the index page
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub tasks: Vec<Task>,
}

// Custom error type for better error handling
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// GET / - Render the main page with all tasks
pub async fn index(State(pool): State<SqlitePool>) -> Result<Html<String>, AppError> {
    let tasks = Task::get_all(&pool).await?;
    let template = IndexTemplate { tasks };
    let html = template.render().map_err(|e| anyhow::anyhow!("Template error: {}", e))?;
    Ok(Html(html))
}

/// GET /tasks - Return all tasks as JSON
pub async fn list_tasks(State(pool): State<SqlitePool>) -> Result<Json<Vec<Task>>, AppError> {
    let tasks = Task::get_all(&pool).await?;
    Ok(Json(tasks))
}

/// POST /tasks - Create a new task
pub async fn create_task(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    let task = Task::create(&pool, &payload.title).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// POST /tasks/:id/complete - Mark a task as completed
pub async fn complete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    match Task::complete(&pool, id).await? {
        Some(task) => Ok(Json(task).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Task not found").into_response()),
    }
}

/// POST /tasks/:id/toggle - Toggle a task's completed state
pub async fn toggle_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    match Task::toggle(&pool, id).await? {
        Some(task) => Ok(Json(task).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Task not found").into_response()),
    }
}

/// PUT /tasks/:id - Update a task's title
pub async fn update_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Response, AppError> {
    match Task::update(&pool, id, &payload.title).await? {
        Some(task) => Ok(Json(task).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Task not found").into_response()),
    }
}

/// DELETE /tasks/:id - Delete a task
pub async fn delete_task(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Response, AppError> {
    if Task::delete(&pool, id).await? {
        Ok(StatusCode::NO_CONTENT.into_response())
    } else {
        Ok((StatusCode::NOT_FOUND, "Task not found").into_response())
    }
}
