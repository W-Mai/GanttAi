use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::db::DbPool;

mod task;
mod analysis;
mod visualization;
mod summary;

#[derive(OpenApi)]
#[openapi(
    paths(
        task::list_tasks,
        task::create_task,
        task::get_task,
    ),
    components(
        schemas(
            task::Task,
            task::TaskStatus,
            task::CreateTaskRequest,
        )
    ),
    tags(
        (name = "tasks", description = "任务管理 API")
    )
)]
struct ApiDoc;

pub fn create_routes() -> Router<DbPool> {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1", api_routes())
}

fn api_routes() -> Router<DbPool> {
    Router::new()
        .merge(task::routes())
        .merge(analysis::routes())
        .merge(visualization::routes())
        .merge(summary::routes())
}
