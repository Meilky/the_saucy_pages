use crate::error::{AppError, RecipeError};
use crate::models::recipe::{CreateRecipe, Recipe, UpdateRecipe};
use crate::repositories::recipe::RecipeRepository;

pub struct RecipeService {
    repo: RecipeRepository,
}

impl RecipeService {
    pub fn new(repo: RecipeRepository) -> Self {
        Self { repo }
    }

    pub async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError> {
        self.repo.find_all().await
    }

    pub async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        if data.name.len() == 0 {
            return Err(RecipeError::NameTooShort.into());
        } else if data.description.len() == 0 {
            return Err(RecipeError::DescriptionTooShort.into());
        }

        self.repo.create(data).await
    }

    pub async fn update_recipe(&self, data: UpdateRecipe) -> Result<Recipe, AppError> {
        if let Some(name) = data.name
            && name.len() == 0
        {
            return Err(RecipeError::NameTooShort.into());
        }

        if let Some(description) = data.description
            && description.len() == 0
        {
            return Err(RecipeError::DescriptionTooShort.into());
        }

        Err(AppError::InternalServerError)
    }
}
