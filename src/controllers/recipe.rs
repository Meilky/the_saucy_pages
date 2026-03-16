use std::sync::Arc;

use uuid::Uuid;

use crate::error::{AppError, RecipeError};
use crate::models::ingredient::Ingredient;
use crate::models::recipe::{CreateRecipe, Recipe};
use crate::repositories::ingredient::IngredientRepository;
use crate::repositories::recipe::RecipeRepository;

pub trait RecipeController: Send + Sync {
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError>;
    async fn find_recipe_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError>;
    async fn list_ingredients_for_recipe_by_uuid(
        &self,
        uuid: Uuid,
    ) -> Result<Vec<Ingredient>, AppError>;
    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError>;
}

#[derive(Debug, Clone)]
pub struct ImplRecipeController<RR: RecipeRepository, IR: IngredientRepository> {
    recipe_repo: Arc<RR>,
    ingredient_repo: Arc<IR>,
}

impl<RR: RecipeRepository, IR: IngredientRepository> ImplRecipeController<RR, IR> {
    pub fn new(recipe_repo: Arc<RR>, ingredient_repo: Arc<IR>) -> Self {
        Self {
            recipe_repo,
            ingredient_repo,
        }
    }
}

impl<RR: RecipeRepository, IR: IngredientRepository> RecipeController for ImplRecipeController<RR, IR> {
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError> {
        self.recipe_repo.find_all().await
    }

    async fn find_recipe_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError> {
        self.recipe_repo.find_by_uuid(uuid).await
    }

    async fn list_ingredients_for_recipe_by_uuid(
        &self,
        uuid: Uuid,
    ) -> Result<Vec<Ingredient>, AppError> {
        let recipe = self.find_recipe_by_uuid(uuid).await?;

        self.ingredient_repo.find_for_recipe(&recipe).await
    }

    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        if data.name.is_empty() {
            return Err(RecipeError::NameTooShort.into());
        }

        if data.description.is_empty() {
            return Err(RecipeError::DescriptionTooShort.into());
        }

        if data.ingredients.is_empty() {
            return Err(RecipeError::NotEnoughtIngredients.into());
        }

        if data.instructions.is_empty() {
            return Err(RecipeError::NotEnoughtInstructions.into());
        }

        self.recipe_repo.create(data).await
    }
}
