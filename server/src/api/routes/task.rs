use axum::{
    extract::{Path, State},
    routing::get,
    Router, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::db::DbPool;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskRequest {
    title: String,
    description: Option<String>,
}

pub(super) fn routes() -> Router<DbPool> {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task))
}

/// 获取所有任务列表
#[utoipa::path(
    get,
    path = "/api/v1/tasks",
    responses(
        (status = 200, description = "成功获取任务列表", body = Vec<Task>)
    )
)]
async fn list_tasks(
    State(_pool): State<DbPool>,
) -> Json<Vec<Task>> {
    let tasks = vec![
        Task {
            id: 1,
            title: "示例任务".to_string(),
            description: Some("这是一个示例任务".to_string()),
            status: TaskStatus::Todo,
        }
    ];
    Json(tasks)
}

/// 创建新任务
#[utoipa::path(
    post,
    path = "/api/v1/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "成功创建任务", body = Task)
    )
)]
async fn create_task(
    State(_pool): State<DbPool>,
    Json(payload): Json<CreateTaskRequest>,
) -> Json<Task> {
    let task = Task {
        id: 1,
        title: payload.title,
        description: payload.description,
        status: TaskStatus::Todo,
    };
    Json(task)
}

/// 获取特定任务
#[utoipa::path(
    get,
    path = "/api/v1/tasks/{id}",
    params(
        ("id" = i32, Path, description = "任务ID")
    ),
    responses(
        (status = 200, description = "成功获取任务", body = Task)
    )
)]
async fn get_task(
    State(_pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Json<Task> {
    let task = Task {
        id,
        title: "示例任务".to_string(),
        description: Some("这是一个示例任务".to_string()),
        status: TaskStatus::Todo,
    };
    Json(task)
}
