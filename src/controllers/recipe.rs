use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    api::HttpAppState,
    error::AppError,
    models::{
        ingredient::Ingredient,
        recipe::{CreateRecipe, Recipe},
    },
    services::recipe::RecipeService,
};

pub async fn get_recipes(
    State(app_state): State<Arc<HttpAppState>>,
) -> Result<Json<Vec<Recipe>>, AppError> {
    let recipes = app_state.recipe_service.list_recipes().await?;

    Ok(Json(recipes))
}

pub async fn get_recipe_by_uuid(
    Path(uuid): Path<Uuid>,
    State(app_state): State<Arc<HttpAppState>>,
) -> Result<Json<Recipe>, AppError> {
    let recipe = app_state.recipe_service.find_recipe_by_uuid(uuid).await?;

    Ok(Json(recipe))
}

pub async fn get_ingredients_for_recipe_by_uuid(
    Path(uuid): Path<Uuid>,
    State(app_state): State<Arc<HttpAppState>>,
) -> Result<Json<Vec<Ingredient>>, AppError> {
    let ingredients = app_state
        .recipe_service
        .list_ingredients_for_recipe_by_uuid(uuid)
        .await?;

    Ok(Json(ingredients))
}

pub async fn post_recipe(
    State(app_state): State<Arc<HttpAppState>>,
    Json(payload): Json<CreateRecipe>,
) -> Result<Json<Recipe>, AppError> {
    let recipe = app_state.recipe_service.create_recipe(payload).await?;

    Ok(Json(recipe))
}
