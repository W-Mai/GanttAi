use std::net::{SocketAddr, IpAddr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::Settings;

mod api;
mod core;
mod db;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let settings = Settings::new().expect("Failed to load settings");

    // 初始化数据库连接池
    let pool = db::create_pool(&settings.database_url).await;

    // 构建应用路由
    let app = api::routes::create_routes()
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(pool);

    // 启动服务器
    let addr = SocketAddr::from((
        settings.server_host.parse::<IpAddr>().unwrap(),
        settings.server_port,
    ));
    tracing::info!("Server listening on {}", addr);
    tracing::info!("API documentation available at /swagger-ui");

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
