use std::sync::Arc;

use crate::{
    error::AppError,
    models::ingredient::{CreateIngredient, Ingredient},
};
use sqlx::{Pool, Sqlite};

pub trait IngredientRepository: Send + Sync {
    async fn create(&self, data: CreateIngredient) -> Result<Ingredient, AppError>;
    async fn find_all(&self) -> Result<Vec<Ingredient>, AppError>;
}

pub struct ImplIngredientRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl ImplIngredientRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl IngredientRepository for ImplIngredientRepository {
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

    #[tokio::test]
    async fn find_all_return_empty_db() {
        let pool = create_pool().await;
        let repo = ImplIngredientRepository::new(Arc::new(pool));

        let result = repo.find_all().await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.len(), 0);
    }

    #[tokio::test]
    async fn insert_return_the_new_ingredient() {
        let pool = create_pool().await;
        let repo = ImplIngredientRepository::new(Arc::new(pool));

        let ingredient_name = "test name".to_string();
        let ingredient_description = Some("test description".to_string());

        let result = repo
            .create(CreateIngredient {
                name: ingredient_name.clone(),
                description: ingredient_description.clone(),
            })
            .await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.name, ingredient_name);
        assert!(data.description.is_some());
        assert_eq!(data.description.unwrap(), ingredient_description.unwrap());
    }

    #[tokio::test]
    async fn insert_return_the_new_ingredient_without_description() {
        let pool = create_pool().await;
        let repo = ImplIngredientRepository::new(Arc::new(pool));

        let ingredient_name = "test name".to_string();
        let ingredient_description: Option<String> = None;

        let result = repo
            .create(CreateIngredient {
                name: ingredient_name.clone(),
                description: ingredient_description.clone(),
            })
            .await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.name, ingredient_name);
        assert!(data.description.is_none());
    }

    #[tokio::test]
    async fn find_all_should_return_one_ingredient_after_creation() {
        let pool = create_pool().await;
        let repo = ImplIngredientRepository::new(Arc::new(pool));

        let ingredient_name = "test name".to_string();
        let ingredient_description = Some("test description".to_string());

        repo.create(CreateIngredient {
            name: ingredient_name.clone(),
            description: ingredient_description.clone(),
        })
        .await
        .unwrap();

        let result = repo.find_all().await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.len(), 1);
    }
}
