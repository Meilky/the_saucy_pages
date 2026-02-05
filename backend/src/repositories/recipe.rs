use crate::{
    error::AppError,
    models::recipe::{CreateRecipe, Recipe},
};
use sqlx::{Pool, Sqlite};

pub struct RecipeRepository {
    pool: Pool<Sqlite>,
}

impl RecipeRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        let recipe: Recipe = data.into();

        sqlx::query(
            r#"
            INSERT INTO recipes (uuid, name, description)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(recipe.uuid)
        .bind(recipe.name.clone())
        .bind(recipe.description.clone())
        .execute(&self.pool)
        .await?;

        Ok(recipe)
    }

    pub async fn find_all(&self) -> Result<Vec<Recipe>, AppError> {
        sqlx::query_as::<_, Recipe>("SELECT uuid, name, description FROM recipes;")
            .fetch_all(&self.pool)
            .await
            .map_err(|_e| AppError::InternalServerError)
    }
}
