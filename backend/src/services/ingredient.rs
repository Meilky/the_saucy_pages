use crate::error::{AppError, IngredientError};
use crate::models::ingredient::{CreateIngredient, Ingredient};
use crate::repositories::ingredient::IngredientRepository;

pub trait IngredientService: Send + Sync {
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError>;
    async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError>;
}

pub struct ImplIngredientService<Repo: IngredientRepository> {
    repo: Repo,
}

impl<Repo: IngredientRepository> ImplIngredientService<Repo> {
    pub fn new(repo: Repo) -> Self {
        Self { repo }
    }
}

impl<Repo: IngredientRepository> IngredientService for ImplIngredientService<Repo> {
    async fn list_ingredients(&self) -> Result<Vec<Ingredient>, AppError> {
        self.repo.find_all().await
    }

    async fn create_ingredient(&self, data: CreateIngredient) -> Result<Ingredient, AppError> {
        if data.name.len() == 0 {
            return Err(IngredientError::NameTooShort.into());
        } else if let Some(description) = data.description.clone()
            && description.len() == 0
        {
            return Err(IngredientError::DescriptionTooShort.into());
        }

        self.repo.create(data).await
    }
}
