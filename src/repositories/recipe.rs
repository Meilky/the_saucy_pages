use std::sync::Arc;

use crate::{
    error::AppError,
    models::recipe::{CreateRecipe, Recipe, RecipeIngredient, RecipeInstruction},
};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

#[derive(sqlx::FromRow, Clone)]
struct RecipeIngredientDTO {
    pub uuid_ingredient: Uuid,
    pub amount: f32,
}

impl From<RecipeIngredientDTO> for RecipeIngredient {
    fn from(val: RecipeIngredientDTO) -> Self {
        RecipeIngredient {
            uuid: val.uuid_ingredient,
            amount: val.amount,
        }
    }
}

#[derive(sqlx::FromRow, Clone)]
struct RecipeInstructionDTO {
    pub uuid_instruction: Uuid,
    pub step_number: u16,
    pub description: String,
}

impl From<RecipeInstructionDTO> for RecipeInstruction {
    fn from(val: RecipeInstructionDTO) -> Self {
        RecipeInstruction {
            uuid: val.uuid_instruction,
            step_number: val.step_number,
            description: val.description,
        }
    }
}

pub trait RecipeRepository: Send + Sync {
    async fn create(&self, data: CreateRecipe) -> Result<Recipe, AppError>;
    async fn find_all(&self) -> Result<Vec<Recipe>, AppError>;
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError>;
}

#[derive(Debug, Clone)]
pub struct SQLiteRecipeRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl SQLiteRecipeRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl RecipeRepository for SQLiteRecipeRepository {
    async fn create(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
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
        .execute(&*self.pool)
        .await?;

        for ingredient in recipe.ingredients.iter() {
            sqlx::query(
                r#"
                INSERT INTO recipes_ingredients (uuid_recipe, uuid_ingredient, amount)
                VALUES (?, ?, ?);
                "#,
            )
            .bind(recipe.uuid)
            .bind(ingredient.uuid)
            .bind(ingredient.amount)
            .execute(&*self.pool)
            .await?;
        }

        for instruction in recipe.instructions.iter() {
            sqlx::query(
                r#"
                INSERT INTO recipes_instructions (uuid_recipe, uuid_instruction, step_number, description)
                VALUES (?, ?, ?, ?);
                "#,
            )
            .bind(recipe.uuid)
            .bind(instruction.uuid)
            .bind(instruction.step_number)
            .bind(instruction.description.clone())
            .execute(&*self.pool)
            .await?;
        }

        self.find_by_uuid(recipe.uuid).await
    }

    async fn find_all(&self) -> Result<Vec<Recipe>, AppError> {
        let mut recipes =
            sqlx::query_as::<_, Recipe>("SELECT uuid, name, description FROM recipes;")
                .fetch_all(&*self.pool)
                .await?;

        for recipe in recipes.iter_mut() {
            let ingredients_dto = sqlx::query_as::<_, RecipeIngredientDTO>(
                "SELECT uuid_ingredient, amount FROM recipes_ingredients WHERE uuid_recipe = ?;",
            )
            .bind(recipe.uuid)
            .fetch_all(&*self.pool)
            .await?;

            recipe.ingredients = ingredients_dto.iter().map(|v| v.clone().into()).collect();

            let instructions_dto = sqlx::query_as::<_, RecipeInstructionDTO>(
                "SELECT uuid_instruction, step_number, description FROM recipes_instructions WHERE uuid_recipe = ?;",
            )
            .bind(recipe.uuid)
            .fetch_all(&*self.pool)
            .await?;

            recipe.instructions = instructions_dto.iter().map(|v| v.clone().into()).collect();
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
            "SELECT uuid_ingredient, amount FROM recipes_ingredients WHERE uuid_recipe = ?;",
        )
        .bind(uuid)
        .fetch_all(&*self.pool)
        .await?;

        recipe.ingredients = ingredients_dto.iter().map(|v| v.clone().into()).collect();

        let instructions_dto = sqlx::query_as::<_, RecipeInstructionDTO>(
            "SELECT uuid_instruction, step_number, description FROM recipes_instructions WHERE uuid_recipe = ?;",
        )
        .bind(uuid)
        .fetch_all(&*self.pool)
        .await?;

        recipe.instructions = instructions_dto.iter().map(|v| v.clone().into()).collect();

        Ok(recipe)
    }
}
