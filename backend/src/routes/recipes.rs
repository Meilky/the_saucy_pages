use axum::{
    Json,
    extract::{Extension, Path},
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::recipe::{CreateRecipe, Recipe};

pub async fn get_recipes(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Vec<Recipe>>, (axum::http::StatusCode, String)> {
    let recipes = sqlx::query_as::<_, Recipe>("SELECT uuid, name, description FROM recipes;")
        .fetch_all(&pool)
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
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateRecipe>,
) -> Result<Json<Recipe>, (axum::http::StatusCode, String)> {
    let recipe: Recipe = payload.into();

    sqlx::query!(
        "INSERT INTO recipes (uuid, name, description) VALUES (?, ?, ?);",
        recipe.uuid,
        recipe.name,
        recipe.description
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {:?}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".into(),
        )
    })?;

    Ok(Json(recipe))
}

pub async fn delete_recipe(
    Path(uuid): Path<Uuid>,
    Extension(pool): Extension<SqlitePool>,
) -> axum::http::StatusCode {
    let exist = sqlx::query!("SELECT uuid FROM recipes WHERE uuid = ?;", uuid)
        .fetch_optional(&pool)
        .await;

    if let Err(e) = exist {
        tracing::error!("DB error: {:?}", e);
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
    }

    let option_exist = exist.unwrap();

    if option_exist.is_none() {
        return axum::http::StatusCode::NOT_FOUND;
    }

    let delete = sqlx::query!("DELETE FROM recipes WHERE uuid = ?;", uuid)
        .execute(&pool)
        .await;

    if let Err(e) = delete {
        tracing::error!("DB error: {:?}", e);
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
    }

    axum::http::StatusCode::NO_CONTENT
}
