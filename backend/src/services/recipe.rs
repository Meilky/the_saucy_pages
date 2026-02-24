use uuid::Uuid;

use crate::error::{AppError, RecipeError};
use crate::models::recipe::{CreateRecipe, Recipe};
use crate::repositories::recipe::RecipeRepository;

pub trait RecipeService: Send + Sync {
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError>;
    async fn find_recipe_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError>;
    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError>;
}

pub struct ImplRecipeService<Repo: RecipeRepository> {
    repo: Repo,
}

impl<Repo: RecipeRepository> ImplRecipeService<Repo> {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl<Repo: RecipeRepository> RecipeService for ImplRecipeService<Repo> {
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError> {
        self.repo.find_all().await
    }

    async fn find_recipe_by_uuid(&self, uuid: Uuid) -> Result<Recipe, AppError> {
        self.repo.find_by_uuid(uuid).await
    }

    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        if data.name.len() == 0 {
            return Err(RecipeError::NameTooShort.into());
        } else if data.description.len() == 0 {
            return Err(RecipeError::DescriptionTooShort.into());
        }

        self.repo.create(data).await
    }
}
