use axum::{Router, routing::post, extract::State};
use crate::db::DbPool;

pub(super) fn routes() -> Router<DbPool> {
    Router::new()
        .route("/analysis", post(analyze_content))
}

async fn analyze_content(State(_pool): State<DbPool>) -> &'static str {
    "Analyze content"
} 