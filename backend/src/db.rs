use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect_to_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("❌ Failed to connect to the PostgreSQL database")
}
