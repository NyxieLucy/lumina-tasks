use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{models::*, error::*, db};
use sqlx::PgPool;

/// Handler: List all tasks
pub async fn list_tasks(
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<Task>>> {
    let tasks = db::list_tasks(&pool).await?;
    Ok(Json(tasks))
}

pub async fn get_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<Json<Task>> {
    let task = db::get_task(&pool, id).await?;
    Ok(Json(task))
}
/// Handler: Create a new task
pub async fn create_task(
    State(pool): State<PgPool>,
    Json(req): Json<CreateTaskRequest>,
) -> AppResult<(StatusCode, Json<Task>)> {
    let task = db::create_task(&pool, req).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Handler: Update a task
pub async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateTaskRequest>,
) -> AppResult<Json<Task>> {
    let task = db::update_task(&pool, id, req).await?;
    Ok(Json(task))
}

/// Handler: Delete a task
pub async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<()> {
    db::delete_task(&pool, id).await?;
    Ok(())
}