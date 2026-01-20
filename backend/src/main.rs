use std::sync::Arc;

use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    api::AppState, repositories::recipe::RecipeRepository, services::recipe::RecipeService,
};

mod api;
mod controllers;
mod models;
mod repositories;
mod services;

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

    let recipe_repository = RecipeRepository::new(sqlite_pool);
    let recipe_service = RecipeService::new(recipe_repository);

    let app_state: Arc<AppState> = Arc::new(AppState { recipe_service });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5055").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, api::get_api(app_state)).await;
}
