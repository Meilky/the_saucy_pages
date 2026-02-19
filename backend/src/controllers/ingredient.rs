use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    api::HttpAppState,
    error::AppError,
    models::ingredient::{CreateIngredient, Ingredient},
};

pub async fn get_ingredients(
    State(app_state): State<Arc<HttpAppState>>,
) -> Result<Json<Vec<Ingredient>>, AppError> {
    let ingredients = app_state.ingredient_service.list_ingredients().await?;

    Ok(Json(ingredients))
}

pub async fn post_ingredient(
    State(app_state): State<Arc<HttpAppState>>,
    Json(payload): Json<CreateIngredient>,
) -> Result<Json<Ingredient>, AppError> {
    let ingredient = app_state
        .ingredient_service
        .create_ingredient(payload)
        .await?;

    Ok(Json(ingredient))
}
