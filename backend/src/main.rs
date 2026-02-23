use std::sync::Arc;

use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

use tokio::signal;

use crate::{
    api::HttpAppState,
    repositories::{ingredient::ImplIngredientRepository, recipe::ImplRecipeRepository},
    services::{ingredient::ImplIngredientService, recipe::ImplRecipeService},
};

mod api;
mod controllers;
mod error;
mod models;
mod repositories;
mod services;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

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

    sqlx::migrate!().run(&sqlite_pool).await.unwrap();

    let arc_pool = Arc::new(sqlite_pool);

    let ingredient_repository = ImplIngredientRepository::new(Arc::clone(&arc_pool));
    let recipe_repository = ImplRecipeRepository::new(Arc::clone(&arc_pool));

    let ingredient_service = ImplIngredientService::new(ingredient_repository);
    let recipe_service = ImplRecipeService::new(recipe_repository);

    let app_state = Arc::new(HttpAppState {
        recipe_service,
        ingredient_service,
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5055").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, api::get_api(app_state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    arc_pool.close().await;
}
