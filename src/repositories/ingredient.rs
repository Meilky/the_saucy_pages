use std::sync::Arc;

use crate::{
    error::AppError,
    models::{
        ingredient::{CreateIngredient, Ingredient},
        recipe::Recipe,
    },
};
use sqlx::{Pool, Sqlite};

pub trait IngredientRepository: Send + Sync {
    async fn create(&self, data: CreateIngredient) -> Result<Ingredient, AppError>;
    async fn find_all(&self) -> Result<Vec<Ingredient>, AppError>;
    async fn find_for_recipe(&self, recipe: &Recipe) -> Result<Vec<Ingredient>, AppError>;
}

#[derive(Debug, Clone)]
pub struct SQLiteIngredientRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl SQLiteIngredientRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl IngredientRepository for SQLiteIngredientRepository {
    async fn create(&self, data: CreateIngredient) -> Result<Ingredient, AppError> {
        let ingredient: Ingredient = data.into();

        sqlx::query(
            r#"
            INSERT INTO ingredients (uuid, name, description)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(ingredient.uuid)
        .bind(ingredient.name.clone())
        .bind(ingredient.description.clone())
        .execute(&*self.pool)
        .await?;

        Ok(ingredient)
    }

    async fn find_all(&self) -> Result<Vec<Ingredient>, AppError> {
        sqlx::query_as::<_, Ingredient>("SELECT uuid, name, description FROM ingredients;")
            .fetch_all(&*self.pool)
            .await
            .map_err(|_e| AppError::InternalServerError)
    }

    async fn find_for_recipe(&self, recipe: &Recipe) -> Result<Vec<Ingredient>, AppError> {
        sqlx::query_as::<_, Ingredient>("SELECT uuid, name, description FROM ingredients WHERE uuid IN (SELECT uuid_ingredient FROM recipes_ingredients WHERE uuid_recipe = ?);")
            .bind(recipe.uuid)
            .fetch_all(&*self.pool)
            .await
            .map_err(|_e| AppError::InternalServerError)
    }
}
