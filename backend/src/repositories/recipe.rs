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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn create_pool() -> Pool<Sqlite> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(":memory:")
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        pool
    }

    async fn create_repo() -> RecipeRepository {
        let pool = create_pool().await;

        RecipeRepository::new(pool)
    }

    #[tokio::test]
    async fn find_all_return_empty_db() {
        let repo = create_repo().await;

        let result = repo.find_all().await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.len(), 0);
    }

    #[tokio::test]
    async fn insert_return_the_new_recipe() {
        let repo = create_repo().await;

        let recipe_name = "test name".to_string();
        let recipe_description = "test description".to_string();

        let result = repo
            .create(CreateRecipe {
                name: recipe_name.clone(),
                description: recipe_description.clone(),
            })
            .await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.name, recipe_name);
        assert_eq!(data.description, recipe_description);
    }

    #[tokio::test]
    async fn find_all_should_return_one_recipe_after_creation() {
        let repo = create_repo().await;

        let recipe_name = "test name".to_string();
        let recipe_description = "test description".to_string();

        repo.create(CreateRecipe {
            name: recipe_name.clone(),
            description: recipe_description.clone(),
        })
        .await
        .unwrap();

        let result = repo.find_all().await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.len(), 1);
    }
}
