use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    api::AppState,
    models::recipe::{CreateRecipe, Recipe},
};

pub async fn get_recipes(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Recipe>>, (axum::http::StatusCode, String)> {
    let recipes = app_state
        .recipe_service
        .list_recipes()
        .await
        .map_err(|error| {
            tracing::error!("DB error: {:?}", error);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".into(),
            )
        })?;

    Ok(Json(recipes))
}

pub async fn post_recipe(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateRecipe>,
) -> Result<Json<Recipe>, (axum::http::StatusCode, String)> {
    let recipe = app_state
        .recipe_service
        .create_recipe(payload)
        .await
        .map_err(|error| {
            tracing::error!("DB error: {:?}", error);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".into(),
            )
        })?;

    Ok(Json(recipe))
}
