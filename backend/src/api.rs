use axum::{
    Extension, Router,
    routing::{delete, get},
};
use sqlx::{Pool, Sqlite};

use crate::routes::recipes;

pub fn get_api(pool: Pool<Sqlite>) -> Router {
    Router::new()
        .route(
            "/api/recipes",
            get(recipes::get_recipes).post(recipes::post_recipe),
        )
        .route("/api/recipes/{uuid}", delete(recipes::delete_recipe))
        .layer(Extension(pool))
}
