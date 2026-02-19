use crate::error::{AppError, RecipeError};
use crate::models::recipe::{CreateRecipe, Recipe, UpdateRecipe};
use crate::repositories::recipe::RecipeRepository;

pub trait RecipeService {
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError>;
    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError>;
    async fn update_recipe(&self, data: UpdateRecipe) -> Result<Recipe, AppError>;
}

pub struct ImplRecipeService<Repo>
where
    Repo: RecipeRepository,
{
    repo: Repo,
}

impl<Repo> ImplRecipeService<Repo>
where
    Repo: RecipeRepository,
{
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl<Repo> RecipeService for ImplRecipeService<Repo>
where
    Repo: RecipeRepository,
{
    async fn list_recipes(&self) -> Result<Vec<Recipe>, AppError> {
        self.repo.find_all().await
    }

    async fn create_recipe(&self, data: CreateRecipe) -> Result<Recipe, AppError> {
        if data.name.len() == 0 {
            return Err(RecipeError::NameTooShort.into());
        } else if data.description.len() == 0 {
            return Err(RecipeError::DescriptionTooShort.into());
        }

        self.repo.create(data).await
    }

    async fn update_recipe(&self, data: UpdateRecipe) -> Result<Recipe, AppError> {
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
