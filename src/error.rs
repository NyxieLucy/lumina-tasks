use axum::{
    Json, http::{StatusCode}, response::{IntoResponse, Response}
};
use serde_json::{self, json};
use std::fmt::{self};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    TaskNotFound,
    ValidationError(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "DATABASE ERROR: {}", msg),
            AppError::InternalError(msg) => write!(f, "INTERNAL ERROR: {}", msg),
            AppError::TaskNotFound => write!(f, "TASK NOT FOUND"),
            AppError::ValidationError(msg) => write!(f, "VALIDATION ERROR: {}", msg)
        }
    }
}
impl std::error::Error for AppError {}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::TaskNotFound => (StatusCode::NOT_FOUND, "TASK NOT FOUND".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };
        let body = Json(json!(
            {
                "error": error_message,
                "status": status.as_u16(),
            }
        ));
        (status, body).into_response()
    }
}
pub type AppResult<T>  = Result<T, AppError>;