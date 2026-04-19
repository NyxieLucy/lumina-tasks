use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub categorie: String,
    pub description: String,
    pub progress: i16,
    pub achieved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskRequest {
    pub title: String,
    pub categorie: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub categorie: Option<String>,
    pub description: Option<String>,
    pub progress: Option<i16>,
    pub achieved: Option<bool>,
}