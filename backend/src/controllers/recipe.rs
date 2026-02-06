use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    api::AppState, error::AppError, models::recipe::{CreateRecipe, Recipe}
};

pub async fn get_recipes(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Recipe>>, AppError> {
    let recipes = app_state.recipe_service.list_recipes().await?;

    Ok(Json(recipes))
}

pub async fn post_recipe(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateRecipe>,
) -> Result<Json<Recipe>, AppError> {
    let recipe = app_state.recipe_service.create_recipe(payload).await?;

    Ok(Json(recipe))
}
