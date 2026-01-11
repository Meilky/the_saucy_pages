use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

mod api;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or shell");

    let sqlite_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5055").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, api::get_api(sqlite_pool)).await;
}
