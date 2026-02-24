use std::sync::Arc;

use crate::{
    error::AppError,
    models::recipe::{CreateRecipe, Recipe, RecipeIngredient},
};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone)]
struct RecipeIngredientDTO {
    pub uuid_ingredient: Uuid,
    pub amount: f32,
}

impl Into<RecipeIngredient> for RecipeIngredientDTO {
    fn into(self) -> RecipeIngredient {
        RecipeIngredient {
            uuid: self.uuid_ingredient,
            amount: self.amount,
        }
    }
}

pub trait RecipeRepository: Send + Sync {
    async fn create(&self, data: CreateRecipe) -> Result<Recipe, AppError>;
    async fn find_all(&self) -> Result<Vec<Recipe>, AppError>;
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError>;
}

pub struct ImplRecipeRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl ImplRecipeRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl RecipeRepository for ImplRecipeRepository {
    async fn create(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        let recipe: Recipe = data.into();

        sqlx::query(
            r#"
            INSERT INTO recipes (uuid, name, description)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(recipe.uuid.clone())
        .bind(recipe.name.clone())
        .bind(recipe.description.clone())
        .execute(&*self.pool)
        .await?;

        for ingredient in recipe.ingredients.iter() {
            sqlx::query(
                r#"
                INSERT INTO recipes_ingredients (uuid_recipe, uuid_ingredient, amount)
                VALUES (?, ?, ?);
                "#,
            )
            .bind(recipe.uuid.clone())
            .bind(ingredient.uuid.clone())
            .bind(ingredient.amount.clone())
            .execute(&*self.pool)
            .await?;
        }

        Ok(recipe)
    }

    async fn find_all(&self) -> Result<Vec<Recipe>, AppError> {
        let mut recipes =
            sqlx::query_as::<_, Recipe>("SELECT uuid, name, description FROM recipes;")
                .fetch_all(&*self.pool)
                .await?;

        for recipe in recipes.iter_mut() {
            let ingredients_dto = sqlx::query_as::<_, RecipeIngredientDTO>(
                "SELECT uuid_ingredient, amount FROM recipes_ingredients;",
            )
            .fetch_all(&*self.pool)
            .await?;

            recipe.ingredients = ingredients_dto.iter().map(|v| v.clone().into()).collect();
        }

        Ok(recipes)
    }

    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError> {
        let mut recipe = sqlx::query_as::<_, Recipe>(
            "SELECT uuid, name, description FROM recipes WHERE uuid = ?;",
        )
        .bind(uuid)
        .fetch_one(&*self.pool)
        .await?;

        let ingredients_dto = sqlx::query_as::<_, RecipeIngredientDTO>(
            "SELECT uuid_ingredient, amount FROM recipes_ingredients;",
        )
        .fetch_all(&*self.pool)
        .await?;

        recipe.ingredients = ingredients_dto.iter().map(|v| v.clone().into()).collect();

        Ok(recipe)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

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
        let repo = ImplRecipeRepository::new(Arc::new(pool));

        let result = repo.find_all().await;

        assert!(result.is_ok());

        let data = result.unwrap();

        assert_eq!(data.len(), 0);
    }

    #[tokio::test]
    async fn insert_return_the_new_recipe() {
        let pool = create_pool().await;
        let repo = ImplRecipeRepository::new(Arc::new(pool));

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
        let pool = create_pool().await;
        let repo = ImplRecipeRepository::new(Arc::new(pool));

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
