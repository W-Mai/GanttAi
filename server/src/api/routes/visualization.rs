use axum::{Router, routing::get, extract::State};
use crate::db::DbPool;

pub(super) fn routes() -> Router<DbPool> {
    Router::new()
        .route("/gantt", get(get_gantt_data))
}

async fn get_gantt_data(State(_pool): State<DbPool>) -> &'static str {
    "Get Gantt chart data"
}