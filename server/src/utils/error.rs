use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.clone()),
            AppError::BadRequest(ref message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::Internal(ref message) => (StatusCode::INTERNAL_SERVER_ERROR, message.clone()),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::Response;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn test_app_error_response() {
        let error = AppError::NotFound("Task not found".to_string());
        let response: Response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(body_str.contains("Task not found"));
    }
}