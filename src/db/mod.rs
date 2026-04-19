use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::{models::*, error::*};
use chrono::Utc;

pub async fn init_db(database_url: &str) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(pool)
}

pub async fn create_task(
    pool: &PgPool,
    req: CreateTaskRequest,
) -> AppResult<Task> {
    let now = Utc::now();
    
    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (title, categorie, description, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, title, categorie, description, progress, achieved, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.categorie)
    .bind(&req.description)
    .bind(now)
    .bind(now)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(task)
}

pub async fn list_tasks(pool: &PgPool) -> AppResult<Vec<Task>> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, title, categorie, description, progress, achieved, created_at, updated_at
         FROM tasks ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(tasks)
}

pub async fn get_task(pool: &PgPool, id: i32) -> AppResult<Task> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, title, categorie, description, progress, achieved, created_at, updated_at
         FROM tasks WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?
    .ok_or(AppError::TaskNotFound)?;

    Ok(task)
}

pub async fn update_task(
    pool: &PgPool,
    id: i32,
    req: UpdateTaskRequest,
) -> AppResult<Task> {
    let _ = get_task(pool, id).await?;
    let now = Utc::now();
    let task = sqlx::query_as::<_, Task>(
        "UPDATE tasks 
         SET title = COALESCE($1, title),
             categorie = COALESCE($2, categorie),
             description = COALESCE($3, description),
             progress = COALESCE($4, progress),
             achieved = COALESCE($5, achieved),
             updated_at = $6
         WHERE id = $7
         RETURNING id, title, categorie, description, progress, achieved, created_at, updated_at"
    )
    .bind(&req.title)
    .bind(&req.categorie)
    .bind(&req.description)
    .bind(req.progress)
    .bind(req.achieved)
    .bind(now)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(task)
}

pub async fn delete_task(pool: &PgPool, id: i32) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    if result.rows_affected() == 0 {
        return Err(AppError::TaskNotFound);
    }
    Ok(())
}