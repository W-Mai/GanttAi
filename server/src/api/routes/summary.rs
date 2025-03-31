use axum::{Router, routing::get, extract::State};
use crate::db::DbPool;

pub(super) fn routes() -> Router<DbPool> {
    Router::new()
        .route("/summary/:period", get(get_summary))
}

async fn get_summary(State(_pool): State<DbPool>) -> &'static str {
    "Get summary"
} 