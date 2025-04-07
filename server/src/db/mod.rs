use bb8::Pool;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

pub mod schema;

pub type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_pool(database_url: &str) -> DbPool {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder()
        .build(config)
        .await
        .expect("Failed to create pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pool() {
        let pool = create_pool("postgres://localhost/test_db").await;
        assert!(pool.get().await.is_ok());
    }
}
