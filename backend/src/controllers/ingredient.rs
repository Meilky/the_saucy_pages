use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    api::AppState,
    error::AppError,
    models::ingredient::{CreateIngredient, Ingredient},
};

pub async fn get_ingredients(
    State(app_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Ingredient>>, (axum::http::StatusCode, String)> {
    let ingredients = app_state
        .ingredient_service
        .list_ingredients()
        .await
        .map_err(|error| {
            tracing::error!("{:?}", error);

            match error {
                AppError::IngredientError(_i_err) => {
                    (axum::http::StatusCode::BAD_REQUEST, "error".into())
                }
                _ => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".into(),
                ),
            }
        })?;

    Ok(Json(ingredients))
}

pub async fn post_ingredient(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateIngredient>,
) -> Result<Json<Ingredient>, (axum::http::StatusCode, String)> {
    let ingredient = app_state
        .ingredient_service
        .create_ingredient(payload)
        .await
        .map_err(|error| {
            tracing::error!("{:?}", error);

            match error {
                AppError::IngredientError(_i_err) => {
                    (axum::http::StatusCode::BAD_REQUEST, "error".into())
                }
                _ => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".into(),
                ),
            }
        })?;

    Ok(Json(ingredient))
}
